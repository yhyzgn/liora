use aura_components::{DateTimePicker, DateTimeValue, Space, Text};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| DateTimePickerDemo::new(cx)).into()
}

struct DateTimePickerDemo {
    basic: Entity<DateTimePicker>,
    formatted: Entity<DateTimePicker>,
    stepped: Entity<DateTimePicker>,
    no_seconds: Entity<DateTimePicker>,
    range: Entity<DateTimePicker>,
    disabled: Entity<DateTimePicker>,
}

impl DateTimePickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| DateTimePicker::new().width_md()),
            formatted: cx.new(|_| {
                DateTimePicker::new()
                    .value(DateTimeValue::new(2026, 5, 8, 9, 30, 15).expect("valid datetime"))
                    .format("YYYY年M月D日 HH:mm:ss")
                    .width_md()
            }),
            stepped: cx.new(|_| {
                DateTimePicker::new()
                    .value(DateTimeValue::new(2026, 5, 8, 14, 30, 0).expect("valid datetime"))
                    .minute_step(15)
                    .second_step(30)
                    .width_md()
            }),
            no_seconds: cx.new(|_| {
                DateTimePicker::new()
                    .without_seconds()
                    .value(DateTimeValue::new(2026, 5, 8, 18, 45, 0).expect("valid datetime"))
                    .width_md()
            }),
            range: cx.new(|_| {
                DateTimePicker::new()
                    .date_time_range()
                    .range(
                        DateTimeValue::new(2026, 5, 8, 9, 0, 0).expect("valid datetime"),
                        DateTimeValue::new(2026, 5, 18, 18, 30, 0).expect("valid datetime"),
                    )
                    .width_lg()
            }),
            disabled: cx.new(|_| DateTimePicker::new().disabled(true).width_md()),
        }
    }
}

impl Render for DateTimePickerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_text = self
            .basic
            .read(cx)
            .value_ref()
            .map(|value| value.format())
            .unwrap_or_else(|| "尚未选择".to_string());

        page(
            "DateTimePicker 日期时间选择器",
            "用于选择日期时间和日期时间范围，支持自定义展示格式、时间步进、隐藏秒和确认操作。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "选择日期时间并读取当前值。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(self.basic.clone())
                        .child(Text::new(format!("当前选择：{}", selected_text))),
                ))
                .child(section(
                    "自定义展示格式",
                    "使用中文格式展示已选日期时间。",
                    self.formatted.clone(),
                ))
                .child(section(
                    "固定步进",
                    "分钟和秒按固定步进展示。",
                    self.stepped.clone(),
                ))
                .child(section(
                    "隐藏秒",
                    "仅选择日期、小时和分钟。",
                    self.no_seconds.clone(),
                ))
                .child(section(
                    "日期时间范围",
                    "选择开始与结束日期时间。",
                    self.range.clone(),
                ))
                .child(section(
                    "禁用状态",
                    "禁用后不可打开面板。",
                    self.disabled.clone(),
                )),
        )
    }
}
