//! Tooltip keyboard focus, Escape dismissal, hover retention, and basic placements.

use gpui::IntoElement;
use liora_components::{Button, Space, Tooltip};
use liora_core::Placement;

pub fn basic_tooltips() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Space::new().wrap().gap_md().children(vec![
            Tooltip::new(Button::new("Focus or hover"))
                .content("Keyboard and hover help")
                .placement(Placement::Top),
            Tooltip::new(Button::new("Bottom"))
                .content("Prompt info")
                .placement(Placement::Bottom),
            Tooltip::new(Button::new("Left"))
                .content("Prompt info")
                .placement(Placement::Left),
            Tooltip::new(Button::new("Right"))
                .content("Prompt info")
                .placement(Placement::Right),
        ]))
        .child(Button::new("After tooltip"))
}
