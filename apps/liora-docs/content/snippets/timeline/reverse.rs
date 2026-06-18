//! Reverse Timeline for latest-first event feeds.

use liora_components::{Card, Timeline, TimelineItem};

pub fn reversed_timeline() -> Card {
    Card::new(
        Timeline::new()
            .reverse(true)
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-01")
                    .content("事件 1"),
            )
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-02")
                    .content("事件 2"),
            )
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-03")
                    .content("事件 3"),
            ),
    )
}
