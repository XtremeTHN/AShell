use amane::Align::Center;
use amane::{Rectangle, Row, Service, Text, children};

use crate::components::mrow::wrap;
use crate::components::statusicons::threshold;
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

    wrap(row, 30.0)
}
