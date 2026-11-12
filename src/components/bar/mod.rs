use crate::components::button::to_button;
use crate::components::statusicons::temps::temperatures;
use crate::services::{clock::Clock, colors::ColorsService};

use super::label::label;
use super::mrow::{ROW_PADDING, row};
use super::statusicons::all::all_status;
use amane::Align::{self, End};
use amane::{
    Full, Justify, Layer, LayerWindow, Padding, Rectangle, Row, Service, Size::Parent,
    Vertical, children,
};

mod active_window;
pub mod bar_container;
mod workspaces;

use bar_container::bar_container;

fn left() -> Row {
    row(
        vec![workspaces::workspaces(), active_window::active_window()],
        ROW_PADDING,
    )
}

fn date() -> Rectangle {
    let clock = Clock::read();

    bar_container(label(clock.date.as_str()), 30.0).align_child(End, End)
}

fn clock() -> Rectangle {
    let clock = Clock::read();
    let lbl = label(clock.time.as_str());
    let status = all_status();

    let row = Row::new(children!(lbl, status));
    bar_container(row, 30.0)
}

fn right() -> Row {
    let btt = to_button(clock(), "clock");
    row(vec![temperatures(), date(), btt], ROW_PADDING)
}

fn content() -> Rectangle {
    let background_color = ColorsService::read().c.background.clone();

    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .fill(background_color.as_str())
        .padding(Padding {
            left: 10.0,
            right: 10.0,
            top: 10.0,
            bottom: 10.0,
            // ..Default::default()
        })
        .align_child(Align::Center, Align::Center)
        .child(
            Row::new(children!(left(), right()))
                .width(Parent)
                .justify(Justify::SpaceBetween),
        )
}

pub fn view() -> LayerWindow {
    LayerWindow::new()
        .width(Full)
        .height(45.0)
        .anchor_vertical(Vertical::Top)
        .layer(Layer::Top)
        .space(amane::Zone::Reserve)
        .child(content())
}
