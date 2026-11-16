pub mod region;
pub mod state;
pub mod ui;

use crate::{components::screenshot::state::ScreenshotState, paths::pictures};
use amane::Service;
use libwayshot::LogicalRegion;
use time::{OffsetDateTime, macros::format_description};
pub use ui::view;

use state::FADE;
use std::{thread, time::Duration};

pub fn default_mode() -> &'static str {
    // TODO
    "region"
}

pub fn screenshot(region: LogicalRegion) {
    ScreenshotState::write().set_visible(false); // starts the fade-out

    thread::spawn(move || {
        // wait for the fade plus a little time for the compositor to unmap it
        thread::sleep(FADE + Duration::from_millis(100));
        capture(region);
    });
}

fn capture(region: LogicalRegion) {
    let conn = match libwayshot::WayshotConnection::new() {
        Ok(c) => c,
        Err(e) => {
            log::warn!("wayshot connection failed: {}", e);
            return;
        }
    };

    let img = match conn.screenshot(region, false) {
        Ok(img) => img,
        Err(e) => {
            log::warn!("Screenshot failed: {}", e);
            return;
        }
    };

    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let fmt =
        format_description!("Screenshot [year]-[month]-[day] at [hour].[minute].[second].png");

    let Ok(name) = now.format(&fmt) else { return };
    let Some(path) = pictures() else { return };

    if let Err(e) = img.save(path.join(name)) {
        log::warn!("Couldn't save screenshot: {}", e);
    }
}
