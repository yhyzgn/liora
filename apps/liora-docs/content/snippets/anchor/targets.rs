//! AnchorTarget reports section bounds back to Anchor.

use gpui::{App, Entity, IntoElement};
use liora_components::{Anchor, AnchorTarget, Flex, Text};
use liora_core::Config;

pub fn anchor_target_panel(cx: &mut App, anchor: Entity<Anchor>) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();

    AnchorTarget::new(
        "basic",
        anchor,
        Flex::new()
            .height_units(240.0)
            .bg(theme.neutral.hover)
            .rounded_md()
            .center()
            .child(Text::new("基础用法内容区域")),
    )
}
