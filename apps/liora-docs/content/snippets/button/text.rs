//! Text Button variants for low-emphasis actions.

use liora_components::{Button, Space};

fn text_buttons() -> Space {
    Space::new()
        .wrap()
        .gap_sm()
        .child(Button::new("Default").text())
        .child(Button::new("Primary").primary().text())
        .child(Button::new("Info").info().text())
        .child(Button::new("Success").success().text())
        .child(Button::new("Warning").warning().text())
        .child(Button::new("Danger").danger().text())
}

fn main() {
    let _ = text_buttons();
}
