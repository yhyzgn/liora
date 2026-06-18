//! Popover placement variants.

use gpui::IntoElement;
use liora_components::{Button, Popover, Space, Text};
use liora_core::Placement;

pub fn popover_placements() -> impl IntoElement {
    Space::new().wrap().gap_sm().children([
        placement_popover("TopStart", Placement::TopStart),
        placement_popover("Top", Placement::Top),
        placement_popover("TopEnd", Placement::TopEnd),
        placement_popover("Left", Placement::Left),
        placement_popover("Right", Placement::Right),
        placement_popover("BottomStart", Placement::BottomStart),
        placement_popover("Bottom", Placement::Bottom),
        placement_popover("BottomEnd", Placement::BottomEnd),
    ])
}

fn placement_popover(label: &'static str, placement: Placement) -> Popover {
    Popover::new(Button::new(label).small())
        .id(format!("docs-popover-placement-{label}"))
        .placement(placement)
        .content(move |_, _| Text::new(format!("Placement: {placement:?}")))
}
