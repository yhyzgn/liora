//! Animated progress and custom center labels.

use aura_components::{Progress, Space};
use aura_core::Config;
use gpui::{App, FontWeight, IntoElement, px};

pub fn custom_progress(cx: &mut App) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();

    Space::new().gap_lg().wrap().children(vec![
        Progress::new(86.0)
            .circle()
            .circle_size(px(148.0))
            .stroke_width(px(12.0))
            .center_text("Deploy")
            .text_size(px(22.0))
            .text_color(theme.primary.base)
            .track_color(theme.neutral.hover),
        Progress::new(42.0)
            .circle()
            .circle_size(px(132.0))
            .stroke_width(px(10.0))
            .center_text("42 / 100")
            .text_size(px(16.0))
            .text_weight(FontWeight::NORMAL)
            .color(theme.success.base),
        Progress::new(68.0)
            .circle()
            .circle_size(px(132.0))
            .stroke_width(px(14.0))
            .center_text("CPU")
            .text_size(px(18.0))
            .text_color(theme.warning.base)
            .color(theme.warning.base)
            .animated(false),
    ])
}
