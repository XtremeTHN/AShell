use crate::components::mrow::wrap;

use super::audio::audio_icon;
use super::battery::battery_icon;
use super::network::network_icon;
use amane::{Align::Center, Rectangle, Row, Size::Parent, children};

pub fn all_status() -> Rectangle {
    let r = Row::new(children!(
        battery_icon(28.0, 16.0),
        network_icon(),
        audio_icon()
    ))
    .align(Center)
    .gap(4.0);
    wrap(r, Parent).align_child(Center, Center)
}
