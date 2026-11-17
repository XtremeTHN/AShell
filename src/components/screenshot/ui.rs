use crate::{
    components::{
        button::{ButtonColors, circular_button, circular_rect, state::ButtonStates},
        icon::glyph_slot,
        mrow::width,
        screenshot::{default_mode, fullscreen, screenshot, state::ScreenshotState, window},
        separator::{Orientation, separator},
    },
    services::colors::ColorsService,
};

use super::{Target, region, state};

use amane::{
    Align, Color, Layer, LayerWindow, Monitor, Radius::Full, Rectangle, Row, Service, Size, Stack,
    Zone, children,
};

const TOOLBAR_HEIGHT: f32 = 50.0;
const TOOLBAR_MARGIN: f32 = 80.0;
pub const ICON_SIZE: f32 = 16.0;
pub const BUTTON_SIZE: f32 = 30.0;

fn button_colors(id: &str) -> ButtonColors {
    let colors = ColorsService::read();
    ButtonColors::toggleable(
        colors.c.background.value(),
        colors.c.on_background.value(),
        &*ButtonStates::get_active_button_in_group("screenshot-toolbar") == id,
    )
}

fn group_click(id: &str, mode: state::Mode) {
    ButtonStates::set_active_button_in_group("screenshot-toolbar", id);
    state::ScreenshotState::write().mode = mode;
}

fn mode_button(
    id: &'static str,
    icon: &'static str,
    mode: state::Mode,
    enabled: bool,
) -> Rectangle {
    let slot = glyph_slot(icon, ICON_SIZE, None);
    let colors = button_colors(id);

    if enabled {
        circular_button(
            id,
            slot,
            BUTTON_SIZE,
            Some(colors),
            Some(move || group_click(id, mode)),
        )
    } else {
        circular_rect(BUTTON_SIZE, slot)
            .fill(colors.idle_color)
            .opacity(0.5)
    }
}

fn action_button<T: Fn() + 'static>(
    id: &'static str,
    icon: &'static str,
    enabled: bool,
    callback: T,
) -> Rectangle {
    let colors = ButtonColors::default();
    let slot = glyph_slot(icon, ICON_SIZE, None);

    if enabled {
        circular_button(id, slot, BUTTON_SIZE, Some(colors), Some(callback))
    } else {
        circular_rect(BUTTON_SIZE, slot)
            .fill(colors.idle_color)
            .opacity(0.5)
    }
}

fn exit() {
    ScreenshotState::write().set_visible(false);
}

fn done() {
    let mode = ScreenshotState::read().mode;
    match mode {
        state::Mode::Region => {
            screenshot(Target::Region(region::region()));
        }
        state::Mode::Window => {
            screenshot(Target::Toplevel(window::toplevel()));
        }
        state::Mode::Fullscreen => {
            screenshot(Target::All);
        }
    }
}

fn toolbar_content() -> Row {
    ButtonStates::new_button_group("screenshot-toolbar", default_mode());

    Row::new(children!(
        mode_button(
            "fullscreen",
            "fullscreen",
            state::Mode::Fullscreen,
            fullscreen::available()
        ),
        mode_button(
            "region",
            "highlight_alt",
            state::Mode::Region,
            region::available()
        ),
        mode_button(
            "window",
            "select_window_2",
            state::Mode::Window,
            window::available()
        ),
        separator(Orientation::Vertical, 15.0),
        action_button("screenshot-toolbar-close", "close", true, exit),
        action_button(
            "screenshot-toolbar-done",
            "check",
            ScreenshotState::read().ready,
            done
        ),
    ))
    .gap(10.0)
    .align(Align::Center)
}

fn toolbar(mon_width: f32) -> Rectangle {
    let c = ColorsService::read();
    let child = toolbar_content();

    let Size::Fixed(_width) = width(&child) else {
        unreachable!();
    };

    let x = (mon_width / 2.0) - _width / 2.0;
    let y = TOOLBAR_MARGIN;

    Rectangle::new()
        .width(_width)
        .height(TOOLBAR_HEIGHT)
        .radius(Full)
        .fill(c.c.background.value())
        .translate(x, y)
        .child(child)
        .align_child(Align::Center, Align::Center)
}

fn content(mon_width: f32, mon_height: f32) -> Stack {
    let mut background = Rectangle::new()
        .width(1920.0)
        .height(1080.0)
        .opacity(0.4)
        .fill(Color::BLACK);

    let mode = state::ScreenshotState::read().mode;
    let child = match mode {
        state::Mode::Region => {
            background = background
                .on_press(region::on_pressed)
                .on_drag(region::on_drag);
            region::selector()
        }
        state::Mode::Window => window::selector(),
        state::Mode::Fullscreen => fullscreen::selector(),
    };

    let rect = toolbar(mon_width);
    Stack::new(children!(background, child, rect))
}

pub fn view(mon: &Monitor) -> LayerWindow {
    let (visible, opacity) = {
        let s = ScreenshotState::read();
        (s.visible(), s.opacity())
    };

    let shown = visible || opacity > 0.0;

    let faded = Rectangle::new()
        .width(Size::Parent)
        .height(Size::Parent)
        .opacity(opacity)
        .child(content(mon.width as f32, mon.height as f32));

    LayerWindow::new()
        .width(1920.0)
        .height(1080.0)
        .space(Zone::Ignore)
        .anchor_horizontal(amane::Horizontal::Left)
        .anchor_vertical(amane::Vertical::Top)
        .layer(Layer::Overlay)
        .visible(shown)
        .child(faded)
}
