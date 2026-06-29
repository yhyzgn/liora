use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_grid};
use liora_components::{GroupBox, Space, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| GroupBoxDemo).into()
}

struct GroupBoxDemo;

impl Render for GroupBoxDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "GroupBox 分组容器",
            "带标题和说明的小型分组容器，适合设置页、表单片段和属性检查器。",
            Space::new().vertical().gap_xl().child(section(
                "基础用法",
                "GroupBox 只负责视觉分组，内容仍由调用方组合。",
                showcase_grid(vec![
                    GroupBox::new(
                        "Editor",
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .child(Text::new("Tab size: 4"))
                            .child(Text::new("Soft tabs enabled")),
                    )
                    .description("Project-level editor preferences.")
                    .into_any_element(),
                    GroupBox::new(
                        "Build",
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .child(Text::new("Incremental compilation"))
                            .child(Text::new("Warnings as diagnostics")),
                    )
                    .description("Workspace build defaults.")
                    .into_any_element(),
                ]),
            )),
        )
    }
}
