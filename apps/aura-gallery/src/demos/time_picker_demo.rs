use aura_components::{Space, Text, TimePicker, TimeValue};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| TimePickerDemo::new(cx)).into()
}

struct TimePickerDemo {
    basic: Entity<TimePicker>,
    formatted: Entity<TimePicker>,
    stepped: Entity<TimePicker>,
    no_seconds: Entity<TimePicker>,
    disabled: Entity<TimePicker>,
}

impl TimePickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| TimePicker::new().width_md()),
            formatted: cx.new(|_| {
                TimePicker::new()
                    .value(TimeValue::new(9, 30, 15).expect("valid time"))
                    .format("HH时mm分ss秒")
                    .close_on_click_outside(false)
                    .close_on_escape(false)
                    .width_md()
            }),
            stepped: cx.new(|_| {
                TimePicker::new()
                    .value(TimeValue::new(14, 30, 0).expect("valid time"))
                    .minute_step(15)
                    .second_step(30)
                    .width_md()
            }),
            no_seconds: cx.new(|_| {
                TimePicker::new()
                    .without_seconds()
                    .value(TimeValue::new(18, 45, 0).expect("valid time"))
                    .width_md()
            }),
            disabled: cx.new(|_| TimePicker::new().disabled(true).width_md()),
        }
    }
}

impl Render for TimePickerDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_text = self
            .basic
            .read(cx)
            .value_ref()
            .map(|value| value.format())
            .unwrap_or_else(|| "尚未选择".to_string());

        page(
            "TimePicker 时间选择器",
            "用于选择固定步进时间，支持自定义展示格式、隐藏秒、禁用状态和变更回调。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "选择时分秒并读取当前值。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(self.basic.clone())
                        .child(Text::new(format!("当前选择：{}", selected_text))),
                ))
                .child(section(
                    "自定义展示格式",
                    "使用中文格式展示已选时间，并禁用点击外部/ESC 自动关闭。",
                    self.formatted.clone(),
                ))
                .child(section(
                    "固定步进",
                    "分钟和秒按固定步进展示。",
                    self.stepped.clone(),
                ))
                .child(section(
                    "隐藏秒",
                    "仅选择小时和分钟。",
                    self.no_seconds.clone(),
                ))
                .child(section(
                    "禁用状态",
                    "禁用后不可打开面板。",
                    self.disabled.clone(),
                )),
        )
    }
}
