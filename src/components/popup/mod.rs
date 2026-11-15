use std::sync::Arc;

use amane::{
    Align, Column, InputArea, Justify, Layer, LayerWindow, Padding, Rectangle, Row, Service,
    Size::{self, Parent},
    Stack, Widget, children,
};

use crate::{
    components::{
        button::to_button_with_callback,
        icon::glyph,
        label::label,
        mrow::{height as get_height, width as get_width},
    },
    services::colors::ColorsService,
};

pub type PopupMenuItemCallback = Arc<dyn Fn() + Send + Sync + 'static>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemRole {
    Button,
    Separator,
}

pub struct PopupMenuItem {
    owner: String,
    icon_name: Option<String>,
    label: Option<String>,
    callback: Option<PopupMenuItemCallback>,
    role: ItemRole,
}

impl PopupMenuItem {
    pub fn new(owner: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            label: None,
            icon_name: None,
            callback: None,
            role: ItemRole::Button,
        }
    }

    pub fn callback<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.callback = Some(Arc::new(callback) as PopupMenuItemCallback);
        self
    }

    pub fn icon_name(mut self, icon_name: impl Into<String>) -> Self {
        self.icon_name = Some(icon_name.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn new_separator() -> Self {
        Self {
            owner: String::from("separator"),
            icon_name: None,
            label: None,
            role: ItemRole::Separator,
            callback: None,
        }
    }
}

#[derive(Default)]
pub struct PopupState {
    pub x: f32,
    pub y: f32,
    pub data: Vec<PopupMenuItem>,
    pub current_owner_id: Option<&'static str>,
    pub visible: bool,
}

impl PopupState {
    pub fn toggle(owner_id: &'static str, data: Vec<PopupMenuItem>, x: f32, y: f32) {
        let mut this = Self::write();

        if this.visible && this.current_owner_id == Some(owner_id) {
            this.visible = false;
            this.current_owner_id = None;
            return;
        }

        this.current_owner_id = Some(owner_id);
        this.data = data;
        this.x = x;
        this.y = y;
        this.visible = true;
    }

    pub fn toggle_from_quicksettings(
        owner_id: &'static str,
        data: Vec<PopupMenuItem>,
        pointer_x: f32,
        pointer_y: f32,
        button_height: f32,
        panel_width: f32,
        screen_width: f32,
        right_margin: f32,
    ) {
        let panel_x = screen_width - panel_width - right_margin;
        let popup_x = panel_x + pointer_x;
        let popup_y = pointer_y + button_height;
        Self::toggle(owner_id, data, popup_x, popup_y);
    }
}

impl Service for PopupState {
    fn new() -> Self {
        Self::default()
    }

    fn listen() {}
}

pub fn fully_wrap(child: impl Widget + 'static) -> Rectangle {
    Rectangle::new()
        .width(get_width(&child))
        .height(get_height(&child))
        .child(child)
        .align_child(Align::Center, Align::Center)
}

fn icon_slot(name: &str) -> Rectangle {
    Rectangle::new()
        .width(20.0)
        .height(20.0)
        .child(glyph(name))
        .align_child(Align::Center, Align::Center)
}

fn popup_separator() -> Rectangle {
    let colors = ColorsService::read();
    Rectangle::new()
        .width(110.0)
        .height(1.0)
        .fill(colors.c.outline_variant.value())
}

fn popup_button(item: &PopupMenuItem) -> Rectangle {
    let icon_slot = if let Some(icon_name) = item.icon_name.as_ref() {
        Rectangle::new()
            .width(24.0)
            .height(35.0)
            .child(glyph(icon_name))
            .align_child(Align::Center, Align::Center)
    } else {
        Rectangle::new().width(24.0).height(35.0)
    };

    let contents = Row::new(children!(icon_slot, label(item.label.clone().unwrap())))
        .gap(8.0)
        .align(Align::Center);

    let rect = Rectangle::new()
        .width(120.0)
        .height(35.0)
        .child(contents)
        .padding(5.0)
        .radius(12.0)
        .align_child(Align::Start, Align::Center);

    let id = format!("{}:{}", item.owner.as_str(), item.label.as_ref().unwrap());

    let cb = item.callback.clone();
    to_button_with_callback(
        id,
        rect,
        None,
        Some(move || {
            if let Some(callback) = &cb {
                callback();
            }
        }),
    )
}

pub fn popup() -> (Rectangle, InputArea) {
    let state = PopupState::read();
    let mut children: Vec<Box<dyn Widget>> = vec![];

    for x in state.data.iter() {
        match x.role {
            ItemRole::Button => {
                children.push(Box::new(popup_button(x)));
            }
            ItemRole::Separator => {
                children.push(Box::new(popup_separator()));
            }
        }
    }

    let row = Column::new(children).gap(4.0).align(Align::Center);
    let width = get_width(&row);
    let height = get_height(&row);

    let colors = ColorsService::read();
    let rect = Rectangle::new()
        .width(width)
        .height(height)
        .padding(10.0)
        .radius(16.0)
        .fill(colors.c.surface_container.value())
        .child(row)
        .align_child(Align::Center, Align::Center)
        .translate(state.x, state.y);

    let Size::Fixed(width) = width else {
        unreachable!()
    };

    let Size::Fixed(height) = height else {
        unreachable!()
    };

    let region = InputArea {
        x: state.x as i32,
        y: state.y as i32,
        width: width.ceil() as i32,
        height: height.ceil() as i32,
    };

    (rect, region)
}

pub fn view() -> LayerWindow {
    let (pop, region) = popup();
    let visible = PopupState::read().visible;

    LayerWindow::new()
        .width(1920.0)
        .height(1080.0)
        .layer(Layer::Overlay)
        .click_through()
        .child(pop)
        .visible(visible)
        .namespace("amane-popup")
        .input_region(vec![region])
}
