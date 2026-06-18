//! Month picker normalizes the day field and displays month-level values.

use liora_components::{DatePicker, DateValue};

pub fn month_picker() -> DatePicker {
    DatePicker::new()
        .id("docs-date-picker-month")
        .month()
        .value(DateValue::new(2026, 5, 1).expect("valid month"))
        .width_md()
}
