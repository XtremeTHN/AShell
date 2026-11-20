// thanks claude
use std::{
    env,
    io::{self, BufRead, BufReader, Read, Write},
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::Duration,
};

use serde_json::Value;

use super::{ActiveToplevel, ActiveWorkspace, Backend, Client, CompositorState};

pub(super) struct Mango;

const DEBOUNCE: Duration = Duration::from_millis(15);

impl Backend for Mango {
    fn listen(&self, on_change: &mut dyn FnMut(CompositorState)) -> io::Result<()> {
        let path = socket_path()?;
        let (tx, rx) = mpsc::channel::<()>();

        for topic in ["all-clients", "all-monitors"] {
            let (path, tx) = (path.clone(), tx.clone());
            thread::spawn(move || {
                if let Err(error) = watch(&path, topic, &tx) {
                    log::error!("Mango watch {topic} stopped: {error}");
                }
            });
        }
        drop(tx);

        publish_state(&path, on_change);

        while rx.recv().is_ok() {
            while rx.recv_timeout(DEBOUNCE).is_ok() {}
            publish_state(&path, on_change);
        }

        Ok(())
    }
}

fn socket_path() -> io::Result<PathBuf> {
    env::var_os("MANGO_INSTANCE_SIGNATURE")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::other("MANGO_INSTANCE_SIGNATURE is not set"))
}

fn watch(path: &Path, topic: &str, tx: &mpsc::Sender<()>) -> io::Result<()> {
    let mut stream = UnixStream::connect(path)?;
    stream.write_all(format!("watch {topic}\n").as_bytes())?;

    for line in BufReader::new(stream).lines() {
        line?;
        if tx.send(()).is_err() {
            break;
        }
    }

    Ok(())
}

fn request(path: &Path, command: &str) -> io::Result<String> {
    let mut stream = UnixStream::connect(path)?;
    stream.write_all(command.as_bytes())?;
    stream.write_all(b"\n")?;

    let mut reply = String::new();
    stream.read_to_string(&mut reply)?;

    Ok(reply)
}

fn publish_state(path: &Path, on_change: &mut dyn FnMut(CompositorState)) {
    let state = request(path, "get all-clients")
        .and_then(|clients| request(path, "get all-monitors").map(|monitors| (clients, monitors)))
        .map_err(|error| error.to_string())
        .and_then(|(clients, monitors)| parse_state(&clients, &monitors));

    match state {
        Ok(state) => on_change(state),
        Err(error) => log::error!("Failed to read Mango state: {error}"),
    }
}

fn parse_state(clients: &str, monitors: &str) -> Result<CompositorState, String> {
    let clients: Value = serde_json::from_str(clients)
        .map_err(|error| format!("invalid all-clients JSON: {error}"))?;
    let monitors: Value = serde_json::from_str(monitors)
        .map_err(|error| format!("invalid all-monitors JSON: {error}"))?;

    let list = clients["clients"]
        .as_array()
        .ok_or_else(|| format!("all-clients reply has no clients array: {clients}"))?;
    let monitors = monitors["monitors"]
        .as_array()
        .ok_or_else(|| format!("all-monitors reply has no monitors array: {monitors}"))?;

    let mut parsed: Vec<(u64, Client)> = list
        .iter()
        .filter_map(|client| match parse_client(client) {
            Ok(parsed) => Some(parsed),
            Err(error) => {
                log::warn!("Skipping malformed Mango client: {error}");
                None
            }
        })
        .collect();

    parsed.sort_by_key(|(id, _)| *id);
    let clients = parsed.into_iter().map(|(_, client)| client).collect();

    let focused = monitors
        .iter()
        .find(|monitor| monitor["active"].as_bool().unwrap_or(false))
        .or_else(|| monitors.first());

    Ok(CompositorState {
        active_toplevel: focused.and_then(active_toplevel),
        active_workspace: focused.and_then(active_workspace),
        clients,
    })
}

fn parse_client(client: &Value) -> Result<(u64, Client), String> {
    let id = client["id"]
        .as_u64()
        .ok_or_else(|| "client has no integer id".to_string())?;

    Ok((
        id,
        Client {
            address: id.to_string(),
            app_id: string(client, "appid")?,
            title: string(client, "title")?,
            stable_id: string(client, "foreign_toplevel_id")?,
            workspace_id: client["tags"]
                .get(0)
                .and_then(Value::as_u64)
                .map(|tag| tag.to_string()),
            x: int(client, "x")? as i32,
            y: int(client, "y")? as i32,
            width: int(client, "width")?.max(0) as u32,
            height: int(client, "height")?.max(0) as u32,
            floating: client["is_floating"].as_bool().unwrap_or(false),
            hidden: client["is_minimized"].as_bool().unwrap_or(false),
        },
    ))
}

fn active_toplevel(monitor: &Value) -> Option<ActiveToplevel> {
    let client = &monitor["active_client"];

    Some(ActiveToplevel {
        app_id: client["appid"].as_str()?.to_string(),
        title: client["title"].as_str()?.to_string(),
    })
}

// Mango tags have an index but no names, so the index doubles as the name.
// With several tags active at once, the first one is reported.
fn active_workspace(monitor: &Value) -> Option<ActiveWorkspace> {
    let tag = monitor["active_tags"].get(0)?.as_u64()?.to_string();

    Some(ActiveWorkspace {
        id: tag.clone(),
        name: tag,
        output: monitor["name"].as_str().map(str::to_string),
    })
}

fn string(value: &Value, key: &str) -> Result<String, String> {
    value[key]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("client has no string {key}"))
}

fn int(value: &Value, key: &str) -> Result<i64, String> {
    value[key]
        .as_i64()
        .ok_or_else(|| format!("client has no integer {key}"))
}
