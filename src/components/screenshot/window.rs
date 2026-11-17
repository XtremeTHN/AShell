use std::{collections::HashMap, sync::Arc};

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
use amane::{Align, Rectangle, Service, Stack};
use libwayshot::WayshotConnection;

#[derive(Default)]
struct State {
    active_client_address: Option<Arc<str>>,
    client_rectangles: HashMap<Arc<str>, bool>,
}

impl State {
    pub fn get_hovered(app_id: Arc<str>) -> bool {
        if let Some(hovered) = State::read().client_rectangles.get(&app_id) {
            *hovered
        } else {
            State::write().client_rectangles.entry(app_id).or_default();
            false
        }
    }

    pub fn change<T: FnOnce(&mut bool)>(app_id: Arc<str>, cb: T) {
        State::write()
            .client_rectangles
            .entry(app_id)
            .and_modify(cb);
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

    let app_id: Arc<str> = client.app_id.clone().into();

    let hover_app_id = Arc::clone(&app_id);

    let mut fill_color = colors.c.primary_container.value().clone();
    let rect = Rectangle::new()
        .width(client.width as f32)
        .height(client.height as f32)
        // .fill(fill_color)
        .radius(8.0)
        .translate(client.x as f32, client.y as f32)
        .on_hover(move |hovered| {
            State::change(hover_app_id.clone(), |b| {
                *b = hovered;
            })
        })
        .align_child(Align::Center, Align::Center);

    let clicked_app_id = Arc::clone(&app_id);
    let slot = glyph_slot("check", ICON_SIZE, None);

    let colors = ButtonColors::default();

    let ret = if State::read().active_client_address == Some(clicked_app_id.clone()) {
        fill_color.a = 180;
        rect.child(circular_rect(BUTTON_SIZE, slot).fill(colors.idle_color))
    } else if State::get_hovered(app_id) {
        fill_color.a = 140;
        rect.child(circular_button(
            format!("{}:{}", client.app_id, client.address),
            slot,
            BUTTON_SIZE,
            Some(colors),
            Some(move || {
                State::write().active_client_address = Some(clicked_app_id.clone());
            }),
        ))
    } else {
        fill_color.a = 120;
        rect
    };

    ret.fill(fill_color)
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

pub fn toplevel() -> FindToplevel {
    Box::new(|t| {
        if let Some(address) = State::read().active_client_address.as_deref() {
            address == t.app_id.as_str()
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
