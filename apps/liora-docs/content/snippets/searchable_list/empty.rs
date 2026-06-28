//! SearchableList empty and limiting example.

use gpui::{IntoElement, px};
use liora_components::{SearchableList, SearchableListItem};

pub fn searchable_list_empty() -> impl IntoElement {
    SearchableList::new(vec![
        SearchableListItem::labeled("button", "Button"),
        SearchableListItem::labeled("input", "Input"),
        SearchableListItem::labeled("select-search", "Select::searchable"),
    ])
    .query("not-found")
    .max_items(2)
    .empty_text("No component found")
    .width(px(340.0))
}
