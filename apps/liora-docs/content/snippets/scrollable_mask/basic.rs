use gpui::{IntoElement, px};
use liora_components::{ScrollableMask, Space, Text};

pub fn scrollable_mask_basic() -> impl IntoElement {
    ScrollableMask::new(
        Space::new()
            .vertical()
            .gap_sm()
            .children((1..=16).map(|index| Text::new(format!("Scrollable row {index}")))),
    )
    .height(px(160.0))
}
