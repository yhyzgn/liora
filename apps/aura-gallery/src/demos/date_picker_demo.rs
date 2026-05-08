use aura_components::{DatePicker, DateValue, Text};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| DatePickerDemo::new(cx)).into()
}

struct DatePickerDemo {
    basic: Entity<DatePicker>,
    formatted: Entity<DatePicker>,
    date_range: Entity<DatePicker>,
    month: Entity<DatePicker>,
    month_range: Entity<DatePicker>,
    year: Entity<DatePicker>,
    year_range: Entity<DatePicker>,
    disabled: Entity<DatePicker>,
    selected_text: String,
}

impl DatePickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| DatePicker::new().width(px(260.0))),
            formatted: cx.new(|_| {
                DatePicker::new()
                    .value(DateValue::new(2026, 5, 8).expect("valid date"))
                    .format("YYYY年M月D日")
                    .width(px(260.0))
            }),
            date_range: cx.new(|_| {
                DatePicker::new()
                    .date_range()
                    .range(
                        DateValue::new(2026, 5, 8).expect("valid date"),
                        DateValue::new(2026, 5, 18).expect("valid date"),
                    )
                    .width(px(320.0))
            }),
            month: cx.new(|_| {
                DatePicker::new()
                    .month()
                    .value(DateValue::new(2026, 5, 1).expect("valid date"))
                    .width(px(260.0))
            }),
            month_range: cx.new(|_| {
                DatePicker::new()
                    .month_range()
                    .range(
                        DateValue::new(2026, 3, 1).expect("valid date"),
                        DateValue::new(2026, 9, 1).expect("valid date"),
                    )
                    .width(px(320.0))
            }),
            year: cx.new(|_| {
                DatePicker::new()
                    .year()
                    .value(DateValue::new(2026, 1, 1).expect("valid date"))
                    .width(px(260.0))
            }),
            year_range: cx.new(|_| {
                DatePicker::new()
                    .year_range()
                    .range(
                        DateValue::new(2024, 1, 1).expect("valid date"),
                        DateValue::new(2028, 1, 1).expect("valid date"),
                    )
                    .format("YYYY年")
                    .width(px(320.0))
            }),
            disabled: cx.new(|_| DatePicker::new().disabled(true).width(px(260.0))),
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
                    .child(div().text_sm().text_color(theme.neutral.text_3).child(
                        "用于选择日期、日期范围、月份/月范围和年份/年范围，支持自定义展示格式。",
                    )),
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
            .child(section("自定义展示格式", self.formatted.clone()))
            .child(section("日期范围", self.date_range.clone()))
            .child(section("月份选择", self.month.clone()))
            .child(section("月份范围", self.month_range.clone()))
            .child(section("年份选择", self.year.clone()))
            .child(section("年份范围", self.year_range.clone()))
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
