//! Tooltip basic placements.

use gpui::IntoElement;
use liora_components::{Button, Space, Tooltip};
use liora_core::Placement;

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
