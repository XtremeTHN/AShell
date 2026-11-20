use amane::{Align, Justify::SpaceBetween, Row, Weight, Widget, children};

use crate::components::{Widgets, button::circular_button, icon::glyph_slot, label::label};

pub fn navigator_topbar(
    id: &'static str,
    width: f32,
    extra: Option<Widgets>,
    on_back: impl Fn() + 'static,
) -> Row {
    let back_btt = circular_button(
        format!("navigator-{}", id),
        glyph_slot("keyboard_arrow_left", 18.0, None),
        28.0,
        None,
        Some(on_back),
    );

    let title = label("Red").size(14.0).weight(Weight::Bold);

    let mut children: Widgets = children![back_btt, title];

    if let Some(extra) = extra {
        children.extend(extra);
    }

    Row::new(children)
        .align(Align::Center)
        .width(width)
        .justify(SpaceBetween)
}
