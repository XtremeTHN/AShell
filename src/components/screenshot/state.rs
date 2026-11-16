use std::time::Duration;

use amane::{Animation, Easing, Service};

pub const FADE: Duration = Duration::from_millis(150);

#[derive(Default, Clone, Copy)]
pub enum Mode {
    #[default]
    Region,
    Fullscreen,
    Window,
}

pub struct ScreenshotState {
    pub mode: Mode,
    pub visible: bool,
    pub ready: bool,
    pub fade: Animation,
}

impl ScreenshotState {
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
        self.fade.to(if visible { 1.0 } else { 0.0 });
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn opacity(&self) -> f32 {
        self.fade.value()
    }
}

impl Service for ScreenshotState {
    fn new() -> Self {
        let mut fade = Animation::new(0.0).duration(FADE).easing(Easing::Out);
        fade.to(1.0);

        Self {
            mode: Mode::default(),
            visible: true,
            ready: false,
            fade,
        }
    }
}
