//! Preselect a date and display it in a custom format.

use liora_components::{DatePicker, DateValue};

pub fn formatted_date_picker() -> DatePicker {
    DatePicker::new()
        .id("docs-date-picker-formatted")
        .value(DateValue::new(2026, 5, 8).expect("valid date"))
        .format("YYYY年M月D日")
        .close_on_click_outside(false)
        .close_on_escape(false)
        .width_md()
}
