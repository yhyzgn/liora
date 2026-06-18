//! Basic DatePicker with a medium width.

use liora_components::DatePicker;

pub fn basic_date_picker() -> DatePicker {
    DatePicker::new().id("docs-date-picker-basic").width_md()
}
