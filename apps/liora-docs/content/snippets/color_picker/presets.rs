//! ColorPicker with custom preset swatches.

use liora_components::ColorPicker;

pub fn preset_color_picker() -> ColorPicker {
    ColorPicker::new("#13c2c2")
        .id("docs-color-picker-presets")
        .width_md()
        .presets([
            "#13C2C2", "#52C41A", "#FAAD14", "#F5222D", "#722ED1", "#EB2F96",
        ])
}
