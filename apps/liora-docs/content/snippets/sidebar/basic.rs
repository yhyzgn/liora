//! Standalone self-sizing sidebar example.

use gpui::{Context, Entity, IntoElement, Render, Window, div, prelude::*, px};
use liora_components::{Button, Card, Menu, MenuMode, Sidebar, Space, Text};
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
                .scrollable()
                .header(
                    Space::new()
                        .vertical()
                        .gap_xs()
                        .p(px(12.0))
                        .child(Text::new("Workspace").bold())
                        .child(Text::new("Navigation shell").sm()),
                )
                .child(self.menu.clone())
                .footer(
                    Space::new()
                        .p(px(12.0))
                        .gap_sm()
                        .child(Button::new("New").small())
                        .child(Button::new("Settings").small()),
                ),
        ))
        .no_shadow()
    }
}
