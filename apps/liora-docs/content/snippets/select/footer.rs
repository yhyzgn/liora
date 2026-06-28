//! Searchable Select footer slot example.

use gpui::{Context, IntoElement};
use liora_components::{Button, SearchableListItem, Select};
use liora_icons_lucide::IconName;

pub fn select_with_footer(cx: &mut Context<Select>) -> Select {
    Select::searchable(
        vec![
            SearchableListItem::labeled("button", "Button"),
            SearchableListItem::labeled("select", "Select"),
        ],
        cx,
    )
    .placeholder("Create or select")
    .footer(|_, _| {
        Button::new("Create component")
            .small()
            .icon_start(IconName::Plus)
            .into_any_element()
    })
}

fn main() {}
