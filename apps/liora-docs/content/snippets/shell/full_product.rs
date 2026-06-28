//! Full product shell with custom titlebar, left navigation, right inspector,
//! scrollable main content, footer, and overlay.

use gpui::{AppContext, Context, Entity, IntoElement, Render, Window};
use liora_components::{
    Button, Card, NavigationMenu, NavigationMenuMode, Shell, ShellOverlayPosition, Sidebar, Space,
    Text, Title, TitleBar, WindowFrameMode,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct FullProductShellExample {
    menu: Entity<NavigationMenu>,
    inspector: Entity<NavigationMenu>,
}

impl FullProductShellExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            menu: cx.new(|_| product_menu("docs-shell-main-menu")),
            inspector: cx.new(|_| inspector_menu("docs-shell-inspector-menu")),
        }
    }
}

impl Render for FullProductShellExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Shell::new(
            Card::new(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(Title::new("Dashboard workspace").h4())
                    .child(Text::new("The main region scrolls independently."))
                    .child(Button::new("Primary action").primary()),
            )
            .no_shadow(),
        )
        .id("docs-shell-full-product")
        .mode(WindowFrameMode::Custom)
        .titlebar(
            TitleBar::new()
                .title("Liora Product")
                .subtitle("Full shell composition")
                .icon(Icon::new(IconName::Sparkles).size_units(16.0))
                .height_units(56.0)
                .background(theme.neutral.card)
                .border_color(theme.neutral.border)
                .title_color(theme.neutral.text_1)
                .subtitle_color(theme.neutral.text_3)
                .window_controls(true)
                .action(Button::new("Sync").small())
                .action(Button::new("Publish").small().primary()),
        )
        .header(
            Space::new()
                .gap_sm()
                .child(Text::new("Production").bold())
                .child(Text::new("All systems nominal").sm()),
        )
        .header_height_units(54.0)
        .header_background(theme.neutral.card)
        .header_border_color(theme.primary.light_7)
        .sidebar(
            Sidebar::new()
                .id("docs-shell-left-sidebar")
                .expanded_width_units(276.0)
                .brand("Workspace")
                .brand_subtitle("Product navigation")
                .logo(Icon::new(IconName::Blocks).size_units(18.0))
                .brand_action(Button::new("+").small())
                .background(theme.neutral.card)
                .border_color(theme.neutral.border)
                .header_padding_units(14.0)
                .content_padding_units(6.0)
                .footer_padding_units(12.0)
                .gap_units(8.0)
                .scrollable()
                .child(self.menu.clone())
                .footer(Text::new("v0.1 shell").xs()),
        )
        .right_sidebar(
            Sidebar::new()
                .id("docs-shell-right-sidebar")
                .right()
                .expanded_width_units(248.0)
                .brand("Inspector")
                .brand_subtitle("Context panel")
                .logo(Icon::new(IconName::PanelRight).size_units(18.0))
                .background(theme.neutral.popover)
                .border_color(theme.neutral.border)
                .header_padding_units(12.0)
                .content_padding_units(8.0)
                .footer_padding_units(12.0)
                .gap_units(8.0)
                .scrollable()
                .child(self.inspector.clone())
                .content(Text::new("Selection properties render here."))
                .footer(Button::new("Open details").small()),
        )
        .main(Text::new("Additional dashboard content can be appended."))
        .footer(Text::new("Workspace ready").xs())
        .footer_height_units(42.0)
        .footer_background(theme.neutral.card)
        .footer_border_color(theme.primary.light_7)
        .main_scroll()
        .main_padding_units(16.0)
        .body_background(theme.neutral.body)
        .main_background(theme.neutral.card)
        .main_rounded_units(18.0)
        .overlay(Text::new("Overlay slot").xs())
        .overlay_position(ShellOverlayPosition::BottomRight)
        .overlay_inset_units(18.0)
    }
}

fn product_menu(id: &'static str) -> NavigationMenu {
    NavigationMenu::new()
        .id(id)
        .mode(NavigationMenuMode::Vertical)
        .default_active("dashboard")
        .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
        .item("components", "Components", Some(IconName::Component))
        .item("releases", "Releases", Some(IconName::Rocket))
        .item("settings", "Settings", Some(IconName::Settings))
}

fn inspector_menu(id: &'static str) -> NavigationMenu {
    NavigationMenu::new()
        .id(id)
        .mode(NavigationMenuMode::Vertical)
        .default_active("layout")
        .item("layout", "Layout", Some(IconName::PanelRight))
        .item("tokens", "Tokens", Some(IconName::Palette))
        .item("events", "Events", Some(IconName::Activity))
}
