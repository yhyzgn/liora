//! Compact ColorPicker trigger without text label.

use liora_components::ColorPicker;

pub fn compact_color_picker() -> ColorPicker {
    ColorPicker::new("#F56C6C")
        .id("docs-color-picker-compact")
        .show_label(false)
        .close_on_click_outside(false)
        .close_on_escape(false)
}
