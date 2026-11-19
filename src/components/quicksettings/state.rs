use amane::Service;

use crate::components::navigation::Navigator;

#[derive(Default)]
pub struct State {
    pub visible: bool,
    pub navigator: Navigator,
}

impl Service for State {
    fn new() -> Self {
        Self::default()
    }

    fn listen() {}
}
