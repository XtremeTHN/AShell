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
}

impl Service for SliderStates {
    fn new() -> Self {
        Default::default()
    }

    fn listen() {}
}
