//! Spinner composition examples.

use gpui::{IntoElement, div, prelude::*, px, rgb};
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
                .w(px(320.0))
                .flex()
                .items_center()
                .justify_between()
                .gap_4()
                .rounded_lg()
                .border_1()
                .border_color(rgb(0xe2e8f0))
                .bg(rgb(0xf8fafc))
                .p_4()
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.0))
                        .child(Label::new("Fetching metrics").custom_icon(Spinner::new().small())),
                )
                .child(
                    div()
                        .flex_none()
                        .child(Text::new("12 jobs queued").xs().nowrap()),
                ),
        )
        .child(
            div()
                .w(px(320.0))
                .min_h(px(84.0))
                .flex()
                .items_center()
                .gap_4()
                .rounded_lg()
                .border_1()
                .border_color(rgb(0xe2e8f0))
                .bg(rgb(0xffffff))
                .p_4()
                .child(
                    div().flex_1().min_w(px(0.0)).child(
                        Space::new()
                            .vertical()
                            .gap_xs()
                            .child(Text::new("Background export").bold().nowrap())
                            .child(Text::new("Exporting reports.zip · 42%").xs().wrap()),
                    ),
                )
                .child(
                    div()
                        .flex_none()
                        .child(Spinner::new().icon(IconName::LoaderCircle).large()),
                ),
        )
}
