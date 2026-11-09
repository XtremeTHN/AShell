use amane::{Align::Center, Rectangle, Service, Size::Parent, Text};
use crate::{components::{mrow, label::label}, services::compositor::CompositorService};

pub fn active_window() -> Rectangle {
    let compositor = CompositorService::read();
    let active = compositor.active_toplevel.as_ref()
        .and_then(|t| Some(t.title.clone()))
        .unwrap_or_else(|| String::from("ArchLinux"));

    let text = label(active);

    let size = mrow::width(&text);
    Rectangle::new()
        .width(size)
        .height(30.0)
        .align_child(Center, Center)
        .child(text)
}