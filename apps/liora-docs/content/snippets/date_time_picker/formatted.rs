//! Preselect a date-time and display it with a custom format.

use liora_components::{DateTimePicker, DateTimeValue};

pub fn formatted_date_time_picker() -> DateTimePicker {
    DateTimePicker::new()
        .id("docs-date-time-picker-formatted")
        .value(DateTimeValue::new(2026, 5, 8, 9, 30, 15).expect("valid datetime"))
        .format("YYYY年M月D日 HH:mm:ss")
        .width_md()
}
