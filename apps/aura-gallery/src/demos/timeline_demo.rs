use aura_components::{Timeline, TimelineItem, TimelinePlacement, Card};
use aura_core::{Config};
use gpui::{prelude::*, App, Context, Render, Window, div, AnyView, px};
use aura_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TimelineDemo).into()
}

struct TimelineDemo;

impl Render for TimelineDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div().flex().flex_col().gap_8().p_4().id("timeline-scroll").overflow_y_scroll()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Timeline 时间线"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("垂直展示一系列信息。"))
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(
                        Card::new(
                            Timeline::new()
                                .item(TimelineItem::new().timestamp("2026-05-01").content("创建成功"))
                                .item(TimelineItem::new().timestamp("2026-05-02").content("通过审核"))
                                .item(TimelineItem::new().timestamp("2026-05-03").content("项目发布"))
                        )
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("自定义节点样式"))
                    .child(
                        Card::new(
                            Timeline::new()
                                .item(TimelineItem::new().timestamp("2026-05-01").content("成功状态").color(theme.success.base))
                                .item(TimelineItem::new().timestamp("2026-05-02").content("警告状态").color(theme.warning.base).hollow(true))
                                .item(TimelineItem::new().timestamp("2026-05-03").content("错误状态").color(theme.danger.base).icon(IconName::CircleX))
                                .item(TimelineItem::new().timestamp("2026-05-04").content("自定义图标").icon(IconName::Star).color(gpui::blue()))
                        )
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("时间戳位置"))
                    .child(
                        Card::new(
                            Timeline::new()
                                .item(TimelineItem::new().timestamp("2026-05-01").content("时间戳在顶部").placement(TimelinePlacement::Top))
                                .item(TimelineItem::new().timestamp("2026-05-02").content("时间戳在底部").placement(TimelinePlacement::Bottom))
                        )
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("倒序排列"))
                    .child(
                        Card::new(
                            Timeline::new().reverse(true)
                                .item(TimelineItem::new().timestamp("2026-05-01").content("事件 1"))
                                .item(TimelineItem::new().timestamp("2026-05-02").content("事件 2"))
                                .item(TimelineItem::new().timestamp("2026-05-03").content("事件 3"))
                        )
                    )
            )
    }
}
