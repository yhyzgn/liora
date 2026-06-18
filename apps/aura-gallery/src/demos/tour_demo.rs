use aura_components::layout_helpers::{page, section};
use aura_components::{
    Button, Card, Space, Text, Tour, TourPlacement, TourStep, toast_info, toast_success,
};
use gpui::{AnyView, App, Context, IntoElement, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TourDemo).into()
}

struct TourDemo;

fn steps() -> Vec<TourStep> {
    vec![
        TourStep::new("选择组件", "左侧菜单按组件名称排序，统计图会统一放在最后。")
            .target("Gallery menu")
            .placement(TourPlacement::Right),
        TourStep::new(
            "查看效果",
            "右侧主面板展示实际原生控件，Tour 卡片浮在窗口顶层。",
        )
        .target("Preview panel")
        .placement(TourPlacement::Bottom),
        TourStep::new("复制代码", "Docs 中每种效果下面紧跟对应代码片段。")
            .target("Code block")
            .placement(TourPlacement::Top),
    ]
}

impl Render for TourDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Tour 漫游引导",
            "Tour 是顶层浮层引导，不是可嵌入页面的内容块；通过 Tour::show(cx) 启动，步骤卡片在 modal overlay 中前进、后退、关闭或完成。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "顶层浮层引导",
                    "点击按钮后 Tour 会覆盖在当前窗口顶层，而不是嵌入当前页面。",
                    demo_panel(
                        Button::new("启动基础 Tour")
                            .primary()
                            .on_click(|_, _, cx| {
                                Tour::new(steps())
                                    .on_change(|index, _, _| toast_info!("Tour step {}", index + 1))
                                    .on_finish(|_, _| toast_success!("Tour finished"))
                                    .on_close(|_, _| toast_info!("Tour closed"))
                                    .show(cx);
                            }),
                    ),
                ))
                .child(section(
                    "从中间步骤开始",
                    "active_index 可设置初始步骤；TourView 内部会推进当前步骤并触发 on_change。",
                    demo_panel(
                        Button::new("从第二步开始")
                            .secondary()
                            .on_click(|_, _, cx| {
                                Tour::new(steps())
                                    .active_index(1)
                                    .previous_text("上一步")
                                    .next_text("下一步")
                                    .finish_text("完成")
                                    .show(cx);
                            }),
                    ),
                ))
                .child(section(
                    "透明遮罩模式",
                    "show_mask(false) 保持顶层浮层行为，但不显示半透明遮罩；close_on_click_outside(true) 可点击外部关闭。",
                    demo_panel(
                        Button::new("无遮罩 Tour")
                            .tertiary()
                            .on_click(|_, _, cx| {
                                Tour::new(steps())
                                    .show_mask(false)
                                    .close_on_click_outside(true)
                                    .finish_text("完成")
                                    .show(cx);
                            }),
                    ),
                ))
                .child(section(
                    "受控关闭策略",
                    "禁用 ESC 与外部点击关闭，避免用户误触中断关键引导流程。",
                    demo_panel(
                        Button::new("启动受控 Tour")
                            .warning()
                            .on_click(|_, _, cx| {
                                Tour::new(steps())
                                    .close_on_escape(false)
                                    .close_on_click_outside(false)
                                    .finish_text("我已了解")
                                    .on_close(|_, _| toast_info!("Tour closed explicitly"))
                                    .show(cx);
                            }),
                    ),
                )),
        )
    }
}

fn demo_panel(button: impl IntoElement) -> impl IntoElement {
    Card::new(
        Space::new()
            .vertical()
            .gap_md()
            .child(
                div()
                    .h(px(160.0))
                    .rounded_lg()
                    .border_1()
                    .border_color(gpui::rgb(0xe2e8f0))
                    .bg(gpui::rgb(0xf8fafc))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Text::new(
                        "页面内容保持在原位；Tour 打开后会覆盖在窗口顶层。",
                    )),
            )
            .child(button),
    )
}
