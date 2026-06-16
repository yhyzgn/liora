use aura_components::layout_helpers::{page, section};
use aura_components::{Calendar, CalendarDate, CalendarEvent, Card, Space, toast_info};
use gpui::{AnyView, App, Context, Render, Window, prelude::*, rgb};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| CalendarDemo).into()
}

struct CalendarDemo;

fn date(day: u32) -> CalendarDate {
    CalendarDate::new(2026, 6, day).expect("valid demo date")
}

impl Render for CalendarDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let events = vec![
            CalendarEvent::new(date(3), "Design review").color(rgb(0x2563eb).into()),
            CalendarEvent::new(date(12), "Release smoke").color(rgb(0x16a34a).into()),
            CalendarEvent::new(date(18), "Docs polish").color(rgb(0xf97316).into()),
            CalendarEvent::new(date(18), "Team sync").color(rgb(0x9333ea).into()),
        ];

        page(
            "Calendar 日历",
            "月视图日历，支持选中日期、范围、高亮事件和禁用日期。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "事件日历",
                    "为日期单元格添加一条或多条事件标记。",
                    Card::new(
                        Calendar::new(2026, 6)
                            .selected(date(16))
                            .events(events.clone())
                            .on_select(|date, _, _| toast_info!("Selected {}", date.format())),
                    ),
                ))
                .child(section(
                    "范围与禁用日期",
                    "range 会以浅色背景标注连续日期，disabled_dates 禁止交互。",
                    Card::new(
                        Calendar::new(2026, 6)
                            .range(date(10), date(18))
                            .disabled_dates([date(6), date(7), date(21)])
                            .events(events),
                    ),
                )),
        )
    }
}
