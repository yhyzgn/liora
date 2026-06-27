use gpui::{App, AppContext, Entity, IntoElement, Render, Window};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Card, Flex, Menu, MenuMode, Sidebar, Space, Text};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::Theme;

pub fn render(cx: &mut App) -> Entity<SidebarDemo> {
    cx.new(|cx| SidebarDemo {
        menu: cx.new(|_| workspace_menu()),
        long_menu: cx.new(|_| long_workspace_menu()),
        inspector_menu: cx.new(|_| inspector_menu()),
        icon_menu: cx.new(|_| icon_only_menu()),
    })
}

pub struct SidebarDemo {
    menu: Entity<Menu>,
    long_menu: Entity<Menu>,
    inspector_menu: Entity<Menu>,
    icon_menu: Entity<Menu>,
}

impl Render for SidebarDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        page(
            "Sidebar 侧栏",
            "单独展示 Sidebar 的品牌头部、左右位置、折叠策略、固定 header/footer、滚动内容区和完全自定义插槽。所有背景、边框和文字都来自当前主题 token。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Brand workspace sidebar",
                    "常见应用导航：顶部 logo + brand、可滚动菜单、底部操作区、独立宽度和圆角。",
                    Card::new(demo_surface(&theme, 392.0).child(
                        Sidebar::new()
                            .id("sidebar-brand-demo")
                            .left()
                            .position(liora_components::SidebarPosition::Left)
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
                            .border(true)
                            .brand("Liora Workspace")
                            .brand_subtitle("Native GPUI shell")
                            .logo(liora_logo_tile(&theme))
                            .brand_action(Button::new("+").small().primary())
                            .scrollable()
                            .child(self.menu.clone())
                            .footer(
                                Space::new()
                                    .gap_sm()
                                    .child(Button::new("New").small())
                                    .child(Button::new("Settings").small()),
                            ),
                    ))
                    .no_shadow(),
                ))
                .child(section(
                    "Scrollable product navigation",
                    "长菜单会在 Sidebar 内容区内滚动，header/footer 固定不动；这个例子也展示更宽松的 logo + brand header。",
                    Card::new(demo_surface(&theme, 320.0).child(
                        Sidebar::new()
                            .id("sidebar-long-scroll-demo")
                            .expanded_width_units(316.0)
                            .header_padding_units(16.0)
                            .content_padding_units(6.0)
                            .footer_padding_units(10.0)
                            .gap_units(10.0)
                            .rounded_units(18.0)
                            .background(theme.neutral.card)
                            .border_color(theme.neutral.border)
                            .header(roomy_brand_header(&theme))
                            .scrollable()
                            .child(self.long_menu.clone())
                            .footer(
                                Flex::new()
                                    .row()
                                    .align_center()
                                    .justify_between()
                                    .child(
                                        Text::new("Pinned footer")
                                            .xs()
                                            .text_color(theme.neutral.text_3),
                                    )
                                    .child(Button::new("Upgrade").small().primary()),
                            ),
                    ))
                    .no_shadow(),
                ))
                .child(section(
                    "Right inspector sidebar",
                    "右侧检查器适合属性面板、预览详情或辅助信息；边框会自动切换到左侧。",
                    Card::new(
                        demo_surface(&theme, 340.0)
                            .justify_end()
                            .child(
                                Sidebar::new()
                                    .id("sidebar-inspector-demo")
                                    .right()
                                    .expanded_width_units(268.0)
                                    .header_padding_units(12.0)
                                    .content_padding_units(10.0)
                                    .footer_padding_units(12.0)
                                    .gap_units(6.0)
                                    .background(theme.neutral.popover)
                                    .border_color(theme.neutral.border)
                                    .rounded_units(14.0)
                                    .brand("Inspector")
                                    .brand_subtitle("Selection details")
                                    .logo(Icon::new(IconName::PanelRight).size_units(20.0))
                                    .brand_action(Button::new("Pin").small())
                                    .scrollable()
                                    .child(self.inspector_menu.clone())
                                    .content(property_stack(&theme))
                                    .footer(
                                        Text::new("Updates when selection changes.")
                                            .xs()
                                            .text_color(theme.neutral.text_3),
                                    ),
                            ),
                    )
                    .no_shadow(),
                ))
                .child(section(
                    "Collapsed icon rail",
                    "IconsOnly 模式使用 collapsed_width，适合极窄导航栏或可折叠 shell。",
                    Card::new(demo_surface(&theme, 316.0).child(
                        Sidebar::new()
                            .id("sidebar-icon-rail-demo")
                            .collapse_mode(liora_components::SidebarCollapseMode::IconsOnly)
                            .collapsed_width_units(72.0)
                            .expanded_width_units(260.0)
                            .header_padding_units(10.0)
                            .content_padding_units(8.0)
                            .footer_padding_units(10.0)
                            .gap_units(8.0)
                            .background(theme.primary.light_9)
                            .border_color(theme.primary.light_7)
                            .rounded_units(18.0)
                            .logo(liora_logo_tile(&theme))
                            .scrollable()
                            .child(self.icon_menu.clone())
                            .footer(
                                Flex::new()
                                    .row()
                                    .justify_center()
                                    .child(Icon::new(IconName::Settings).size_units(18.0)),
                            ),
                    ))
                    .no_shadow(),
                ))
                .child(section(
                    "Fully custom slots",
                    "当品牌 API 不够用时，可直接替换 header/content/footer，并用 children 批量组合任何 Liora 元素。",
                    Card::new(demo_surface(&theme, 336.0).child(
                        Sidebar::new()
                            .id("sidebar-custom-slots-demo")
                            .expanded_width_units(320.0)
                            .header_padding_units(14.0)
                            .content_padding_units(12.0)
                            .footer_padding_units(14.0)
                            .gap_units(10.0)
                            .background(theme.warning.light_9)
                            .border(false)
                            .rounded_units(20.0)
                            .header(custom_header(&theme))
                            .children([
                                quick_stat(&theme, "Open PRs", "12", theme.warning.base),
                                quick_stat(&theme, "Queued jobs", "7", theme.info.base),
                                quick_stat(&theme, "Warnings", "3", theme.danger.base),
                            ])
                            .footer(
                                Space::new()
                                    .vertical()
                                    .gap_sm()
                                    .child(
                                        Text::new("Custom footer slot")
                                            .xs()
                                            .bold()
                                            .text_color(theme.neutral.text_2),
                                    )
                                    .child(Button::new("Review release").small().primary()),
                            ),
                    ))
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

fn long_workspace_menu() -> Menu {
    Menu::new()
        .id("sidebar-long-menu")
        .mode(MenuMode::Vertical)
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
        .item("roles", "Roles", Some(IconName::ShieldCheck))
        .item("billing", "Billing", Some(IconName::CreditCard))
        .item("support", "Support", Some(IconName::MessagesSquare))
        .item("settings", "Settings", Some(IconName::Settings))
        .item("changelog", "Changelog", Some(IconName::ScrollText))
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

fn demo_surface(theme: &Theme, height: f32) -> Flex {
    shell_stage(theme)
        .height_units(height)
        .row()
        .align_start()
        .overflow_hidden()
}

fn shell_stage(theme: &Theme) -> Flex {
    Flex::new()
        .w_full()
        .rounded_units(18.0)
        .border()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.body)
        .padding_md()
}

fn roomy_brand_header(theme: &Theme) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .gap_md()
        .padding_md()
        .rounded_units(16.0)
        .bg(theme.primary.light_9)
        .border()
        .border_color(theme.primary.light_7)
        .child(roomy_logo_tile(theme))
        .child(
            Space::new()
                .vertical()
                .gap_xs()
                .grow()
                .shrink()
                .child(
                    Text::new("Liora Product")
                        .bold()
                        .text_color(theme.neutral.text_1),
                )
                .child(
                    Text::new("Scrollable navigation shell")
                        .xs()
                        .text_color(theme.neutral.text_3),
                ),
        )
        .child(Button::new("Pro").small().primary())
}

fn roomy_logo_tile(theme: &Theme) -> Flex {
    Flex::new()
        .row()
        .center()
        .flex_none()
        .width_px(46.0)
        .height_units(46.0)
        .rounded_units(16.0)
        .bg(theme.primary.base)
        .child(
            Icon::new(IconName::Sparkles)
                .size_units(22.0)
                .color(theme.neutral.inverted),
        )
}

fn liora_logo_tile(theme: &Theme) -> Icon {
    Icon::new(IconName::Sparkles)
        .size_units(20.0)
        .color(theme.primary.base)
}

fn property_stack(theme: &Theme) -> Space {
    Space::new()
        .vertical()
        .gap_sm()
        .child(property_row(theme, "Width", "268 px"))
        .child(property_row(theme, "Mode", "Full"))
        .child(property_row(theme, "Pinned", "Yes"))
}

fn property_row(theme: &Theme, label: &'static str, value: &'static str) -> Flex {
    Flex::new()
        .row()
        .justify_between()
        .padding_x_units(8.0)
        .padding_y_px(4.0)
        .rounded_units(8.0)
        .bg(theme.neutral.hover)
        .child(Text::new(label).xs().text_color(theme.neutral.text_3))
        .child(
            Text::new(value)
                .xs()
                .bold()
                .text_color(theme.neutral.text_1),
        )
}

fn custom_header(theme: &Theme) -> Space {
    Space::new()
        .vertical()
        .gap_sm()
        .child(
            Text::new("Release cockpit")
                .bold()
                .text_color(theme.neutral.text_1),
        )
        .child(
            Text::new("Everything here is supplied by the app.")
                .xs()
                .text_color(theme.neutral.text_3),
        )
}

fn quick_stat(theme: &Theme, label: &'static str, value: &'static str, color: gpui::Hsla) -> Flex {
    Flex::new()
        .row()
        .justify_between()
        .align_center()
        .padding_x_units(12.0)
        .padding_y_px(8.0)
        .rounded_units(12.0)
        .bg(theme.neutral.card)
        .border()
        .border_color(color.opacity(0.24))
        .child(Text::new(label).sm().text_color(theme.neutral.text_2))
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
            ".expanded_width_units(",
            ".collapsed_width_units(",
            ".min_width_units(",
            ".max_width_units(",
            ".resizable()",
            ".scrollable()",
            ".brand(",
            ".brand_subtitle(",
            ".logo(",
            ".brand_action(",
            ".header_padding_units(",
            ".content_padding_units(",
            ".footer_padding_units(",
            ".gap_units(",
            ".background(",
            ".border_color(",
            ".border(",
            ".rounded_units(",
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

    #[test]
    fn sidebar_demo_uses_theme_tokens_and_stable_showcase_layouts() {
        let source = include_str!("sidebar_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("cx.global::<Config>().theme"));
        assert!(source.contains("shell_stage("));
        assert!(source.contains("demo_surface("));
        assert!(source.contains("theme.neutral.card"));
        assert!(source.contains("theme.neutral.border"));
        assert!(!source.contains("rgb("));
    }

    #[test]
    fn sidebar_demo_does_not_build_raw_gpui_layout_elements() {
        let source = include_str!("sidebar_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(!source.contains(" div,"));
        assert!(!source.contains("div()"));
        assert!(!source.contains("gpui::Div"));
        assert!(!source.contains("gpui::div"));
    }
    #[test]
    fn sidebar_demo_covers_long_menu_scrolling_and_roomy_brand_header() {
        let source = include_str!("sidebar_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("long_menu: Entity<Menu>"));
        assert!(source.contains("long_workspace_menu()"));
        assert!(source.contains("roomy_brand_header("));
        assert!(source.contains("roomy_logo_tile("));
        assert!(source.contains(".content_padding_units(6.0)"));
        assert!(source.contains(".footer_padding_units(10.0)"));
        assert!(source.matches(".item(").count() >= 18);
    }
}
