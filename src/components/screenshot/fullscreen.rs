use amane::{Fill::Mask, Rectangle, Service, Size::Parent, Stack, children};

use crate::components::screenshot::state::ScreenshotState;

pub fn available() -> bool {
    true
}

pub fn selector() -> Stack {
    ScreenshotState::write().ready = true;
    Stack::new(children!(
        Rectangle::new().width(Parent).height(Parent).fill(Mask)
    ))
}
