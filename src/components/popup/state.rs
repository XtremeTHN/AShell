use super::menu::PopupMenuItem;
use amane::{Animation, Service};

pub struct PopupState {
    pub x: Animation<f32>,
    pub y: Animation<f32>,
    pub opacity: Animation<f32>,
    pub data: Vec<PopupMenuItem>,
    pub current_owner_id: Option<&'static str>,
    pub visible: bool, // logical state: open or closed
}

impl PopupState {
    pub fn toggle(owner_id: &'static str, data: Vec<PopupMenuItem>, x: f32, y: f32) {
        let mut this = Self::write();

        if this.visible && this.current_owner_id == Some(owner_id) {
            this.visible = false;
            this.current_owner_id = None;
            this.opacity.to(0.0); // fade out
            return;
        }

        if this.visible {
            this.x.to(x);
            this.y.to(y);
        } else {
            this.x = Animation::new(x);
            this.y = Animation::new(y);
            this.opacity = Animation::new(0.0);
            this.opacity.to(1.0);
        }

        this.current_owner_id = Some(owner_id);
        this.data = data;
        this.visible = true;
    }
}

impl Service for PopupState {
    fn new() -> Self {
        Self {
            x: Animation::new(0.0),
            y: Animation::new(0.0),
            opacity: Animation::new(0.0),
            data: vec![],
            current_owner_id: None,
            visible: false,
        }
    }

    fn listen() {}
}
