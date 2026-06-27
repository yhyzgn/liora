//! Right-side inspector sidebar for properties, previews, or contextual help.

use gpui::{AppContext, Context, Entity, IntoElement, Render, Window};
use liora_components::{Button, Card, Flex, Menu, MenuMode, Sidebar, Space, Text};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct InspectorSidebarExample {
    menu: Entity<Menu>,
}

impl InspectorSidebarExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            menu: cx.new(|_| {
                Menu::new()
                    .id("docs-sidebar-inspector-menu")
                    .mode(MenuMode::Vertical)
                    .default_active("layout")
                    .item("layout", "Layout", Some(IconName::PanelRight))
                    .item("tokens", "Tokens", Some(IconName::Palette))
                    .item("events", "Events", Some(IconName::Activity))
            }),
        }
    }
}

impl Render for InspectorSidebarExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Card::new(
            Flex::new()
                .height_units(340.0)
                .row()
                .justify_end()
                .overflow_hidden()
                .child(
                    Sidebar::new()
                        .id("docs-sidebar-inspector")
                        .right()
                        .expanded_width_units(268.0)
                        .brand("Inspector")
                        .brand_subtitle("Selection details")
                        .logo(Icon::new(IconName::PanelRight).size_units(20.0))
                        .brand_action(Button::new("Pin").small())
                        .background(theme.neutral.popover)
                        .border_color(theme.neutral.border)
                        .rounded_units(14.0)
                        .header_padding_units(12.0)
                        .content_padding_units(10.0)
                        .footer_padding_units(12.0)
                        .gap_units(6.0)
                        .scrollable()
                        .child(self.menu.clone())
                        .content(
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Text::new("Width 268 px"))
                                .child(Text::new("Mode Full"))
                                .child(Text::new("Pinned Yes")),
                        )
                        .footer(Text::new("Updates when selection changes.").xs()),
                ),
        )
        .no_shadow()
    }
}
