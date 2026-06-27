//! Brand sidebar with logo, brand text, scrollable menu, and footer actions.

use gpui::{AppContext, Context, Entity, IntoElement, Render, Window};
use liora_components::{Button, Card, Flex, Menu, MenuMode, Sidebar, Space};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct BrandSidebarExample {
    menu: Entity<Menu>,
}

impl BrandSidebarExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            menu: cx.new(|_| workspace_menu("docs-sidebar-brand-menu")),
        }
    }
}

impl Render for BrandSidebarExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Card::new(
            Flex::new()
                .height_units(392.0)
                .row()
                .overflow_hidden()
                .child(
                    Sidebar::new()
                        .id("docs-sidebar-brand")
                        .left()
                        .expanded_width_units(286.0)
                        .min_width_units(220.0)
                        .max_width_units(360.0)
                        .resizable()
                        .header_padding_units(14.0)
                        .content_padding_units(8.0)
                        .footer_padding_units(12.0)
                        .gap_units(8.0)
                        .rounded_units(16.0)
                        .background(theme.neutral.card)
                        .border_color(theme.neutral.border)
                        .brand("Liora Workspace")
                        .brand_subtitle("Native GPUI shell")
                        .logo(
                            Icon::new(IconName::Sparkles)
                                .size_units(20.0)
                                .color(theme.primary.base),
                        )
                        .brand_action(Button::new("+").small().primary())
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

fn workspace_menu(id: &'static str) -> Menu {
    Menu::new()
        .id(id)
        .mode(MenuMode::Vertical)
        .default_active("dashboard")
        .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
        .item("projects", "Projects", Some(IconName::Blocks))
        .item("components", "Components", Some(IconName::Component))
        .item("settings", "Settings", Some(IconName::Settings))
}
