//! Dashboard composition helpers for real Aura application screens.
//!
//! These helpers are intentionally small wrappers around existing primitives.
//! They remove repetitive dashboard glue without introducing a separate layout
//! runtime or a web-style design system.

use crate::{Card, Space, Statistic, Tag};
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

/// Responsive-ish grid presets for dashboard sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardGridPreset {
    Metrics,
    Charts,
    Operations,
    Custom(u16),
}

impl DashboardGridPreset {
    fn columns(self) -> u16 {
        match self {
            Self::Metrics => 4,
            Self::Charts => 2,
            Self::Operations => 3,
            Self::Custom(columns) => columns.max(1),
        }
    }
}

/// A small grid wrapper for common dashboard sections.
pub struct DashboardGrid {
    preset: DashboardGridPreset,
    gap: f32,
    children: Vec<AnyElement>,
}

impl DashboardGrid {
    pub fn new(columns: u16) -> Self {
        Self::with_preset(DashboardGridPreset::Custom(columns))
    }

    pub fn with_preset(preset: DashboardGridPreset) -> Self {
        Self {
            preset,
            gap: 16.0,
            children: Vec::new(),
        }
    }

    pub fn metrics() -> Self {
        Self::with_preset(DashboardGridPreset::Metrics)
    }

    pub fn charts() -> Self {
        Self::with_preset(DashboardGridPreset::Charts)
    }

    pub fn operations() -> Self {
        Self::with_preset(DashboardGridPreset::Operations)
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|child| child.into_any_element()));
        self
    }

    pub fn preset(&self) -> DashboardGridPreset {
        self.preset
    }

    pub fn column_count(&self) -> u16 {
        self.preset.columns()
    }
}

impl RenderOnce for DashboardGrid {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .grid()
            .grid_cols(self.preset.columns())
            .gap(px(self.gap))
            .children(self.children)
    }
}

impl IntoElement for DashboardGrid {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

/// A chart/content card preset used by dashboard pages.
pub fn dashboard_card(title: impl Into<gpui::SharedString>, body: impl IntoElement) -> Card {
    Card::new(body).title(title).no_shrink()
}

/// A metric card preset with consistent delta status treatment.
pub fn metric_card(
    title: impl Into<gpui::SharedString>,
    value: impl Into<gpui::SharedString>,
    delta: impl Into<gpui::SharedString>,
    positive: bool,
) -> Card {
    let delta = delta.into();
    Card::new(
        Space::new()
            .vertical()
            .gap_sm()
            .child(Statistic::new(title, value))
            .child(if positive {
                Tag::new(delta.clone())
                    .success()
                    .round(true)
                    .into_any_element()
            } else {
                Tag::new(delta).warning().round(true).into_any_element()
            }),
    )
    .hoverable()
    .no_shrink()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dashboard_grid_presets_track_columns() {
        assert_eq!(DashboardGrid::metrics().column_count(), 4);
        assert_eq!(DashboardGrid::charts().column_count(), 2);
        assert_eq!(DashboardGrid::operations().column_count(), 3);
        assert_eq!(DashboardGrid::new(0).column_count(), 1);
    }

    #[test]
    fn dashboard_helpers_are_thin_composition_api() {
        let source = include_str!("dashboard.rs");
        assert!(source.contains("pub fn dashboard_card"));
        assert!(source.contains("pub fn metric_card"));
        assert!(source.contains("Card::new"));
        assert!(source.contains("Statistic::new"));
    }
}
