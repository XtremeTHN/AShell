use std::{collections::HashMap, time::Duration};

use amane::{Animation, Color, Service};

#[derive(Default, Clone, Copy)]
pub struct ButtonState {
    pub hovered: bool,
    pub pressed: bool,
}

#[derive(Default, Clone, Copy)]
pub struct ToggleButtonState {
    pub hovered: bool,
    pub pressed: bool,
    pub checked: bool,
}

pub struct ButtonFillAnimState {
    pub fill: Animation<Color>,
    target: Color,
}

impl ButtonFillAnimState {
    pub fn new(initial: Color) -> Self {
        Self {
            fill: Animation::new(initial),
            target: initial,
        }
    }

    pub fn animate_to(&mut self, target: Color) -> Color {
        if target != self.target {
            self.target = target;
            self.fill.to(target);
        }
        self.fill.value()
    }
}

#[derive(Default)]
pub struct ButtonStates {
    pub states: HashMap<&'static str, ButtonState>,
    pub fills: HashMap<&'static str, ButtonFillAnimState>,
    pub toggle_states: HashMap<&'static str, ToggleButtonState>,
}

impl ButtonStates {
    pub fn get_btt_state(id: &'static str) -> ButtonState {
        ButtonStates::read()
            .states
            .get(id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_toggle_btt_state(id: &'static str) -> ToggleButtonState {
        ButtonStates::read()
            .toggle_states
            .get(id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn animated_fill(id: &'static str, target: Color) -> Color {
        {
            let states = ButtonStates::read();
            if let Some(fill) = states.fills.get(id) {
                if fill.target == target {
                    return fill.fill.value();
                }
            }
        }

        let mut s = ButtonStates::write();
        s.fills
            .entry(id)
            .or_insert_with(|| ButtonFillAnimState::new(target)) // first frame: no animation
            .animate_to(target)
    }

    pub fn change_btt_state(id: &'static str, cb: impl Fn(&mut ButtonState)) {
        let mut s = ButtonStates::write();
        cb(s.states.entry(id).or_default());
    }
}

impl Service for ButtonStates {
    fn new() -> Self {
        Default::default()
    }

    fn listen() {}
}
