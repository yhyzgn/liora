//! Basic grouped SearchableList example.

use gpui::{App, AppContext, Entity, px};
use liora_components::{SearchableListItem, SearchableListPanel};

pub fn searchable_list_basic(cx: &mut App) -> Entity<SearchableListPanel> {
    cx.new(|cx| {
        SearchableListPanel::new(
            vec![
                SearchableListItem::labeled("button", "Button")
                    .description("Primary actions")
                    .group("Basic"),
                SearchableListItem::labeled("input", "Input")
                    .description("Text entry")
                    .group("Basic"),
                SearchableListItem::labeled("select-search", "Select::searchable")
                    .description("Searchable select")
                    .group("Input"),
            ],
            cx,
        )
        .selected("select-search")
        .placeholder("Search components...")
        .width(px(340.0))
    })
}
