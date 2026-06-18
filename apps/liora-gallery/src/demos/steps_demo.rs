use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Space, StepItem, StepStatus, Steps, StepsDirection};
use liora_icons_lucide::IconName;

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| StepsDemo).into()
}

struct StepsDemo;

impl Render for StepsDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Steps 步骤条",
            "引导用户按照流程完成任务。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "通过 active 标识当前步骤。",
                    Steps::new()
                        .active(1)
                        .step(StepItem::new("步骤 1"))
                        .step(StepItem::new("步骤 2"))
                        .step(StepItem::new("步骤 3")),
                ))
                .child(section(
                    "有描述及图标",
                    "步骤项可以同时展示描述文字和图标。",
                    Steps::new()
                        .active(1)
                        .step(
                            StepItem::new("步骤 1")
                                .description("这是一段描述性文字")
                                .icon(IconName::User),
                        )
                        .step(
                            StepItem::new("步骤 2")
                                .description("这是一段描述性文字")
                                .icon(IconName::Settings),
                        )
                        .step(
                            StepItem::new("步骤 3")
                                .description("这是一段描述性文字")
                                .icon(IconName::Check),
                        ),
                ))
                .child(section(
                    "步骤状态 (Error)",
                    "单个步骤可以显式设置完成或错误状态。",
                    Steps::new()
                        .active(1)
                        .step(StepItem::new("已完成").status(StepStatus::Finish))
                        .step(StepItem::new("发生错误").status(StepStatus::Error))
                        .step(StepItem::new("等待中")),
                ))
                .child(section(
                    "垂直方向",
                    "切换为垂直方向以适配纵向流程。",
                    Steps::new()
                        .active(1)
                        .direction(StepsDirection::Vertical)
                        .step(
                            StepItem::new("步骤 1").description("这是一段很长很长很长的描述性文字"),
                        )
                        .step(StepItem::new("步骤 2"))
                        .step(StepItem::new("步骤 3")),
                )),
        )
    }
}
