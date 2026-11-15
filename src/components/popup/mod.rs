use std::sync::Arc;

use amane::{
    Align, Animation, Column, InputArea, Justify, Layer, LayerWindow, Padding, Rectangle, Row,
    Service,
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

pub struct PopupState {
    pub x: Animation<f32>,
    pub y: Animation<f32>,
    pub opacity: Animation<f32>,
    pub data: Vec<PopupMenuItem>,
    pub current_owner_id: Option<&'static str>,
    pub visible: bool, // logical state: open or closed
}

impl PopupState {
    pub fn toggle(owner_id: &'static str, data: Vec<PopupMenuItem>, x: f32, y: f32) {
        let mut this = Self::write();

        if this.visible && this.current_owner_id == Some(owner_id) {
            this.visible = false;
            this.current_owner_id = None;
            this.opacity.to(0.0); // fade out
            return;
        }

        if this.visible {
            this.x.to(x);
            this.y.to(y);
        } else {
            this.x = Animation::new(x);
            this.y = Animation::new(y);
            this.opacity = Animation::new(0.0);
            this.opacity.to(1.0);
        }

        this.current_owner_id = Some(owner_id);
        this.data = data;
        this.visible = true;
    }
}

impl Service for PopupState {
    fn new() -> Self {
        Self {
            x: Animation::new(0.0),
            y: Animation::new(0.0),
            opacity: Animation::new(0.0),
            data: vec![],
            current_owner_id: None,
            visible: false,
        }
    }

    fn listen() {}
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

    let (x, y) = (state.x.value(), state.y.value());
    let colors = ColorsService::read();
    let rect = Rectangle::new()
        .width(width)
        .height(height)
        .padding(10.0)
        .radius(16.0)
        .fill(colors.c.surface_container.value())
        .child(row)
        .opacity(state.opacity.value())
        .align_child(Align::Center, Align::Center)
        .translate(x, y);

    let Size::Fixed(width) = width else {
        unreachable!()
    };

    let Size::Fixed(height) = height else {
        unreachable!()
    };

    let region = InputArea {
        x: x as i32,
        y: y as i32,
        width: width.ceil() as i32,
        height: height.ceil() as i32,
    };

    (rect, region)
}

pub fn view() -> LayerWindow {
    let (pop, region) = popup();
    let shown = {
        let s = PopupState::read();
        s.visible || s.opacity.value() > 0.0
    };

    LayerWindow::new()
        .width(1920.0)
        .height(1080.0)
        .layer(Layer::Overlay)
        .click_through()
        .child(pop)
        .visible(shown)
        .namespace("amane-popup")
        .input_region(vec![region])
}
