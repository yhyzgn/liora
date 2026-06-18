//! Basic Timeline events in chronological order.

use liora_components::{Card, Timeline, TimelineItem};

pub fn basic_timeline() -> Card {
    Card::new(
        Timeline::new()
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-01")
                    .content("创建成功"),
            )
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-02")
                    .content("通过审核"),
            )
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-03")
                    .content("项目发布"),
            ),
    )
}
