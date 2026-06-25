//! Spinner composition examples.

use gpui::IntoElement;
use liora_components::{Button, Label, Space, Spinner};

pub fn spinner_composition() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_lg()
        .child(
            Button::new("Syncing")
                .primary()
                .icon_start(Spinner::new().small().into_any_element()),
        )
        .child(Label::new("Fetching metrics").custom_icon(Spinner::new().small()))
}
