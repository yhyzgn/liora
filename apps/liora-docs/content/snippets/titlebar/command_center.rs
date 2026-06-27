//! Command-center titlebar with leading status, centered command entry, and
//! right-side action buttons.

use gpui::{Context, IntoElement, Render, Window};
use liora_components::{Button, Card, Flex, Space, Text, TitleBar, TitleBarContentAlign};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct CommandCenterTitleBarExample;

impl Render for CommandCenterTitleBarExample {
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
                        .id("docs-titlebar-command-center")
                        .title("Command shell")
                        .subtitle("Centered slot")
                        .leading(
                            Space::new()
                                .gap_xs()
                                .child(
                                    Icon::new(IconName::Circle)
                                        .size_units(10.0)
                                        .color(theme.success.base),
                                )
                                .child(Text::new("Online").xs().bold()),
                        )
                        .center(
                            Space::new()
                                .gap_sm()
                                .child(
                                    Icon::new(IconName::Search)
                                        .size_units(14.0)
                                        .color(theme.primary.base),
                                )
                                .child(Text::new("Search commands or files…").sm()),
                        )
                        .actions([
                            Button::new("Inspect").small(),
                            Button::new("Publish").small().primary(),
                        ])
                        .height_units(58.0)
                        .padding_x_units(18.0)
                        .gap_units(10.0)
                        .actions_gap_units(8.0)
                        .background(theme.neutral.card)
                        .border_color(theme.neutral.border)
                        .title_color(theme.neutral.text_1)
                        .subtitle_color(theme.neutral.text_3)
                        .content_align(TitleBarContentAlign::Center)
                        .window_controls(false),
                ),
        )
        .no_shadow()
    }
}
