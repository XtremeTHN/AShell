pub mod region;
pub mod state;

use crate::{
    components::{
        Widgets,
        button::{ButtonColors, circular_button, state::ButtonStates},
        icon::glyph_slot,
        mrow::width,
        separator::{Orientation, separator},
    },
    services::colors::ColorsService,
};
use amane::{
    Align, Color, InputArea, Layer, LayerWindow, Monitor, Radius::Full, Rectangle, Row, Service,
    Size, Stack, Zone, children,
};

const TOOLBAR_WIDTH: f32 = 500.0;
const TOOLBAR_HEIGHT: f32 = 50.0;
const TOOLBAR_MARGIN: f32 = 80.0;
const ICON_SIZE: f32 = 16.0;
const BUTTON_SIZE: f32 = 30.0;

fn button_colors(id: &str) -> ButtonColors {
    let colors = ColorsService::read();
    ButtonColors::toggleable(
        colors.c.background.value(),
        colors.c.on_background.value(),
        &*ButtonStates::get_active_button_in_group("screenshot-toolbar") == id,
    )
}

fn group_click(id: &str, mode: state::Mode) {
    ButtonStates::set_active_button_in_group("screenshot-toolbar", id);
    state::ScreenshotState::write().mode = mode;
}

fn mode_button(id: &'static str, icon: &'static str, mode: state::Mode) -> Rectangle {
    circular_button(
        id,
        glyph_slot(icon, ICON_SIZE, None),
        BUTTON_SIZE,
        Some(button_colors(id)),
        Some(move || group_click(id, mode)),
    )
}

fn action_button(id: &'static str, icon: &'static str) -> Rectangle {
    circular_button(
        id,
        glyph_slot(icon, ICON_SIZE, None),
        BUTTON_SIZE,
        None,
        Some(|| {}),
    )
}

fn toolbar_content() -> Row {
    ButtonStates::new_button_group("screenshot-toolbar", "fullscreen");

    Row::new(children!(
        mode_button("fullscreen", "fullscreen", state::Mode::Fullscreen),
        mode_button("region", "highlight_alt", state::Mode::Region),
        mode_button("window", "select_window_2", state::Mode::Window),
        separator(Orientation::Vertical, 15.0),
        action_button("screenshot-toolbar-close", "close"),
        action_button("screenshot-toolbar-done", "check"),
    ))
    .gap(10.0)
    .align(Align::Center)
}

fn toolbar(mon_width: f32, mon_height: f32) -> (Rectangle, InputArea) {
    let c = ColorsService::read();
    let child = toolbar_content();

    let Size::Fixed(_width) = width(&child) else {
        unreachable!();
    };

    let x = (mon_width / 2.0) - _width / 2.0;
    let y = TOOLBAR_MARGIN;

    (
        Rectangle::new()
            .width(_width)
            .height(TOOLBAR_HEIGHT)
            .radius(Full)
            .fill(c.c.background.value())
            .translate(x, y)
            .child(child)
            .align_child(Align::Center, Align::Center),
        InputArea {
            x: x as i32,
            y: y as i32,
            width: _width as i32,
            height: TOOLBAR_HEIGHT as i32,
        },
    )
}

fn content(mon_width: f32, mon_height: f32) -> (Stack, InputArea) {
    let background = Rectangle::new()
        .width(1920.0)
        .height(1080.0)
        .opacity(0.4)
        .fill(Color::BLACK)
        .on_press(region::on_pressed)
        .on_drag(region::on_drag);

    let mode = state::ScreenshotState::read().mode;
    let mut children: Widgets = vec![Box::new(background)];

    match mode {
        state::Mode::Region => {
            children.push(Box::new(region::selector()));
        }
        _ => {
            unreachable!()
        }
    }

    let (rect, area) = toolbar(mon_width, mon_height);
    children.push(Box::new(rect));

    (Stack::new(children), area)
}

pub fn view(mon: &Monitor) -> LayerWindow {
    let (child, area) = content(mon.width as f32, mon.height as f32);
    LayerWindow::new()
        .width(1920.0)
        .height(1080.0)
        .space(Zone::Ignore)
        .anchor_horizontal(amane::Horizontal::Left)
        .anchor_vertical(amane::Vertical::Top)
        .layer(Layer::Overlay)
        .child(child)
}
