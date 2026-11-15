use crate::components::mrow::wrap;

use super::audio::audio_icon;
use super::battery::battery_icon;
use super::network::network_icon;
use amane::{Align::Center, Battery, Rectangle, Row, Service, Size::Parent, children};

pub fn all_status() -> Rectangle {
    let battery_present = Battery::read().present();
    let mut icons = children!(network_icon(), audio_icon());

    if battery_present {
        icons.push(Box::new(battery_icon(28.0, 16.0)));
    }

    let r = Row::new(icons).align(Center).gap(4.0);
    wrap(r, Parent).align_child(Center, Center)
}
