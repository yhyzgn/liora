use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Card, Space, Timeline, TimelineItem, TimelinePlacement};
use liora_icons_lucide::IconName;

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TimelineDemo).into()
}

struct TimelineDemo;

impl Render for TimelineDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Timeline 时间线",
            "垂直展示一系列信息。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "按时间顺序展示事件。",
                    Card::new(
                        Timeline::new()
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-01")
                                    .content("创建成功"),
                            )
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-02")
                                    .content("通过审核"),
                            )
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-03")
                                    .content("项目发布"),
                            ),
                    ),
                ))
                .child(section(
                    "自定义节点样式",
                    "使用语义状态、空心节点和图标节点。",
                    Card::new(
                        Timeline::new()
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-01")
                                    .content("成功状态")
                                    .success(),
                            )
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-02")
                                    .content("警告状态")
                                    .warning()
                                    .hollow(true),
                            )
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-03")
                                    .content("错误状态")
                                    .danger()
                                    .icon(IconName::CircleX),
                            )
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-04")
                                    .content("自定义图标")
                                    .primary()
                                    .icon(IconName::Star),
                            ),
                    ),
                ))
                .child(section(
                    "时间戳位置",
                    "时间戳可以展示在内容顶部或底部。",
                    Card::new(
                        Timeline::new()
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-01")
                                    .content("时间戳在顶部")
                                    .placement(TimelinePlacement::Top),
                            )
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-02")
                                    .content("时间戳在底部")
                                    .placement(TimelinePlacement::Bottom),
                            ),
                    ),
                ))
                .child(section(
                    "倒序排列",
                    "reverse 模式会反向展示事件。",
                    Card::new(
                        Timeline::new()
                            .reverse(true)
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-01")
                                    .content("事件 1"),
                            )
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-02")
                                    .content("事件 2"),
                            )
                            .item(
                                TimelineItem::new()
                                    .timestamp("2026-05-03")
                                    .content("事件 3"),
                            ),
                    ),
                )),
        )
    }
}
