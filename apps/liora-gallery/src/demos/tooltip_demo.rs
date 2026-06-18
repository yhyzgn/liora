use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, Space, Tooltip};
use liora_core::Placement;

use liora_components::layout_helpers::{page, row, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TooltipDemo).into()
}

struct TooltipDemo;

impl Render for TooltipDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Tooltip 基础用法",
            "鼠标悬停在按钮上显示提示信息。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础方位",
                    "支持上下左右四个基础方向。",
                    row(vec![
                        tip("Top Top Top Top Top Top", "Prompt info", Placement::Top),
                        tip("Bottom", "Prompt info", Placement::Bottom),
                        tip("Left", "Prompt info", Placement::Left),
                        tip("Right", "Prompt info", Placement::Right),
                    ]),
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
                )),
        )
    }
}

fn tip(label: &'static str, content: &'static str, placement: Placement) -> Tooltip {
    Tooltip::new(Button::new(label))
        .content(content)
        .placement(placement)
}
