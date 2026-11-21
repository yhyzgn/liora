//! A bounded native menu with wheel and keyboard scrolling.

use liora_components::{DropdownButton, toast_info};

pub fn scrollable_dropdown_button() -> DropdownButton {
    let mut menu = DropdownButton::new("Long button menu").id("docs-dropdown-button-scrollable");
    for label in [
        "Action 01",
        "Action 02",
        "Action 03",
        "Action 04",
        "Action 05",
        "Action 06",
        "Action 07",
        "Action 08",
        "Action 09",
        "Action 10",
        "Action 11",
        "Last action",
    ] {
        menu = menu.item(label, move |_, _| toast_info!("{label}"));
    }
    menu
}

fn main() {
    let _ = scrollable_dropdown_button();
}
