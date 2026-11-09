use super::Widgets;
use crate::services::colors::ColorsService;
use amane::{Radius, Rectangle, Row, Service, Size, Widget};

const OUTTER_RADIUS: f32 = 16.0;
const INNER_RADIUS: f32 = 4.0;
pub const ROW_PADDING: f32 = 8.0;

fn item(widget: Rectangle) -> Box<dyn Widget> {
    Box::new(widget)
}

pub fn width(child: &impl Widget) -> Size {
    match Widget::width(child) {
        Size::Fixed(width) => Size::Fixed(width + 2.0 * ROW_PADDING),
        Size::Parent => Size::Parent,
    }
}

pub fn height(child: &impl Widget) -> Size {
    match Widget::height(child) {
        Size::Fixed(height) => Size::Fixed(height + 2.0 * ROW_PADDING),
        Size::Parent => Size::Parent,
    }
}

pub fn wrap(child: impl Widget + 'static, height: impl Into<amane::Size>) -> Rectangle {
    Rectangle::new()
        .width(width(&child))
        .height(height)
        .child(child)
}

pub fn row(children: Vec<Rectangle>, padding: f32) -> Row {
    let colors = ColorsService::read();
    let mut items: Widgets = vec![];

    let end = children.len().saturating_sub(1);

    for (counter, child) in children.into_iter().enumerate() {
        let mut widget = child.padding(padding);

        if counter == 0 && counter == end {
            widget = widget.radius(OUTTER_RADIUS);
        } else if counter == 0 {
            widget = widget
                .radius_top_left(OUTTER_RADIUS)
                .radius_bottom_left(OUTTER_RADIUS)
                .radius_bottom_right(INNER_RADIUS)
                .radius_top_right(INNER_RADIUS);
        } else if counter == end {
            widget = widget
                .radius_top_right(OUTTER_RADIUS)
                .radius_bottom_right(OUTTER_RADIUS)
                .radius_top_left(INNER_RADIUS)
                .radius_bottom_left(INNER_RADIUS);
        } else {
            widget = widget.radius(INNER_RADIUS);
        }

        items.push(item(widget));
    }

    Row::new(items).gap(2.0)
}
