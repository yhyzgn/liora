//! Date-time range picker for start and end timestamps.

use liora_components::{DateTimePicker, DateTimeValue};

pub fn date_time_range_picker() -> DateTimePicker {
    DateTimePicker::new()
        .id("docs-date-time-picker-range")
        .date_time_range()
        .range(
            DateTimeValue::new(2026, 5, 8, 9, 0, 0).expect("valid start"),
            DateTimeValue::new(2026, 5, 18, 18, 30, 0).expect("valid end"),
        )
        .width_lg()
}
