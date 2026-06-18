//! Basic Dropdown menu.

use liora_components::{Button, Dropdown, toast_info};
use liora_core::Placement;

pub fn basic_dropdown() -> Dropdown {
    Dropdown::new(Button::new("Actions"))
        .id("docs-dropdown-actions")
        .placement(Placement::BottomStart)
        .item("Create", |_, _| toast_info!("Create clicked"))
        .item("Duplicate", |_, _| toast_info!("Duplicate clicked"))
        .item("Archive", |_, _| toast_info!("Archive clicked"))
}

fn main() {
    let _ = basic_dropdown();
}
