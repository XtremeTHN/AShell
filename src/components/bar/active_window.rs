use crate::{
    components::{bar::bar_container::bar_container, label::label},
    services::compositor::CompositorService,
};
use amane::{Align::Center, Rectangle, Service};

pub fn active_window() -> Rectangle {
    let compositor = CompositorService::read();
    let active = compositor
        .active_toplevel
        .as_ref()
        .and_then(|t| Some(t.title.clone()))
        .unwrap_or_else(|| String::from("ArchLinux"));

    let text = label(active);

    bar_container(text, 30.0).align_child(Center, Center)
}
