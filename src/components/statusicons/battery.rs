use crate::components::progress::{battery::battery_progress, state::ProgressStates};
use amane::{Battery, Row, Service};

const PROGRESS_ID: &str = "status-battery";

pub fn battery(width: f32, height: f32) -> Row {
    let battery = Battery::read();

    ProgressStates::change_value(PROGRESS_ID, battery.percent() as f32 / 100.0);
    battery_progress(PROGRESS_ID, width, height)
}
