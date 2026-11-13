use super::{ButtonColors, state::ButtonStates};
use amane::Rectangle;

pub fn to_stateful_button<F: Fn() + 'static>(
    id: &'static str,
    colors: ButtonColors,
    on_click: Option<F>,
    widget: Rectangle,
) -> Rectangle {
    let state = ButtonStates::get_btt_state(id);

    let target = if state.pressed {
        colors.pressed_color
    } else if state.hovered {
        colors.hovered_color
    } else {
        colors.idle_color
    };

    let fill = ButtonStates::animated_fill(id, target);

    widget
        .fill(fill)
        .on_hover(move |h| ButtonStates::change_btt_state(id, move |s| s.hovered = h))
        .on_press(move |_| ButtonStates::change_btt_state(id, |s| s.pressed = true))
        .on_release(move |_| {
            ButtonStates::change_btt_state(id, |s| s.pressed = false);
            if let Some(cb) = &on_click {
                cb();
            }
        })
}
