use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Card, Statistic};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

use liora_components::layout_helpers::{page, row, row_md, section};

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
                    "兼容任意元素作为前缀或后缀。",
                    row(vec![
                        Statistic::new("增长率", "12.5").suffix(Icon::new(IconName::TrendingUp)),
                        Statistic::new("月活下降", "5.2").suffix(Icon::new(IconName::TrendingDown)),
                        Statistic::new("待办事项", "12").prefix(Icon::new(IconName::ListTodo)),
                    ]),
                )
                .into_any_element(),
                section(
                    "内置图标",
                    "图标默认跟随数字颜色，也可以独立指定颜色，并支持左右位置。",
                    row(vec![
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
                    ]),
                )
                .into_any_element(),
                section(
                    "水平布局",
                    "标题在左，数字和图标在右；支持紧凑排列和两端对齐。",
                    row_md(vec![
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
                    ]),
                )
                .into_any_element(),
            ]),
        )
    }
}
