//! Completion color examples for line progress gradients.

use aura_components::{Progress, Space};
use aura_core::Config;
use gpui::{App, IntoElement};

pub fn gradient_complete_progress(cx: &mut App) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();

    Space::new().vertical().gap_md().children(vec![
        // In-progress bars use the beginning of the gradient as their resolved color.
        Progress::new(88.0)
            .gradient(vec![
                theme.info.base,
                theme.primary.base,
                theme.success.base,
            ])
            .track_color(theme.neutral.hover)
            .text("88%"),
        // At 100%, complete_color becomes the final fill color.
        Progress::new(100.0)
            .gradient(vec![
                theme.info.base,
                theme.primary.base,
                theme.success.base,
            ])
            .complete_color(theme.success.base)
            .track_color(theme.neutral.hover)
            .text("Completed"),
    ])
}
