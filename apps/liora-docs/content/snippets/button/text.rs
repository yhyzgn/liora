//! Text Button variants for low-emphasis actions.

use liora_components::{Button, Space};

fn text_buttons() -> Space {
    Space::new()
        .wrap()
        .gap_sm()
        .child(Button::new("Default").id("button-text-default").text())
        .child(Button::new("Primary").id("button-text-primary").primary().text())
        .child(Button::new("Info").id("button-text-info").info().text())
        .child(Button::new("Success").id("button-text-success").success().text())
        .child(Button::new("Warning").id("button-text-warning").warning().text())
        .child(Button::new("Danger").id("button-text-danger").danger().text())
}

fn main() {
    let _ = text_buttons();
}
