//! Dropdown close policy.

use aura_components::{Button, Dropdown, toast_info};
use aura_core::Placement;

pub fn manual_close_dropdown() -> Dropdown {
    Dropdown::new(Button::new("Manual close menu"))
        .id("docs-dropdown-manual-close")
        .placement(Placement::BottomStart)
        // Keep the menu open when the user clicks outside. Menu item clicks
        // still close the underlying Popover after the item callback runs.
        .close_on_click_outside(false)
        .close_on_escape(false)
        .item("Save draft", |_, _| toast_info!("Save draft"))
        .item("Duplicate", |_, _| toast_info!("Duplicate"))
}

fn main() {
    let _ = manual_close_dropdown();
}
