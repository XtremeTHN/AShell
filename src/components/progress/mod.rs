use amane::{Justify, Rectangle, Row, Service, children};

use crate::services::colors::ColorsService;

pub mod battery;
pub mod state;

const BARS_GAP: f32 = 5.0;

fn bar(width: f32, height: f32) -> Rectangle {
    let colors = ColorsService::read();
    Rectangle::new()
        .width(width)
        .height(height)
        .fill(colors.c.primary_container.value())
}

fn background(width: f32, height: f32) -> Rectangle {
    let colors = ColorsService::read();

    Rectangle::new()
        .width(width)
        .height(height)
        .fill(colors.c.surface_container.value())
}

fn calc_widths(id: &'static str, width: f32, gap: f32) -> (f32, f32, f32) {
    let state = state::ProgressStates::get_state(id);

    let bar_width = state.value * width;

    let background_width = if bar_width == 0.0 {
        width
    } else {
        width - bar_width - gap
    };

    (bar_width, background_width, state.value)
}

pub fn progress(id: &'static str, width: f32, height: f32) -> Row {
    let (bar_width, background_width, _) = calc_widths(id, width, BARS_GAP);

    let bar = bar(bar_width, height);
    let background = background(background_width, height);

    Row::new(children!(bar, background))
        .justify(Justify::SpaceBetween)
        .gap(5.0)
}
