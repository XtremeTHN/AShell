
use amane::Service;
use serde::Deserialize;

use crate::paths;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Colors {
    pub background: String,
    pub error: String,
    pub error_container: String,
    pub inverse_on_surface: String,
    pub inverse_primary: String,
    pub inverse_surface: String,
    pub on_background: String,
    pub on_error: String,
    pub on_error_container: String,
    pub on_primary: String,
    pub on_primary_container: String,
    pub on_primary_fixed: String,
    pub on_primary_fixed_variant: String,
    pub on_secondary: String,
    pub on_secondary_container: String,
    pub on_secondary_fixed: String,
    pub on_secondary_fixed_variant: String,
    pub on_surface: String,
    pub on_surface_variant: String,
    pub on_tertiary: String,
    pub on_tertiary_container: String,
    pub on_tertiary_fixed: String,
    pub on_tertiary_fixed_variant: String,
    pub outline: String,
    pub outline_variant: String,
    pub primary: String,
    pub primary_container: String,
    pub primary_fixed: String,
    pub primary_fixed_dim: String,
    pub scrim: String,
    pub secondary: String,
    pub secondary_container: String,
    pub secondary_fixed: String,
    pub secondary_fixed_dim: String,
    pub shadow: String,
    pub source_color: String,
    pub surface: String,
    pub surface_bright: String,
    pub surface_container: String,
    pub surface_container_high: String,
    pub surface_container_highest: String,
    pub surface_container_low: String,
    pub surface_container_lowest: String,
    pub surface_dim: String,
    pub surface_tint: String,
    pub surface_variant: String,
    pub tertiary: String,
    pub tertiary_container: String,
    pub tertiary_fixed: String,
    pub tertiary_fixed_dim: String,
}

#[derive(Default, Debug)]
pub struct ColorsService {
    pub c: Colors,
}

fn read_colors() -> Colors {
    let Some(path) = paths::colors() else {
        return Default::default();
    };

    std::fs::read_to_string(path.as_str())
        .inspect_err(|e| log::warn!("Error while loading colors: {}", e))
        .ok()
        .and_then(|v| {
            serde_json::from_str(v.as_str())
                .inspect_err(|e| log::warn!("Error while parsing colors: {}", e))
                .unwrap_or_default()
        })
        .unwrap_or_default()
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

        Self::write().c = read_colors();

        for _ in amane::watch_file(path.as_str()) {
            Self::write().c = read_colors();
        }
    }
}
