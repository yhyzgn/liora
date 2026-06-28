//! SearchableList empty and limiting example.

use gpui::{App, AppContext, Entity, px};
use liora_components::{SearchableListItem, SearchableListPanel};

pub fn searchable_list_empty(cx: &mut App) -> Entity<SearchableListPanel> {
    cx.new(|cx| {
        SearchableListPanel::new(
            vec![
                SearchableListItem::labeled("button", "Button"),
                SearchableListItem::labeled("input", "Input"),
                SearchableListItem::labeled("select-search", "Select::searchable"),
            ],
            cx,
        )
        .placeholder("Try not-found...")
        .max_items(2)
        .empty_text("No component found")
        .width(px(340.0))
    })
}
