use amane::{Audio, Rectangle, Service};

use crate::components::{
    slider::{slider, state::SliderStates},
    statusicons::audio::audio_icon_string,
};

const SLIDER_ID: &'static str = "quicksettings-audio-slider";

pub fn audio_slider(width: f32, height: f32) -> Rectangle {
    let state = SliderStates::get_state(SLIDER_ID);
    let icon = audio_icon_string(state.value * 100.0);

    slider(width, height, Some(icon), SLIDER_ID)
}
