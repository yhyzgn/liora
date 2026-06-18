//! Custom Skeleton template with loaded content fallback.

use gpui::{AnyElement, Hsla, IntoElement};
use liora_components::{Avatar, Skeleton, SkeletonItem, SkeletonVariant, Space, Text};

pub fn templated_skeleton(loading: bool, avatar_bg: Hsla) -> Skeleton {
    Skeleton::new()
        .loading(loading)
        .template(|_, _| skeleton_template())
        .child(loaded_content(avatar_bg))
}

fn skeleton_template() -> AnyElement {
    Space::new()
        .align_start()
        .gap_lg()
        .child(SkeletonItem::new(SkeletonVariant::Circle))
        .child(
            Space::new()
                .vertical()
                .grow()
                .gap_sm()
                .child(SkeletonItem::new(SkeletonVariant::Paragraph).width_2_5())
                .child(Skeleton::new().rows(2)),
        )
        .into_any_element()
}

fn loaded_content(avatar_bg: Hsla) -> impl IntoElement {
    Space::new()
        .align_start()
        .gap_lg()
        .child(Avatar::new().background(avatar_bg))
        .child(
            Space::new()
                .vertical()
                .grow()
                .gap_sm()
                .child(Text::new("Zed Industries").bold())
                .child(Text::new("GPUI renders native Rust views on the GPU.")),
        )
}
