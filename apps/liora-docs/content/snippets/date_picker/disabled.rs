//! Disabled DatePicker keeps layout stable but blocks interaction.

use liora_components::DatePicker;

pub fn disabled_date_picker() -> DatePicker {
    DatePicker::new()
        .id("docs-date-picker-disabled")
        .disabled(true)
        .width_md()
}
