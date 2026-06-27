//! Application shell with custom titlebar and sidebar navigation.

use gpui::{App, AppContext, Context, Entity, IntoElement, Render, Window};
use liora_components::{
    Button, Card, Menu, MenuMode, Shell, ShellOverlayPosition, Sidebar, Space, Text, Title,
    TitleBar, WindowFrameMode,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct ShellExample {
    menu: Entity<Menu>,
}

impl ShellExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            menu: cx.new(|_| {
                Menu::new()
                    .id("shell-example-menu")
                    .mode(MenuMode::Vertical)
                    .default_active("dashboard")
                    .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
                    .item("components", "Components", Some(IconName::Blocks))
                    .item("settings", "Settings", Some(IconName::Settings))
            }),
        }
    }
}

impl Render for ShellExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Shell::new(
            Card::new(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(Title::new("Application content").h4())
                    .child(Text::new("Your app view renders here.")),
            )
            .no_shadow(),
        )
        .id("docs-shell-example")
        .mode(WindowFrameMode::Custom)
        .titlebar(
            TitleBar::new()
                .title("Liora Shell")
                .subtitle("TitleBar + Sidebar")
                .window_controls(true)
                .action(Button::new("Action").small()),
        )
        .sidebar(
            Sidebar::new()
                .id("app-sidebar")
                .brand("Workspace")
                .brand_subtitle("Project navigation")
                .logo(Icon::new(IconName::Sparkles).size_units(18.0))
                .expanded_width_units(260.0)
                .header_padding_units(12.0)
                .content_padding_units(6.0)
                .footer_padding_units(10.0)
                .gap_units(8.0)
                .scrollable()
                .child(self.menu.clone())
                .footer(Text::new("v0.1").sm()),
        )
        .main_padding_units(16.0)
        .body_background(theme.neutral.body)
        .main_background(theme.neutral.card)
        .main_rounded_units(16.0)
        .overlay(Text::new("Overlay slot").xs())
        .overlay_position(ShellOverlayPosition::TopRight)
        .overlay_inset_units(14.0)
        .main_scroll()
    }
}

pub fn mount(cx: &mut App) -> Entity<ShellExample> {
    cx.new(ShellExample::new)
}
