//! Borderless embedded toolbar for drawers, floating panels, or local content
//! surfaces where real window controls would be unsafe.

use gpui::{Context, IntoElement, Render, Window};
use liora_components::{Button, Card, Flex, TitleBar};
use liora_core::Config;

pub struct BorderlessTitleBarExample;

impl Render for BorderlessTitleBarExample {
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
                        .id("docs-titlebar-borderless")
                        .borderless()
                        .border(false)
                        .title("Preview canvas")
                        .subtitle("Embedded toolbar")
                        .height_units(46.0)
                        .padding_x_units(16.0)
                        .background(theme.primary.light_9)
                        .border_color(theme.primary.light_7)
                        .title_color(theme.neutral.text_1)
                        .subtitle_color(theme.neutral.text_2)
                        .window_controls(false)
                        .actions([Button::new("Fit").small(), Button::new("Export").small()]),
                ),
        )
        .no_shadow()
    }
}
