//! Horizontal and vertical Space examples.

use gpui::IntoElement;
use liora_components::{Button, Space, Text};

pub fn space_examples() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(Text::new("Horizontal gap (default 8px):"))
        .child(
            Space::new()
                .child(Button::new("Button 1"))
                .child(Button::new("Button 2"))
                .child(Button::new("Button 3")),
        )
        .child(Text::new("Vertical gap:"))
        .child(
            Space::new()
                .vertical()
                .gap_xl()
                .child(Button::new("Vertical 1").primary())
                .child(Button::new("Vertical 2").primary()),
        )
}
