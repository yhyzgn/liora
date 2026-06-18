//! Disabled ColorPicker.

use liora_components::ColorPicker;

pub fn disabled_color_picker() -> ColorPicker {
    ColorPicker::new("#909399")
        .id("docs-color-picker-disabled")
        .disabled(true)
        .width_md()
}
