use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{Card, Space};

#[path = "../../../liora-docs/content/snippets/calendar/events.rs"]
mod calendar_events_example;
#[path = "../../../liora-docs/content/snippets/calendar/range.rs"]
mod calendar_range_example;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| CalendarDemo).into()
}

struct CalendarDemo;

impl Render for CalendarDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Calendar 日历",
            "月视图日历，支持选中日期、范围、高亮事件和禁用日期。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "事件日历",
                    "为日期单元格添加一条或多条事件标记。",
                    Card::new(calendar_events_example::events_calendar()),
                ))
                .child(section(
                    "范围与禁用日期",
                    "隐藏相邻月份日期；禁用 6 月 6、7、21 日。点击 6 月 30 日可查看选择反馈。",
                    Card::new(calendar_range_example::range_calendar()),
                )),
        )
    }
}
