//! Skeleton item variants for common placeholder shapes.

use aura_components::{SkeletonItem, SkeletonVariant, Space};
use gpui::IntoElement;

pub fn skeleton_variants() -> impl IntoElement {
    Space::new().gap_lg().children(vec![
        SkeletonItem::new(SkeletonVariant::Circle),
        SkeletonItem::new(SkeletonVariant::Square),
        SkeletonItem::new(SkeletonVariant::Image),
    ])
}
