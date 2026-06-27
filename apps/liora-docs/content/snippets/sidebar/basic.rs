//! Standalone self-sizing sidebar example.

use gpui::{AppContext, Context, Entity, IntoElement, Render, Window};
use liora_components::{Button, Card, Flex, Menu, MenuMode, Sidebar, Space};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct SidebarExample {
    menu: Entity<Menu>,
}

impl SidebarExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            menu: cx.new(|_| {
                Menu::new()
                    .id("sidebar-example-menu")
                    .mode(MenuMode::Vertical)
                    .default_active("dashboard")
                    .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
                    .item("projects", "Projects", Some(IconName::Blocks))
                    .item("settings", "Settings", Some(IconName::Settings))
            }),
        }
    }
}

impl Render for SidebarExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Card::new(
            Flex::new().height_units(360.0).row().child(
                Sidebar::new()
                    .id("app-sidebar")
                    .expanded_width_units(280.0)
                    .header_padding_units(14.0)
                    .content_padding_units(8.0)
                    .footer_padding_units(12.0)
                    .gap_units(8.0)
                    .rounded_units(16.0)
                    .brand("Liora Workspace")
                    .brand_subtitle("Native GPUI shell")
                    .logo(Icon::new(IconName::Sparkles).size_units(20.0))
                    .brand_action(Button::new("+").small())
                    .scrollable()
                    .child(self.menu.clone())
                    .footer(
                        Space::new()
                            .gap_sm()
                            .child(Button::new("New").small())
                            .child(Button::new("Settings").small()),
                    ),
            ),
        )
        .no_shadow()
    }
}
