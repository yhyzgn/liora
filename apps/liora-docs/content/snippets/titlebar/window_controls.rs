//! Titlebar with real native minimize, maximize/restore, and close controls.

use gpui::{Context, IntoElement, Render, Window};
use liora_components::{
    Button, Card, Flex, Space, Text, TitleBar, TitleBarContentAlign, WindowControlsPosition,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct TitleBarControlsExample;

impl Render for TitleBarControlsExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Card::new(
            Space::new()
                .vertical()
                .gap_md()
                .child(Text::new("Right controls / brand shell").xs().bold())
                .child(
                    Flex::new()
                        .overflow_hidden()
                        .rounded_units(14.0)
                        .border()
                        .border_color(theme.neutral.border)
                        .child(
                            TitleBar::new()
                                .id("docs-titlebar-controls-right")
                                .title("Liora Studio")
                                .subtitle("Theme-aware native chrome")
                                .icon(Icon::new(IconName::Sparkles).size_units(16.0))
                                .height_units(62.0)
                                .padding_x_units(20.0)
                                .gap_units(12.0)
                                .actions_gap_units(8.0)
                                .background(theme.neutral.card)
                                .border_color(theme.neutral.border)
                                .title_color(theme.neutral.text_1)
                                .subtitle_color(theme.neutral.text_3)
                                .content_align(TitleBarContentAlign::Start)
                                .window_controls_position(WindowControlsPosition::Right)
                                .window_controls(true)
                                .action(Button::new("Share").small())
                                .action(Button::new("Deploy").small().primary()),
                        ),
                )
                .child(Text::new("Left controls / utility titlebar").xs().bold())
                .child(
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
                ),
        )
        .no_shadow()
    }
}
