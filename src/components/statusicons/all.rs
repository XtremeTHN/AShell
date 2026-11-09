use crate::components::mrow::wrap;

use super::audio::audio_icon;
use super::network::network_icon;
use amane::{Align::Center, Rectangle, Row, Size::Parent, Text, children};

pub fn all_status() -> Rectangle {
    let r = Row::new(children!(network_icon(), audio_icon())).gap(4.0);
    wrap(r, Parent).align_child(Center, Center)
}
