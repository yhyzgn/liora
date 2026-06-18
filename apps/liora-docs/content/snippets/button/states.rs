//! Button disabled and loading states.

use liora_components::{Button, Space};

fn state_buttons() -> Space {
    Space::new()
        .wrap()
        .gap_sm()
        .child(Button::new("Disabled").primary().disabled(true))
        .child(Button::new("Loading").primary().loading(true))
        .child(Button::new("Saving").success().loading(true))
}

fn main() {
    let _ = state_buttons();
}
