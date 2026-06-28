//! Searchable Select example.

use gpui::Context;
use liora_components::{SearchableListItem, Select};

pub fn searchable_select(cx: &mut Context<Select>) -> Select {
    Select::searchable(
        vec![
            SearchableListItem::labeled("gpui", "GPUI").description("Native Rust UI runtime"),
            SearchableListItem::labeled("liora", "Liora").description("Component SDK"),
            SearchableListItem::labeled("iced", "Iced"),
            SearchableListItem::labeled("egui", "egui"),
        ],
        cx,
    )
    .placeholder("Choose framework")
}

fn main() {}
