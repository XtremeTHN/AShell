use std::collections::HashMap;

use amane::Service;

#[derive(Default)]
pub struct ButtonState {
    pub(crate) pressed: bool,
    pub(crate) hovered: bool,
}

#[derive(Default)]
pub struct States {
    pub(crate) states: HashMap<String, ButtonState>,
}

impl Service for States {
    fn new() -> Self {
        Default::default()
    }

    // changes through input
    fn listen() {}
}
