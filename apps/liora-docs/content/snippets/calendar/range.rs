use gpui::IntoElement;
use liora_components::{Calendar, CalendarDate};

pub fn range_calendar() -> impl IntoElement {
    let date = |day| CalendarDate::new(2026, 6, day).unwrap();
    Calendar::new(2026, 6)
        .range(date(10), date(18))
        .disabled_dates([date(6), date(7), date(21)])
}
