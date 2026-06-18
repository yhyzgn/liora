//! Popconfirm placement variants.

use gpui::IntoElement;
use liora_components::{Button, Popconfirm, Space};
use liora_core::Placement;

pub fn popconfirm_placements() -> impl IntoElement {
    Space::new().wrap().gap_md().children([
        confirm_at("Top", Placement::Top),
        confirm_at("Bottom", Placement::Bottom),
        confirm_at("Left", Placement::Left),
        confirm_at("Right", Placement::Right),
        confirm_at("BottomEnd", Placement::BottomEnd),
    ])
}

fn confirm_at(label: &'static str, placement: Placement) -> Popconfirm {
    Popconfirm::new(Button::new(label).small())
        .id(format!("docs-popconfirm-placement-{label}"))
        .title(format!("Confirm at {placement:?}?"))
        .placement(placement)
}
