use crate::services::colors::ColorsService;
use amane::{Color, Service, Text, Weight};

pub fn label_dimmed(content: impl Into<String>, color: Option<Color>) -> Text {
    let colors = ColorsService::read();
    let mut t: amane::Color = color.unwrap_or(colors.c.on_background.value());
    t.a = 150;

    Text::new(content).color(t).size(12.0).weight(470)
}

pub fn label(content: impl Into<String>) -> Text {
    let colors = ColorsService::read();

    Text::new(content)
        .color(colors.c.on_background.value())
        .size(12.0)
        .weight(Weight::Medium)
}
