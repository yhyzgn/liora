//! Button gradients with automatic hover/active/disabled derivation.

use gpui::{IntoElement, rgb};
use liora_components::{Button, Space};

fn gradient_buttons() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_sm()
        .child(
            Button::new("Aurora")
                .gradient(rgb(0x6366f1).into(), rgb(0x06b6d4).into())
                .pill(),
        )
        .child(
            Button::new("Sunset")
                .gradient_with_angle(110.0, rgb(0xf97316).into(), rgb(0xec4899).into())
                .large()
                .rounded_lg(),
        )
        .child(
            Button::new("Loading")
                .gradient(rgb(0x22c55e).into(), rgb(0x14b8a6).into())
                .loading(true),
        )
        .child(
            Button::new("Disabled")
                .gradient(rgb(0x8b5cf6).into(), rgb(0x3b82f6).into())
                .disabled(true),
        )
}

fn main() {
    let _ = gradient_buttons();
}
