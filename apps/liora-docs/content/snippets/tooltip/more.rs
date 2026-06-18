//! Tooltip start/end aligned placements.

use gpui::IntoElement;
use liora_components::{Button, Space, Tooltip};
use liora_core::Placement;

pub fn aligned_tooltips() -> impl IntoElement {
    Space::new().wrap().gap_md().children(vec![
        Tooltip::new(Button::new("Top Start"))
            .content("Top Start")
            .placement(Placement::TopStart),
        Tooltip::new(Button::new("Top End"))
            .content("Top End")
            .placement(Placement::TopEnd),
        Tooltip::new(Button::new("Bottom Start"))
            .content("Bottom Start")
            .placement(Placement::BottomStart),
        Tooltip::new(Button::new("Bottom End"))
            .content("Bottom End")
            .placement(Placement::BottomEnd),
    ])
}
