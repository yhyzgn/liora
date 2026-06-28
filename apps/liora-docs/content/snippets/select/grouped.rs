//! Grouped searchable Select example.

use gpui::Context;
use liora_components::{SearchableListItem, Select};

pub fn grouped_select(cx: &mut Context<Select>) -> Select {
    Select::searchable(
        vec![
            SearchableListItem::labeled("button", "Button").group("Basic"),
            SearchableListItem::labeled("input", "Input").group("Basic"),
            SearchableListItem::labeled("select", "Select").group("Input"),
            SearchableListItem::labeled("sidebar", "Sidebar").group("Shell"),
            SearchableListItem::labeled("dock-layout", "DockLayout")
                .group("Shell")
                .disabled(true),
        ],
        cx,
    )
    .placeholder("Search components")
    .width(gpui::px(340.0))
}

fn main() {}
