use std::fmt::Alignment::Center;

use crate::services::colors::ColorsService;

use super::{background, bar, calc_widths};
use amane::{Align, Full, Justify, Rectangle, Row, Service, children};

pub fn battery_progress(id: &'static str, width: f32, height: f32) -> Row {
    let colors = ColorsService::read();
    let (bar_width, background_width, value) = calc_widths(id, width, 0.0);

    let bar = bar(bar_width, height)
        .radius_top_left(8.0)
        .radius_bottom_left(8.0);

    let mut background = background(background_width, height)
        .radius_top_right(8.0)
        .radius_bottom_right(8.0);

    let mut tip = Rectangle::new()
        .width(8.0)
        .height(16.0)
        .fill(colors.c.surface_container.value())
        .radius_top_right(Full)
        .radius_bottom_right(Full);

    if bar_width == 0.0 {
        background = background.radius_top_left(8.0).radius_bottom_left(8.0);
    }

    if value == 1.0 {
        tip = tip.fill(colors.c.primary_container.value());
    }

    Row::new(children!(
        Rectangle::new()
            .width(width)
            .height(height)
            .radius(8.0)
            .clip()
            .child(Row::new(children!(bar, background)).justify(Justify::SpaceBetween)),
        tip
    ))
    .align(Align::Center)
}
