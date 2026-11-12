mod state;


use crate::{
    components::{
        button::state::ButtonState,
        mrow::wrap,
    },
    services::colors::ColorsService,
};

use amane::{Align::Center, Rectangle, Service, Widget};

fn handler(id: &'static str, c: impl Fn(&mut ButtonState)) {
    let mut s = state::ButtonStates::write();
    let b = s.states.entry(id).or_default();

    c(b)
}

pub fn to_button_with_callback<T: Fn() + 'static>(
    rectangle: Rectangle,
    id: &'static str,
    on_clicked: Option<T>,
) -> Rectangle {
    let colors = ColorsService::read();
    let state = {
        let service = state::ButtonStates::read();
        service.states.get(&id).copied().unwrap_or_default()
    };

    let fill = if state.pressed {
        colors.c.surface_container_highest.as_str()
    } else if state.hovered {
        colors.c.surface_container_high.as_str()
    } else {
        colors.c.surface_container.as_str()
    };

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

pub fn to_button(rectangle: Rectangle, id: &'static str) -> Rectangle {
    to_button_with_callback::<fn()>(rectangle, id, None)
}

pub fn button<T: Fn() + 'static>(
    child: impl Widget + 'static,
    height: f32,
    id: &'static str,
    on_clicked: Option<T>,
) -> Rectangle {
    let r = wrap(child, height).align_child(Center, Center);

    to_button_with_callback(r, id, on_clicked)
}
