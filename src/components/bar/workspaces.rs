use crate::services::colors::ColorsService;
use amane::{
    Align, Animation, Radius, Rectangle, Row, Service, Widget, Workspace, Workspaces,
};
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
    time::Duration,
};

use super::bar_container::bar_container;

static WIDTHS: OnceLock<Mutex<HashMap<i64, Animation>>> = OnceLock::new();

fn item(wk: &Workspace) -> Rectangle {
    let widths = WIDTHS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut widths = widths.lock().expect("workspace width animations poisoned");

    let animation = widths
        .entry(wk.id())
        .or_insert_with(|| Animation::new(14.0).duration(Duration::from_millis(180)));
    animation.to(if wk.focused() { 32.0 } else { 14.0 });
    let width = animation.value();

    let colors = ColorsService::read();
    let background_color = if wk.focused() {
        colors.c.primary.as_str()
    } else {
        colors.c.primary_container.as_str()
    };

    Rectangle::new()
        .width(width)
        .height(14.0)
        .radius(Radius::Full)
        .fill(background_color)
}
pub fn workspaces() -> Rectangle {
    let wkspcs = Workspaces::read();

    let mut items: Vec<Box<dyn Widget>> = vec![];

    for x in wkspcs.list() {
        items.push(Box::new(item(x)));
    }

    let row = Row::new(items).gap(3.0);
    bar_container(row, 30.0).align_child(Align::Start, Align::Center)
}
