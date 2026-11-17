use std::{
    collections::{HashMap, hash_map::Entry},
    sync::Arc,
    time::Duration,
};

use super::ui::{BUTTON_SIZE, ICON_SIZE};
use crate::{
    components::{
        Widgets,
        button::{ButtonColors, circular_button, circular_rect},
        icon::glyph_slot,
        screenshot::{FindToplevel, state::ScreenshotState},
    },
    services::{
        colors::ColorsService,
        compositor::{Client, CompositorService},
    },
};
use amane::{Align, Animation, Color, Rectangle, Service, Stack};
use libwayshot::WayshotConnection;

const FADE: Duration = Duration::from_millis(120);

struct Hover {
    hovered: bool,
    target: Color, // what the fade is currently heading towards
    fill: Animation<Color>,
}

#[derive(Default)]
struct State {
    active_client_address: Option<Arc<str>>,
    client_rectangles: HashMap<Arc<str>, Hover>,
}

impl State {
    pub fn is_hovered(app_id: &Arc<str>) -> bool {
        State::read()
            .client_rectangles
            .get(app_id)
            .is_some_and(|h| h.hovered)
    }

    pub fn set_hovered(app_id: Arc<str>, hovered: bool) {
        if let Some(h) = State::write().client_rectangles.get_mut(&app_id) {
            h.hovered = hovered;
        }
    }

    pub fn fill(app_id: &Arc<str>, target: Color) -> Color {
        let stale = {
            let s = State::read();
            s.client_rectangles
                .get(app_id)
                .is_none_or(|h| h.target != target)
        };

        if stale {
            let mut s = State::write();
            match s.client_rectangles.entry(app_id.clone()) {
                Entry::Occupied(mut e) => {
                    let h = e.get_mut();
                    h.target = target;
                    h.fill.to(target);
                }
                Entry::Vacant(e) => {
                    e.insert(Hover {
                        hovered: false,
                        target,
                        fill: Animation::new(target).duration(FADE),
                    });
                }
            }
        }

        State::read()
            .client_rectangles
            .get(app_id)
            .map_or(target, |h| h.fill.value())
    }

    pub fn reset() {
        let mut s = Self::write();
        s.active_client_address = None;
    }
}

impl Service for State {
    fn new() -> Self {
        Self::default()
    }

    fn listen() {}
}

pub fn window(client: &Client) -> Rectangle {
    let colors = ColorsService::read();
    let base = colors.c.primary_container.value();
    let with_alpha = |a: u8| {
        let mut c = base;
        c.a = a;
        c
    };

    let app_id: Arc<str> = client.stable_id.clone().into();
    let hover_app_id = Arc::clone(&app_id);
    let clicked_app_id = Arc::clone(&app_id);

    let active = State::read().active_client_address == Some(app_id.clone());
    let hovered = State::is_hovered(&app_id);

    let target = if active {
        with_alpha(180)
    } else if hovered {
        with_alpha(140)
    } else {
        with_alpha(120)
    };
    let fill = State::fill(&app_id, target);

    let rect = Rectangle::new()
        .width(client.width as f32)
        .height(client.height as f32)
        .radius(8.0)
        .translate(client.x as f32, client.y as f32)
        .on_hover(move |hovered| State::set_hovered(hover_app_id.clone(), hovered))
        .align_child(Align::Center, Align::Center);

    let slot = glyph_slot("check", ICON_SIZE, None);
    let button_colors = ButtonColors::default();

    let ret = if active {
        rect.child(circular_rect(BUTTON_SIZE, slot).fill(button_colors.idle_color))
    } else if hovered {
        rect.child(circular_button(
            format!("{}:{}", client.app_id, client.address),
            slot,
            BUTTON_SIZE,
            Some(button_colors),
            Some(move || {
                State::write().active_client_address = Some(clicked_app_id.clone());
            }),
        ))
    } else {
        rect
    };

    ret.fill(fill)
}

pub fn windows() -> Widgets {
    let comp = CompositorService::read();
    let mut children = Widgets::new();
    let current_workspace_id = comp.active_workspace.as_ref().map(|e| &e.id);

    for x in comp.clients.iter() {
        if current_workspace_id != x.workspace_id.as_ref() {
            continue;
        }

        children.push(Box::new(window(x)));
    }

    children
}

pub fn available() -> bool {
    let Ok(conn) = WayshotConnection::new() else {
        log::warn!("couldn't connect to wayland");
        return false;
    };

    conn.toplevel_capture_support()
}

pub fn reset() {
    State::reset();
}

pub fn toplevel() -> FindToplevel {
    Box::new(|t| {
        if let Some(address) = State::read().active_client_address.as_deref() {
            address == t.identifier.as_str()
        } else {
            false
        }
    })
}

pub fn selector() -> Stack {
    let s = Stack::new(windows());

    let state = State::read();
    ScreenshotState::write().ready = state.active_client_address.is_some();

    s
}
