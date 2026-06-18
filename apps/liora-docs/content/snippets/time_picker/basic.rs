//! Basic TimePicker.

use liora_components::TimePicker;

pub fn basic_time_picker() -> TimePicker {
    TimePicker::new().id("docs-time-picker-basic").width_md()
}
