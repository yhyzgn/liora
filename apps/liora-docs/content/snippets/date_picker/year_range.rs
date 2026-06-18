//! Year range picker with custom year-only formatting.

use liora_components::{DatePicker, DateValue};

pub fn year_range_picker() -> DatePicker {
    DatePicker::new()
        .id("docs-date-picker-year-range")
        .year_range()
        .range(
            DateValue::new(2024, 1, 1).expect("valid start year"),
            DateValue::new(2028, 1, 1).expect("valid end year"),
        )
        .format("YYYY年")
        .width_lg()
}
