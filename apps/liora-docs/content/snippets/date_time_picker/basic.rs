//! Basic DateTimePicker with a medium width.

use liora_components::DateTimePicker;

pub fn basic_date_time_picker() -> DateTimePicker {
    DateTimePicker::new()
        .id("docs-date-time-picker-basic")
        .width_md()
}
