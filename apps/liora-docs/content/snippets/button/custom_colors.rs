//! Button custom solid colors with derived hover/active/disabled states.

use gpui::{IntoElement, rgb};
use liora_components::{Button, ButtonColors, Space};

fn custom_color_buttons() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_sm()
        // Simple filled color: hover, active, and disabled colors are derived.
        .child(Button::new("Violet").custom_color(rgb(0x7c3aed).into(), gpui::white()))
        // Fully custom style helper for outline/emphasis buttons.
        .child(Button::new("Outline").colors(ButtonColors::outline(
            rgb(0x0891b2).into(),
            rgb(0x0f172a).into(),
            gpui::transparent_black(),
        )))
        // Disabled state keeps the same API and derives muted colors.
        .child(
            Button::new("Disabled")
                .custom_color(rgb(0xdb2777).into(), gpui::white())
                .disabled(true),
        )
}

fn main() {
    let _ = custom_color_buttons();
}
