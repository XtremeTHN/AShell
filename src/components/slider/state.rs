use amane::Service;
use std::{collections::HashMap, time::Duration};

#[derive(Default, Clone, Copy)]
pub struct SliderState {
    pub(crate) pressed: bool,
    pub(crate) value: f32,
}

#[derive(Default)]
pub struct SliderStates {
    pub states: HashMap<&'static str, SliderState>,
}

impl SliderStates {
    pub fn get_state(id: &'static str) -> SliderState {
        SliderStates::read()
            .states
            .get(id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn change_value(id: &'static str, fraction: f32, state: SliderState) -> f32 {
        if !state.pressed && (state.value - fraction).abs() > 0.005 {
            SliderStates::write().states.entry(id).or_default().value = fraction;
        }

        if state.pressed { fraction } else { state.value }
    }
}

impl Service for SliderStates {
    fn new() -> Self {
        Default::default()
    }

    fn listen() {}
}
