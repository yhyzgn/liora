//! Spinner size and icon examples.

use gpui::{IntoElement, div, prelude::*, px, rgb};
use liora_components::{Space, Spinner, Text};
use liora_icons_lucide::IconName;

pub fn spinner_sizes() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(spinner_size_row(
            "Small / inline",
            "Button labels and status bars",
            Spinner::new().small(),
        ))
        .child(spinner_size_row(
            "Default / row",
            "List rows and toolbar jobs",
            Spinner::new(),
        ))
        .child(spinner_size_row(
            "Large / panel",
            "Card-level refresh state",
            Spinner::new().large(),
        ))
        .child(spinner_size_row(
            "Custom icon",
            "RefreshCw with spin motion",
            Spinner::new().icon(IconName::RefreshCw).size(px(22.0)),
        ))
}

fn spinner_size_row(
    title: &'static str,
    detail: &'static str,
    spinner: Spinner,
) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_4()
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xf8fafc))
        .p_3()
        .child(
            Space::new()
                .vertical()
                .gap_xs()
                .child(Text::new(title).bold())
                .child(Text::new(detail).xs()),
        )
        .child(spinner)
}
