//! Application shell status bar example.

use gpui::IntoElement;
use liora_components::{StatusBar, StatusBarItem};
use liora_icons_lucide::IconName;

pub fn status_bar_shell() -> impl IntoElement {
    StatusBar::new()
        .left_item(
            StatusBarItem::new("Ready")
                .success()
                .icon(IconName::CircleCheck)
                .pill(),
        )
        .left_item(StatusBarItem::new("Syncing").loading(true).info())
        .center_item(
            StatusBarItem::new("src/main.rs")
                .primary()
                .icon(IconName::FileCode),
        )
        .right_item(StatusBarItem::new("UTF-8").compact())
        .right_item(StatusBarItem::new("Ln 42, Col 7").compact())
        .right_item(StatusBarItem::new("v0.1.18").pill())
}
