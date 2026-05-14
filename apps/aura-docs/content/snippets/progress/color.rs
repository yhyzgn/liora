//! Custom single-color and gradient progress bars.

use aura_components::{Progress, Space};
use aura_core::Config;
use gpui::{App, IntoElement};

pub fn color_progress(cx: &mut App) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();

    Space::new().vertical().gap_md().children(vec![
        Progress::new(50.0).primary(),
        Progress::new(75.0).gradient(vec![
            theme.success.base,
            theme.warning.base,
            theme.danger.base,
            theme.primary.base,
        ]),
        Progress::new(64.0)
            .color(theme.info.base)
            .track_color(theme.neutral.hover)
            .animated(false),
    ])
}
