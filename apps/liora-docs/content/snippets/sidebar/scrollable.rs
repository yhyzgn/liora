//! Scrollable sidebar keeps header and footer fixed while a long Menu scrolls
//! inside the content region owned by Sidebar.

use gpui::{AppContext, Context, Entity, IntoElement, Render, Window};
use liora_components::{
    Button, Card, Flex, NavigationMenu, NavigationMenuMode, Sidebar, Space, Text,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct ScrollableSidebarExample {
    menu: Entity<NavigationMenu>,
}

impl ScrollableSidebarExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            menu: cx.new(|_| long_workspace_menu()),
        }
    }
}

impl Render for ScrollableSidebarExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Card::new(
            Flex::new()
                .height_units(320.0)
                .row()
                .overflow_hidden()
                .child(
                    Sidebar::new()
                        .id("docs-sidebar-long-scroll")
                        .expanded_width_units(316.0)
                        .header_padding_units(16.0)
                        .content_padding_units(6.0)
                        .footer_padding_units(10.0)
                        .gap_units(10.0)
                        .rounded_units(18.0)
                        .background(theme.neutral.card)
                        .border_color(theme.neutral.border)
                        .header(
                            Space::new()
                                .gap_sm()
                                .child(
                                    Icon::new(IconName::Sparkles)
                                        .size_units(22.0)
                                        .color(theme.primary.base),
                                )
                                .child(Text::new("Liora Product").bold())
                                .child(Button::new("Pro").small().primary()),
                        )
                        .scrollable()
                        .child(self.menu.clone())
                        .footer(
                            Space::new()
                                .gap_sm()
                                .child(Text::new("Pinned footer").xs())
                                .child(Button::new("Upgrade").small().primary()),
                        ),
                ),
        )
        .no_shadow()
    }
}

fn long_workspace_menu() -> NavigationMenu {
    NavigationMenu::new()
        .id("docs-sidebar-long-menu")
        .mode(NavigationMenuMode::Vertical)
        .default_active("overview")
        .item("overview", "Overview", Some(IconName::LayoutDashboard))
        .item("activity", "Activity", Some(IconName::Activity))
        .item("inbox", "Inbox", Some(IconName::Inbox))
        .item("calendar", "Calendar", Some(IconName::CalendarDays))
        .item("files", "Files", Some(IconName::Files))
        .item("components", "Components", Some(IconName::Component))
        .item("packages", "Packages", Some(IconName::Package))
        .item("experiments", "Experiments", Some(IconName::FlaskConical))
        .item("analytics", "Analytics", Some(IconName::ChartNoAxesColumn))
        .item("reports", "Reports", Some(IconName::FileText))
        .item("automations", "Automations", Some(IconName::Bot))
        .item("integrations", "Integrations", Some(IconName::Plug))
        .item("members", "Members", Some(IconName::Users))
        .item("billing", "Billing", Some(IconName::CreditCard))
        .item("support", "Support", Some(IconName::MessagesSquare))
        .item("settings", "Settings", Some(IconName::Settings))
}
