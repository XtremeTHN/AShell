use crate::{
    components::{
        button::button,
        icon::glyph,
        mrow::{self, height, wrap},
    },
    services::colors::ColorsService,
};
use amane::{
    Align::Center, Column, Justify, LayerWindow, Margin, Padding, Radius::Full, Rectangle, Row,
    Service, Widget, children,
};

const TOP_MAX_HEIGHT: f32 = 30.0;

const QUICKSETTINGS_MAX_WIDTH: f32 = 300.0;
const QUICKSETTINGS_MAX_HEIGHT: f32 = 100.0;
const QUICKSETTINGS_MIN_HEIGHT: f32 = QUICKSETTINGS_MAX_WIDTH - 20.0;

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
        .width(QUICKSETTINGS_MIN_HEIGHT)
        .justify(Justify::SpaceBetween)
}

fn middle() {}
fn bottom() {}

fn content() -> Rectangle {
    let colors = ColorsService::read();

    let row = Column::new(children!(top()));
    let height = height(&row);

    // wrap(row, height)
    Rectangle::new()
        .width(QUICKSETTINGS_MAX_WIDTH)
        .height(height)
        .child(row)
        .fill(colors.c.background.as_str())
        .align_child(Center, Center)
        .radius(16.0)
        .border(1.0, colors.c.outline_variant.as_str())
}

pub fn view() -> LayerWindow {
    LayerWindow::new()
        .width(QUICKSETTINGS_MAX_WIDTH)
        .height(QUICKSETTINGS_MAX_HEIGHT)
        .margin(Margin {
            top: 10,
            right: 10,
            ..Default::default()
        })
        .anchor_vertical(amane::Vertical::Top)
        .anchor_horizontal(amane::Horizontal::Right)
        .child(content())
}
