use aura_components::layout_helpers::{page, section};
use aura_components::{Card, Space, Tour, TourPlacement, TourStep, toast_info, toast_success};
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TourDemo).into()
}

struct TourDemo;

fn steps() -> Vec<TourStep> {
    vec![
        TourStep::new("选择组件", "左侧菜单按组件名称排序，统计图会统一放在最后。")
            .target("Gallery menu")
            .placement(TourPlacement::Right),
        TourStep::new("查看效果", "每个 demo 大方展示主要配置，避免挤在一起。 ")
            .target("Preview panel")
            .placement(TourPlacement::Bottom),
        TourStep::new("复制代码", "Docs 中每种效果下面紧跟对应代码片段。 ")
            .target("Code block")
            .placement(TourPlacement::Top),
    ]
}

impl Render for TourDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Tour 漫游引导",
            "用步骤卡片说明界面关键区域，支持进度、前进后退、关闭和完成回调。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础引导",
                    "active_index 控制当前步骤，on_change 可由上层更新状态。",
                    Card::new(
                        Tour::new(steps())
                            .active_index(0)
                            .on_change(|index, _, _| toast_info!("Tour step {}", index + 1))
                            .on_finish(|_, _| toast_success!("Tour finished")),
                    ),
                ))
                .child(section(
                    "中间步骤",
                    "展示前进/后退按钮同时可用的状态。",
                    Card::new(Tour::new(steps()).active_index(1)),
                ))
                .child(section(
                    "无遮罩简洁模式",
                    "show_mask(false) 适合内嵌说明卡片。",
                    Card::new(
                        Tour::new(steps())
                            .active_index(2)
                            .show_mask(false)
                            .finish_text("完成"),
                    ),
                )),
        )
    }
}
