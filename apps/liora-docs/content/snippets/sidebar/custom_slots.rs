//! Fully custom header/content/footer slots when the brand helpers are not
//! enough for a product-specific sidebar.

use gpui::{Context, IntoElement, Render, Window};
use liora_components::{Button, Card, Flex, Sidebar, Space, Text};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct CustomSidebarSlotsExample;

impl Render for CustomSidebarSlotsExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Card::new(
            Flex::new()
                .height_units(336.0)
                .row()
                .overflow_hidden()
                .child(
                    Sidebar::new()
                        .id("docs-sidebar-custom-slots")
                        .expanded_width_units(320.0)
                        .header_padding_units(14.0)
                        .content_padding_units(12.0)
                        .footer_padding_units(14.0)
                        .gap_units(10.0)
                        .background(theme.warning.light_9)
                        .border(false)
                        .rounded_units(20.0)
                        .header(
                            Space::new()
                                .gap_sm()
                                .child(Icon::new(IconName::Rocket).size_units(20.0))
                                .child(Text::new("Release cockpit").bold()),
                        )
                        .children([
                            quick_stat("Open PRs", "12"),
                            quick_stat("Queued jobs", "7"),
                            quick_stat("Warnings", "3"),
                        ])
                        .footer(
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Text::new("Custom footer slot").xs().bold())
                                .child(Button::new("Review release").small().primary()),
                        ),
                ),
        )
        .no_shadow()
    }
}

fn quick_stat(label: &'static str, value: &'static str) -> impl IntoElement {
    Space::new()
        .gap_sm()
        .child(Text::new(label).xs())
        .child(Text::new(value).bold())
}
