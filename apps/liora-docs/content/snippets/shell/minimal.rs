//! Minimal embedded Shell for plugin hosts, settings pages, or previews.

use gpui::{Context, IntoElement, Render, Window};
use liora_components::{Button, Shell, ShellOverlayPosition, Space, Text, Title};
use liora_core::Config;

pub struct MinimalShellExample;

impl Render for MinimalShellExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Shell::new(
            Space::new()
                .vertical()
                .gap_md()
                .child(Title::new("Embedded surface").h4())
                .child(Text::new(
                    "Keep consistent shell background, padding, and overlay policy.",
                ))
                .child(Button::new("Run preview").primary()),
        )
        .id("docs-shell-minimal")
        .main_padding_units(24.0)
        .main_background(theme.neutral.card)
        .main_rounded_units(18.0)
        .overlay(Text::new("Overlay slot").xs())
        .overlay_position(ShellOverlayPosition::BottomRight)
        .overlay_inset_units(18.0)
        .background(theme.neutral.body)
    }
}
