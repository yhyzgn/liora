//! StatusBar semantic tone example.

use gpui::IntoElement;
use liora_components::{StatusBar, StatusBarItem};
use liora_icons_lucide::IconName;

pub fn status_bar_tones() -> impl IntoElement {
    StatusBar::new()
        .left_item(
            StatusBarItem::new("Connected")
                .success()
                .icon(IconName::Wifi)
                .detail("42ms")
                .pill(),
        )
        .left_item(
            StatusBarItem::new("Queue")
                .warning()
                .icon(IconName::Clock3)
                .detail("3 jobs"),
        )
        .center_item(
            StatusBarItem::new("Preview mode")
                .primary()
                .icon(IconName::Monitor),
        )
        .right_item(
            StatusBarItem::new("Offline cache")
                .danger()
                .icon(IconName::WifiOff)
                .pill(),
        )
}
