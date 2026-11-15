pub mod menu;
pub mod state;

use amane::{
    Align, Column, InputArea, Layer, LayerWindow, Rectangle, Row, Service, Size, Widget, children,
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

fn popup_separator() -> Rectangle {
    let colors = ColorsService::read();
    Rectangle::new()
        .width(110.0)
        .height(1.0)
        .fill(colors.c.outline_variant.value())
}

fn popup_button(item: &menu::PopupMenuItem) -> Rectangle {
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
    let state = state::PopupState::read();
    let mut children: Vec<Box<dyn Widget>> = vec![];

    for x in state.data.iter() {
        match x.role {
            menu::ItemRole::Button => {
                children.push(Box::new(popup_button(x)));
            }
            menu::ItemRole::Separator => {
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
        let s = state::PopupState::read();
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
