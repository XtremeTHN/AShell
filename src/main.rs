use amane::{App, Service};

use crate::services::temps::Temperature;
use crate::services::{backlight, clock};

mod components;
mod paths;
mod process;
mod services;
mod states;

fn main() {
    let env = env_logger::Env::default().filter_or("LIFT_LOG", "info");
    env_logger::init_from_env(env);

    clock::init_offset();
    drop(clock::Clock::read());
    drop(backlight::Backlight::read());

    Temperature::set_cpu_name("k10temp");
    Temperature::set_gpu_name("amdgpu");

    App::new()
        .window(components::bar::view)
        .window(components::quicksettings::view)
        .ipc("quicksettings", components::quicksettings::toggle)
        .font("Roboto")
        .animation_fps_limit(90)
        .run();
}
