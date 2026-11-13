use amane::{AccessPoint, Link, Network, Rectangle, Service};

use crate::components::statusicons::network::network_icon_string;
use crate::components::statusicons::{Compare, threshold};
use crate::process::network;

use super::quickbutton::{QuickButton, State};
use super::{QUICKSETTINGS_BUTTON_HEIGHT, QUICKSETTINGS_BUTTON_WIDTH};

pub fn find_active_network() -> Option<AccessPoint> {
    let net = Network::read();
    net.access_points().iter().find(|a| a.active()).cloned()
}

pub fn quicknetwork() -> Rectangle {
    let net = Network::read();
    let icon_name = network_icon_string();

    let default_title = String::from("Network");
    let title = match net.link() {
        Link::Wifi => find_active_network()
            .map(|a| a.ssid().to_owned())
            .unwrap_or(default_title),
        Link::Wired => String::from("Wired"),
        _ => default_title,
    };

    let subtitle = threshold(
        net.strength(),
        &[20, 60],
        vec!["Low", "Medium"],
        "Strong",
        Compare::Less,
    );

    let state = match net.link() {
        Link::Wifi | Link::Wired => {
            if net.connected() {
                State::Active
            } else {
                State::On
            }
        }
        _ => State::Off,
    };

    QuickButton::new("quicksettings-button-wifi", title)
        .icon(icon_name)
        .size(QUICKSETTINGS_BUTTON_WIDTH, QUICKSETTINGS_BUTTON_HEIGHT)
        .subtitle(subtitle)
        .state(state)
        .on_click(move || network::toggle(state))
        .build()
}
