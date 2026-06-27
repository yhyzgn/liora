//! Basic Combobox example.

use gpui::Context;
use liora_components::{Combobox, SearchableListItem};

pub fn combobox_basic(cx: &mut Context<Combobox>) -> Combobox {
    Combobox::new(
        vec![
            SearchableListItem::labeled("gpui", "GPUI").description("Native Rust UI runtime"),
            SearchableListItem::labeled("liora", "Liora").description("Component SDK"),
            SearchableListItem::labeled("iced", "Iced").description("Cross-platform Rust GUI"),
        ],
        cx,
    )
    .placeholder("Choose framework")
}
