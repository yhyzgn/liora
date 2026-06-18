use gpui::{IntoElement, rgb};
use liora_components::{Calendar, CalendarDate, CalendarEvent};

pub fn events_calendar() -> impl IntoElement {
    let date = |day| CalendarDate::new(2026, 6, day).unwrap();
    Calendar::new(2026, 6).selected(date(16)).events([
        CalendarEvent::new(date(3), "Design review").color(rgb(0x2563eb).into()),
        CalendarEvent::new(date(18), "Docs polish").color(rgb(0xf97316).into()),
    ])
}
