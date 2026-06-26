//! Basic DropdownButton menu.

use liora_components::{DropdownButton, toast_info, toast_success};

pub fn basic_dropdown_button() -> DropdownButton {
    DropdownButton::new("Actions")
        .id("docs-dropdown-button-actions")
        .primary()
        .item("Create task", |_, _| toast_success!("Create task"))
        .item("Duplicate", |_, _| toast_info!("Duplicate"))
        .item("Archive", |_, _| toast_info!("Archive"))
}

fn main() {
    let _ = basic_dropdown_button();
}
