use super::{ButtonColors, state::ButtonStates};
use amane::Rectangle;
use std::sync::Arc;

pub fn to_stateful_button<F: Fn() + 'static>(
    id: impl Into<Arc<str>>,
    colors: ButtonColors,
    on_click: Option<F>,
    widget: Rectangle,
) -> Rectangle {
    let id = id.into();
    let state = ButtonStates::get_btt_state(id.as_ref());

    let target = if state.pressed {
        colors.pressed_color
    } else if state.hovered {
        colors.hovered_color
    } else {
        colors.idle_color
    };

    let fill = ButtonStates::animated_fill(Arc::clone(&id), target);
    let hover_id = Arc::clone(&id);
    let press_id = Arc::clone(&id);

    widget
        .fill(fill)
        .on_hover(move |h| ButtonStates::change_btt_state(Arc::clone(&hover_id), |s| s.hovered = h))
        .on_press(move |_| {
            ButtonStates::change_btt_state(Arc::clone(&press_id), |s| s.pressed = true)
        })
        .on_release(move |_| {
            ButtonStates::change_btt_state(Arc::clone(&id), |s| s.pressed = false);
            if let Some(cb) = &on_click {
                cb();
            }
        })
}
