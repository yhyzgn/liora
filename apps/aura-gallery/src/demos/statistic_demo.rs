use aura_components::Statistic;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use super::common::{page, row, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| StatisticDemo).into()
}

struct StatisticDemo;

impl Render for StatisticDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Statistic 统计数值",
            "用于展示强调的数值数据。",
            row(vec![
                section(
                    "基础用法",
                    "常规数值展示。",
                    row(vec![
                        Statistic::new("今日活跃用户", "114,514"),
                        Statistic::new("总交易额", "¥ 9,999.00"),
                    ]),
                )
                .into_any_element(),
                section(
                    "自定义前缀/后缀",
                    "为数值添加图标。",
                    row(vec![
                        Statistic::new("增长率", "12.5").suffix(Icon::new(IconName::TrendingUp)),
                        Statistic::new("月活下降", "5.2").suffix(Icon::new(IconName::TrendingDown)),
                        Statistic::new("待办事项", "12").prefix(Icon::new(IconName::ListTodo)),
                    ]),
                )
                .into_any_element(),
            ]),
        )
    }
}
