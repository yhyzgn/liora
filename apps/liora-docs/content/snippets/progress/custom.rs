//! Animated circular progress with fully customized ring and center labels.

use gpui::{App, FontWeight, IntoElement, px};
use liora_components::{Progress, Space};
use liora_core::Config;

pub fn custom_progress(cx: &mut App) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();

    Space::new().gap_lg().wrap().children(vec![
        Progress::new(86.0)
            .circle()
            .circle_size(px(148.0))
            .ring_width(px(12.0))
            .ring_color(theme.neutral.hover)
            .progress_color(theme.primary.base)
            .inner_color(theme.neutral.card)
            .center_text("Deploy")
            .text_size(px(22.0))
            .text_color(theme.primary.base),
        Progress::new(42.0)
            .circle()
            .circle_size(px(132.0))
            .ring_width(px(10.0))
            .ring_color(theme.success.hover.opacity(0.24))
            .progress_color(theme.success.base)
            .inner_color(theme.success.base.opacity(0.10))
            .center_text("Inner BG")
            .text_size(px(16.0))
            .text_weight(FontWeight::NORMAL),
        Progress::new(68.0)
            .circle()
            .circle_size(px(132.0))
            .ring_width(px(14.0))
            .ring_color(theme.warning.hover.opacity(0.28))
            .progress_color(theme.warning.base)
            .inner_color(theme.neutral.card.opacity(0.78))
            .center_text("CPU")
            .text_size(px(18.0))
            .text_color(theme.warning.base)
            .animated(false),
    ])
}
