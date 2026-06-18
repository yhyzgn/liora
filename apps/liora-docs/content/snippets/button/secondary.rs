//! Secondary Button variants.

use liora_components::{Button, Space};

fn secondary_buttons() -> Space {
    Space::new()
        .wrap()
        .gap_sm()
        .child(Button::new("Default").secondary())
        .child(Button::new("Primary").primary().secondary())
        .child(Button::new("Info").info().secondary())
        .child(Button::new("Success").success().secondary())
        .child(Button::new("Warning").warning().secondary())
        .child(Button::new("Danger").danger().secondary())
}

fn main() {
    let _ = secondary_buttons();
}
