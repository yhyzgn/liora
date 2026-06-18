//! Timeline nodes with semantic tones, hollow state, and icons.

use liora_components::{Card, Timeline, TimelineItem};
use liora_icons_lucide::IconName;

pub fn custom_timeline_nodes() -> Card {
    Card::new(
        Timeline::new()
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-01")
                    .content("成功状态")
                    .success(),
            )
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-02")
                    .content("警告状态")
                    .warning()
                    .hollow(true),
            )
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-03")
                    .content("错误状态")
                    .danger()
                    .icon(IconName::CircleX),
            )
            .item(
                TimelineItem::new()
                    .timestamp("2026-05-04")
                    .content("自定义图标")
                    .primary()
                    .icon(IconName::Star),
            ),
    )
}
