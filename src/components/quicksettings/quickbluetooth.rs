use amane::{Bluetooth, BluetoothDevice, Rectangle, Service};

use crate::components::quicksettings::{
    QUICKSETTINGS_BUTTON_HEIGHT, QUICKSETTINGS_BUTTON_WIDTH,
    quickbutton::{QuickButton, State},
};

pub fn active_device() -> Option<BluetoothDevice> {
    let blue = Bluetooth::read();
    blue.devices().iter().find(|d| d.connected()).cloned()
}

pub fn quickbluetooth() -> Rectangle {
    let blue = Bluetooth::read();

    let state: State;
    let icon_name: String;
    let subtitle: String;

    if blue.scanning() {
        icon_name = String::from("bluetooth_searching");
        subtitle = String::from("Searching");
        state = State::On;
    } else if let Some(dev) = active_device() {
        icon_name = dev.icon().to_owned();
        subtitle = dev.name().to_owned();
        state = State::Active;
    } else if blue.available() && blue.powered() {
        icon_name = String::from("bluetooth");
        subtitle = String::from("On");
        state = State::On;
    } else {
        icon_name = String::from("bluetooth_disabled");
        subtitle = String::from("Off");
        state = State::Off;
    };

    QuickButton::new("quicksettings-button-bluetooth", "Bluetooth")
        .icon(icon_name)
        .size(QUICKSETTINGS_BUTTON_WIDTH, QUICKSETTINGS_BUTTON_HEIGHT)
        .subtitle(subtitle)
        .state(state)
        .on_click(|| {})
        .build()
}
