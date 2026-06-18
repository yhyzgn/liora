//! TimePicker with minute and second steps.

use liora_components::{TimePicker, TimeValue};

pub fn stepped_time_picker() -> TimePicker {
    TimePicker::new()
        .id("docs-time-picker-stepped")
        .value(TimeValue::new(14, 30, 0).expect("valid time"))
        .minute_step(15)
        .second_step(30)
        .width_md()
}
