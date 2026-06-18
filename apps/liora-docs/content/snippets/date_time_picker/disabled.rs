//! Disabled DateTimePicker blocks panel opening while preserving layout.

use liora_components::DateTimePicker;

pub fn disabled_date_time_picker() -> DateTimePicker {
    DateTimePicker::new()
        .id("docs-date-time-picker-disabled")
        .disabled(true)
        .width_md()
}
