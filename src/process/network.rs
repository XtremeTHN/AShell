use amane::spawn;

use crate::components::quicksettings::quickbutton::State;

pub fn toggle(state: State) {
    let cond = match state {
        State::Active | State::On => "off",
        State::Off => "on",
    };

    spawn(&format!("nmcli n {}", cond));
}
