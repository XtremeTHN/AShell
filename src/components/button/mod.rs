pub mod state;
pub mod stateful;
// pub mod toggle;

use crate::{
    components::{
        button::state::{ButtonState, ButtonStates},
        mrow::wrap,
    },
    services::colors::{ColorMix, ColorsService},
};

use amane::{Align::Center, Color, Full, Rectangle, Service, Widget};
use std::sync::Arc;

fn handler(id: Arc<str>, c: impl FnOnce(&mut ButtonState)) {
    state::ButtonStates::change_btt_state(id, c);
}

pub struct ButtonColors {
    pub pressed_color: Color,
    pub hovered_color: Color,
    pub idle_color: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        let colors = ColorsService::read();
        Self::layered(
            colors.c.surface_container.value(),
            colors.c.on_surface.value(),
        )
    }
}

impl ButtonColors {
    pub fn layered(base_color: Color, on_base_color: Color) -> Self {
        Self {
            pressed_color: base_color.mix(on_base_color, 0.12),
            hovered_color: base_color.mix(on_base_color, 0.08),
            idle_color: base_color,
        }
    }

    pub fn toggleable(base_color: Color, on_base_color: Color, active: bool) -> Self {
        let colors = ColorsService::read();

        let (base, layer) = if active {
            (
                colors.c.primary_container.value(),
                colors.c.on_primary_container.value(),
            )
        } else {
            (base_color, on_base_color)
        };

        ButtonColors {
            pressed_color: base.mix(layer, 0.12),
            hovered_color: base.mix(layer, 0.08),
            idle_color: base,
        }
    }
}

pub fn to_button_with_callback<T: Fn() + 'static>(
    id: impl Into<Arc<str>>,
    rectangle: Rectangle,
    colors: Option<ButtonColors>,
    on_clicked: Option<T>,
) -> Rectangle {
    let id = id.into();
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

    let fill = ButtonStates::animated_fill(Arc::clone(&id), target);
    let drag_id = Arc::clone(&id);
    let hover_id = Arc::clone(&id);
    let press_id = Arc::clone(&id);

    rectangle
        .fill(fill)
        .on_drag(move |p| {
            handler(Arc::clone(&drag_id), move |s| {
                s.point = Some(p);
            });
        })
        .on_hover(move |hovered| {
            handler(Arc::clone(&hover_id), |s| {
                s.hovered = hovered;
            })
        })
        .on_press(move |_| {
            handler(Arc::clone(&press_id), |s| {
                s.pressed = true;
            });
        })
        .on_release(move |_| {
            handler(Arc::clone(&id), |s| s.pressed = false);
            if let Some(cb) = on_clicked.as_ref() {
                cb()
            }
        })
}

pub fn to_button(
    id: impl Into<Arc<str>>,
    rectangle: Rectangle,
    colors: Option<ButtonColors>,
) -> Rectangle {
    to_button_with_callback::<fn()>(id, rectangle, colors, None)
}

pub fn circular_rect(size: f32, child: impl Widget + 'static) -> Rectangle {
    Rectangle::new()
        .width(size)
        .height(size)
        .align_child(Center, Center)
        .child(child)
        .radius(Full)
}

pub fn circular_button<T: Fn() + 'static>(
    id: impl Into<Arc<str>>,
    child: impl Widget + 'static,
    size: f32,
    colors: Option<ButtonColors>,
    on_clicked: Option<T>,
) -> Rectangle {
    let r = circular_rect(size, child);
    to_button_with_callback(id, r, colors, on_clicked)
}

pub fn button<T: Fn() + 'static>(
    id: impl Into<Arc<str>>,
    child: impl Widget + 'static,
    height: f32,
    colors: Option<ButtonColors>,
    on_clicked: Option<T>,
) -> Rectangle {
    let r = wrap(child, height).align_child(Center, Center);

    to_button_with_callback(id, r, colors, on_clicked)
}
