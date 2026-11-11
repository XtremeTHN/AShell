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

impl Service for SliderStates {
    fn new() -> Self {
        Default::default()
    }

    fn interval() -> Duration {
        Duration::from_secs(3600)
    }
}
