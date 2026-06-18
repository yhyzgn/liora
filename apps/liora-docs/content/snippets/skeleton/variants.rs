//! Skeleton item variants for common placeholder shapes.

use gpui::IntoElement;
use liora_components::{SkeletonItem, SkeletonVariant, Space};

pub fn skeleton_variants() -> impl IntoElement {
    Space::new().gap_lg().children(vec![
        SkeletonItem::new(SkeletonVariant::Circle),
        SkeletonItem::new(SkeletonVariant::Square),
        SkeletonItem::new(SkeletonVariant::Image),
    ])
}
