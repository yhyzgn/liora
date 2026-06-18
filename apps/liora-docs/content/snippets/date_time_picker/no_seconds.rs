//! Hide seconds when the product only needs minute-level precision.

use liora_components::{DateTimePicker, DateTimeValue};

pub fn no_seconds_date_time_picker() -> DateTimePicker {
    DateTimePicker::new()
        .id("docs-date-time-picker-no-seconds")
        .without_seconds()
        .value(DateTimeValue::new(2026, 5, 8, 18, 45, 0).expect("valid datetime"))
        .width_md()
}
