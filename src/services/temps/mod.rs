pub mod device;
use std::{path::PathBuf, sync::Mutex, time::Duration};

use amane::Service;
use glob::glob;
use log::warn;

const MONS_DIR: &'static str = "/sys/class/hwmon";

fn match_name(path: &PathBuf, _match: &str) -> Option<String> {
    let mut name_path = path.clone();
    name_path.push("name");

    let name = std::fs::read_to_string(name_path)
        .inspect_err(|e| warn!("Couldn't read \"{}\" hardware monitor name: {}", _match, e))
        .ok()?;

    if _match == name.trim() {
        Some(name)
    } else {
        None
    }
}

fn find_temp_monitor(device_name: &str) -> Option<PathBuf> {
    let mons = glob(format!("{}/hwmon*", MONS_DIR).as_str())
        .inspect_err(|e| {
            warn!(
                "Couldn't find \"{}\" hardware temp monitor: {}",
                device_name, e
            )
        })
        .ok()?;

    for x in mons {
        match x {
            Ok(path) => {
                let Some(_) = match_name(&path, device_name) else {
                    continue;
                };

                let mut temp_path = path.clone();
                temp_path.push("temp1_input");

                return Some(temp_path);
            }
            Err(e) => {
                warn!("Couldn't unpack glob path: {}", e);
            }
        }
    }

    None
}

fn read_temp(path: &PathBuf) -> f32 {
    std::fs::read_to_string(path)
        .inspect_err(|e| warn!("Couldn't read temperature for path {:?}: {}", path, e))
        .ok()
        .and_then(|s| {
            s.trim()
                .parse::<f32>()
                .inspect_err(|e| warn!("Couldn't parse temperature for path {:?}: {}", path, e))
                .ok()
        })
        .map(|m| m / 1000.0)
        .unwrap_or(0.0)
}

#[derive(Default)]
pub struct Temperature {
    pub cpu: f32,
    pub gpu: f32,

    cpu_path: Option<PathBuf>,
    gpu_path: Option<PathBuf>,
}

impl Temperature {
    pub fn set_cpu_name(name: &str) {
        let mut this = Self::write();
        this.cpu_path = find_temp_monitor(name);
        this.refresh();
    }

    pub fn set_gpu_name(name: &str) {
        let mut this = Self::write();
        this.gpu_path = find_temp_monitor(name);
        this.refresh();
    }

    fn get_temp(path: Option<&PathBuf>) -> f32 {
        path.and_then(|e| Some(read_temp(e))).unwrap_or(0.0)
    }

    fn refresh(&mut self) -> bool {
        let cpu_temp = Self::get_temp(self.cpu_path.as_ref());
        let gpu_temp = Self::get_temp(self.gpu_path.as_ref());

        if self.cpu != cpu_temp || self.gpu != gpu_temp {
            self.cpu = cpu_temp;
            self.gpu = gpu_temp;

            true
        } else {
            false
        }
    }
}

impl Service for Temperature {
    fn new() -> Self {
        Default::default()
    }

    fn interval() -> std::time::Duration {
        Duration::from_secs(10)
    }

    fn update(&mut self) -> bool {
        self.refresh()
    }
}
