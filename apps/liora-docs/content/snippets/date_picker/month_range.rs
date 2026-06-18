//! Month range picker for month-level intervals.

use liora_components::{DatePicker, DateValue};

pub fn month_range_picker() -> DatePicker {
    DatePicker::new()
        .id("docs-date-picker-month-range")
        .month_range()
        .range(
            DateValue::new(2026, 3, 1).expect("valid start month"),
            DateValue::new(2026, 9, 1).expect("valid end month"),
        )
        .width_lg()
}
