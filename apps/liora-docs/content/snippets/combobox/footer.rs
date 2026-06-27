//! Combobox footer slot example.

use gpui::{Context, IntoElement, px};
use liora_components::{Combobox, SearchableListItem, combobox_create_footer};

pub fn combobox_footer(cx: &mut Context<Combobox>) -> Combobox {
    Combobox::new(
        vec![
            SearchableListItem::labeled("button", "Button"),
            SearchableListItem::labeled("combobox", "Combobox"),
            SearchableListItem::labeled("status-bar", "StatusBar"),
        ],
        cx,
    )
    .placeholder("Create or select")
    .width(px(340.0))
    .footer(|_, _| combobox_create_footer("Create component").into_any_element())
}
