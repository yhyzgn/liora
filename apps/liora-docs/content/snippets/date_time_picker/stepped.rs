//! Restrict minute and second options to fixed steps.

use liora_components::{DateTimePicker, DateTimeValue};

pub fn stepped_date_time_picker() -> DateTimePicker {
    DateTimePicker::new()
        .id("docs-date-time-picker-stepped")
        .value(DateTimeValue::new(2026, 5, 8, 14, 30, 0).expect("valid datetime"))
        .minute_step(15)
        .second_step(30)
        .width_md()
}
