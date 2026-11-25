use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, Space, Tooltip};
use liora_core::Placement;

use liora_components::layout_helpers::{page, row, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| TooltipDemo {
        lifecycle: super::overlay_lifecycle_demo::render(
            super::overlay_lifecycle_demo::Kind::Tooltip,
            cx,
        ),
    })
    .into()
}

struct TooltipDemo {
    lifecycle: AnyView,
}

impl Render for TooltipDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Tooltip 基础用法",
            "悬停或 Tab 聚焦原按钮显示；Escape 关闭且保留焦点，离开后重新进入才再次显示。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础方位",
                    "在 Focus or hover 上按 Escape，再按 Tab / Shift+Tab 离开并返回；也可跨过间距移入提示。",
                    Space::new().vertical().gap_md()
                        .child(row(vec![
                            tip("Focus or hover", "Keyboard and hover help", Placement::Top),
                            tip("Bottom", "Prompt info", Placement::Bottom),
                            tip("Left", "Prompt info", Placement::Left),
                            tip("Right", "Prompt info", Placement::Right),
                        ]))
                        .child(Button::new("After tooltip")),
                ))
                .child(section(
                    "更多方位",
                    "支持 start/end 对齐方式。",
                    row(vec![
                        tip("Top Start", "Top Start", Placement::TopStart),
                        tip("Top End", "Top End", Placement::TopEnd),
                        tip("Bottom Start", "Bottom Start", Placement::BottomStart),
                        tip("Bottom End", "Bottom End", Placement::BottomEnd),
                    ]),
                ))
                .child(section(
                    "Anchor lifecycle 锚点生命周期",
                    "延时移动后提示跟随焦点锚点；移开静止指针或卸载后提示应关闭，Reset 不应复活旧提示。",
                    self.lifecycle.clone(),
                )),
        )
    }
}

fn tip(label: &'static str, content: &'static str, placement: Placement) -> Tooltip {
    Tooltip::new(Button::new(label))
        .content(content)
        .placement(placement)
}
