//! StatusBar custom region example.

use gpui::IntoElement;
use liora_components::{Button, Space, StatusBar, StatusBarItem};
use liora_icons_lucide::IconName;

pub fn status_bar_custom() -> impl IntoElement {
    StatusBar::new()
        .height(gpui::px(38.0))
        .left_item(StatusBarItem::new("Workspace: Liora").icon(IconName::FolderOpen))
        .center_item(StatusBarItem::custom(
            Space::new()
                .gap_sm()
                .child(
                    Button::new("Run")
                        .small()
                        .primary()
                        .icon_start(IconName::Play),
                )
                .child(Button::new("Build").small().icon_start(IconName::Hammer)),
        ))
        .right_item(StatusBarItem::new("Native GPUI").info().pill())
}
