//! Multiple Combobox example.

use gpui::{Context, px};
use liora_components::{Combobox, SearchableListItem};

pub fn combobox_multiple(cx: &mut Context<Combobox>) -> Combobox {
    Combobox::new(
        vec![
            SearchableListItem::labeled("button", "Button").group("Basic"),
            SearchableListItem::labeled("input", "Input").group("Basic"),
            SearchableListItem::labeled("combobox", "Combobox").group("Input"),
            SearchableListItem::labeled("status-bar", "StatusBar").group("Shell"),
        ],
        cx,
    )
    .multiple()
    .selected_values(vec!["button", "combobox"])
    .placeholder("Pick multiple components")
    .width(px(340.0))
}
