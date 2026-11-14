use crate::{
    components::{
        mrow::width,
        progress::{ProgressColors, battery::battery_progress, state::ProgressStates},
    },
    services::colors::ColorsService,
};
use amane::{Battery, Row, Service, Size};

const PROGRESS_ID: &str = "status-battery";

pub fn battery_icon(width: f32, height: f32) -> Row {
    let battery = Battery::read();
    let colors = ColorsService::read();
    let battery_colors = ProgressColors {
        background: colors.c.surface_container_high.value(),
        ..Default::default()
    };

    ProgressStates::change_value(PROGRESS_ID, battery.percent() as f32 / 100.0);
    battery_progress(PROGRESS_ID, Some(battery_colors), width, height, 6.0, 11.0)
}
