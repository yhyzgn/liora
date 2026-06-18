//! Space examples inside a container page.

use gpui::IntoElement;
use liora_components::{Button, Space};

pub fn container_space_examples() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(
            Space::new()
                .child(Button::new("Button 1"))
                .child(Button::new("Button 2"))
                .child(Button::new("Button 3")),
        )
        .child(
            Space::new()
                .vertical()
                .child(Button::new("Vertical 1").primary())
                .child(Button::new("Vertical 2").primary()),
        )
}
