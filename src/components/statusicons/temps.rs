use amane::Align::Center;
use amane::{Rectangle, Row, Service, children};

use crate::components::bar::bar_container::bar_container;
use crate::components::button::to_button;
use crate::components::{icon::glyph, label::label};
use crate::services::temps::Temperature;

pub fn temperature_string(cpu_temp: f32, gpu_temp: f32) -> String {
    let icon: &str = if cpu_temp > 70.0 || gpu_temp > 70.0 {
        "emergency_heat"
    } else if cpu_temp < 30.0 || gpu_temp < 30.0 {
        "ac_unit"
    } else {
        "device_thermostat"
    };

    String::from(icon)
}

pub fn temperatures() -> Rectangle {
    let temps = Temperature::read();

    let icon = glyph(temperature_string(temps.cpu, temps.gpu));
    let ricon = Rectangle::new()
        .width(18.0)
        .height(14.0)
        .align_child(Center, Center)
        .child(icon);
    let cpu_temp = label(format!("{} °C", temps.cpu.ceil()));
    let gpu_temp = label(format!("{} °C", temps.gpu.ceil()));

    let row = Row::new(children!(ricon, cpu_temp, gpu_temp)).gap(5.0);

    to_button(bar_container(row, 30.0), "temperatures")
}
