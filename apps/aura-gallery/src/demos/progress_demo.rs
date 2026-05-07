use aura_components::{Progress, ProgressStatus};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| ProgressDemo).into()
}

struct ProgressDemo;

impl Render for ProgressDemo {
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
                            .child("Progress 进度条"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("展示操作进度，告知用户当前状态。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(Progress::new(0.0))
                    .child(Progress::new(30.0))
                    .child(Progress::new(50.0))
                    .child(Progress::new(100.0).status(ProgressStatus::Success)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("百分比内显"),
                    )
                    .child(Progress::new(15.0).stroke_width(px(20.0)).text_inside(true))
                    .child(Progress::new(70.0).stroke_width(px(20.0)).text_inside(true))
                    .child(
                        Progress::new(70.0)
                            .stroke_width(px(20.0))
                            .text_inside_centered(),
                    )
                    .child(
                        Progress::new(100.0)
                            .stroke_width(px(20.0))
                            .text_inside(true)
                            .status(ProgressStatus::Success),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("不同状态"))
                    .child(Progress::new(30.0))
                    .child(Progress::new(50.0).status(ProgressStatus::Warning))
                    .child(Progress::new(70.0).status(ProgressStatus::Exception))
                    .child(Progress::new(100.0).status(ProgressStatus::Success)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("自定义颜色"),
                    )
                    .child(Progress::new(50.0).color(gpui::blue())),
            )
    }
}
