use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    AppWindowFrame, Button, Card, Container, Flex, Menu, MenuMode, Sidebar, Space, Text, Title,
    TitleBar, WindowFrameMode,
};
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> Entity<ShellDemo> {
    cx.new(|cx| ShellDemo {
        menu: cx.new(|_| shell_demo_menu()),
    })
}

pub struct ShellDemo {
    menu: Entity<Menu>,
}

impl Render for ShellDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let shell = AppWindowFrame::new(
            "Liora Shell",
            Container::new()
                .aside(
                    Sidebar::new()
                        .id("shell-demo-sidebar")
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
                                .child(Text::new(
                                    "TitleBar owns native window chrome; Sidebar owns app navigation shell layout.",
                                )),
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
        );

        page(
            "Shell 应用框架",
            "组合 TitleBar、Sidebar、Container 和 Menu 构建原生 GPUI 应用框架。",
            Space::new().vertical().gap_xl().child(section(
                "TitleBar + Sidebar",
                "TitleBar 负责窗口拖动与窗口控制；Sidebar 负责侧栏宽度、折叠和滚动容器。",
                Flex::new()
                    .height_units(360.0)
                    .w_full()
                    .border()
                    .child(shell),
            )),
        )
    }
}

fn shell_demo_menu() -> Menu {
    Menu::new()
        .id("shell-demo-menu")
        .mode(MenuMode::Vertical)
        .default_active("dashboard")
        .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
        .item("components", "Components", Some(IconName::Blocks))
        .item("settings", "Settings", Some(IconName::Settings))
}

#[cfg(test)]
mod tests {
    #[test]
    fn shell_demo_uses_sdk_shell_components() {
        let source = include_str!("shell_demo.rs");

        assert!(source.contains("TitleBar::new()"));
        assert!(source.contains("Sidebar::new()"));
        assert!(source.contains("AppWindowFrame::new"));
        assert!(source.contains("WindowFrameMode::Custom"));
        assert!(source.contains("Menu::new()"));
        assert!(source.contains("menu: Entity<Menu>"));
    }
}
