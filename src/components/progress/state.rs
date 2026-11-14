use std::collections::HashMap;

use amane::Service;

#[derive(Clone, Copy, Default)]
pub struct ProgressState {
    pub value: f32,
}

#[derive(Default)]
pub struct ProgressStates {
    states: HashMap<&'static str, ProgressState>,
}

impl ProgressStates {
    pub fn get_state(id: &'static str) -> ProgressState {
        ProgressStates::read()
            .states
            .get(id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn change_value(id: &'static str, value: f32) {
        ProgressStates::write().states.entry(id).or_default().value = value;
    }
}

impl Service for ProgressStates {
    fn new() -> Self {
        Self::default()
    }

    fn listen() {}
}
