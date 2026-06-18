//! Statistic prefix and suffix elements.

use gpui::IntoElement;
use liora_components::{Space, Statistic};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn affix_statistics() -> impl IntoElement {
    Space::new().wrap().gap_lg().children(vec![
        Statistic::new("增长率", "12.5").suffix(Icon::new(IconName::TrendingUp)),
        Statistic::new("月活下降", "5.2").suffix(Icon::new(IconName::TrendingDown)),
        Statistic::new("待办事项", "12").prefix(Icon::new(IconName::ListTodo)),
    ])
}
