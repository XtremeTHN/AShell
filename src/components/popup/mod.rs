use std::collections::HashMap;

use amane::{Layer, LayerWindow, Padding, Rectangle, Row, Service, Size::Parent, Widget, children};

use crate::components::{
    button::to_button_with_callback,
    icon::glyph,
    label::label,
    mrow::{height, width},
};

#[derive(Default, Clone)]
pub struct PopupMenuItem {
    owner: String,
    icon_name: Option<String>,
    label: String,
}

#[derive(Default)]
pub struct PopupState {
    pub x: f32,
    pub y: f32,
    pub data: Vec<PopupMenuItem>,
    pub visible: bool,
}

impl PopupState {
    pub fn change_content(data: Vec<PopupMenuItem>, x: f32, y: f32) {
        let mut this = Self::write();
        this.data = data;
        this.x = x;
        this.y = y;
    }
}

impl Service for PopupState {
    fn new() -> Self {
        Self::default()
    }

    fn listen() {}
}

fn popup_button(item: PopupMenuItem) -> Rectangle {
    let mut children: Vec<Box<dyn Widget>> = vec![];
    if let Some(icon_name) = item.icon_name {
        children.push(Box::new(glyph(icon_name)));
    }
    children.push(Box::new(label(item.label)));

    let row = Row::new(children);
    let rect = Rectangle::new()
        .width(Parent)
        .height(50.0)
        .child(row)
        .padding(10.0);

    to_button_with_callback("", rectangle, colors, on_clicked)
}

pub fn popup() -> Rectangle {
    let state = PopupState::read();
    let children = children!();

    for x in state.data {}

    let row = Row::new();
    Rectangle::new()
        .width(_width)
        .height(_height)
        .padding(padding)
        .radius(8.0)
        .child(row)
}

pub fn view() -> LayerWindow {
    LayerWindow::new()
        .width(1920.0)
        .height(1080.0)
        .layer(Layer::Overlay)
        .click_through()
        .child(popup())
    // .child(cont)
}
