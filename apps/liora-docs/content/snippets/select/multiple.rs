//! Multiple searchable Select example.

use gpui::Context;
use liora_components::{SearchableListItem, Select};

pub fn multiple_select(cx: &mut Context<Select>) -> Select {
    Select::searchable(
        vec![
            SearchableListItem::labeled("button", "Button").group("Basic"),
            SearchableListItem::labeled("input", "Input").group("Basic"),
            SearchableListItem::labeled("select-search", "Searchable Select").group("Input"),
        ],
        cx,
    )
    .multiple()
    .selected_values(vec!["button", "select-search"])
    .placeholder("Pick multiple components")
    .width(gpui::px(340.0))
}

fn main() {}
