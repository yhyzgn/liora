//! Collapsed icon rail for compact navigation.

use gpui::{AppContext, Context, Entity, IntoElement, Render, Window};
use liora_components::{Card, Flex, Menu, MenuMode, Sidebar, SidebarCollapseMode};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct IconRailSidebarExample {
    menu: Entity<Menu>,
}

impl IconRailSidebarExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            menu: cx.new(|_| {
                Menu::new()
                    .id("docs-sidebar-icon-menu")
                    .mode(MenuMode::Vertical)
                    .default_active("home")
                    .collapse(true)
                    .item("home", "Home", Some(IconName::House))
                    .item("search", "Search", Some(IconName::Search))
                    .item("inbox", "Inbox", Some(IconName::Inbox))
                    .item("settings", "Settings", Some(IconName::Settings))
            }),
        }
    }
}

impl Render for IconRailSidebarExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Card::new(
            Flex::new()
                .height_units(316.0)
                .row()
                .overflow_hidden()
                .child(
                    Sidebar::new()
                        .id("docs-sidebar-icon-rail")
                        .collapse_mode(SidebarCollapseMode::IconsOnly)
                        .collapsed_width_units(72.0)
                        .expanded_width_units(260.0)
                        .header_padding_units(10.0)
                        .content_padding_units(8.0)
                        .footer_padding_units(10.0)
                        .gap_units(8.0)
                        .background(theme.primary.light_9)
                        .border_color(theme.primary.light_7)
                        .rounded_units(18.0)
                        .logo(
                            Icon::new(IconName::Sparkles)
                                .size_units(20.0)
                                .color(theme.primary.base),
                        )
                        .scrollable()
                        .child(self.menu.clone())
                        .footer(Icon::new(IconName::Settings).size_units(18.0)),
                ),
        )
        .no_shadow()
    }
}
