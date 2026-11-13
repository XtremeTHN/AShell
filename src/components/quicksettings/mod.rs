pub mod audio_slider;
pub mod brightness_slider;
pub mod quickbluetooth;
pub mod quickbutton;
pub mod quicknetwork;

use crate::{
    components::{
        button::button,
        icon::glyph,
        mrow::{self, height},
    },
    services::colors::ColorsService,
};
use amane::{
    Align::{Center, End, Start},
    Column, InputArea, Justify, LayerWindow, Margin,
    Radius::Full,
    Rectangle, Row, Service, Widget, children,
};

pub const TOP_MAX_HEIGHT: f32 = 30.0;

pub const QUICKSETTINGS_MAX_PANEL_WIDTH: f32 = 340.0;
pub const QUICKSETTINGS_MAX_PANEL_HEIGHT: f32 = 1080.0;
pub const QUICKSETTINGS_MAX_CONTENT_WIDTH: f32 = QUICKSETTINGS_MAX_PANEL_WIDTH - 30.0;

pub const QUICKSETTINGS_BUTTON_WIDTH: f32 = QUICKSETTINGS_MAX_CONTENT_WIDTH / 2.0 - 5.0;
pub const QUICKSETTINGS_BUTTON_HEIGHT: f32 = 60.0;

struct QuicksettingsIpc {
    visible: bool,
}

impl Service for QuicksettingsIpc {
    fn new() -> Self {
        Self { visible: false }
    }

    fn listen() {}
}

fn settings_btt() -> Rectangle {
    button(
        glyph("settings").size(16.0),
        TOP_MAX_HEIGHT,
        "quicksettings-settings-btt",
        Some(|| {}),
    )
    .padding(10.0)
    .radius(Full)
}

fn power_btt() -> Row {
    let pw = button(
        glyph("mode_off_on").size(16.0),
        TOP_MAX_HEIGHT,
        "quicksettings-power-btt",
        Some(|| {}),
    );

    let more = button(
        glyph("keyboard_arrow_down").size(18.0),
        TOP_MAX_HEIGHT,
        "quicksettings-power-more-bt",
        Some(|| {}),
    );

    mrow::row(vec![pw, more], 10.0)
}

fn top() -> Row {
    Row::new(children!(settings_btt(), power_btt()))
        .gap(5.0)
        .width(QUICKSETTINGS_MAX_CONTENT_WIDTH)
        .justify(Justify::SpaceBetween)
}

fn middle() -> Column {
    let first_row = Row::new(children!(
        quicknetwork::quicknetwork(),
        quickbluetooth::quickbluetooth()
    ))
    .width(QUICKSETTINGS_MAX_CONTENT_WIDTH)
    .justify(Justify::SpaceBetween);

    Column::new(children!(
        first_row,
        brightness_slider::brightness_slider(QUICKSETTINGS_MAX_CONTENT_WIDTH, 32.0),
        audio_slider::audio_slider(QUICKSETTINGS_MAX_CONTENT_WIDTH, 32.0)
    ))
    .gap(10.0)
}

// fn bottom() {}

fn content() -> Rectangle {
    let background = ColorsService::read().c.background.value();

    let row = Column::new(children!(top(), middle()))
        .align(Start)
        .gap(10.0);
    let amane::Size::Fixed(height) = height(&row) else {
        unreachable!()
    };

    Rectangle::new()
        .width(QUICKSETTINGS_MAX_PANEL_WIDTH)
        .height(height + 20.0)
        .child(row)
        .fill(background)
        .align_child(Center, End)
        .radius(18.0)
        .padding(20.0)
}

pub fn toggle(_: &[String]) -> String {
    let mut ipc = QuicksettingsIpc::write();
    ipc.visible = !ipc.visible;

    String::new()
}

pub fn view() -> LayerWindow {
    let c = content();
    let amane::Size::Fixed(height) = c.height() else {
        unreachable!();
    };
    let visible = QuicksettingsIpc::read().visible;

    let a = InputArea {
        x: 0,
        y: 0,
        width: QUICKSETTINGS_MAX_PANEL_WIDTH as i32,
        height: height as i32,
    };

    let mut w = LayerWindow::new()
        .width(QUICKSETTINGS_MAX_PANEL_WIDTH)
        .height(QUICKSETTINGS_MAX_PANEL_HEIGHT)
        .margin(Margin {
            top: 10,
            right: 10,
            ..Default::default()
        })
        .anchor_vertical(amane::Vertical::Top)
        .anchor_horizontal(amane::Horizontal::Right)
        .child(c);

    if visible {
        w = w.click_through().input_region(vec![a]);
    }

    w.visible(visible)
}
