use crate::components::button::ButtonBuilder;
use crate::components::statusicons::temps::temperatures;
use crate::services::{clock::Clock, colors::ColorsService};

use super::label::label;
use super::mrow::{ROW_PADDING, row, wrap};
use super::statusicons::all::all_status;
use amane::Align::{self, Center, End};
use amane::{
    Full, Justify, Layer, LayerWindow, Padding, Rectangle, Row, Service, Size::Parent, Text,
    Vertical, children,
};

mod active_window;
mod workspaces;

fn left() -> Row {
    row(
        vec![workspaces::workspaces(), active_window::active_window()],
        ROW_PADDING,
    )
}

fn date() -> Rectangle {
    let clock = Clock::read();

    wrap(label(clock.date.as_str()), 30.0).align_child(End, End)
}

fn clock() -> Rectangle {
    let clock = Clock::read();
    let lbl = label(clock.time.as_str());
    let status = all_status();

    let row = Row::new(children!(lbl, status));
    wrap(row, 30.0)
}

fn right() -> Row {
    let btt = ButtonBuilder::new().id("clock").wrap(clock()).build();
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
            ..Default::default()
        })
        .child(
            Row::new(children!(left(), right()))
                .width(Parent)
                .justify(Justify::SpaceBetween),
        )
}

pub fn view() -> LayerWindow {
    LayerWindow::new()
        .width(Full)
        .height(40.0)
        .anchor_vertical(Vertical::Top)
        .layer(Layer::Top)
        .child(content())
}
