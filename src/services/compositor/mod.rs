mod hyprland;
mod mango;

use std::env;

use amane::Service;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client {
    pub address: String,
    pub app_id: String,
    pub title: String,
    pub workspace_id: Option<String>,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub floating: bool,
    pub hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveToplevel {
    pub app_id: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveWorkspace {
    pub id: String,
    pub name: String,
    pub output: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct CompositorState {
    active_toplevel: Option<ActiveToplevel>,
    active_workspace: Option<ActiveWorkspace>,
    clients: Vec<Client>,
}

trait Backend {
    fn listen(&self, on_change: &mut dyn FnMut(CompositorState)) -> std::io::Result<()>;
}

fn backend() -> Option<Box<dyn Backend>> {
    if env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_some() {
        return Some(Box::new(hyprland::Hyprland));
    }

    if env::var_os("MANGO_INSTANCE_SIGNATURE").is_some() {
        return Some(Box::new(mango::Mango));
    }

    None
}

#[derive(Debug, Default)]
pub struct CompositorService {
    pub active_toplevel: Option<ActiveToplevel>,
    pub active_workspace: Option<ActiveWorkspace>,
    pub clients: Vec<Client>,
}

impl Service for CompositorService {
    fn new() -> Self {
        Self::default()
    }

    fn listen() {
        let Some(backend) = backend() else {
            log::warn!("No supported compositor found for CompositorService");
            return;
        };

        let result = backend.listen(&mut |state| {
            let changed = {
                let service = Self::read();
                service.active_toplevel != state.active_toplevel
                    || service.active_workspace != state.active_workspace
                    || service.clients != state.clients
            };
            if !changed {
                return;
            }

            let mut service = Self::write();
            service.active_toplevel = state.active_toplevel;
            service.active_workspace = state.active_workspace;
            service.clients = state.clients;
        });

        if let Err(error) = result {
            log::error!("Compositor listener stopped: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_models_are_compositor_independent() {
        let toplevel = ActiveToplevel {
            app_id: "org.example.App".to_string(),
            title: "Example".to_string(),
        };
        let workspace = ActiveWorkspace {
            id: "4".to_string(),
            name: "work".to_string(),
            output: Some("DP-1".to_string()),
        };

        assert_eq!(toplevel.app_id, "org.example.App");
        assert_eq!(workspace.name, "work");
    }
}
