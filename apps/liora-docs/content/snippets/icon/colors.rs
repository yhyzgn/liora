//! Use Liora theme colors to communicate semantic state.

use gpui::{Context, IntoElement, Render, Window, prelude::*};
use liora_components::{Space, Text};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct IconColorsDemo;

impl Render for IconColorsDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Space::new()
            .wrap()
            .gap_md()
            .child(labeled_icon(
                Icon::new(IconName::Star)
                    .size_lg()
                    .color(theme.primary.base),
                "Primary",
            ))
            .child(labeled_icon(
                Icon::new(IconName::Star)
                    .size_lg()
                    .color(theme.success.base),
                "Success",
            ))
            .child(labeled_icon(
                Icon::new(IconName::Star)
                    .size_lg()
                    .color(theme.warning.base),
                "Warning",
            ))
            .child(labeled_icon(
                Icon::new(IconName::Star).size_lg().color(theme.danger.base),
                "Danger",
            ))
    }
}

fn labeled_icon(icon: Icon, label: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .align_center()
        .gap_xs()
        .child(icon)
        .child(Text::new(label).nowrap())
}
