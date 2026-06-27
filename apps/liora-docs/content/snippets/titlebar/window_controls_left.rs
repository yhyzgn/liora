//! Titlebar with real native controls on the left side.

use gpui::{Context, IntoElement, Render, Window};
use liora_components::{
    Button, Card, Flex, TitleBar, TitleBarContentAlign, WindowControlsPosition,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct TitleBarLeftControlsExample;

impl Render for TitleBarLeftControlsExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Card::new(
            Flex::new()
                .overflow_hidden()
                .rounded_units(14.0)
                .border()
                .border_color(theme.neutral.border)
                .child(
                    TitleBar::new()
                        .id("docs-titlebar-controls-left")
                        .title("Inspector")
                        .subtitle("Left controls + manual drag policy")
                        .icon(Icon::new(IconName::SlidersHorizontal).size_units(16.0))
                        .compact()
                        .draggable(false)
                        .background(theme.neutral.popover)
                        .border_color(theme.neutral.border)
                        .title_color(theme.neutral.text_1)
                        .subtitle_color(theme.neutral.text_3)
                        .content_align(TitleBarContentAlign::End)
                        .window_controls_position(WindowControlsPosition::Left)
                        .window_controls(true)
                        .action(Button::new("Reset").small()),
                ),
        )
        .no_shadow()
    }
}
