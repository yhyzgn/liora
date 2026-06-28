//! StatusBar advanced interactive layout example.

use gpui::IntoElement;
use liora_components::{StatusBar, StatusBarItem};
use liora_icons_lucide::IconName;

pub fn status_bar_advanced() -> impl IntoElement {
    StatusBar::new()
        .height(gpui::px(40.0))
        .borderless()
        .left_item(
            StatusBarItem::new("Deploy")
                .icon(IconName::Rocket)
                .dot()
                .min_width(gpui::px(108.0))
                .pill()
                .on_click(|_, _| {}),
        )
        .left_item(StatusBarItem::separator())
        .left_item(StatusBarItem::new("main").icon(IconName::GitBranch))
        .center_item(StatusBarItem::spacer())
        .right_item(
            StatusBarItem::new("Updates ready")
                .info()
                .icon(IconName::Download)
                .on_click(|_, _| {})
                .pill(),
        )
}
