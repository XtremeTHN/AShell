use amane::{Audio, Rectangle, Service};

use crate::components::{
    slider::{slider, state::SliderStates},
    statusicons::audio::audio_icon_string,
};

const SLIDER_ID: &str = "quicksettings-audio-slider";

pub fn audio_slider(width: f32, height: f32) -> Rectangle {
    let audio = Audio::read();
    let state = SliderStates::get_state(SLIDER_ID);

    // follows the service unless the user is holding the slider
    let volume_fraction = audio.volume() as f32 / 100.0;
    let value = SliderStates::change_value(SLIDER_ID, volume_fraction, state);
    let icon = audio_icon_string(value * 100.0);

    slider(width, height, Some(icon), SLIDER_ID, |v| {
        Audio::set_volume((v * 100.0).ceil() as u8);
    })
}
