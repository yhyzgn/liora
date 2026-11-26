use gpui::{IntoElement, rgb};
use liora_components::{Calendar, CalendarDate, CalendarEvent, toast_info};

pub fn range_calendar() -> impl IntoElement {
    let date = |day| CalendarDate {
        year: 2026,
        month: 6,
        day,
    };
    Calendar::new(2026, 6)
        .id("calendar-range")
        .range(date(10), date(18))
        .show_adjacent_months(false)
        .disabled_dates([
            date(6),
            date(7),
            date(21),
            CalendarDate {
                year: 2026,
                month: 7,
                day: 2,
            },
        ])
        .events([
            CalendarEvent::new(date(3), "Design review").color(rgb(0x2563eb).into()),
            CalendarEvent::new(date(12), "Release smoke").color(rgb(0x16a34a).into()),
            CalendarEvent::new(date(18), "Docs polish").color(rgb(0xf97316).into()),
            CalendarEvent::new(date(18), "Team sync").color(rgb(0x9333ea).into()),
        ])
        .on_select(|date, _, _| toast_info!("Selected {}", date.format()))
}
