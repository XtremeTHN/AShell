use amane::{Rectangle, Row};

use super::{QUICKSETTINGS_MAX_PANEL_WIDTH, TOP_MAX_HEIGHT};
use crate::components::{
    button::button,
    icon::glyph,
    mrow::row,
    popup::{menu::PopupMenuItem, state::PopupState},
};
use crate::services::logind::Logind;

const ITEMS_OWNER: &str = "quicksettings-power-btt";

const SCREEN_W: f32 = 1920.0;
const PANEL_MARGIN_RIGHT: f32 = 10.0; // Margin.right in quicksettings::view
const PANEL_MARGIN_TOP: f32 = 10.0; // Margin.top in quicksettings::view
const PANEL_PADDING: f32 = 20.0; // .padding(20.0) in content()
const POPUP_WIDTH: f32 = 140.0; // 120 button + 2 * 10 padding in popup()
const POPUP_GAP: f32 = 4.0;

fn power_btt_menu() -> Rectangle {
    let panel_x = SCREEN_W - QUICKSETTINGS_MAX_PANEL_WIDTH - PANEL_MARGIN_RIGHT;
    let content_right = panel_x + QUICKSETTINGS_MAX_PANEL_WIDTH - PANEL_PADDING;

    let popup_x = content_right - POPUP_WIDTH + 10.0;
    let popup_y = PANEL_MARGIN_TOP + PANEL_PADDING + TOP_MAX_HEIGHT + POPUP_GAP + 40.0;

    button(
        glyph("keyboard_arrow_down").size(18.0),
        TOP_MAX_HEIGHT,
        "quicksettings-power-more-bt",
        Some(move || {
            PopupState::toggle(
                ITEMS_OWNER,
                vec![
                    PopupMenuItem::new(ITEMS_OWNER)
                        .icon_name("restart_alt")
                        .label("Reboot"),
                    PopupMenuItem::new(ITEMS_OWNER)
                        .icon_name("bedtime")
                        .label("Suspend"),
                    PopupMenuItem::new(ITEMS_OWNER)
                        .icon_name("logout")
                        .label("Log Out"),
                ],
                popup_x,
                popup_y,
            )
        }),
    )
}
fn power_btt() -> Rectangle {
    button(
        glyph("mode_off_on").size(16.0),
        TOP_MAX_HEIGHT,
        "quicksettings-power-btt",
        Some(|| {}),
    )
}

pub fn power_group() -> Row {
    row(vec![power_btt(), power_btt_menu()], 10.0)
}
