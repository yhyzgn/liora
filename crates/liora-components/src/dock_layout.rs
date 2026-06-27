//! Dock Layout module.
//!
//! This public module implements the Liora dock-layout component for workbench-style native applications. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use crate::{Card, Space, Text};
use gpui::{
    AnyElement, App, Component, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels,
    RenderOnce, SharedString, Styled, Window, div, prelude::*, px,
};
use liora_core::{Config, unique_id};

/// Edge used by dock panels around the center content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DockEdge {
    /// Left side of the workbench.
    Left,
    /// Right side of the workbench.
    Right,
    /// Top side of the workbench.
    Top,
    /// Bottom side of the workbench.
    Bottom,
    /// Center tab group or editor surface.
    Center,
}

/// Dock panel metadata and content.
pub struct DockPanel {
    /// Stable key used to identify the panel in layout state.
    pub key: SharedString,
    /// User-facing panel title.
    pub title: SharedString,
    /// Preferred edge for the panel.
    pub edge: DockEdge,
    /// Preferred size for side or bottom/top panels.
    pub size: Option<Pixels>,
    /// Whether a header strip is rendered above the panel content.
    pub header: bool,
    /// Panel content rendered as native GPUI elements.
    pub content: AnyElement,
}

impl DockPanel {
    /// Creates a dock panel from key, title, edge, and content.
    pub fn new(
        key: impl Into<SharedString>,
        title: impl Into<SharedString>,
        edge: DockEdge,
        content: impl IntoElement,
    ) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            edge,
            size: None,
            header: true,
            content: content.into_any_element(),
        }
    }

    /// Sets a preferred panel size.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into().max(px(48.0)));
        self
    }

    /// Hides or shows the panel header strip.
    pub fn header(mut self, header: bool) -> Self {
        self.header = header;
        self
    }
}

/// Tab-like center document pane rendered by [`DockLayout`].
pub struct DockTab {
    /// Stable key used to identify the tab.
    pub key: SharedString,
    /// User-facing tab label.
    pub title: SharedString,
    /// Tab content rendered as native GPUI elements.
    pub content: AnyElement,
    /// Whether the tab can be closed by host state.
    pub closable: bool,
}

impl DockTab {
    /// Creates a center dock tab from key, title, and content.
    pub fn new(
        key: impl Into<SharedString>,
        title: impl Into<SharedString>,
        content: impl IntoElement,
    ) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            content: content.into_any_element(),
            closable: true,
        }
    }

    /// Toggles whether host UI should expose close behavior for this tab.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

/// Workbench-style dock layout with side panels and center document tabs.
pub struct DockLayout {
    id: SharedString,
    panels: Vec<DockPanel>,
    tabs: Vec<DockTab>,
    active_tab: Option<SharedString>,
    height: Option<Pixels>,
    bordered: bool,
    panel_gap: Pixels,
}

impl DockLayout {
    /// Creates an empty dock layout.
    pub fn new() -> Self {
        Self {
            id: unique_id("dock-layout"),
            panels: Vec::new(),
            tabs: Vec::new(),
            active_tab: None,
            height: None,
            bordered: true,
            panel_gap: px(0.0),
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Adds a panel to one of the dock edges.
    pub fn panel(mut self, panel: DockPanel) -> Self {
        self.panels.push(panel);
        self
    }

    /// Adds a center document tab.
    pub fn tab(mut self, tab: DockTab) -> Self {
        if self.active_tab.is_none() {
            self.active_tab = Some(tab.key.clone());
        }
        self.tabs.push(tab);
        self
    }

    /// Sets the active center tab key.
    pub fn active_tab(mut self, key: impl Into<SharedString>) -> Self {
        self.active_tab = Some(key.into());
        self
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Applies a desktop workbench default height.
    pub fn height_lg(self) -> Self {
        self.height(px(520.0))
    }

    /// Toggles the outer border around the dock workbench.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Sets spacing between panel regions.
    pub fn panel_gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.panel_gap = gap.into().max(px(0.0));
        self
    }

    /// Returns the number of panels assigned to an edge.
    pub fn panel_count(&self, edge: DockEdge) -> usize {
        self.panels
            .iter()
            .filter(|panel| panel.edge == edge)
            .count()
    }

    /// Returns the currently active tab key.
    pub fn active_tab_key(&self) -> Option<&SharedString> {
        self.active_tab.as_ref()
    }
}

impl IntoElement for DockLayout {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for DockLayout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let active_tab_key = self.active_tab.clone();
        let (top, remaining) = partition_panels(self.panels, DockEdge::Top);
        let (bottom, remaining) = partition_panels(remaining, DockEdge::Bottom);
        let (left, remaining) = partition_panels(remaining, DockEdge::Left);
        let (right, center_panels) = partition_panels(remaining, DockEdge::Right);

        let center = render_center(self.tabs, active_tab_key, center_panels, &theme);
        let middle = div()
            .flex()
            .flex_row()
            .flex_1()
            .min_h(px(0.0))
            .gap(self.panel_gap)
            .children(left.into_iter().map(|panel| render_panel(panel, &theme)))
            .child(center)
            .children(right.into_iter().map(|panel| render_panel(panel, &theme)));

        div()
            .id(ElementId::from(self.id.clone()))
            .flex()
            .flex_col()
            .w_full()
            .overflow_hidden()
            .when_some(self.height, |style, height| style.h(height))
            .when(self.bordered, |style| {
                style
                    .rounded_lg()
                    .border_1()
                    .border_color(theme.neutral.border)
            })
            .bg(theme.neutral.card)
            .gap(self.panel_gap)
            .children(top.into_iter().map(|panel| render_panel(panel, &theme)))
            .child(middle)
            .children(bottom.into_iter().map(|panel| render_panel(panel, &theme)))
    }
}

fn partition_panels(panels: Vec<DockPanel>, edge: DockEdge) -> (Vec<DockPanel>, Vec<DockPanel>) {
    panels.into_iter().partition(|panel| panel.edge == edge)
}

fn render_panel(panel: DockPanel, theme: &liora_theme::Theme) -> AnyElement {
    let vertical = matches!(
        panel.edge,
        DockEdge::Left | DockEdge::Right | DockEdge::Center
    );
    let header = panel.header.then(|| {
        div()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(theme.neutral.border)
            .text_xs()
            .font_weight(gpui::FontWeight::BOLD)
            .text_color(theme.neutral.text_2)
            .child(panel.title.clone())
    });
    div()
        .flex()
        .flex_col()
        .min_w(px(0.0))
        .min_h(px(0.0))
        .when(vertical, |style| style.h_full())
        .when(!vertical, |style| style.w_full())
        .when_some(panel.size, |style, size| {
            if vertical {
                style.w(size).flex_shrink_0()
            } else {
                style.h(size).flex_shrink_0()
            }
        })
        .border_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card)
        .children(header)
        .child(div().flex_1().min_h(px(0.0)).p_3().child(panel.content))
        .into_any_element()
}

fn render_center(
    tabs: Vec<DockTab>,
    active_tab_key: Option<SharedString>,
    center_panels: Vec<DockPanel>,
    theme: &liora_theme::Theme,
) -> AnyElement {
    if tabs.is_empty() {
        return div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .child(
                Space::new()
                    .vertical()
                    .gap_md()
                    .child(Text::new("No dock tabs"))
                    .child(Text::new(format!("{} center panels", center_panels.len())).sm()),
            )
            .into_any_element();
    }

    let active_index = tabs
        .iter()
        .position(|tab| Some(&tab.key) == active_tab_key.as_ref())
        .unwrap_or(0);
    let mut active_content = None;
    let mut headers = div()
        .flex()
        .items_center()
        .border_b_1()
        .border_color(theme.neutral.border);
    for (index, tab) in tabs.into_iter().enumerate() {
        let active = index == active_index;
        headers = headers.child(
            div()
                .px_3()
                .py_2()
                .text_sm()
                .font_weight(if active {
                    gpui::FontWeight::BOLD
                } else {
                    gpui::FontWeight::NORMAL
                })
                .text_color(if active {
                    theme.primary.base
                } else {
                    theme.neutral.text_2
                })
                .bg(if active {
                    theme.primary.light_9
                } else {
                    theme.neutral.card
                })
                .child(tab.title.clone()),
        );
        if active {
            active_content = Some(tab.content);
        }
    }

    Card::new(
        div().flex().flex_col().size_full().child(headers).child(
            div()
                .flex_1()
                .min_h(px(0.0))
                .p_3()
                .child(active_content.unwrap_or_else(|| div().into_any_element())),
        ),
    )
    .no_shadow()
    .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dock_layout_tracks_panels_tabs_and_active_tab() {
        let layout = DockLayout::new()
            .panel(DockPanel::new("explorer", "Explorer", DockEdge::Left, div()).size(px(220.0)))
            .panel(DockPanel::new("terminal", "Terminal", DockEdge::Bottom, div()).size(px(160.0)))
            .tab(DockTab::new("main", "main.rs", div()).closable(false))
            .tab(DockTab::new("readme", "README.md", div()))
            .active_tab("readme")
            .height_lg()
            .panel_gap(px(6.0));

        assert_eq!(layout.panel_count(DockEdge::Left), 1);
        assert_eq!(layout.panel_count(DockEdge::Bottom), 1);
        assert_eq!(
            layout.active_tab_key().map(|key| key.as_ref()),
            Some("readme")
        );
        assert_eq!(layout.height, Some(px(520.0)));
    }

    #[test]
    fn dock_panel_clamps_tiny_sizes() {
        let panel = DockPanel::new("outline", "Outline", DockEdge::Right, div()).size(px(1.0));
        assert_eq!(panel.size, Some(px(48.0)));
    }
}
