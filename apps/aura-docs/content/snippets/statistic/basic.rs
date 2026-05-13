//! Basic Statistic values.

use aura_components::{Space, Statistic};
use gpui::IntoElement;

pub fn basic_statistics() -> impl IntoElement {
    Space::new().wrap().gap_lg().children(vec![
        Statistic::new("今日活跃用户", "114,514"),
        Statistic::new("总交易额", "¥ 9,999.00"),
    ])
}
