use gpui::{App, Entity, IntoElement, Render, Window, div, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Card, Menu, MenuMode, Sidebar, Space, Text};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> Entity<SidebarDemo> {
    cx.new(|cx| SidebarDemo {
        menu: cx.new(|_| workspace_menu()),
        inspector_menu: cx.new(|_| inspector_menu()),
        icon_menu: cx.new(|_| icon_only_menu()),
    })
}

pub struct SidebarDemo {
    menu: Entity<Menu>,
    inspector_menu: Entity<Menu>,
    icon_menu: Entity<Menu>,
}

impl Render for SidebarDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        page(
            "Sidebar 侧栏",
            "单独展示 Sidebar 的品牌头部、左右位置、折叠策略、固定 header/footer、滚动内容区和完全自定义插槽。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Brand workspace sidebar",
                    "常见应用导航：顶部 logo + brand、可滚动菜单、底部操作区、独立宽度和圆角。",
                    Card::new(
                        div().h(px(380.0)).flex().child(
                            Sidebar::new()
                                .id("sidebar-brand-demo")
                                .left()
                                .position(liora_components::SidebarPosition::Left)
                                .expanded_width(px(286.0))
                                .min_width(px(220.0))
                                .max_width(px(360.0))
                                .resizable()
                                .header_padding(px(14.0))
                                .content_padding(px(8.0))
                                .footer_padding(px(12.0))
                                .gap(px(8.0))
                                .rounded(px(16.0))
                                .background(rgb(0xffffff).into())
                                .border_color(rgb(0xe2e8f0).into())
                                .border(true)
                                .brand("Liora Workspace")
                                .brand_subtitle("Native GPUI shell")
                                .logo(liora_logo_tile())
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
                    .no_shadow(),
                ))
                .child(section(
                    "Right inspector sidebar",
                    "右侧检查器适合属性面板、预览详情或辅助信息；边框会自动切换到左侧。",
                    Card::new(
                        div()
                            .h(px(320.0))
                            .flex()
                            .justify_end()
                            .bg(rgb(0xf8fafc))
                            .rounded(px(12.0))
                            .child(
                                Sidebar::new()
                                    .id("sidebar-inspector-demo")
                                    .right()
                                    .expanded_width(px(260.0))
                                    .header_padding(px(12.0))
                                    .content_padding(px(10.0))
                                    .footer_padding(px(12.0))
                                    .gap(px(6.0))
                                    .background(rgb(0xffffff).into())
                                    .border_color(rgb(0xcbd5e1).into())
                                    .rounded(px(14.0))
                                    .brand("Inspector")
                                    .brand_subtitle("Selection details")
                                    .logo(Icon::new(IconName::PanelRight).size(px(20.0)))
                                    .brand_action(Button::new("Pin").small())
                                    .scrollable()
                                    .child(self.inspector_menu.clone())
                                    .content(property_stack())
                                    .footer(Text::new("Updates when selection changes.").xs()),
                            ),
                    )
                    .no_shadow(),
                ))
                .child(section(
                    "Collapsed icon rail",
                    "IconsOnly 模式使用 collapsed_width，适合极窄导航栏或可折叠 shell。",
                    Card::new(
                        div().h(px(300.0)).flex().child(
                            Sidebar::new()
                                .id("sidebar-icon-rail-demo")
                                .collapse_mode(liora_components::SidebarCollapseMode::IconsOnly)
                                .collapsed_width(px(72.0))
                                .expanded_width(px(260.0))
                                .header_padding(px(10.0))
                                .content_padding(px(8.0))
                                .footer_padding(px(10.0))
                                .gap(px(8.0))
                                .background(rgb(0xf5f3ff).into())
                                .border_color(rgb(0xddd6fe).into())
                                .rounded(px(18.0))
                                .logo(liora_logo_tile())
                                .scrollable()
                                .child(self.icon_menu.clone())
                                .footer(
                                    div()
                                        .flex()
                                        .justify_center()
                                        .child(Icon::new(IconName::Settings).size(px(18.0))),
                                ),
                        ),
                    )
                    .no_shadow(),
                ))
                .child(section(
                    "Fully custom slots",
                    "当品牌 API 不够用时，可直接替换 header/content/footer，并用 children 批量组合任何 GPUI 元素。",
                    Card::new(
                        div().h(px(320.0)).flex().child(
                            Sidebar::new()
                                .id("sidebar-custom-slots-demo")
                                .expanded_width(px(320.0))
                                .header_padding(px(14.0))
                                .content_padding(px(12.0))
                                .footer_padding(px(14.0))
                                .gap(px(10.0))
                                .background(rgb(0xfffbeb).into())
                                .border(false)
                                .rounded(px(20.0))
                                .header(custom_header())
                                .children([
                                    quick_stat("Open PRs", "12", rgb(0xd97706).into()),
                                    quick_stat("Queued jobs", "7", rgb(0x2563eb).into()),
                                    quick_stat("Warnings", "3", rgb(0xdc2626).into()),
                                ])
                                .footer(
                                    Space::new()
                                        .vertical()
                                        .gap_sm()
                                        .child(Text::new("Custom footer slot").xs().bold())
                                        .child(Button::new("Review release").small().primary()),
                                ),
                        ),
                    )
                    .no_shadow(),
                )),
        )
    }
}

fn workspace_menu() -> Menu {
    Menu::new()
        .id("sidebar-demo-menu")
        .mode(MenuMode::Vertical)
        .default_active("dashboard")
        .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
        .item("projects", "Projects", Some(IconName::Blocks))
        .item("components", "Components", Some(IconName::Component))
        .item("settings", "Settings", Some(IconName::Settings))
}

fn inspector_menu() -> Menu {
    Menu::new()
        .id("sidebar-inspector-menu")
        .mode(MenuMode::Vertical)
        .default_active("layout")
        .item("layout", "Layout", Some(IconName::PanelRight))
        .item("tokens", "Tokens", Some(IconName::Palette))
        .item("events", "Events", Some(IconName::Activity))
}

fn icon_only_menu() -> Menu {
    Menu::new()
        .id("sidebar-icon-only-menu")
        .mode(MenuMode::Vertical)
        .default_active("home")
        .collapse(true)
        .item("home", "Home", Some(IconName::House))
        .item("search", "Search", Some(IconName::Search))
        .item("inbox", "Inbox", Some(IconName::Inbox))
        .item("settings", "Settings", Some(IconName::Settings))
}

fn liora_logo_tile() -> impl IntoElement {
    div()
        .size(px(34.0))
        .rounded(px(12.0))
        .bg(rgb(0x6366f1))
        .shadow_md()
        .child(
            div()
                .m(px(8.0))
                .size(px(18.0))
                .rounded_full()
                .bg(rgb(0x22d3ee))
                .opacity(0.86),
        )
}

fn property_stack() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(property_row("Width", "286 px"))
        .child(property_row("Mode", "Full"))
        .child(property_row("Pinned", "Yes"))
}

fn property_row(label: &'static str, value: &'static str) -> impl IntoElement {
    div()
        .flex()
        .justify_between()
        .px_2()
        .py_1()
        .rounded(px(8.0))
        .bg(rgb(0x1e293b))
        .child(Text::new(label).xs().text_color(rgb(0x94a3b8).into()))
        .child(
            Text::new(value)
                .xs()
                .bold()
                .text_color(rgb(0xf8fafc).into()),
        )
}

fn custom_header() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(
            Text::new("Release cockpit")
                .bold()
                .text_color(rgb(0x78350f).into()),
        )
        .child(Text::new("Everything here is supplied by the app.").xs())
}

fn quick_stat(label: &'static str, value: &'static str, color: gpui::Hsla) -> impl IntoElement {
    div()
        .flex()
        .justify_between()
        .items_center()
        .px_3()
        .py_2()
        .rounded(px(12.0))
        .bg(rgb(0xffffff))
        .border_1()
        .border_color(color.opacity(0.24))
        .child(Text::new(label).sm())
        .child(Text::new(value).bold().text_color(color))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sidebar_demo_is_standalone() {
        let source = include_str!("sidebar_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        for builder in [
            ".id(",
            ".left()",
            ".right()",
            ".position(",
            ".collapse_mode(",
            ".expanded_width(",
            ".collapsed_width(",
            ".min_width(",
            ".max_width(",
            ".resizable()",
            ".scrollable()",
            ".brand(",
            ".brand_subtitle(",
            ".logo(",
            ".brand_action(",
            ".header_padding(",
            ".content_padding(",
            ".footer_padding(",
            ".gap(",
            ".background(",
            ".border_color(",
            ".border(",
            ".rounded(",
            ".header(",
            ".child(",
            ".content(",
            ".children(",
            ".footer(",
        ] {
            assert!(
                source.contains(builder),
                "missing Sidebar builder {builder}"
            );
        }
        assert!(source.contains("Sidebar::new()"));
        assert!(source.contains("menu: Entity<Menu>"));
        assert!(!source.contains(concat!("TitleBa", "r::new()")));
        assert!(!source.contains(concat!("AppWindow", "Frame::new")));
    }
}
