//! Statistic prefix and suffix elements.

use aura_components::{Space, Statistic};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::IntoElement;

pub fn affix_statistics() -> impl IntoElement {
    Space::new().wrap().gap_lg().children(vec![
        Statistic::new("增长率", "12.5").suffix(Icon::new(IconName::TrendingUp)),
        Statistic::new("月活下降", "5.2").suffix(Icon::new(IconName::TrendingDown)),
        Statistic::new("待办事项", "12").prefix(Icon::new(IconName::ListTodo)),
    ])
}
