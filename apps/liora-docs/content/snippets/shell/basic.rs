//! Application shell with custom titlebar and sidebar navigation.

use gpui::{App, Context, Entity, IntoElement, Render, Window};
use liora_components::{
    AppWindowFrame, Button, Card, Container, Flex, Menu, MenuMode, Sidebar, Space, Text, Title,
    TitleBar, WindowFrameMode,
};
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        AppWindowFrame::new(
            "Liora Shell",
            Container::new()
                .aside(
                    Sidebar::new()
                        .id("app-sidebar")
                        .header(
                            Flex::new()
                                .column()
                                .padding_md()
                                .gap_sm()
                                .child(Text::new("Workspace").bold())
                                .child(Text::new("Project navigation").sm()),
                        )
                        .child(self.menu.clone())
                        .footer(Flex::new().padding_md().child(Text::new("v0.1").sm())),
                )
                .child(
                    Flex::new().padding_lg().child(
                        Card::new(
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Title::new("Application content").h4())
                                .child(Text::new("Your app view renders here.")),
                        )
                        .no_shadow(),
                    ),
                ),
        )
        .mode(WindowFrameMode::Custom)
        .titlebar(
            TitleBar::new()
                .title("Liora Shell")
                .subtitle("TitleBar + Sidebar")
                .action(Button::new("Action").small()),
        )
    }
}

pub fn mount(cx: &mut App) -> Entity<ShellExample> {
    cx.new(ShellExample::new)
}
