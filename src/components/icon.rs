use crate::services::colors::ColorsService;
use amane::{Align, Rectangle, Service, Text};

pub fn glyph_slot(icon_name: impl Into<String>, size: f32, icon_size: Option<f32>) -> Rectangle {
    Rectangle::new()
        .width(size)
        .height(size)
        .align_child(Align::Center, Align::Center)
        .child(glyph(icon_name).size(icon_size.unwrap_or(size)))
}

pub fn glyph(icon_name: impl Into<String>) -> Text {
    let colors = ColorsService::read();
    Text::new(icon_name)
        .font("Material Symbols Rounded")
        .tight()
        .color(colors.c.on_background.value())
}
