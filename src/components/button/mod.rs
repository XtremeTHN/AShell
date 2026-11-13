pub mod state;
pub mod stateful;
// pub mod toggle;

use crate::{
    components::{
        button::state::{ButtonState, ButtonStates},
        mrow::wrap,
    },
    services::colors::ColorsService,
};

use amane::{Align::Center, Color, Rectangle, Service, Widget};

fn handler(id: &'static str, c: impl Fn(&mut ButtonState)) {
    let mut s = state::ButtonStates::write();
    let b = s.states.entry(id).or_default();

    c(b)
}

pub struct ButtonColors {
    pub pressed_color: Color,
    pub hovered_color: Color,
    pub idle_color: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        let colors = ColorsService::read();
        Self {
            idle_color: colors.c.surface_container.value(),
            hovered_color: colors.c.surface_container_high.value(),
            pressed_color: colors.c.surface_container_highest.value(),
        }
    }
}

pub fn to_button_with_callback<T: Fn() + 'static>(
    id: &'static str,
    rectangle: Rectangle,
    colors: Option<ButtonColors>,
    on_clicked: Option<T>,
) -> Rectangle {
    let colors = colors.unwrap_or_default();
    let state = {
        let service = state::ButtonStates::read();
        service.states.get(&id).copied().unwrap_or_default()
    };

    let target = if state.pressed {
        colors.pressed_color
    } else if state.hovered {
        colors.hovered_color
    } else {
        colors.idle_color
    };

    let fill = ButtonStates::animated_fill(id, target);

    rectangle
        .fill(fill)
        .on_hover(move |hovered| {
            handler(id, move |s| {
                s.hovered = hovered;
            })
        })
        .on_press(move |_| {
            handler(id, |s| {
                s.pressed = true;
            });
        })
        .on_release(move |_| {
            handler(id, |s| s.pressed = false);
            if let Some(cb) = on_clicked.as_ref() {
                cb()
            }
        })
}

pub fn to_button(
    id: &'static str,
    rectangle: Rectangle,
    colors: Option<ButtonColors>,
) -> Rectangle {
    to_button_with_callback::<fn()>(id, rectangle, colors, None)
}

pub fn button(
    child: impl Widget + 'static,
    height: f32,
    id: &'static str,
    on_clicked: Option<impl Fn() + 'static>,
) -> Rectangle {
    let r = wrap(child, height).align_child(Center, Center);

    to_button_with_callback(id, r, None, on_clicked)
}
