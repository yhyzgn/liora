//! A bounded native menu with wheel and keyboard scrolling.

use liora_components::{Button, Dropdown, toast_info};

pub fn scrollable_dropdown() -> Dropdown {
    let mut menu = Dropdown::new(Button::new("Long menu")).id("docs-dropdown-scrollable");
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
    let _ = scrollable_dropdown();
}
