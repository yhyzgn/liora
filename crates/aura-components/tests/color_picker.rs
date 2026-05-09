use aura_components::ColorPicker;

#[test]
fn normalizes_hex_colors() {
    assert_eq!(
        ColorPicker::normalize_hex("#409EFF"),
        Some("#409EFF".into())
    );
    assert_eq!(ColorPicker::normalize_hex("409eff"), Some("#409EFF".into()));
    assert_eq!(ColorPicker::normalize_hex("#abc"), Some("#AABBCC".into()));
}

#[test]
fn rejects_invalid_hex_colors() {
    assert_eq!(ColorPicker::normalize_hex("#12"), None);
    assert_eq!(ColorPicker::normalize_hex("#xyzxyz"), None);
}

#[test]
fn converts_hex_to_rgb_channels() {
    assert_eq!(ColorPicker::hex_rgb("#409EFF"), Some((64, 158, 255)));
    assert_eq!(ColorPicker::hex_rgb("#AABBCC"), Some((170, 187, 204)));
}

#[test]
fn provides_a_rainbow_panel_palette() {
    let palette = ColorPicker::rainbow_palette();

    assert!(palette.len() >= 24);
    assert!(palette.contains(&"#FF0000".into()));
    assert!(palette.contains(&"#0000FF".into()));
}

#[test]
fn formats_rgba_display_with_alpha() {
    assert_eq!(
        ColorPicker::rgba_display("#FFFF00", 0.68),
        Some("rgba(255, 255, 0, 0.68)".into())
    );
}

#[test]
fn builds_color_from_hsv_and_alpha() {
    assert_eq!(ColorPicker::hex_from_hsv(60.0, 1.0, 1.0), "#FFFF00");
    assert_eq!(ColorPicker::hex_from_hsv(0.0, 1.0, 1.0), "#FF0000");
}

#[test]
fn rgba_display_clamps_alpha() {
    assert_eq!(
        ColorPicker::rgba_display("#000000", 2.0),
        Some("rgba(0, 0, 0, 1.00)".into())
    );
    assert_eq!(
        ColorPicker::rgba_display("#000000", -1.0),
        Some("rgba(0, 0, 0, 0.00)".into())
    );
}
