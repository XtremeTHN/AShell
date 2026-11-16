use amane::Service;

#[derive(Default, Clone, Copy)]
pub enum Mode {
    #[default]
    Region,
    Fullscreen,
    Window,
}

#[derive(Default)]
pub struct ScreenshotState {
    pub mode: Mode,
}

impl Service for ScreenshotState {
    fn new() -> Self {
        Self::default()
    }
}
