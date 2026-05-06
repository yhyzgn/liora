use aura_components::{Segmented, SegmentedOption};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SegmentedDemo).into()
}

struct SegmentedDemo;

impl Render for SegmentedDemo {
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
                            .child("Segmented 分段控制器"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于展示多个选项并允许用户选择其中单个选项。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(cx.new(|_| {
                        Segmented::new(vec![
                            SegmentedOption::new("Daily", "daily"),
                            SegmentedOption::new("Weekly", "weekly"),
                            SegmentedOption::new("Monthly", "monthly"),
                            SegmentedOption::new("Quarterly", "quarterly"),
                            SegmentedOption::new("Yearly", "yearly"),
                        ])
                        .on_change(|val, _, _| println!("Selected: {}", val))
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("不可用状态"),
                    )
                    .child(cx.new(|_| {
                        Segmented::new(vec![
                            SegmentedOption::new("Map", "Map"),
                            SegmentedOption::new("Transit", "Transit"),
                            SegmentedOption::new("Satellite", "Satellite").disabled(true),
                        ])
                        .value("Map")
                        .on_change(|val, _, _| println!("Selected: {}", val))
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Block 模式 (撑满宽度)"),
                    )
                    .child(cx.new(|_| {
                        Segmented::new(vec![
                            SegmentedOption::new("123", "123"),
                            SegmentedOption::new("456", "456"),
                            SegmentedOption::new("long-text-option", "long"),
                        ])
                        .block(true)
                        .on_change(|val, _, _| println!("Selected: {}", val))
                    })),
            )
    }
}
