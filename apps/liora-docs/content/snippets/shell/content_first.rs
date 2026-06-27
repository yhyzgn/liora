//! System-frame shell that keeps platform decorations while still using Shell
//! for header, sidebar, main, footer, and inspector regions.

use gpui::{AppContext, Context, Entity, IntoElement, Render, Window};
use liora_components::{
    Button, Card, Menu, MenuMode, Shell, Sidebar, Space, Text, Title, WindowFrameMode,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct ContentFirstShellExample {
    menu: Entity<Menu>,
}

impl ContentFirstShellExample {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            menu: cx.new(|_| {
                Menu::new()
                    .id("docs-shell-compact-menu")
                    .mode(MenuMode::Vertical)
                    .default_active("overview")
                    .item("overview", "Overview", Some(IconName::BookOpen))
                    .item("authoring", "Authoring", Some(IconName::PencilLine))
                    .item("release", "Release", Some(IconName::Rocket))
            }),
        }
    }
}

impl Render for ContentFirstShellExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Shell::new(
            Card::new(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(Title::new("Document canvas").h4())
                    .child(Text::new("System frame mode keeps platform decorations."))
                    .child(Button::new("Edit page").primary()),
            )
            .no_shadow(),
        )
        .id("docs-shell-content-first")
        .mode(WindowFrameMode::System)
        .header(
            Space::new()
                .gap_sm()
                .child(Text::new("Docs workspace").bold())
                .child(Text::new("System frame + app shell").sm()),
        )
        .header_height_units(58.0)
        .header_background(theme.primary.light_9)
        .header_border_color(theme.primary.light_7)
        .sidebar(
            Sidebar::new()
                .id("docs-shell-content-sidebar")
                .expanded_width_units(220.0)
                .brand("Docs")
                .brand_subtitle("Compact nav")
                .logo(Icon::new(IconName::BookOpen).size_units(18.0))
                .background(theme.primary.light_9)
                .border_color(theme.primary.light_7)
                .header_padding_units(12.0)
                .content_padding_units(6.0)
                .footer_padding_units(10.0)
                .gap_units(8.0)
                .scrollable()
                .child(self.menu.clone()),
        )
        .right_sidebar(
            Sidebar::new()
                .id("docs-shell-context-note")
                .right()
                .expanded_width_units(220.0)
                .brand("Context")
                .brand_subtitle("Page metadata")
                .logo(Icon::new(IconName::Info).size_units(18.0))
                .content(Text::new("Use this region for metadata or help.")),
        )
        .footer(Text::new("Autosaved just now").xs())
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
}
