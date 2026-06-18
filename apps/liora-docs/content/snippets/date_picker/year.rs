//! Year picker for year-level values.

use liora_components::{DatePicker, DateValue};

pub fn year_picker() -> DatePicker {
    DatePicker::new()
        .id("docs-date-picker-year")
        .year()
        .value(DateValue::new(2026, 1, 1).expect("valid year"))
        .width_md()
}
