use amane::{Color, Rectangle, Service, Widget};

use crate::components::button::state::ButtonFillAnimState;
use crate::services::colors::ColorsService;

use super::state::{ButtonStates, ToggleButtonState};
use super::{ButtonColors, to_button_with_callback};

pub struct ToggleButtonColors {
    button: ButtonColors,
    checked_color: Color,
}

impl Default for ToggleButtonColors {
    fn default() -> Self {
        let colors = ColorsService::read();

        Self {
            button: ButtonColors {
                pressed_color: colors.c.primary.value(),
                ..Default::default()
            },
            checked_color: colors.c.primary.value(),
        }
    }
}

pub fn to_toggle_button(
    id: &'static str,
    colors: Option<ToggleButtonColors>,
    widget: Rectangle,
) -> Rectangle {
    let toggle_colors = colors.unwrap_or_default();
    let state = ButtonStates::get_toggle_btt_state(id);

    let target = if state.checked {
        toggle_colors.checked_color
    } else if state.pressed {
        toggle_colors.button.pressed_color
    } else if state.hovered {
        toggle_colors.button.hovered_color
    } else {
        toggle_colors.button.idle_color
    };

    let fill = ButtonStates::animated_fill(id, target);

    widget
        .fill(fill)
        .on_hover(move |hovered| {
            ButtonStates::change_state(id, move |s| {
                s.hovered = hovered;
            })
        })
        .on_press(move |_| {
            ButtonStates::change_state(id, move |s| {
                s.pressed = true;
            });
        })
        .on_release(move |_| {
            ButtonStates::change_state(id, move |s| {
                s.pressed = false;
                s.checked = !s.checked;
            });
        })
}
