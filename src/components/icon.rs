use crate::services::colors::ColorsService;
use amane::{Service, Text};

pub fn glyph(icon_name: impl Into<String>) -> Text {
    let colors = ColorsService::read();
    Text::new(icon_name)
        .font("Material Symbols Rounded")
        .tight()
        .color(colors.c.on_background.value())
}
