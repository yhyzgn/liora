//! Tooltip basic placements.

use aura_components::{Button, Space, Tooltip};
use aura_core::Placement;
use gpui::IntoElement;

pub fn basic_tooltips() -> impl IntoElement {
    Space::new().wrap().gap_md().children(vec![
        Tooltip::new(Button::new("Top"))
            .content("Prompt info")
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
    ])
}
