//! Basic ColorPicker with current color label.

use liora_components::ColorPicker;

pub fn basic_color_picker() -> ColorPicker {
    ColorPicker::new("#409eff")
        .id("docs-color-picker-basic")
        .width_md()
}
