use gpui::{App, Entity, IntoElement, Render, Window, div, prelude::*, px};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Card, Menu, MenuMode, Sidebar, Space, Text};
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> Entity<SidebarDemo> {
    cx.new(|cx| SidebarDemo {
        menu: cx.new(|_| sidebar_demo_menu()),
    })
}

pub struct SidebarDemo {
    menu: Entity<Menu>,
}

impl Render for SidebarDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        page(
            "Sidebar 侧栏",
            "单独展示 Sidebar 的自管理宽度、固定 header/footer 和滚动内容区。",
            Space::new().vertical().gap_xl().child(section(
                "Self-sizing sidebar",
                "Sidebar 自己拥有宽度和边框。放进 Container::aside(...) 时应配合 aside_passthrough()，避免 Container 再包一层固定宽度侧栏。",
                Card::new(
                    div().h(px(360.0)).flex().child(
                        Sidebar::new()
                            .id("sidebar-demo")
                            .expanded_width(px(280.0))
                            .scrollable()
                            .header(
                                div().p(px(12.0)).child(
                                    Space::new()
                                        .vertical()
                                        .gap_xs()
                                        .child(Text::new("Workspace").bold())
                                        .child(Text::new("Navigation shell").sm()),
                                ),
                            )
                            .child(self.menu.clone())
                            .footer(
                                div().p(px(12.0)).child(
                                    Space::new()
                                        .gap_sm()
                                        .child(Button::new("New").small())
                                        .child(Button::new("Settings").small()),
                                ),
                            ),
                    ),
                )
                .no_shadow(),
            )),
        )
    }
}

fn sidebar_demo_menu() -> Menu {
    Menu::new()
        .id("sidebar-demo-menu")
        .mode(MenuMode::Vertical)
        .default_active("dashboard")
        .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
        .item("projects", "Projects", Some(IconName::Blocks))
        .item("settings", "Settings", Some(IconName::Settings))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sidebar_demo_is_standalone() {
        let source = include_str!("sidebar_demo.rs");

        assert!(source.contains("Sidebar::new()"));
        assert!(source.contains("menu: Entity<Menu>"));
        assert!(source.contains(".scrollable()"));
        assert!(!source.contains(concat!("TitleBa", "r::new()")));
        assert!(!source.contains(concat!("AppWindow", "Frame::new")));
    }
}
