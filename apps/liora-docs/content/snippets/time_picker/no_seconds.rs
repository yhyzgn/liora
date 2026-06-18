//! TimePicker without seconds column.

use liora_components::{TimePicker, TimeValue};

pub fn no_seconds_time_picker() -> TimePicker {
    TimePicker::new()
        .id("docs-time-picker-no-seconds")
        .without_seconds()
        .value(TimeValue::new(18, 45, 0).expect("valid time"))
        .width_md()
}
