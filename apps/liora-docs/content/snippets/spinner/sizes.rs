//! Spinner size and icon examples.

use gpui::{AnyElement, IntoElement, div, prelude::*, px, rgb};
use liora_components::{Space, Spinner, Text};
use liora_icons_lucide::IconName;

pub fn spinner_sizes() -> impl IntoElement {
    spinner_snippet_grid(vec![
        spinner_size_row(
            "Small / inline",
            "Button labels and status bars",
            Spinner::new().small(),
        ),
        spinner_size_row(
            "Default / row",
            "List rows and toolbar jobs",
            Spinner::new(),
        ),
        spinner_size_row(
            "Large / panel",
            "Card-level refresh state",
            Spinner::new().large(),
        ),
        spinner_size_row(
            "Custom icon",
            "RefreshCw with spin motion",
            Spinner::new().icon(IconName::RefreshCw).size(px(22.0)),
        ),
    ])
}

fn spinner_snippet_grid(children: Vec<AnyElement>) -> impl IntoElement {
    div().flex().flex_wrap().gap_3().children(children)
}

fn spinner_text(title: &'static str, detail: &'static str) -> impl IntoElement {
    div().flex_1().min_w(px(0.0)).child(
        Space::new()
            .vertical()
            .gap_xs()
            .child(Text::new(title).bold().nowrap())
            .child(Text::new(detail).xs().wrap()),
    )
}

fn spinner_size_row(title: &'static str, detail: &'static str, spinner: Spinner) -> AnyElement {
    div()
        .w(px(320.0))
        .min_h(px(84.0))
        .flex()
        .items_center()
        .gap_4()
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xf8fafc))
        .p_4()
        .child(spinner_text(title, detail))
        .child(div().flex_none().child(spinner))
        .into_any_element()
}
