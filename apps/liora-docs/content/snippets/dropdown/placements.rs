//! Dropdown placement variants.

use gpui::IntoElement;
use liora_components::{Button, Dropdown, Space, toast_info};
use liora_core::Placement;

pub fn dropdown_placements() -> impl IntoElement {
    Space::new().wrap().gap_md().children([
        menu("docs-dropdown-top", "Top", Placement::Top),
        menu("docs-dropdown-bottom", "Bottom", Placement::Bottom),
        menu("docs-dropdown-left", "Left", Placement::Left),
        menu("docs-dropdown-right", "Right", Placement::Right),
    ])
}

fn menu(id: &'static str, label: &'static str, placement: Placement) -> Dropdown {
    Dropdown::new(Button::new(label))
        .id(id)
        .placement(placement)
        .item("Action 1", |_, _| toast_info!("Action 1"))
        .item("Action 2", |_, _| toast_info!("Action 2"))
}
