use crate::services::colors::ColorsService;
use amane::{Rectangle, Service};

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub fn separator(orientation: Orientation, size: f32) -> Rectangle {
    let colors = ColorsService::read();
    let (width, height) = match orientation {
        Orientation::Vertical => (1.0, size),
        Orientation::Horizontal => (size, 1.0),
    };

    Rectangle::new()
        .width(width)
        .height(height)
        .fill(colors.c.outline_variant.value())
}
