use std::{
    env,
    io::{self, BufRead, BufReader, Read, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
};

use serde_json::Value;

use super::{ActiveToplevel, ActiveWorkspace, Backend, CompositorState};

pub(super) struct Hyprland;

impl Backend for Hyprland {
    fn listen(&self, on_change: &mut dyn FnMut(CompositorState)) -> io::Result<()> {
        let event_path = socket(".socket2.sock")?;
        let stream = UnixStream::connect(event_path)?;
        let mut events = BufReader::new(stream);

        publish_state(on_change);

        let mut event = String::new();
        loop {
            event.clear();
            if events.read_line(&mut event)? == 0 {
                return Ok(());
            }

            if updates_active_state(&event) {
                publish_state(on_change);
            }
        }
    }
}

fn updates_active_state(event: &str) -> bool {
    matches!(
        event.split_once(">>").map(|(name, _)| name),
        Some(
            "activewindow"
                | "activewindowv2"
                | "workspace"
                | "workspacev2"
                | "focusedmon"
                | "windowtitle"
                | "closewindow"
                | "openwindow"
        )
    )
}

fn socket(name: &str) -> io::Result<PathBuf> {
    let runtime = env::var_os("XDG_RUNTIME_DIR")
        .ok_or_else(|| io::Error::other("XDG_RUNTIME_DIR is not set"))?;
    let instance = env::var_os("HYPRLAND_INSTANCE_SIGNATURE")
        .ok_or_else(|| io::Error::other("HYPRLAND_INSTANCE_SIGNATURE is not set"))?;

    Ok(PathBuf::from(runtime)
        .join("hypr")
        .join(instance)
        .join(name))
}

fn request(command: &str) -> io::Result<String> {
    let mut stream = UnixStream::connect(socket(".socket.sock")?)?;
    stream.write_all(command.as_bytes())?;

    let mut reply = String::new();
    stream.read_to_string(&mut reply)?;

    Ok(reply)
}

fn publish_state(on_change: &mut dyn FnMut(CompositorState)) {
    let state = match (request("j/activewindow"), request("j/activeworkspace")) {
        (Ok(toplevel), Ok(workspace)) => parse_state(&toplevel, &workspace),
        (Err(error), _) | (_, Err(error)) => Err(error.to_string()),
    };

    match state {
        Ok(state) => on_change(state),
        Err(error) => log::error!("Failed to read Hyprland active state: {error}"),
    }
}

fn parse_state(toplevel: &str, workspace: &str) -> Result<CompositorState, String> {
    let toplevel: Value = serde_json::from_str(toplevel)
        .map_err(|error| format!("invalid active-window JSON: {error}"))?;
    let workspace: Value = serde_json::from_str(workspace)
        .map_err(|error| format!("invalid active-workspace JSON: {error}"))?;

    let active_toplevel = if toplevel["address"]
        .as_str()
        .is_some_and(|address| !address.is_empty())
    {
        Some(ActiveToplevel {
            app_id: required_string(&toplevel, "class", "active window")?,
            title: required_string(&toplevel, "title", "active window")?,
        })
    } else {
        None
    };

    let id = workspace["id"]
        .as_i64()
        .ok_or_else(|| "active workspace has no integer id".to_string())?
        .to_string();
    let name = required_string(&workspace, "name", "active workspace")?;

    Ok(CompositorState {
        active_toplevel,
        active_workspace: Some(ActiveWorkspace {
            id,
            name,
            output: workspace["monitor"].as_str().map(str::to_string),
        }),
    })
}

fn required_string(value: &Value, key: &str, object: &str) -> Result<String, String> {
    value[key]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("{object} has no string {key}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_refreshes_for_active_state_events() {
        assert!(updates_active_state("activewindowv2>>0x123\n"));
        assert!(updates_active_state("workspacev2>>2,work\n"));
        assert!(!updates_active_state("movewindow>>0x123,1\n"));
    }

    #[test]
    fn parses_hyprland_active_state() {
        let state = parse_state(
            r#"{"address":"0x123","class":"org.example.App","title":"Example"}"#,
            r#"{"id":4,"name":"work","monitor":"DP-1"}"#,
        )
        .expect("valid active state should parse");

        assert_eq!(
            state.active_toplevel,
            Some(ActiveToplevel {
                app_id: "org.example.App".to_string(),
                title: "Example".to_string(),
            })
        );
        assert_eq!(
            state.active_workspace,
            Some(ActiveWorkspace {
                id: "4".to_string(),
                name: "work".to_string(),
                output: Some("DP-1".to_string()),
            })
        );
    }

    #[test]
    fn parses_no_active_toplevel() {
        let state = parse_state("{}", r#"{"id":1,"name":"1","monitor":"DP-1"}"#)
            .expect("valid workspace with no active toplevel should parse");

        assert_eq!(state.active_toplevel, None);
        assert_eq!(
            state
                .active_workspace
                .as_ref()
                .map(|workspace| workspace.name.as_str()),
            Some("1")
        );
    }
}
