use crate::{components::mrow::wrap, services::colors::ColorsService};
use amane::{Rectangle, Service, Widget};

pub fn bar_container(child: impl Widget + 'static, height: impl Into<amane::Size>) -> Rectangle {
    let colors = ColorsService::read();
    wrap(child, height).fill(colors.c.surface_container.as_str())
}
