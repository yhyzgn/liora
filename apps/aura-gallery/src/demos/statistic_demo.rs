use aura_components::Statistic;
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| StatisticDemo).into()
}

struct StatisticDemo;

impl Render for StatisticDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Statistic 统计数值"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于展示强调的数值数据。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_10()
                            .child(Statistic::new("今日活跃用户", "114,514"))
                            .child(Statistic::new("总交易额", "¥ 9,999.00")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("自定义前缀/后缀"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_10()
                            .child(
                                Statistic::new("增长率", "12.5")
                                    .suffix(
                                        Icon::new(IconName::TrendingUp)
                                            .size(px(14.0))
                                            .color(theme.danger.base),
                                    )
                                    .value_color(theme.danger.base),
                            )
                            .child(
                                Statistic::new("月活下降", "5.2")
                                    .suffix(
                                        Icon::new(IconName::TrendingDown)
                                            .size(px(14.0))
                                            .color(theme.success.base),
                                    )
                                    .value_color(theme.success.base),
                            )
                            .child(
                                Statistic::new("待办事项", "12").prefix(
                                    Icon::new(IconName::ListTodo)
                                        .size(px(14.0))
                                        .color(theme.primary.base),
                                ),
                            ),
                    ),
            )
    }
}
