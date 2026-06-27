//! Sheet module.
//!
//! `Sheet` is a lightweight edge panel API for short app-shell flows such as
//! filters, inspectors, quick creation, or contextual settings. It intentionally
//! reuses Liora's drawer overlay runtime instead of duplicating modal state,
//! while exposing sheet-oriented defaults and naming.

use crate::{Drawer, DrawerPlacement};
use gpui::{AnyElement, App, IntoElement, Pixels, SharedString, Window, div, prelude::*, px};
use std::sync::Arc;

/// Edge from which a [`Sheet`] enters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SheetPlacement {
    /// Right edge sheet, suitable for inspectors and settings.
    #[default]
    Right,
    /// Left edge sheet, suitable for navigation helpers.
    Left,
    /// Top edge sheet, suitable for command or notice flows.
    Top,
    /// Bottom edge sheet, suitable for mobile-like action flows.
    Bottom,
}

impl From<SheetPlacement> for DrawerPlacement {
    fn from(value: SheetPlacement) -> Self {
        match value {
            SheetPlacement::Right => DrawerPlacement::Right,
            SheetPlacement::Left => DrawerPlacement::Left,
            SheetPlacement::Top => DrawerPlacement::Top,
            SheetPlacement::Bottom => DrawerPlacement::Bottom,
        }
    }
}

/// Fluent native GPUI component for opening lightweight sheet panels.
pub struct Sheet {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window) -> AnyElement + 'static>,
    placement: SheetPlacement,
    width: Pixels,
    height: Pixels,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl Sheet {
    /// Creates a right-side sheet with compact app-shell defaults.
    pub fn new() -> Self {
        Self {
            id: liora_core::unique_id("sheet"),
            title: SharedString::default(),
            content: Arc::new(|_| div().child("Sheet content").into_any_element()),
            placement: SheetPlacement::Right,
            width: px(360.0),
            height: px(260.0),
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    /// Assigns a stable id used for open/close operations.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the title displayed in the sheet header.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the sheet placement.
    pub fn placement(mut self, placement: SheetPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// Places the sheet on the left edge.
    pub fn left(self) -> Self {
        self.placement(SheetPlacement::Left)
    }

    /// Places the sheet on the right edge.
    pub fn right(self) -> Self {
        self.placement(SheetPlacement::Right)
    }

    /// Places the sheet on the top edge.
    pub fn top(self) -> Self {
        self.placement(SheetPlacement::Top)
    }

    /// Places the sheet on the bottom edge.
    pub fn bottom(self) -> Self {
        self.placement(SheetPlacement::Bottom)
    }

    /// Sets side-sheet width.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into().max(px(240.0));
        self
    }

    /// Sets top/bottom-sheet height.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into().max(px(160.0));
        self
    }

    /// Applies a wide inspector preset.
    pub fn width_lg(self) -> Self {
        self.width(px(440.0))
    }

    /// Applies a compact bottom-sheet preset.
    pub fn height_sm(self) -> Self {
        self.height(px(220.0))
    }

    /// Toggles outside-click dismissal.
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Toggles escape-key dismissal.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Sets the rendered content element or text for this sheet.
    pub fn content<F, E>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window| f(window).into_any_element());
        self
    }

    /// Opens this sheet through the shared Liora drawer overlay runtime.
    pub fn show(self, cx: &mut App) {
        let content = self.content;
        Drawer::new()
            .id(self.id)
            .title(self.title)
            .placement(self.placement.into())
            .width(self.width)
            .height(self.height)
            .close_on_click_outside(self.close_on_click_outside)
            .close_on_escape(self.close_on_escape)
            .content(move |window, _cx| {
                // The sheet API intentionally hides DrawerView from callers.
                // Internally, both callbacks render native GPUI elements.
                content(window)
            })
            .show(cx);
    }

    /// Closes the active sheet/drawer overlay.
    pub fn close(cx: &mut App) {
        Drawer::close(cx);
    }

    /// Closes a sheet by id.
    pub fn close_id(id: impl Into<SharedString>, cx: &mut App) {
        Drawer::close_id(id, cx);
    }

    /// Returns placement and dimensions for tests and app state previews.
    pub fn layout_state(&self) -> (SheetPlacement, Pixels, Pixels) {
        (self.placement, self.width, self.height)
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sheet_defaults_are_lighter_than_drawer_defaults() {
        let (_, width, height) = Sheet::new().layout_state();
        assert_eq!(width, px(360.0));
        assert_eq!(height, px(260.0));
    }

    #[test]
    fn sheet_placement_maps_to_drawer_runtime() {
        assert_eq!(
            DrawerPlacement::from(SheetPlacement::Left),
            DrawerPlacement::Left
        );
        assert_eq!(
            DrawerPlacement::from(SheetPlacement::Right),
            DrawerPlacement::Right
        );
        assert_eq!(
            DrawerPlacement::from(SheetPlacement::Top),
            DrawerPlacement::Top
        );
        assert_eq!(
            DrawerPlacement::from(SheetPlacement::Bottom),
            DrawerPlacement::Bottom
        );
    }

    #[test]
    fn sheet_source_reuses_drawer_runtime_instead_of_parallel_overlay_state() {
        let source = include_str!("sheet.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        assert!(source.contains("Drawer::new()"));
        assert!(source.contains(".show(cx)"));
        assert!(!source.contains("set_active_drawer("));
    }
}
