use super::{Compare, threshold};
use crate::components::icon::glyph;
use amane::{Align::Center, Audio, Rectangle, Service};

pub fn audio_icon_string<T>(volume: T) -> String
where
    T: PartialOrd + From<u8>,
{
    let thresholds = [T::from(1), T::from(60)];
    let icon = threshold(
        volume,
        &thresholds,
        vec!["volume_mute", "volume_down"],
        "volume_up",
        Compare::LESS,
    );

    String::from(icon)
}
pub fn audio_icon() -> Rectangle {
    let audio = Audio::read();

    let icon_name = audio_icon_string(audio.volume());
    Rectangle::new()
        .width(16.0)
        .height(22.0)
        .align_child(Center, Center)
        .child(glyph(icon_name).size(16.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_volume_icon_by_level() {
        assert_eq!(audio_icon_string(0), "volume_mute");
        assert_eq!(audio_icon_string(1), "volume_down");
        assert_eq!(audio_icon_string(59), "volume_down");
        assert_eq!(audio_icon_string(60), "volume_up");
        assert_eq!(audio_icon_string(100), "volume_up");
    }
}
