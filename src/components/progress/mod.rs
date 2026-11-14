use amane::{Color, Justify, Rectangle, Row, Service, Size, children};

use crate::services::colors::ColorsService;

pub mod battery;
pub mod state;

const BARS_GAP: f32 = 5.0;

pub struct ProgressColors {
    pub background: Color,
    pub foreground: Color,
    pub text_over: Color,
    pub text_behind: Color,
}

impl Default for ProgressColors {
    fn default() -> Self {
        let colors = ColorsService::read();
        Self {
            background: colors.c.surface_container.value(),
            foreground: colors.c.primary_container.value(),
            text_over: colors.c.on_primary_container.value(),
            text_behind: colors.c.on_surface.value(),
        }
    }
}

fn bar(background: Color, width: f32, height: impl Into<Size>) -> Rectangle {
    Rectangle::new()
        .width(width)
        .height(height)
        .fill(background)
}

fn background(foreground: Color, width: f32, height: impl Into<Size>) -> Rectangle {
    Rectangle::new()
        .width(width)
        .height(height)
        .fill(foreground)
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

pub fn progress(id: &'static str, colors: Option<ProgressColors>, width: f32, height: f32) -> Row {
    let (bar_width, background_width, _) = calc_widths(id, width, BARS_GAP);
    let _colors = colors.unwrap_or_default();

    let bar = bar(_colors.background, bar_width, height);
    let background = background(_colors.foreground, background_width, height);

    Row::new(children!(bar, background))
        .justify(Justify::SpaceBetween)
        .gap(5.0)
}
