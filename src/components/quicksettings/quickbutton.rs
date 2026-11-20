use crate::{
    components::{
        button::{ButtonColors, stateful::to_stateful_button},
        icon::glyph,
        label::{label, label_dimmed},
    },
    services::colors::{ColorMix, ColorsService},
};
use amane::{Align, Color, Column, Fill, Rectangle, Row, Service, Size, Weight, children};

pub type QuickButtonCallback = Box<dyn Fn() + 'static>;

const PAD: f32 = 10.0;
const GAP: f32 = 8.0;
const ARROW_W: f32 = 30.0;

const HOVER: f32 = 0.08;
const PRESSED: f32 = 0.12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum State {
    Active,
    On,
    #[default]
    Off,
}

impl State {
    fn button_colors(self) -> ButtonColors {
        let colors = ColorsService::read();

        let (base, layer) = match self {
            State::Off | State::On => (
                colors.c.surface_container.value(),
                colors.c.on_surface.value(),
            ),
            State::Active => (
                colors.c.primary_container.value(),
                colors.c.on_primary_container.value(),
            ),
        };

        ButtonColors {
            idle_color: base,
            hovered_color: base.mix(layer, HOVER),
            pressed_color: base.mix(layer, PRESSED),
        }
    }
}

pub struct QuickButton {
    id: &'static str,
    title: String,
    subtitle: Option<String>,
    icon_name: Option<String>,
    state: State,
    size: (f32, f32),
    callback: Option<QuickButtonCallback>,
    on_more: Option<QuickButtonCallback>,
}

impl QuickButton {
    pub fn new(id: &'static str, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            subtitle: None,
            icon_name: None,
            state: State::default(),
            size: (0.0, 0.0),
            callback: None,
            on_more: None,
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn icon(mut self, icon_name: impl Into<String>) -> Self {
        self.icon_name = Some(icon_name.into());
        self
    }

    pub fn state(mut self, state: State) -> Self {
        self.state = state;
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = (width, height);
        self
    }

    pub fn on_click(mut self, callback: impl Fn() + 'static) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }

    pub fn on_more(mut self, callback: impl Fn() + 'static) -> Self {
        self.on_more = Some(Box::new(callback));
        self
    }

    pub fn build(self) -> Rectangle {
        let state = self.state;
        let gcolors = ColorsService::read();

        let has_more = self.on_more.is_some();

        let box_size = self.size.1 - PAD * 2.0;
        // without the arrow, the main part gets the whole width
        let main_w = if has_more {
            self.size.0 - ARROW_W
        } else {
            self.size.0
        };
        let inner_width = main_w - PAD * 2.0;

        let icon_fill = match state {
            State::On => Fill::from(gcolors.c.primary_container.value()),
            _ => Fill::Color(Color::TRANSPARENT),
        };

        let text_color = match state {
            State::Active => gcolors.c.on_primary_container.value(),
            State::Off | State::On => gcolors.c.on_surface.value(),
        };

        let icon = Rectangle::new()
            .width(box_size)
            .height(box_size)
            .fill(icon_fill)
            .radius(box_size / 2.0 * 0.4)
            .align_child(Align::Center, Align::Center)
            .child(
                glyph(self.icon_name.unwrap_or_default())
                    .size(24.0)
                    .color(text_color),
            );

        let title = label(self.title)
            .color(text_color)
            .size(14.0)
            .weight(Weight::Bold)
            .elide();
        let subtitle = label_dimmed(self.subtitle.unwrap_or_default(), Some(text_color)).size(11.0);

        let column = Column::new(children!(title, subtitle))
            .width(inner_width - box_size - GAP)
            .align(Align::Start);

        let row = Row::new(children!(icon, column))
            .width(inner_width)
            .align(Align::Center)
            .gap(GAP);

        let main_rect = Rectangle::new()
            .width(main_w)
            .height(self.size.1)
            .child(row)
            .align_child(Align::Center, Align::Center);

        // all corners rounded when alone, only the left ones when the arrow follows
        let main_rect = if has_more {
            main_rect.radius_top_left(16.0).radius_bottom_left(16.0)
        } else {
            main_rect.radius(16.0)
        };

        let main_btt = to_stateful_button(self.id, state.button_colors(), self.callback, main_rect);

        let Some(on_more) = self.on_more else {
            return main_btt;
        };

        let more_btt = to_stateful_button(
            format!("{}-more", self.id),
            state.button_colors(),
            Some(on_more),
            Rectangle::new()
                .width(ARROW_W)
                .height(self.size.1)
                .align_child(Align::Center, Align::Center)
                .child(glyph("keyboard_arrow_right"))
                .radius_top_right(16.0)
                .radius_bottom_right(16.0),
        );

        Rectangle::new()
            .width(self.size.0)
            .height(self.size.1)
            .child(Row::new(children!(main_btt, more_btt)))
    }
}
