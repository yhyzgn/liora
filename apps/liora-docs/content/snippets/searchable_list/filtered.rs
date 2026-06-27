//! SearchableList filtering example.

use gpui::{IntoElement, px};
use liora_components::{SearchableList, SearchableListItem};

pub fn searchable_list_filtered() -> impl IntoElement {
    SearchableList::new(vec![
        SearchableListItem::labeled("button", "Button").group("Basic"),
        SearchableListItem::labeled("sidebar", "Sidebar").group("Shell"),
        SearchableListItem::labeled("status-bar", "StatusBar").group("Shell"),
        SearchableListItem::labeled("dock-layout", "DockLayout")
            .group("Shell")
            .disabled(true),
    ])
    .query("shell")
    .selected_values(vec!["status-bar"])
    .width(px(340.0))
}
