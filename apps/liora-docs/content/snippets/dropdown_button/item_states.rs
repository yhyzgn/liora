//! DropdownButton menu item states.

use liora_components::{DropdownButton, DropdownButtonItem, toast_info};
use liora_icons_lucide::IconName;

pub fn dropdown_button_item_states() -> DropdownButton {
    DropdownButton::new("Item states")
        .id("docs-dropdown-button-item-states")
        .menu_item(
            DropdownButtonItem::new("Rename", |_, _| toast_info!("Rename")).icon(IconName::Pencil),
        )
        .menu_item(
            DropdownButtonItem::new("Move", |_, _| toast_info!("Move")).icon(IconName::FolderInput),
        )
        .disabled_item("No permission")
        .danger_item("Delete permanently", |_, _| {
            toast_info!("Delete permanently")
        })
}

fn main() {
    let _ = dropdown_button_item_states();
}
