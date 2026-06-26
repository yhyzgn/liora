//! DropdownButton sizes, placement, and close policy.

use gpui::IntoElement;
use liora_components::{DropdownButton, Space, toast_info};
use liora_core::Placement;

pub fn dropdown_button_sizes_and_policy() -> impl IntoElement {
    Space::new().wrap().gap_md().children([
        DropdownButton::new("Small")
            .id("docs-dropdown-button-small")
            .small()
            .item("Action", |_, _| toast_info!("Small action")),
        DropdownButton::new("Large top")
            .id("docs-dropdown-button-large-top")
            .large()
            .warning()
            .secondary()
            .placement(Placement::TopEnd)
            .item("Action", |_, _| toast_info!("Large action")),
        DropdownButton::new("Manual close")
            .id("docs-dropdown-button-manual-close")
            .close_on_click_outside(false)
            .close_on_escape(false)
            .item("Item click still closes", |_, _| toast_info!("Item click")),
    ])
}
