mod state;

use crate::{
    components::{
        button::state::States,
        label,
        mrow::{height, wrap},
    },
    services::colors::ColorsService,
};

use super::icon::glyph;
use amane::{Rectangle, Row, Service, Widget};

pub fn convert_to_btt(rectangle: Rectangle, id: impl Into<String>) -> Rectangle {
    let colors = ColorsService::read();
    let id = id.into();

    let (pressed, hovered) = {
        let service = States::read();
        let state = service.states.get(&id);
        state.map_or((false, false), |state| (state.pressed, state.hovered))
    };

    let fill = if pressed {
        colors.c.primary.as_str()
    } else if hovered {
        colors.c.surface_container_high.as_str()
    } else {
        colors.c.surface_container.as_str()
    };

    let hover_id = id.clone();
    let press_id = id.clone();

    rectangle
        .fill(fill)
        .on_hover(move |hovered| {
            States::write()
                .states
                .entry(hover_id.clone())
                .or_default()
                .hovered = hovered;
        })
        .on_press(move |_| {
            States::write()
                .states
                .entry(press_id.clone())
                .or_default()
                .pressed = true;
        })
        .on_release(move |_| {
            States::write()
                .states
                .entry(id.clone())
                .or_default()
                .pressed = false;
        })
}

pub fn button(child: impl Widget + 'static, id: impl Into<String>) -> Rectangle {
    let height = height(&child);
    convert_to_btt(wrap(child, height), id)
}

#[derive(Default)]
pub struct ButtonBuilder {
    id: Option<String>,
    icon_name: Option<String>,
    _label: Option<String>,
    _widget: Option<Box<dyn Widget>>,
    _wrap: Option<Rectangle>,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn glyph(mut self, icon_name: impl Into<String>) -> Self {
        self.icon_name = Some(icon_name.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self._label = Some(label.into());
        self
    }

    pub fn widget(mut self, _widget: impl Widget + 'static) -> Self {
        self._widget = Some(Box::new(_widget));
        self
    }

    pub fn wrap(mut self, rectangle: Rectangle) -> Self {
        self._wrap = Some(rectangle);
        self
    }

    pub fn build(self) -> Rectangle {
        if let Some(rect) = self._wrap {
            convert_to_btt(rect, self.id.unwrap())
        } else {
            let mut children: Vec<Box<dyn Widget>> = vec![];
            let mut id: String = String::new();

            if let Some(_widget) = self._widget {
                children.push(_widget);
            } else {
                if let Some(icon_name) = self.icon_name {
                    children.push(Box::new(glyph(&icon_name)));
                    id = icon_name;
                }

                if let Some(label) = self._label {
                    children.push(Box::new(label::label(&label)));
                    id = label;
                }
            }

            let row = Row::new(children);
            let id = self.id.unwrap_or(id);
            button(row, id)
        }
    }
}
