pub mod state;

use amane::{Align, Radius::Full, Rectangle, Row, Service, Stack, children};

use crate::{components::icon::glyph, services::colors::ColorsService};

const HANDLE_WIDTH: f32 = 3.0;
const HANDLE_HEIGHT: f32 = 6.0;
const SLIDER_LEFT_RADIUS: f32 = 8.0;
const SLIDER_RIGHT_RADIUS: f32 = 1.0;
const HANDLE_GAP: f32 = 4.0;

fn handle(base_height: f32, id: &'static str) -> Rectangle {
    let colors = ColorsService::read();
    let state = {
        let s = state::SliderStates::read();
        s.states.get(id).cloned().unwrap_or_default()
    };

    let fill = if state.pressed {
        colors.c.primary_container.as_str()
    } else {
        colors.c.primary.as_str()
    };

    Rectangle::new()
        .width(HANDLE_WIDTH)
        .height(base_height + HANDLE_HEIGHT)
        .fill(fill)
        .radius(Full)
}

fn bar(width: f32, height: f32, id: &'static str) -> Row {
    let colors = ColorsService::read();
    let state = {
        let s = state::SliderStates::read();
        s.states.get(id).cloned().unwrap_or_default()
    };

    let travel = width - HANDLE_WIDTH;
    let handle_x = state.value * travel;

    let fill_w = (handle_x - HANDLE_GAP).max(0.0);
    let gap = handle_x - fill_w;

    let rect = Rectangle::new()
        .width(fill_w)
        .height(height)
        .fill(colors.c.primary.as_str())
        .radius_top_left(SLIDER_LEFT_RADIUS)
        .radius_bottom_left(SLIDER_LEFT_RADIUS)
        .radius_top_right(SLIDER_RIGHT_RADIUS)
        .radius_bottom_right(SLIDER_RIGHT_RADIUS);

    Row::new(children!(rect, handle(height, id)))
        .gap(gap)
        .align(Align::Center)
}

fn change_value(id: &'static str, cb: impl Fn(&mut state::SliderState)) {
    let mut s = state::SliderStates::write();
    let b = s.states.entry(id).or_default();

    cb(b)
}

fn background(foreground: Row, width: f32, height: f32, id: &'static str) -> Rectangle {
    let colors = ColorsService::read();

    let icon = Rectangle::new()
        .width(24.0)
        .height(height)
        .child(glyph("volume_up").size(16.0))
        .align_child(Align::Center, Align::Center)
        .translate(4.0, HANDLE_HEIGHT / 2.0);

    Rectangle::new()
        .width(width)
        .height(height)
        .fill(colors.c.surface_container.as_str())
        .child(Stack::new(vec![Box::new(foreground), Box::new(icon)]))
        .radius(SLIDER_LEFT_RADIUS)
        .align_child(Align::Start, Align::Center)
        .on_press(move |_| {
            change_value(id, |s| s.pressed = true);
        })
        .on_release(move |_| {
            change_value(id, |s| s.pressed = false);
        })
        .on_drag(move |point| {
            let travel = width - HANDLE_WIDTH;
            let v = ((point.x - HANDLE_WIDTH / 2.0) / travel).clamp(0.0, 1.0);

            change_value(id, |s| s.value = v);
        })
}

pub fn slider(width: f32, height: f32, id: &'static str) -> Rectangle {
    background(bar(width, height, id), width, height, id)
}
