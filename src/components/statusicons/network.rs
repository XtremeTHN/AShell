use crate::components::icon::glyph;
use amane::{Align::Center, Link, Network, Rectangle, Service};

use super::{Compare, threshold};

pub fn wifi_icon_string(strength: u8) -> String {
    let val = threshold(
        strength,
        &[20, 40, 60, 80],
        vec![
            "signal_wifi_0_bar",
            "signal_wifi_1_bar",
            "signal_wifi_2_bar",
            "signal_wifi_3_bar",
        ],
        "signal_wifi_4_bar",
        Compare::LESS,
    );

    String::from(val)
}

pub fn network_icon() -> Rectangle {
    let net = Network::read();
    let icon_name = if net.connected() {
        match net.link() {
            Link::Wifi => wifi_icon_string(net.strength()),
            Link::Wired => {
                if net.connecting() {
                    String::from("signal_wifi_0_bar")
                } else {
                    String::from("settings_ethernet")
                }
            }
            _ => String::new(),
        }
    } else {
        String::from("signal_wifi_bad")
    };

    Rectangle::new()
        .width(16.0)
        .height(22.0)
        .align_child(Center, Center)
        .child(glyph(icon_name).size(16.0))
}
