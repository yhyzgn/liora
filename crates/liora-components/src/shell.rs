//! Shell module.
//!
//! This public module implements the high-level Liora application shell. It
//! composes native GPUI frame chrome, titlebar, sidebar, header, main, footer,
//! right-sidebar, and overlay slots behind one fluent SDK component so Gallery,
//! Docs, and downstream applications do not need to assemble raw GPUI layout
//! trees for common app-window structures.

use crate::{AppWindowFrame, TitleBar, WindowFrameMode};
use gpui::{
    AnyElement, App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::{Config, stable_unique_id};

/// Built-in placement policy for Shell overlay content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ShellOverlayPosition {
    /// Pins overlays near the top-left corner.
    TopLeft,
    /// Pins overlays near the top-right corner.
    #[default]
    TopRight,
    /// Pins overlays near the bottom-left corner.
    BottomLeft,
    /// Pins overlays near the bottom-right corner.
    BottomRight,
    /// Centers overlays over the shell surface.
    Center,
}

/// Fluent native GPUI component for rendering a complete app shell.
pub struct Shell {
    id: SharedString,
    mode: WindowFrameMode,
    titlebar: Option<TitleBar>,
    header: Option<AnyElement>,
    sidebar: Option<AnyElement>,
    right_sidebar: Option<AnyElement>,
    main: Vec<AnyElement>,
    footer: Option<AnyElement>,
    overlays: Vec<AnyElement>,
    overlay_position: ShellOverlayPosition,
    overlay_inset: Pixels,
    header_height: Option<Pixels>,
    footer_height: Option<Pixels>,
    main_scroll: bool,
    main_padding: Option<Pixels>,
    background: Option<Hsla>,
    body_background: Option<Hsla>,
    main_background: Option<Hsla>,
    header_background: Option<Hsla>,
    footer_background: Option<Hsla>,
    header_border_color: Option<Hsla>,
    footer_border_color: Option<Hsla>,
    main_radius: Option<Pixels>,
    body_gap: Option<Pixels>,
}

impl Shell {
    /// Creates a shell with initial main content.
    pub fn new(content: impl IntoElement) -> Self {
        Self {
            id: "liora-shell".into(),
            mode: WindowFrameMode::System,
            titlebar: None,
            header: None,
            sidebar: None,
            right_sidebar: None,
            main: vec![content.into_any_element()],
            footer: None,
            overlays: Vec::new(),
            overlay_position: ShellOverlayPosition::default(),
            overlay_inset: px(16.0),
            header_height: None,
            footer_height: None,
            main_scroll: false,
            main_padding: None,
            background: None,
            body_background: None,
            main_background: None,
            header_background: None,
            footer_background: None,
            header_border_color: None,
            footer_border_color: None,
            main_radius: None,
            body_gap: None,
        }
    }

    /// Assigns a stable element id for the shell root.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Selects system or custom native window-frame rendering.
    pub fn mode(mut self, mode: WindowFrameMode) -> Self {
        self.mode = mode;
        self
    }

    /// Replaces the titlebar used when custom frame mode is active.
    pub fn titlebar(mut self, titlebar: TitleBar) -> Self {
        self.titlebar = Some(titlebar);
        self
    }

    /// Sets the fixed top header slot below the optional custom titlebar.
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// Sets the left shell sidebar slot.
    pub fn sidebar(mut self, sidebar: impl IntoElement) -> Self {
        self.sidebar = Some(sidebar.into_any_element());
        self
    }

    /// Sets the right shell sidebar / inspector slot.
    pub fn right_sidebar(mut self, sidebar: impl IntoElement) -> Self {
        self.right_sidebar = Some(sidebar.into_any_element());
        self
    }

    /// Adds main content to the central shell region.
    pub fn main(mut self, content: impl IntoElement) -> Self {
        self.main.push(content.into_any_element());
        self
    }

    /// Alias for adding main content to the central shell region.
    pub fn child(self, content: impl IntoElement) -> Self {
        self.main(content)
    }

    /// Sets the fixed footer slot.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// Adds an overlay element above the shell layout.
    ///
    /// Overlays are positioned by Shell itself so application demos and docs do
    /// not need raw GPUI absolute-positioning primitives for common badges,
    /// command palettes, floating actions, or transient hints.
    pub fn overlay(mut self, overlay: impl IntoElement) -> Self {
        self.overlays.push(overlay.into_any_element());
        self
    }

    /// Sets where overlay content is pinned inside the shell root.
    pub fn overlay_position(mut self, position: ShellOverlayPosition) -> Self {
        self.overlay_position = position;
        self
    }

    /// Sets the inset used by corner-pinned overlay placements.
    pub fn overlay_inset(mut self, inset: impl Into<Pixels>) -> Self {
        self.overlay_inset = inset.into();
        self
    }

    /// Sets the overlay inset from application-facing design units.
    pub fn overlay_inset_units(self, inset: f32) -> Self {
        self.overlay_inset(px(inset))
    }

    /// Sets header height.
    pub fn header_height(mut self, height: impl Into<Pixels>) -> Self {
        self.header_height = Some(height.into());
        self
    }

    /// Sets header height from application-facing design units.
    pub fn header_height_units(self, height: f32) -> Self {
        self.header_height(px(height))
    }

    /// Sets footer height.
    pub fn footer_height(mut self, height: impl Into<Pixels>) -> Self {
        self.footer_height = Some(height.into());
        self
    }

    /// Sets footer height from application-facing design units.
    pub fn footer_height_units(self, height: f32) -> Self {
        self.footer_height(px(height))
    }

    /// Enables vertical scrolling for the central main region.
    pub fn main_scroll(mut self) -> Self {
        self.main_scroll = true;
        self
    }

    /// Sets main-region padding.
    pub fn main_padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.main_padding = Some(padding.into());
        self
    }

    /// Sets main-region padding from application-facing design units.
    pub fn main_padding_units(self, padding: f32) -> Self {
        self.main_padding(px(padding))
    }

    /// Overrides the shell root background color.
    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    /// Overrides the shell body background behind the main and sidebar regions.
    pub fn body_background(mut self, color: Hsla) -> Self {
        self.body_background = Some(color);
        self
    }

    /// Overrides the main-region background color.
    pub fn main_background(mut self, color: Hsla) -> Self {
        self.main_background = Some(color);
        self
    }

    /// Overrides the fixed header background color.
    pub fn header_background(mut self, color: Hsla) -> Self {
        self.header_background = Some(color);
        self
    }

    /// Overrides the fixed footer background color.
    pub fn footer_background(mut self, color: Hsla) -> Self {
        self.footer_background = Some(color);
        self
    }

    /// Overrides the fixed header bottom-border color.
    pub fn header_border_color(mut self, color: Hsla) -> Self {
        self.header_border_color = Some(color);
        self
    }

    /// Overrides the fixed footer top-border color.
    pub fn footer_border_color(mut self, color: Hsla) -> Self {
        self.footer_border_color = Some(color);
        self
    }

    /// Sets a corner radius on the main-region surface.
    pub fn main_rounded(mut self, radius: impl Into<Pixels>) -> Self {
        self.main_radius = Some(radius.into());
        self
    }

    /// Sets a corner radius on the main-region surface from design units.
    pub fn main_rounded_units(self, radius: f32) -> Self {
        self.main_rounded(px(radius))
    }

    /// Sets spacing between body children: left sidebar, main, and right sidebar.
    pub fn body_gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.body_gap = Some(gap.into());
        self
    }

    /// Sets spacing between body children from application-facing design units.
    pub fn body_gap_units(self, gap: f32) -> Self {
        self.body_gap(px(gap))
    }
}

impl RenderOnce for Shell {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let background = self.background.unwrap_or(theme.neutral.body);
        let body_background = self.body_background.unwrap_or(background);
        let main_background = self.main_background.unwrap_or(body_background);
        let header_background = self.header_background.unwrap_or(background);
        let footer_background = self.footer_background.unwrap_or(background);
        let header_border = self.header_border_color.unwrap_or(theme.neutral.border);
        let footer_border = self.footer_border_color.unwrap_or(theme.neutral.border);
        let header_height = self.header_height.unwrap_or(px(48.0));
        let footer_height = self.footer_height.unwrap_or(px(48.0));
        let main_id = stable_unique_id(self.id.clone(), "shell-main", window, cx);

        let mut root = div()
            .id(self.id.clone())
            .size_full()
            .relative()
            .flex()
            .flex_col()
            .bg(background);

        if let Some(header) = self.header {
            root = root.child(
                div()
                    .flex_none()
                    .h(header_height)
                    .w_full()
                    .border_b_1()
                    .border_color(header_border)
                    .bg(header_background)
                    .flex()
                    .items_center()
                    .child(header),
            );
        }

        let mut body = div()
            .flex_1()
            .min_h_0()
            .flex()
            .flex_row()
            .bg(body_background)
            .when_some(self.body_gap, |s, gap| s.gap(gap));
        if let Some(sidebar) = self.sidebar {
            body = body.child(sidebar);
        }

        body = body.child(
            div()
                .flex_1()
                .min_w_0()
                .min_h_0()
                .h_full()
                .id(main_id)
                .flex()
                .flex_col()
                .bg(main_background)
                .when_some(self.main_radius, |s, radius| s.rounded(radius))
                .when(self.main_scroll, |s| s.overflow_y_scroll())
                .when_some(self.main_padding, |s, padding| s.p(padding))
                .children(self.main),
        );

        if let Some(right_sidebar) = self.right_sidebar {
            body = body.child(right_sidebar);
        }

        root = root.child(body);

        if let Some(footer) = self.footer {
            root = root.child(
                div()
                    .flex_none()
                    .h(footer_height)
                    .w_full()
                    .border_t_1()
                    .border_color(footer_border)
                    .bg(footer_background)
                    .flex()
                    .items_center()
                    .child(footer),
            );
        }

        if !self.overlays.is_empty() {
            root = root.child(positioned_overlays(
                self.overlays,
                self.overlay_position,
                self.overlay_inset,
            ));
        }

        let content = root.into_any_element();

        if self.mode.is_custom() || self.titlebar.is_some() {
            AppWindowFrame::new("Liora", content)
                .mode(self.mode)
                .titlebar(self.titlebar.unwrap_or_else(TitleBar::new))
                .into_any_element()
        } else {
            content
        }
    }
}

fn positioned_overlays(
    overlays: Vec<AnyElement>,
    position: ShellOverlayPosition,
    inset: Pixels,
) -> AnyElement {
    div()
        .absolute()
        .when(
            matches!(
                position,
                ShellOverlayPosition::TopLeft | ShellOverlayPosition::TopRight
            ),
            |s| s.top(inset),
        )
        .when(
            matches!(
                position,
                ShellOverlayPosition::BottomLeft | ShellOverlayPosition::BottomRight
            ),
            |s| s.bottom(inset),
        )
        .when(
            matches!(
                position,
                ShellOverlayPosition::TopLeft | ShellOverlayPosition::BottomLeft
            ),
            |s| s.left(inset),
        )
        .when(
            matches!(
                position,
                ShellOverlayPosition::TopRight | ShellOverlayPosition::BottomRight
            ),
            |s| s.right(inset),
        )
        .when(matches!(position, ShellOverlayPosition::Center), |s| {
            s.top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
        })
        .children(overlays)
        .into_any_element()
}

impl IntoElement for Shell {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sidebar;

    #[test]
    fn shell_builder_tracks_all_custom_regions() {
        let shell = Shell::new("body")
            .id("app-shell")
            .mode(WindowFrameMode::Custom)
            .titlebar(TitleBar::new().title("Liora"))
            .header("header")
            .sidebar(Sidebar::new())
            .right_sidebar(Sidebar::new().right())
            .main("main")
            .footer("footer")
            .overlay("overlay")
            .overlay_position(ShellOverlayPosition::BottomRight)
            .overlay_inset_units(20.0)
            .header_height_units(56.0)
            .footer_height_units(40.0)
            .main_scroll()
            .main_padding_units(24.0)
            .background(gpui::transparent_black())
            .body_background(gpui::transparent_black())
            .main_background(gpui::transparent_black())
            .header_background(gpui::transparent_black())
            .footer_background(gpui::transparent_black())
            .header_border_color(gpui::transparent_black())
            .footer_border_color(gpui::transparent_black())
            .main_rounded_units(18.0)
            .body_gap_units(10.0);

        assert_eq!(shell.id.as_ref(), "app-shell");
        assert_eq!(shell.mode, WindowFrameMode::Custom);
        assert!(shell.titlebar.is_some());
        assert!(shell.header.is_some());
        assert!(shell.sidebar.is_some());
        assert!(shell.right_sidebar.is_some());
        assert_eq!(shell.main.len(), 2);
        assert!(shell.footer.is_some());
        assert_eq!(shell.overlays.len(), 1);
        assert_eq!(shell.overlay_position, ShellOverlayPosition::BottomRight);
        assert_eq!(shell.overlay_inset, px(20.0));
        assert_eq!(shell.header_height, Some(px(56.0)));
        assert_eq!(shell.footer_height, Some(px(40.0)));
        assert!(shell.main_scroll);
        assert_eq!(shell.main_padding, Some(px(24.0)));
        assert!(shell.background.is_some());
        assert!(shell.body_background.is_some());
        assert!(shell.main_background.is_some());
        assert!(shell.header_background.is_some());
        assert!(shell.footer_background.is_some());
        assert!(shell.header_border_color.is_some());
        assert!(shell.footer_border_color.is_some());
        assert_eq!(shell.main_radius, Some(px(18.0)));
        assert_eq!(shell.body_gap, Some(px(10.0)));
    }

    #[test]
    fn shell_source_owns_shell_layout_primitives() {
        let production = include_str!("shell.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(production.contains("AppWindowFrame::new"));
        assert!(production.contains(".titlebar("));
        assert!(production.contains(".flex_row()"));
        assert!(production.contains("stable_unique_id"));
        assert!(production.contains("shell-main"));
        assert!(production.contains(".overflow_y_scroll()"));
        assert!(production.contains("right_sidebar"));
        assert!(production.contains("body_background"));
        assert!(production.contains("main_background"));
        assert!(production.contains("header_border_color"));
        assert!(production.contains("footer_border_color"));
        assert!(production.contains("main_radius"));
        assert!(production.contains("ShellOverlayPosition"));
        assert!(production.contains("positioned_overlays"));
        assert!(production.contains(".absolute()"));
    }
}
