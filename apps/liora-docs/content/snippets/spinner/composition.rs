//! Spinner composition examples.

use gpui::{IntoElement, div, prelude::*, rgb};
use liora_components::{Button, Label, Space, Spinner, Text};
use liora_icons_lucide::IconName;

pub fn spinner_composition() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    Button::new("Syncing")
                        .primary()
                        .icon_start(Spinner::new().small().into_any_element()),
                )
                .child(
                    Button::new("Exporting").icon_start(Spinner::new().small().into_any_element()),
                ),
        )
        .child(
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
                .child(Label::new("Fetching metrics").custom_icon(Spinner::new().small()))
                .child(Text::new("12 jobs queued").xs()),
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_4()
                .rounded_lg()
                .border_1()
                .border_color(rgb(0xe2e8f0))
                .bg(rgb(0xffffff))
                .p_3()
                .child(
                    Space::new()
                        .vertical()
                        .gap_xs()
                        .child(Text::new("Background export").bold())
                        .child(Text::new("Exporting reports.zip · 42%").xs()),
                )
                .child(Spinner::new().icon(IconName::LoaderCircle).large()),
        )
}
