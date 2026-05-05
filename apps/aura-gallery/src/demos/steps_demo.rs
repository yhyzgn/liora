use aura_components::{Steps, StepItem, StepsDirection, StepStatus};
use aura_core::{Config};
use gpui::{prelude::*, App, Context, Render, Window, div, AnyView, px};
use aura_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| StepsDemo).into()
}

struct StepsDemo;

impl Render for StepsDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div().flex().flex_col().gap_8().p_4()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Steps 步骤条"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("引导用户按照流程完成任务。"))
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(
                        Steps::new().active(1)
                            .step(StepItem::new("步骤 1"))
                            .step(StepItem::new("步骤 2"))
                            .step(StepItem::new("步骤 3"))
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("有描述及图标"))
                    .child(
                        Steps::new().active(1)
                            .step(StepItem::new("步骤 1").description("这是一段描述性文字").icon(IconName::User))
                            .step(StepItem::new("步骤 2").description("这是一段描述性文字").icon(IconName::Settings))
                            .step(StepItem::new("步骤 3").description("这是一段描述性文字").icon(IconName::Check))
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("步骤状态 (Error)"))
                    .child(
                        Steps::new().active(1)
                            .step(StepItem::new("已完成").status(StepStatus::Finish))
                            .step(StepItem::new("发生错误").status(StepStatus::Error))
                            .step(StepItem::new("等待中"))
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("垂直方向"))
                    .child(
                        div().h(px(300.0))
                            .child(
                                Steps::new().active(1).direction(StepsDirection::Vertical)
                                    .step(StepItem::new("步骤 1").description("这是一段很长很长很长的描述性文字"))
                                    .step(StepItem::new("步骤 2"))
                                    .step(StepItem::new("步骤 3"))
                            )
                    )
            )
    }
}
