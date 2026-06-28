use gpui::{App, AppContext, Context, Entity, IntoElement, Render, Window};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    Button, Card, Flex, NavigationMenu, NavigationMenuMode, Shell, ShellOverlayPosition, Sidebar,
    Space, Text, Title, TitleBar, WindowFrameMode,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::Theme;

pub fn render(cx: &mut App) -> Entity<ShellDemo> {
    cx.new(|cx| ShellDemo {
        primary_menu: cx.new(|_| shell_primary_menu()),
        dense_menu: cx.new(|_| shell_dense_menu()),
        inspector_menu: cx.new(|_| shell_inspector_menu()),
    })
}

pub struct ShellDemo {
    primary_menu: Entity<NavigationMenu>,
    dense_menu: Entity<NavigationMenu>,
    inspector_menu: Entity<NavigationMenu>,
}

impl Render for ShellDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        page(
            "Shell 应用框架",
            "Shell 是 Liora 的高层应用框架控件：统一封装 titlebar、header、left/right sidebar、main、footer、overlay 和 custom/system frame。示例只组合 SDK 控件，不直接写 GPUI 原生布局。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Full product shell",
                    "Custom titlebar + 顶部工具栏 + 左侧导航 + 右侧 inspector + 可滚动 main + footer。",
                    shell_stage(&theme, full_product_shell(&theme, self.primary_menu.clone(), self.inspector_menu.clone())),
                ))
                .child(section(
                    "Content-first workspace",
                    "System frame 下的轻量 shell：隐藏 titlebar，自定义 header/footer，并用右侧栏承载上下文说明。",
                    shell_stage(&theme, content_first_shell(&theme, self.dense_menu.clone())),
                ))
                .child(section(
                    "Minimal embedded shell",
                    "嵌入式场景只保留 main + overlay，适合设置页、向导、内嵌预览或插件宿主区域。",
                    shell_stage(&theme, minimal_embedded_shell(&theme)),
                )),
        )
    }
}

fn full_product_shell(
    theme: &Theme,
    menu: Entity<NavigationMenu>,
    inspector: Entity<NavigationMenu>,
) -> Shell {
    Shell::new(content_panel(
        theme,
        "Dashboard workspace",
        "The main region can scroll independently while sidebars keep their own shell layout.",
    ))
    .id("shell-full-product-demo")
    .mode(WindowFrameMode::Custom)
    .titlebar(
        TitleBar::new()
            .title("Liora Product")
            .subtitle("Full shell composition")
            .icon(Icon::new(IconName::Sparkles).size_units(16.0))
            .height_units(56.0)
            .padding_x_units(18.0)
            .gap_units(10.0)
            .actions_gap_units(8.0)
            .background(theme.neutral.card)
            .border_color(theme.neutral.border)
            .title_color(theme.neutral.text_1)
            .subtitle_color(theme.neutral.text_3)
            .window_controls(true)
            .action(Button::new("Sync").small())
            .action(Button::new("Publish").small().primary()),
    )
    .header(shell_header(theme, "Production", "All systems nominal"))
    .header_height_units(54.0)
    .header_background(theme.neutral.card)
    .header_border_color(theme.primary.light_7)
    .sidebar(
        Sidebar::new()
            .id("shell-full-left-sidebar")
            .expanded_width_units(276.0)
            .header_padding_units(14.0)
            .content_padding_units(6.0)
            .footer_padding_units(12.0)
            .gap_units(8.0)
            .background(theme.neutral.card)
            .border_color(theme.neutral.border)
            .brand("Workspace")
            .brand_subtitle("Product navigation")
            .logo(Icon::new(IconName::Blocks).size_units(18.0))
            .brand_action(Button::new("+").small())
            .scrollable()
            .child(menu)
            .footer(
                Text::new("v0.1 shell")
                    .xs()
                    .text_color(theme.neutral.text_3),
            ),
    )
    .right_sidebar(
        Sidebar::new()
            .id("shell-full-right-sidebar")
            .right()
            .expanded_width_units(248.0)
            .header_padding_units(12.0)
            .content_padding_units(8.0)
            .footer_padding_units(12.0)
            .gap_units(8.0)
            .background(theme.neutral.popover)
            .border_color(theme.neutral.border)
            .brand("Inspector")
            .brand_subtitle("Context panel")
            .logo(Icon::new(IconName::PanelRight).size_units(18.0))
            .scrollable()
            .child(inspector)
            .content(property_stack(theme))
            .footer(Button::new("Open details").small()),
    )
    .main(metrics_grid(theme))
    .footer(shell_footer(theme, "Workspace ready"))
    .footer_height_units(42.0)
    .footer_background(theme.neutral.card)
    .footer_border_color(theme.primary.light_7)
    .main_scroll()
    .main_padding_units(16.0)
    .body_background(theme.neutral.body)
    .main_background(theme.neutral.body)
    .background(theme.neutral.body)
}

fn content_first_shell(theme: &Theme, menu: Entity<NavigationMenu>) -> Shell {
    Shell::new(content_panel(
        theme,
        "Document canvas",
        "System frame mode keeps platform decorations while Shell still owns all app regions.",
    ))
    .id("shell-content-first-demo")
    .mode(WindowFrameMode::System)
    .header(shell_header(
        theme,
        "Docs workspace",
        "System frame + app shell",
    ))
    .header_height_units(58.0)
    .header_background(theme.primary.light_9)
    .header_border_color(theme.primary.light_7)
    .sidebar(
        Sidebar::new()
            .id("shell-content-sidebar")
            .expanded_width_units(220.0)
            .header_padding_units(12.0)
            .content_padding_units(6.0)
            .footer_padding_units(10.0)
            .gap_units(8.0)
            .background(theme.primary.light_9)
            .border_color(theme.primary.light_7)
            .brand("Docs")
            .brand_subtitle("Compact nav")
            .logo(Icon::new(IconName::BookOpen).size_units(18.0))
            .scrollable()
            .child(menu),
    )
    .right_sidebar(context_note(theme))
    .footer(shell_footer(theme, "Autosaved just now"))
    .footer_height_units(38.0)
    .footer_background(theme.primary.light_9)
    .footer_border_color(theme.primary.light_7)
    .main_padding_units(18.0)
    .main_scroll()
    .body_gap_units(10.0)
    .body_background(theme.neutral.body)
    .main_background(theme.neutral.card)
    .main_rounded_units(18.0)
}

fn minimal_embedded_shell(theme: &Theme) -> Shell {
    Shell::new(
        Space::new()
            .vertical()
            .gap_md()
            .child(Title::new("Embedded surface").h4())
            .child(Text::new("Use Shell without titlebar or sidebars when a plugin, settings page, or preview still needs consistent body/background/scroll behavior."))
            .child(Button::new("Run preview").primary()),
    )
    .id("shell-minimal-demo")
    .main_padding_units(24.0)
    .main_background(theme.neutral.card)
    .main_rounded_units(18.0)
    .overlay(status_badge(theme, "Overlay slot"))
    .overlay_position(ShellOverlayPosition::BottomRight)
    .overlay_inset_units(18.0)
    .background(theme.neutral.body)
}

fn shell_stage(theme: &Theme, shell: Shell) -> Flex {
    Flex::new()
        .height_units(420.0)
        .w_full()
        .overflow_hidden()
        .rounded_units(18.0)
        .border()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.body)
        .child(shell)
}

fn shell_header(theme: &Theme, title: &'static str, subtitle: &'static str) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .justify_between()
        .w_full()
        .padding_x_units(16.0)
        .child(
            Space::new()
                .vertical()
                .gap_xs()
                .child(Text::new(title).bold().text_color(theme.neutral.text_1))
                .child(Text::new(subtitle).xs().text_color(theme.neutral.text_3)),
        )
        .child(
            Space::new()
                .gap_sm()
                .child(Button::new("Search").small())
                .child(Button::new("New").small().primary()),
        )
}

fn shell_footer(theme: &Theme, status: &'static str) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .justify_between()
        .w_full()
        .padding_x_units(16.0)
        .child(Text::new(status).xs().text_color(theme.neutral.text_3))
        .child(
            Text::new("Liora Shell")
                .xs()
                .bold()
                .text_color(theme.neutral.text_2),
        )
}

fn content_panel(theme: &Theme, title: &'static str, body: &'static str) -> Card {
    Card::new(
        Space::new()
            .vertical()
            .gap_sm()
            .child(Title::new(title).h4())
            .child(Text::new(body).text_color(theme.neutral.text_2))
            .child(Button::new("Primary action").primary()),
    )
    .no_shadow()
}

fn metrics_grid(theme: &Theme) -> Space {
    Space::new()
        .wrap()
        .gap_md()
        .child(metric_card(theme, "Active tasks", "128"))
        .child(metric_card(theme, "Open reviews", "24"))
        .child(metric_card(theme, "Release score", "98%"))
}

fn metric_card(theme: &Theme, label: &'static str, value: &'static str) -> Card {
    Card::new(
        Space::new()
            .vertical()
            .gap_xs()
            .child(Text::new(label).xs().text_color(theme.neutral.text_3))
            .child(Title::new(value).h3()),
    )
    .width_md()
    .no_shadow()
}

fn property_stack(theme: &Theme) -> Space {
    Space::new()
        .vertical()
        .gap_sm()
        .child(
            Text::new("Layout: full shell")
                .sm()
                .text_color(theme.neutral.text_2),
        )
        .child(
            Text::new("Frame: custom")
                .sm()
                .text_color(theme.neutral.text_2),
        )
        .child(
            Text::new("Main: scrollable")
                .sm()
                .text_color(theme.neutral.text_2),
        )
}

fn context_note(theme: &Theme) -> Sidebar {
    Sidebar::new()
        .id("shell-content-right-note")
        .right()
        .expanded_width_units(230.0)
        .header_padding_units(12.0)
        .content_padding_units(12.0)
        .footer_padding_units(12.0)
        .gap_units(8.0)
        .background(theme.warning.light_9)
        .border(false)
        .brand("Context")
        .brand_subtitle("Optional right area")
        .logo(Icon::new(IconName::Info).size_units(18.0))
        .content(Text::new(
            "Right-side slots can host inspectors, tips, AI panels, logs, or secondary navigation.",
        ))
}

fn status_badge(theme: &Theme, label: &'static str) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .padding_x_units(12.0)
        .padding_y_px(6.0)
        .rounded_pill()
        .bg(theme.success.light_9)
        .border()
        .border_color(theme.success.light_7)
        .child(Text::new(label).xs().bold().text_color(theme.success.base))
}

fn shell_primary_menu() -> NavigationMenu {
    NavigationMenu::new()
        .id("shell-primary-menu")
        .mode(NavigationMenuMode::Vertical)
        .default_active("dashboard")
        .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
        .item("components", "Components", Some(IconName::Blocks))
        .item("workflows", "Workflows", Some(IconName::GitBranch))
        .item("settings", "Settings", Some(IconName::Settings))
}

fn shell_dense_menu() -> NavigationMenu {
    NavigationMenu::new()
        .id("shell-dense-menu")
        .mode(NavigationMenuMode::Vertical)
        .default_active("overview")
        .item("overview", "Overview", Some(IconName::BookOpen))
        .item("guides", "Guides", Some(IconName::Map))
        .item("api", "API", Some(IconName::Code))
        .item("examples", "Examples", Some(IconName::Blocks))
        .item("faq", "FAQ", Some(IconName::Info))
}

fn shell_inspector_menu() -> NavigationMenu {
    NavigationMenu::new()
        .id("shell-inspector-menu")
        .mode(NavigationMenuMode::Vertical)
        .default_active("state")
        .item("state", "State", Some(IconName::Activity))
        .item("tokens", "Tokens", Some(IconName::Palette))
        .item("events", "Events", Some(IconName::RadioTower))
}

#[cfg(test)]
mod tests {
    #[test]
    fn shell_demo_uses_sdk_shell_components() {
        let source = include_str!("shell_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("Shell::new("));
        assert!(source.contains("TitleBar::new()"));
        assert!(source.contains("Sidebar::new()"));
        assert!(source.contains("WindowFrameMode::Custom"));
        assert!(source.contains("WindowFrameMode::System"));
        assert!(source.contains(".right_sidebar("));
        assert!(source.contains(".overlay("));
        assert!(source.contains("ShellOverlayPosition::BottomRight"));
        assert!(source.contains(".overlay_inset_units("));
        assert!(source.contains(".header_height_units("));
        assert!(source.contains(".footer_height_units("));
        assert!(source.contains(".header_background("));
        assert!(source.contains(".footer_background("));
        assert!(source.contains(".body_background("));
        assert!(source.contains(".main_background("));
        assert!(source.contains(".main_rounded_units("));
        assert!(source.contains(".body_gap_units("));
        assert!(source.contains("NavigationMenu::new()"));
        assert!(source.contains("primary_menu: Entity<NavigationMenu>"));
        assert!(source.contains("inspector_menu: Entity<NavigationMenu>"));
    }

    #[test]
    fn shell_demo_does_not_build_raw_gpui_layout_elements() {
        let source = include_str!("shell_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(!source.contains(" div,"));
        assert!(!source.contains("div()"));
        assert!(!source.contains("gpui::Div"));
        assert!(!source.contains("gpui::div"));
    }
}
