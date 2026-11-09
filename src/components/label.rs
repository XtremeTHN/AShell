use amane::{Service, Text};
use crate::services::colors::ColorsService;

pub fn label(content: impl Into<String>) -> Text {
    let colors = ColorsService::read();

    Text::new(content)
        .color(colors.c.on_background.as_str().into())
        .size(12.0)
        .weight(470)
}