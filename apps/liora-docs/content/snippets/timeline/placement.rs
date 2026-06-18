//! Timeline timestamp placement variants.

use liora_components::{Card, Timeline, TimelineItem, TimelinePlacement};

pub fn timeline_timestamp_placement() -> Card {
    Card::new(
        Timeline::new()
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-01")
                    .content("时间戳在顶部")
                    .placement(TimelinePlacement::Top),
            )
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-02")
                    .content("时间戳在底部")
                    .placement(TimelinePlacement::Bottom),
            ),
    )
}
