use gpui::{Hsla, Rgba};
use liora_theme::{Theme, filled_foreground};

fn luminance(color: Hsla) -> f64 {
    let color: Rgba = color.into();
    [color.r, color.g, color.b]
        .into_iter()
        .zip([0.2126, 0.7152, 0.0722])
        .map(|(channel, weight)| {
            let channel = f64::from(channel);
            weight
                * if channel <= 0.04045 {
                    channel / 12.92
                } else {
                    ((channel + 0.055) / 1.055).powf(2.4)
                }
        })
        .sum()
}

fn contrast(foreground: Hsla, background: Hsla) -> f64 {
    let foreground = luminance(foreground);
    let background = luminance(background);
    (foreground.max(background) + 0.05) / (foreground.min(background) + 0.05)
}

#[test]
fn component_filled_foreground_meets_wcag_for_light_and_dark_semantic_states() {
    for (mode, theme) in [("light", Theme::light()), ("dark", Theme::dark())] {
        for (name, family) in [
            ("primary", &theme.primary),
            ("info", &theme.info),
            ("success", &theme.success),
            ("warning", &theme.warning),
            ("danger", &theme.danger),
        ] {
            for (state, background) in [
                ("base", family.base),
                ("hover", family.hover),
                ("active", family.active),
                ("suppl", family.suppl),
            ] {
                let foreground = filled_foreground(background);
                let ratio = contrast(foreground, background);
                assert_eq!(foreground.a, 1.0);
                assert_eq!(background.a, 1.0);
                assert!(ratio >= 4.5, "{mode} {name} {state}: {ratio}");
                assert!(ratio >= contrast(gpui::black(), background));
                assert!(ratio >= contrast(gpui::white(), background));
            }
        }
    }
}

#[test]
fn component_filled_foreground_selects_both_black_and_white() {
    assert_eq!(filled_foreground(gpui::white()), gpui::black());
    assert_eq!(filled_foreground(gpui::black()), gpui::white());
    assert_eq!(filled_foreground(gpui::rgb(0x757575).into()), gpui::white());
    assert_eq!(filled_foreground(gpui::rgb(0x767676).into()), gpui::black());
    assert_eq!(filled_foreground(Theme::dark().primary.base), gpui::black());
    assert_eq!(
        filled_foreground(Theme::light().warning.base),
        gpui::black()
    );
}
