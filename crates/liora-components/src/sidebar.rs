//! Sidebar module.
//!
//! This public module implements a reusable native GPUI sidebar shell for Liora
//! applications. The sidebar owns panel layout, width, collapse mode, header,
//! footer, and scroll behavior; navigation data and selection remain delegated to
//! components such as [`crate::Menu`].

use gpui::{
    AnyElement, App, Component, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::{Config, stable_unique_id};

/// Side of the app shell where a [`Sidebar`] is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarPosition {
    /// Renders the sidebar on the left side of the content area.
    #[default]
    Left,
    /// Renders the sidebar on the right side of the content area.
    Right,
}

/// Width/content policy used by a [`Sidebar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SidebarCollapseMode {
    /// Uses the expanded width and renders all child content.
    #[default]
    Full,
    /// Uses the collapsed width while still rendering children for icon-only menus.
    IconsOnly,
    /// Hides the sidebar completely.
    Hidden,
}

/// Fluent native GPUI component for rendering app sidebars.
pub struct Sidebar {
    id: SharedString,
    position: SidebarPosition,
    collapse_mode: SidebarCollapseMode,
    expanded_width: Pixels,
    collapsed_width: Pixels,
    min_width: Pixels,
    max_width: Pixels,
    resizable: bool,
    scrollable: bool,
    header: Option<AnyElement>,
    content: Vec<AnyElement>,
    footer: Option<AnyElement>,
}

impl Sidebar {
    /// Creates a left, expanded, scrollable-ready sidebar with Liora defaults.
    pub fn new() -> Self {
        Self {
            id: "liora-sidebar".into(),
            position: SidebarPosition::Left,
            collapse_mode: SidebarCollapseMode::Full,
            expanded_width: px(280.0),
            collapsed_width: px(64.0),
            min_width: px(180.0),
            max_width: px(420.0),
            resizable: false,
            scrollable: false,
            header: None,
            content: Vec::new(),
            footer: None,
        }
    }

    /// Assigns a stable element id used by GPUI state and automation tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Renders the sidebar on the left side.
    pub fn left(mut self) -> Self {
        self.position = SidebarPosition::Left;
        self
    }

    /// Renders the sidebar on the right side.
    pub fn right(mut self) -> Self {
        self.position = SidebarPosition::Right;
        self
    }

    /// Sets the sidebar position explicitly.
    pub fn position(mut self, position: SidebarPosition) -> Self {
        self.position = position;
        self
    }

    /// Sets the collapse mode explicitly.
    pub fn collapse_mode(mut self, mode: SidebarCollapseMode) -> Self {
        self.collapse_mode = mode;
        self
    }

    /// Sets the expanded width.
    pub fn expanded_width(mut self, width: impl Into<Pixels>) -> Self {
        self.expanded_width = width.into();
        self
    }

    /// Sets the collapsed width.
    pub fn collapsed_width(mut self, width: impl Into<Pixels>) -> Self {
        self.collapsed_width = width.into();
        self
    }

    /// Sets the minimum width reserved for future resizable sidebars.
    pub fn min_width(mut self, width: impl Into<Pixels>) -> Self {
        self.min_width = width.into();
        self
    }

    /// Sets the maximum width reserved for future resizable sidebars.
    pub fn max_width(mut self, width: impl Into<Pixels>) -> Self {
        self.max_width = width.into();
        self
    }

    /// Marks the sidebar as resizable for API-forward compatibility.
    pub fn resizable(mut self) -> Self {
        self.resizable = true;
        self
    }

    /// Enables vertical scrolling for the content region.
    pub fn scrollable(mut self) -> Self {
        self.scrollable = true;
        self
    }

    /// Sets the fixed header slot.
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// Adds a child to the scrollable content slot.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.content.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the scrollable content slot.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.content
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }

    /// Sets the fixed footer slot.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    fn current_width(&self) -> Pixels {
        match self.collapse_mode {
            SidebarCollapseMode::Full => self.expanded_width,
            SidebarCollapseMode::IconsOnly => self.collapsed_width,
            SidebarCollapseMode::Hidden => px(0.0),
        }
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Sidebar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let width = self.current_width();
        let hidden = matches!(self.collapse_mode, SidebarCollapseMode::Hidden);
        let right = matches!(self.position, SidebarPosition::Right);
        let content_id = stable_unique_id(self.id.clone(), "sidebar-content", window, cx);

        div()
            .id(self.id)
            .flex_none()
            .h_full()
            .min_h_0()
            .w(width)
            .when(hidden, |s| s.hidden())
            .when(!hidden, |s| {
                s.bg(theme.neutral.card)
                    .when(!right, |s| {
                        s.border_r_1().border_color(theme.neutral.border)
                    })
                    .when(right, |s| s.border_l_1().border_color(theme.neutral.border))
            })
            .flex()
            .flex_col()
            .child(
                div()
                    .flex_none()
                    .when_some(self.header, |s, header| s.child(header)),
            )
            .child(
                div()
                    .id(content_id)
                    .flex_1()
                    .min_h_0()
                    .when(self.scrollable, |s| s.overflow_y_scroll())
                    .children(self.content),
            )
            .child(
                div()
                    .flex_none()
                    .when_some(self.footer, |s, footer| s.child(footer)),
            )
    }
}

impl IntoElement for Sidebar {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sidebar_builder_tracks_layout_state() {
        let sidebar = Sidebar::new()
            .id("nav")
            .right()
            .collapse_mode(SidebarCollapseMode::IconsOnly)
            .expanded_width(px(300.0))
            .collapsed_width(px(72.0))
            .min_width(px(200.0))
            .max_width(px(480.0))
            .resizable()
            .scrollable()
            .header("header")
            .child("content")
            .footer("footer");

        assert_eq!(sidebar.id.as_ref(), "nav");
        assert_eq!(sidebar.position, SidebarPosition::Right);
        assert_eq!(sidebar.collapse_mode, SidebarCollapseMode::IconsOnly);
        assert_eq!(sidebar.current_width(), px(72.0));
        assert_eq!(sidebar.expanded_width, px(300.0));
        assert_eq!(sidebar.min_width, px(200.0));
        assert_eq!(sidebar.max_width, px(480.0));
        assert!(sidebar.resizable);
        assert!(sidebar.scrollable);
        assert!(sidebar.header.is_some());
        assert_eq!(sidebar.content.len(), 1);
        assert!(sidebar.footer.is_some());
    }

    #[test]
    fn sidebar_source_owns_scroll_and_border_layout() {
        let production = include_str!("sidebar.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(production.contains("stable_unique_id"));
        assert!(production.contains("overflow_y_scroll"));
        assert!(production.contains("border_r_1"));
        assert!(production.contains("border_l_1"));
        assert!(production.contains("theme.neutral.card"));
        assert!(production.contains("theme.neutral.border"));
        assert!(production.contains("SidebarCollapseMode::Hidden"));
    }
}
