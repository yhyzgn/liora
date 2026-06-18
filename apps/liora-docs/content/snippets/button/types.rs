//! Button visual variants.

use liora_components::{Button, Space};

fn button_types() -> Space {
    // Each variant maps to Liora theme tokens while staying fully native GPUI.
    Space::new()
        .wrap()
        .gap_sm()
        .child(Button::new("Default"))
        .child(Button::new("Tertiary").tertiary())
        .child(Button::new("Primary").primary())
        .child(Button::new("Info").info())
        .child(Button::new("Success").success())
        .child(Button::new("Warning").warning())
        .child(Button::new("Danger").danger())
}

fn main() {
    let _ = button_types();
}
