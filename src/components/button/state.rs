use std::{collections::HashMap, time::Duration};

use amane::Service;

#[derive(Default, Clone, Copy)]
pub struct ButtonState {
    pub(crate) hovered: bool,
    pub(crate) pressed: bool,
}

#[derive(Default)]
pub struct ButtonStates {
    pub states: HashMap<&'static str, ButtonState>,
}

impl Service for ButtonStates {
    fn new() -> Self {
        Default::default()
    }

    fn interval() -> Duration {
        Duration::from_secs(3600)
    }
}
