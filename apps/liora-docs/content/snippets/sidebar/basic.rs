//! Standalone self-sizing sidebar example.

use gpui::{Context, Entity, IntoElement, Render, Window, div, prelude::*, px};
use liora_components::{Button, Card, Menu, MenuMode, Sidebar, Space};
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
        Card::new(div().h(px(360.0)).flex().child(
            Sidebar::new()
                .id("app-sidebar")
                .expanded_width(px(280.0))
                .header_padding(px(14.0))
                .content_padding(px(8.0))
                .footer_padding(px(12.0))
                .gap(px(8.0))
                .rounded(px(16.0))
                .brand("Liora Workspace")
                .brand_subtitle("Native GPUI shell")
                .logo(
                    div()
                        .size(px(34.0))
                        .rounded(px(10.0))
                        .bg(gpui::transparent_black()),
                )
                .brand_action(Button::new("+").small())
                .scrollable()
                .child(self.menu.clone())
                .footer(
                    Space::new()
                        .gap_sm()
                        .child(Button::new("New").small())
                        .child(Button::new("Settings").small()),
                ),
        ))
        .no_shadow()
    }
}
