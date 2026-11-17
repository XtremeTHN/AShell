use std::{
    env,
    io::{self, BufRead, BufReader, Read, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
};

use serde_json::Value;

use super::{ActiveToplevel, ActiveWorkspace, Backend, Client, CompositorState};

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

            if updates_state(&event) {
                publish_state(on_change);
            }
        }
    }
}

fn updates_state(event: &str) -> bool {
    matches!(
        event.split_once(">>").map(|(name, _)| name),
        Some(
            "activewindow"
                | "activewindowv2"
                | "workspace"
                | "workspacev2"
                | "focusedmon"
                | "windowtitle"
                | "windowtitlev2"
                | "closewindow"
                | "openwindow"
                | "movewindow"
                | "movewindowv2"
                | "changefloatingmode"
                | "fullscreen"
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
    let state = match (
        request("j/activewindow"),
        request("j/activeworkspace"),
        request("j/clients"),
    ) {
        (Ok(toplevel), Ok(workspace), Ok(clients)) => parse_state(&toplevel, &workspace, &clients),
        (Err(error), _, _) | (_, Err(error), _) | (_, _, Err(error)) => Err(error.to_string()),
    };

    match state {
        Ok(state) => on_change(state),
        Err(error) => log::error!("Failed to read Hyprland state: {error}"),
    }
}

fn parse_clients(json: &str) -> Result<Vec<Client>, String> {
    let value: Value =
        serde_json::from_str(json).map_err(|error| format!("invalid clients JSON: {error}"))?;
    let list = value
        .as_array()
        .ok_or_else(|| "clients reply is not an array".to_string())?;

    let mut clients = list
        .iter()
        .filter(|c| c["mapped"].as_bool().unwrap_or(true))
        .map(parse_client)
        .collect::<Result<Vec<_>, _>>()?;

    clients.sort_by(|a, b| a.address.cmp(&b.address));
    Ok(clients)
}

fn parse_client(client: &Value) -> Result<Client, String> {
    let (x, y) = pair(client, "at")?;
    let (width, height) = pair(client, "size")?;

    Ok(Client {
        address: required_string(client, "address", "client")?,
        app_id: required_string(client, "class", "client")?,
        title: required_string(client, "title", "client")?,
        stable_id: required_string(client, "stableId", "client")?,
        workspace_id: client["workspace"]["id"].as_i64().map(|i| i.to_string()),
        x: x as i32,
        y: y as i32,
        width: width.max(0) as u32,
        height: height.max(0) as u32,
        floating: client["floating"].as_bool().unwrap_or(false),
        hidden: client["hidden"].as_bool().unwrap_or(false),
    })
}

fn pair(value: &Value, key: &str) -> Result<(i64, i64), String> {
    let array = value[key]
        .as_array()
        .ok_or_else(|| format!("client has no array {key}"))?;
    match (
        array.first().and_then(Value::as_i64),
        array.get(1).and_then(Value::as_i64),
    ) {
        (Some(a), Some(b)) => Ok((a, b)),
        _ => Err(format!("client {key} is not two integers")),
    }
}

fn parse_state(toplevel: &str, workspace: &str, clients: &str) -> Result<CompositorState, String> {
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
        clients: parse_clients(clients)?,
    })
}

fn required_string(value: &Value, key: &str, object: &str) -> Result<String, String> {
    value[key]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("{object} has no string {key}"))
}
