//! Button radius presets.

use liora_components::{Button, Space};

fn rounded_buttons() -> Space {
    Space::new()
        .wrap()
        .gap_sm()
        .child(Button::new("4px").primary().rounded_sm())
        .child(Button::new("12px").primary().rounded_md())
        .child(Button::new("20px").primary().rounded_lg())
        .child(Button::new("Pill").primary().pill())
}

fn main() {
    let _ = rounded_buttons();
}
