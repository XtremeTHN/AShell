use amane::{Rectangle, Service};

use crate::{
    components::{
        slider::{slider, state::SliderStates},
        statusicons::{Compare, threshold},
    },
    services::backlight::Backlight,
};

const SLIDER_ID: &str = "quicksettings-brightness-slider";

pub fn brightness_slider(width: f32, height: f32) -> Rectangle {
    let state = SliderStates::get_state(SLIDER_ID);
    let percentage = Backlight::read().percentage;
    let value = SliderStates::change_value(SLIDER_ID, percentage, state);

    let icon = threshold(
        value,
        &[0.20, 0.80],
        vec!["brightness_empty", "brightness_medium"],
        "brightness_7",
        Compare::Less,
    );

    slider(width, height, Some(icon), SLIDER_ID, |v| {
        Backlight::set_brightness(v);
    })
}
