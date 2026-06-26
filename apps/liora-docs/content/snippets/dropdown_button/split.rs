//! Split DropdownButton.

use liora_components::{DropdownButton, DropdownButtonItem, toast_info, toast_success};
use liora_icons_lucide::IconName;

pub fn split_dropdown_button() -> DropdownButton {
    DropdownButton::new("Deploy")
        .id("docs-dropdown-button-split")
        .primary()
        .split(true)
        .icon_start(IconName::Rocket)
        .on_click(|_, _| toast_success!("Deploy clicked"))
        .menu_item(
            DropdownButtonItem::new("Preview deployment", |_, _| {
                toast_info!("Preview deployment")
            })
            .icon(IconName::Eye),
        )
        .menu_item(
            DropdownButtonItem::new("Rollback", |_, _| toast_info!("Rollback"))
                .icon(IconName::Undo2),
        )
        .danger_item("Delete release", |_, _| toast_info!("Delete release"))
}

fn main() {
    let _ = split_dropdown_button();
}
