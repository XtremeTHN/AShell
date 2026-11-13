use amane::{Animation, Color, Service};
use serde::Deserialize;

use crate::paths;

/// Used only before the first real palette is loaded.
const FALLBACK: &str = "#000000";

macro_rules! colors {
    ($($name:ident),* $(,)?) => {
        #[derive(Debug, Clone, Default, Deserialize)]
        #[serde(default)]
        pub struct RawColors {
            $(pub $name: String,)*
        }

        pub struct Colors {
            $(pub $name: Animation<Color>,)*
        }

        impl Default for Colors {
            fn default() -> Self {
                Self {
                    $($name: Animation::new(Color::from(FALLBACK)),)*
                }
            }
        }

        impl Colors {
            fn apply(&mut self, raw: &RawColors, animate: bool) {
                $(
                    // An empty string means the key was missing: keep the current color.
                    if !raw.$name.is_empty() {
                        let target = Color::from(raw.$name.as_str());
                        if animate {
                            self.$name.to(target);
                        } else {
                            self.$name = Animation::new(target);
                        }
                    }
                )*
            }
        }
    };
}

colors! {
    background, error, error_container, inverse_on_surface, inverse_primary,
    inverse_surface, on_background, on_error, on_error_container, on_primary,
    on_primary_container, on_primary_fixed, on_primary_fixed_variant,
    on_secondary, on_secondary_container, on_secondary_fixed,
    on_secondary_fixed_variant, on_surface, on_surface_variant, on_tertiary,
    on_tertiary_container, on_tertiary_fixed, on_tertiary_fixed_variant,
    outline, outline_variant, primary, primary_container, primary_fixed,
    primary_fixed_dim, scrim, secondary, secondary_container, secondary_fixed,
    secondary_fixed_dim, shadow, source_color, surface, surface_bright,
    surface_container, surface_container_high, surface_container_highest,
    surface_container_low, surface_container_lowest, surface_dim, surface_tint,
    surface_variant, tertiary, tertiary_container, tertiary_fixed,
    tertiary_fixed_dim,
}

#[derive(Default)]
pub struct ColorsService {
    pub c: Colors,
}

fn read_colors() -> Option<RawColors> {
    let path = paths::colors()?;

    let text = std::fs::read_to_string(path.as_str())
        .inspect_err(|e| log::warn!("Error while loading colors: {}", e))
        .ok()?;

    serde_json::from_str(&text)
        .inspect_err(|e| log::warn!("Error while parsing colors: {}", e))
        .ok()
}

impl Service for ColorsService {
    fn new() -> Self {
        Default::default()
    }

    fn listen() {
        let Some(path) = paths::colors() else {
            log::warn!("No home dir");
            return;
        };

        if let Some(raw) = read_colors() {
            Self::write().c.apply(&raw, false);
        }

        for _ in amane::watch_file(path.as_str()) {
            if let Some(raw) = read_colors() {
                Self::write().c.apply(&raw, true);
            }
        }
    }
}

pub trait ColorMix {
    fn mix(self, other: Color, t: f32) -> Color;
}

impl ColorMix for Color {
    fn mix(self, other: Color, t: f32) -> Color {
        let m = |a: f32, b: f32| (a + (b - a) * t) as u8;
        Color {
            r: m(self.r.into(), other.r.into()),
            g: m(self.g.into(), other.g.into()),
            b: m(self.b.into(), other.b.into()),
            a: m(self.a.into(), other.a.into()),
        }
    }
}
