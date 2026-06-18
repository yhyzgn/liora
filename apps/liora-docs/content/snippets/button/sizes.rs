//! Button size presets.

use liora_components::{Button, Space};

fn sized_buttons() -> Space {
    Space::new()
        .wrap()
        .gap_sm()
        .child(Button::new("Small").primary().small())
        .child(Button::new("Default").primary())
        .child(Button::new("Large").primary().large())
}

fn main() {
    let _ = sized_buttons();
}
