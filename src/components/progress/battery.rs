use crate::components::label::label;

use super::{ProgressColors, background, bar, calc_widths};
use amane::{Align, Justify, Radius, Rectangle, Row, Stack, children};

pub fn battery_progress(
    id: &'static str,
    colors: Option<ProgressColors>,
    width: f32,
    height: f32,
    radius: impl Into<Radius>,
    text_size: f32,
) -> Row {
    let _colors = colors.unwrap_or_default();
    let (bar_width, background_width, value) = calc_widths(id, width, 0.0);
    let _radius = radius.into();

    let bar = bar(_colors.foreground, bar_width, height)
        .radius_top_left(_radius)
        .radius_bottom_left(_radius);

    let mut background = background(_colors.background, background_width, height)
        .radius_top_right(_radius)
        .radius_bottom_right(_radius);

    let mut tip = Rectangle::new()
        .width(height * 0.2)
        .height(height * 0.5)
        .fill(_colors.background)
        .radius(_radius);

    let percentage_color = if value < 0.45 {
        _colors.text_behind
    } else {
        _colors.text_over
    };

    let percentage = label((value * 100.0).ceil().to_string())
        .color(percentage_color)
        .size(text_size);

    let percentage_rect = Rectangle::new()
        .width(width)
        .height(height)
        .child(percentage)
        .align_child(Align::Center, Align::Center);

    if bar_width == 0.0 {
        background = background
            .radius_top_left(_radius)
            .radius_bottom_left(_radius);
    }

    if value == 1.0 {
        tip = tip.fill(_colors.foreground);
    }

    let row = Row::new(children!(bar, background)).justify(Justify::SpaceBetween);
    let stack = Stack::new(children!(row, percentage_rect));

    Row::new(children!(
        Rectangle::new()
            .width(width)
            .height(height)
            .radius(_radius)
            .clip()
            .child(stack),
        tip
    ))
    .align(Align::Center)
    .gap(1.0)
}
