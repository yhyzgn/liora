//! Date range selection with a preset start and end date.

use liora_components::{DatePicker, DateValue};

pub fn date_range_picker() -> DatePicker {
    DatePicker::new()
        .id("docs-date-picker-range")
        .date_range()
        .range(
            DateValue::new(2026, 5, 8).expect("valid start"),
            DateValue::new(2026, 5, 18).expect("valid end"),
        )
        .width_lg()
}
