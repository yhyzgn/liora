//! SearchableList filtering example.

use gpui::{App, AppContext, Entity, px};
use liora_components::{SearchableListItem, SearchableListPanel};

pub fn searchable_list_filtered(cx: &mut App) -> Entity<SearchableListPanel> {
    cx.new(|cx| {
        SearchableListPanel::new(
            vec![
                SearchableListItem::labeled("button", "Button").group("Basic"),
                SearchableListItem::labeled("sidebar", "Sidebar").group("Shell"),
                SearchableListItem::labeled("status-bar", "StatusBar").group("Shell"),
                SearchableListItem::labeled("dock-layout", "DockLayout")
                    .group("Shell")
                    .disabled(true),
            ],
            cx,
        )
        .placeholder("Type shell...")
        .selected_values(vec!["status-bar"])
        .width(px(340.0))
    })
}
