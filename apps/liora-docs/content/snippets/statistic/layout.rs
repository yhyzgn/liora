//! Horizontal Statistic layouts inside cards.

use gpui::IntoElement;
use liora_components::{Card, Space, Statistic};
use liora_icons_lucide::IconName;

pub fn statistic_layouts() -> impl IntoElement {
    Space::new().wrap().gap_md().children(vec![
        Card::new(
            Statistic::new("紧凑水平", "1,280")
                .icon(IconName::Activity)
                .horizontal_compact(),
        )
        .width_lg(),
        Card::new(
            Statistic::new("两端对齐", "¥ 86,420")
                .icon(IconName::Wallet)
                .icon_left()
                .horizontal_between(),
        )
        .width_lg(),
    ])
}
