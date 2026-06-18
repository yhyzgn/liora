//! TimePicker with custom display format.

use liora_components::{TimePicker, TimeValue};

pub fn formatted_time_picker() -> TimePicker {
    TimePicker::new()
        .id("docs-time-picker-formatted")
        .value(TimeValue::new(9, 30, 15).expect("valid time"))
        .format("HH时mm分ss秒")
        .close_on_click_outside(false)
        .close_on_escape(false)
        .width_md()
}
