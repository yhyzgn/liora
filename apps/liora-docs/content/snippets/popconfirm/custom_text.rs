//! Popconfirm custom button text and escape policy.

use gpui::IntoElement;
use liora_components::{Button, Popconfirm, Space};
use liora_core::Placement;

pub fn popconfirm_custom_text() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(
            Popconfirm::new(Button::new("Publish").success())
                .id("docs-popconfirm-publish")
                .title("Publish current draft?")
                .confirm_text("Publish")
                .cancel_text("Keep editing")
                .placement(Placement::Top),
        )
        .child(
            Popconfirm::new(Button::new("Danger action").danger())
                .id("docs-popconfirm-danger")
                .title("This action cannot be undone.")
                .confirm_text("I understand")
                .cancel_text("Abort")
                .close_on_escape(false)
                .close_on_click_outside(false)
                .placement(Placement::BottomStart),
        )
}
