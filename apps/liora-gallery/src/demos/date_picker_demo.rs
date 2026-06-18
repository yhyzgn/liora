use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::{DatePicker, DateValue, Space, Text};

use liora_components::layout_helpers::{page, section};

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
}

impl DatePickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| DatePicker::new().width_md()),
            formatted: cx.new(|_| {
                DatePicker::new()
                    .value(DateValue::new(2026, 5, 8).expect("valid date"))
                    .format("YYYY年M月D日")
                    .close_on_click_outside(false)
                    .close_on_escape(false)
                    .width_md()
            }),
            date_range: cx.new(|_| {
                DatePicker::new()
                    .date_range()
                    .range(
                        DateValue::new(2026, 5, 8).expect("valid date"),
                        DateValue::new(2026, 5, 18).expect("valid date"),
                    )
                    .width_lg()
            }),
            month: cx.new(|_| {
                DatePicker::new()
                    .month()
                    .value(DateValue::new(2026, 5, 1).expect("valid date"))
                    .width_md()
            }),
            month_range: cx.new(|_| {
                DatePicker::new()
                    .month_range()
                    .range(
                        DateValue::new(2026, 3, 1).expect("valid date"),
                        DateValue::new(2026, 9, 1).expect("valid date"),
                    )
                    .width_lg()
            }),
            year: cx.new(|_| {
                DatePicker::new()
                    .year()
                    .value(DateValue::new(2026, 1, 1).expect("valid date"))
                    .width_md()
            }),
            year_range: cx.new(|_| {
                DatePicker::new()
                    .year_range()
                    .range(
                        DateValue::new(2024, 1, 1).expect("valid date"),
                        DateValue::new(2028, 1, 1).expect("valid date"),
                    )
                    .format("YYYY年")
                    .width_lg()
            }),
            disabled: cx.new(|_| DatePicker::new().disabled(true).width_md()),
        }
    }
}

impl Render for DatePickerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_text = self
            .basic
            .read(cx)
            .value_ref()
            .map(|value| value.format())
            .unwrap_or_else(|| "尚未选择".to_string());

        page(
            "DatePicker 日期选择器",
            "用于选择日期、日期范围、月份/月范围和年份/年范围，支持自定义展示格式。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "选择单个日期并读取当前值。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(self.basic.clone())
                        .child(Text::new(format!("当前选择：{}", selected_text))),
                ))
                .child(section(
                    "自定义展示格式",
                    "使用中文格式展示已选日期，并禁用点击外部/ESC 自动关闭。",
                    self.formatted.clone(),
                ))
                .child(section(
                    "日期范围",
                    "选择开始日期和结束日期。",
                    self.date_range.clone(),
                ))
                .child(section("月份选择", "按月粒度选择。", self.month.clone()))
                .child(section(
                    "月份范围",
                    "选择月份区间。",
                    self.month_range.clone(),
                ))
                .child(section("年份选择", "按年粒度选择。", self.year.clone()))
                .child(section(
                    "年份范围",
                    "选择年份区间。",
                    self.year_range.clone(),
                ))
                .child(section(
                    "禁用状态",
                    "禁用后不可打开面板。",
                    self.disabled.clone(),
                )),
        )
    }
}
