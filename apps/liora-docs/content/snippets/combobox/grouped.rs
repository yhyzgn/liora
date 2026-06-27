//! Grouped Combobox example.

use gpui::{Context, px};
use liora_components::{Combobox, SearchableListItem};

pub fn combobox_grouped(cx: &mut Context<Combobox>) -> Combobox {
    Combobox::new(
        vec![
            SearchableListItem::labeled("button", "Button").group("Basic"),
            SearchableListItem::labeled("input", "Input").group("Basic"),
            SearchableListItem::labeled("combobox", "Combobox").group("Input"),
            SearchableListItem::labeled("sidebar", "Sidebar").group("Shell"),
            SearchableListItem::labeled("dock-layout", "DockLayout")
                .group("Shell")
                .disabled(true),
        ],
        cx,
    )
    .placeholder("Search components")
    .width(px(340.0))
}
