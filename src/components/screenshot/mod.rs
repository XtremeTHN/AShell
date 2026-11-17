pub mod fullscreen;
pub mod region;
pub mod state;
pub mod ui;
pub mod window;

use crate::{components::screenshot::state::ScreenshotState, paths::pictures};
use amane::Service;
use libwayshot::{LogicalRegion, WayshotConnection};
use time::{OffsetDateTime, macros::format_description};
pub use ui::view;

use image::DynamicImage;
use state::FADE;
use std::{thread, time::Duration};

pub type FindToplevel = Box<dyn Fn(&libwayshot::region::TopLevel) -> bool + Send>;
pub enum Target {
    Region(LogicalRegion),
    Toplevel(FindToplevel),
    All,
}

impl Target {
    fn needs_overlay_hidden(&self) -> bool {
        !matches!(self, Target::Toplevel(_))
    }
}

pub fn screenshot(target: Target) {
    let wait = if target.needs_overlay_hidden() {
        ScreenshotState::write().set_visible(false);
        Some(FADE + Duration::from_millis(100))
    } else {
        None
    };

    thread::spawn(move || {
        if let Some(d) = wait {
            thread::sleep(d);
        }
        if let Err(e) = capture(target) {
            log::warn!("Screenshot failed: {e}");
        }

        ScreenshotState::write().set_visible(false);
    });
}

fn grab(
    conn: &WayshotConnection,
    target: Target,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    Ok(match target {
        Target::Region(region) => conn.screenshot(region, false)?,
        Target::All => conn.screenshot_all(false)?,
        Target::Toplevel(matches) => {
            let toplevels = conn.get_all_toplevels();
            let tl = toplevels
                .iter()
                .filter(|t| t.active)
                .find(|t| matches(t))
                .ok_or("no matching window")?;
            conn.screenshot_toplevel(tl, false)?
        }
    })
}

fn capture(target: Target) -> Result<(), Box<dyn std::error::Error>> {
    let conn = WayshotConnection::new()?;
    let img = grab(&conn, target)?;

    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let fmt =
        format_description!("Screenshot [year]-[month]-[day] at [hour].[minute].[second].png");
    let name = now.format(&fmt)?;

    let path = pictures().ok_or("no pictures directory")?;
    img.save(path.join(name))?;
    Ok(())
}

pub fn default_mode() -> &'static str {
    "region"
}
