use std::path::PathBuf;

use amane::{Bus, Service, arguments, watch_file};

const SYS_BACKLIGHT: &str = "/sys/class/backlight/";

pub struct Backlight {
    pub percentage: f32,
    pub max_brightness: f32,
    pub current_brightness: i32,
    pub device_name: Option<String>,

    pub available: bool,
    bus: Bus,
}

fn find_backlight() -> Option<PathBuf> {
    let devices = glob::glob(&format!("{}/*", SYS_BACKLIGHT))
        .inspect_err(|e| log::warn!("Could search for backlight devices: {}", e))
        .ok()?;

    for device in devices {
        if let Ok(path) = device {
            return Some(path);
        }
    }

    None
}

fn init(backlight: &PathBuf) {
    let device_name = backlight
        .file_name()
        .and_then(|n| n.to_str())
        .map(str::to_owned);
    let max_brightness = std::fs::read_to_string(backlight.join("max_brightness"))
        .ok()
        .and_then(|value| value.trim().parse::<f32>().ok())
        .filter(|value| value.is_finite() && *value > 0.0);

    let mut b = Backlight::write();
    b.device_name = device_name;
    if let Some(max_brightness) = max_brightness {
        b.max_brightness = max_brightness;
        b.available = b.device_name.is_some();
    } else {
        b.available = false;
        log::warn!(
            "Could not read a valid maximum brightness for {}",
            backlight.display()
        );
    }
}

fn refresh(current_brightness: &PathBuf) {
    if let Ok(current_brightness) = std::fs::read_to_string(current_brightness)
        && let Ok(current_brightness) = current_brightness.trim().parse::<i32>()
    {
        let mut b = Backlight::write();
        if b.max_brightness <= 0.0 {
            return;
        }

        b.current_brightness = current_brightness;
        b.percentage = current_brightness as f32 / b.max_brightness;
    }
}

impl Backlight {
    pub fn set_brightness(fraction: f32) {
        let backlight = Self::read();
        if !backlight.available {
            log::warn!("Backlight is not available");
            return;
        }

        let value = (fraction * backlight.max_brightness) as u32;
        backlight.bus.call(
            "org.freedesktop.login1",
            "/org/freedesktop/login1/session/auto",
            "org.freedesktop.login1.Session",
            "SetBrightness",
            &arguments!(
                "backlight",
                backlight.device_name.as_deref().unwrap_or(""),
                value,
            ),
        );
    }
}

impl Service for Backlight {
    fn new() -> Self {
        Self {
            percentage: 0.0,
            max_brightness: 0.0,
            current_brightness: 0,
            available: false,
            device_name: None,
            bus: Bus::system(),
        }
    }

    fn listen() {
        let Some(backlight) = find_backlight() else {
            log::warn!("No backlight device found");
            Self::write().available = false;
            return;
        };

        let current_brightness = backlight.join("brightness");

        init(&backlight);
        refresh(&current_brightness);

        for _ in watch_file(&current_brightness.to_string_lossy()) {
            refresh(&current_brightness);
        }
    }
}
