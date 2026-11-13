use amane::{App, Service};

use crate::services::clock;
use crate::services::temps::Temperature;

mod components;
mod paths;
mod process;
mod services;

fn main() {
    let env = env_logger::Env::default().filter_or("LIFT_LOG", "info");
    env_logger::init_from_env(env);

    clock::init_offset();
    drop(clock::Clock::read());

    Temperature::set_cpu_name("k10temp");
    Temperature::set_gpu_name("amdgpu");

    App::new()
        .window(components::bar::view)
        .window(components::quicksettings::view)
        .font("Roboto")
        .run();
}
