use amane::Rectangle;

use crate::components::{
    slider::{slider, state::SliderStates},
    statusicons::{Compare, threshold},
};

const SLIDER_ID: &str = "quicksettings-brightness-slider";

pub fn brightness_slider(width: f32, height: f32) -> Rectangle {
    let state = SliderStates::get_state(SLIDER_ID);

    let icon = threshold(
        state.value,
        &[0.20, 0.80],
        vec!["brightness_empty", "brightness_medium"],
        "brightness_7",
        Compare::Less,
    );

    slider(width, height, Some(icon), SLIDER_ID, |v| {})
}
