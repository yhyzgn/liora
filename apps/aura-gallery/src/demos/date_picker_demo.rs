use aura_components::{DatePicker, DateValue, Text};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| DatePickerDemo::new(cx)).into()
}

struct DatePickerDemo {
    basic: Entity<DatePicker>,
    preset: Entity<DatePicker>,
    disabled: Entity<DatePicker>,
    selected_text: String,
}

impl DatePickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| DatePicker::new().width(px(240.0))),
            preset: cx.new(|_| {
                DatePicker::new()
                    .value(DateValue::new(2026, 5, 8).expect("valid date"))
                    .width(px(240.0))
            }),
            disabled: cx.new(|_| DatePicker::new().disabled(true).width(px(240.0))),
            selected_text: "尚未选择".to_string(),
        }
    }
}

impl Render for DatePickerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().clone();
        self.basic.update(cx, |picker, cx| {
            let view = view.clone();
            picker.set_on_change(
                move |value, _, cx| {
                    view.update(cx, |this, cx| {
                        this.selected_text = value
                            .map(|value| value.format())
                            .unwrap_or_else(|| "尚未选择".to_string());
                        cx.notify();
                    });
                },
                cx,
            );
        });
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
                            .child("DatePicker 日期选择器"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于选择单个日期，支持月份切换、预设值、禁用状态和变更回调。"),
                    ),
            )
            .child(section(
                "基础用法",
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(self.basic.clone())
                    .child(
                        Text::new(format!("当前选择：{}", self.selected_text))
                            .size(px(theme.font_size.sm)),
                    ),
            ))
            .child(section("预设值", self.preset.clone()))
            .child(section("禁用状态", self.disabled.clone()))
    }
}

fn section(title: &'static str, content: impl IntoElement) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(content)
}
