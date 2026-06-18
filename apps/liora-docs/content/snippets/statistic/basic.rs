//! Basic Statistic values.

use gpui::IntoElement;
use liora_components::{Space, Statistic};

pub fn basic_statistics() -> impl IntoElement {
    Space::new().wrap().gap_lg().children(vec![
        Statistic::new("今日活跃用户", "114,514"),
        Statistic::new("总交易额", "¥ 9,999.00"),
    ])
}
