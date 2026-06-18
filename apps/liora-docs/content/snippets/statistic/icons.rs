//! Built-in Statistic icon helpers.

use gpui::IntoElement;
use liora_components::{Space, Statistic};
use liora_icons_lucide::IconName;

pub fn icon_statistics() -> impl IntoElement {
    Space::new().wrap().gap_lg().children(vec![
        Statistic::new("转化率", "68%")
            .value_color(gpui::green())
            .icon(IconName::TrendingUp),
        Statistic::new("告警数", "7")
            .icon(IconName::Bell)
            .icon_left()
            .icon_color(gpui::red()),
        Statistic::new("完成率", "92%")
            .icon(IconName::CircleCheck)
            .icon_right()
            .icon_color(gpui::blue()),
    ])
}
