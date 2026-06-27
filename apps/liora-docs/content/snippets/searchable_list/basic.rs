//! Basic grouped SearchableList example.

use gpui::{IntoElement, px};
use liora_components::{SearchableList, SearchableListItem};

pub fn searchable_list_basic() -> impl IntoElement {
    SearchableList::new(vec![
        SearchableListItem::labeled("button", "Button")
            .description("Primary actions")
            .group("Basic"),
        SearchableListItem::labeled("input", "Input")
            .description("Text entry")
            .group("Basic"),
        SearchableListItem::labeled("combobox", "Combobox")
            .description("Searchable select")
            .group("Input"),
    ])
    .selected("combobox")
    .width(px(340.0))
}
