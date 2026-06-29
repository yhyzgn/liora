//! Draggable module.
//!
//! This public module implements the Liora drag state and reorder helpers shared by draggable list surfaces. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
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
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use gpui::{AnyElement, Bounds, Div, Hsla, Pixels, Point, div, point, prelude::*, px};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Axis used by Liora's native drag helpers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragAxis {
    /// Lays out content in the horizontal direction.
    Horizontal,
    /// Lays out content in the vertical direction.
    Vertical,
    /// Uses the `Free` option for `DragAxis`.
    Free,
}

/// Runtime pointer state for handle-based dragging.
///
/// The helper intentionally stores only coordinates and item positions, never
/// rendered GPUI elements. This mirrors the safe part of drag-rs' model: a drag
/// operation has a start point, a current cursor point and a result callback,
/// while each UI renders its own preview/handle natively.
#[derive(Clone, Debug, Default)]
pub struct DragState {
    origin_index: Option<usize>,
    active_index: Option<usize>,
    over_index: Option<usize>,
    start_position: Option<Point<Pixels>>,
    current_position: Option<Point<Pixels>>,
    grab_offset: Option<Point<Pixels>>,
}

impl DragState {
    /// Performs the start operation used by this component.
    pub fn start(&mut self, index: usize, position: Point<Pixels>) {
        self.start_at(index, position, None);
    }

    /// Performs the start at operation used by this component.
    pub fn start_at(
        &mut self,
        index: usize,
        position: Point<Pixels>,
        bounds: Option<Bounds<Pixels>>,
    ) {
        self.origin_index = Some(index);
        self.active_index = Some(index);
        self.over_index = Some(index);
        self.start_position = Some(position);
        self.current_position = Some(position);
        self.grab_offset =
            bounds.map(|bounds| point(position.x - bounds.origin.x, position.y - bounds.origin.y));
    }

    /// Performs the update position operation used by this component.
    pub fn update_position(&mut self, position: Point<Pixels>) {
        if self.active_index.is_some() {
            self.current_position = Some(position);
        }
    }

    /// Updates the stored over value and keeps the existing component identity.
    pub fn set_over(&mut self, index: usize) {
        if self.active_index.is_some() {
            self.over_index = Some(index);
        }
    }

    /// Performs the move active to operation used by this component.
    pub fn move_active_to(&mut self, index: usize) {
        self.active_index = Some(index);
        self.over_index = Some(index);
        self.start_position = self.current_position;
    }

    /// Performs the finish operation used by this component.
    pub fn finish(&mut self) -> Option<(usize, usize)> {
        let origin = self.origin_index.take()?;
        let active = self.active_index.take()?;
        let target = self.over_index.take().unwrap_or(active);
        self.start_position = None;
        self.current_position = None;
        self.grab_offset = None;
        Some((origin, target))
    }

    /// Performs the cancel operation used by this component.
    pub fn cancel(&mut self) {
        self.origin_index = None;
        self.active_index = None;
        self.over_index = None;
        self.start_position = None;
        self.current_position = None;
        self.grab_offset = None;
    }

    /// Performs the active index operation used by this component.
    pub fn active_index(&self) -> Option<usize> {
        self.active_index
    }

    /// Performs the origin index operation used by this component.
    pub fn origin_index(&self) -> Option<usize> {
        self.origin_index
    }

    /// Performs the over index operation used by this component.
    pub fn over_index(&self) -> Option<usize> {
        self.over_index
    }

    /// Returns whether active is currently true for this value.
    pub fn is_active(&self, index: usize) -> bool {
        self.active_index == Some(index)
    }

    /// Returns whether over is currently true for this value.
    pub fn is_over(&self, index: usize) -> bool {
        self.over_index == Some(index) && self.active_index != Some(index)
    }

    /// Performs the offset operation used by this component.
    pub fn offset(&self, axis: DragAxis) -> (Pixels, Pixels) {
        let Some(start) = self.start_position else {
            return (px(0.0), px(0.0));
        };
        let Some(current) = self.current_position else {
            return (px(0.0), px(0.0));
        };
        let dx = current.x - start.x;
        let dy = current.y - start.y;
        match axis {
            DragAxis::Horizontal => (dx, px(0.0)),
            DragAxis::Vertical => (px(0.0), dy),
            DragAxis::Free => (dx, dy),
        }
    }

    /// Offset the active item from its current layout slot so the original
    /// grabbed point remains under the pointer.
    ///
    /// Reorderable lists may move the active item to a new slot while the
    /// pointer is still down. Using only `current - start` makes the item jump
    /// when that layout slot changes. When the caller can provide the active
    /// slot's latest bounds, this method compensates by anchoring the visual
    /// preview to the grab offset captured on mouse down.
    pub fn offset_from_bounds(
        &self,
        axis: DragAxis,
        bounds: Option<Bounds<Pixels>>,
    ) -> (Pixels, Pixels) {
        let Some(bounds) = bounds else {
            return self.offset(axis);
        };
        let Some(current) = self.current_position else {
            return (px(0.0), px(0.0));
        };
        let grab_offset = self.grab_offset.unwrap_or_else(|| point(px(0.0), px(0.0)));
        let dx = current.x - grab_offset.x - bounds.origin.x;
        let dy = current.y - grab_offset.y - bounds.origin.y;
        match axis {
            DragAxis::Horizontal => (dx, px(0.0)),
            DragAxis::Vertical => (px(0.0), dy),
            DragAxis::Free => (dx, dy),
        }
    }
}

/// Move an item inside a vector, returning whether a move happened.
pub fn reorder_indices<T>(items: &mut Vec<T>, from: usize, to: usize) -> bool {
    if from >= items.len() || to >= items.len() || from == to {
        return false;
    }
    let item = items.remove(from);
    items.insert(to, item);
    true
}

/// Default front-side drag handle used by reorderable components.
pub fn drag_handle(color: Hsla, active: bool, width: Pixels) -> Div {
    div()
        .flex()
        .flex_none()
        .w(width)
        .h_full()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .rounded_md()
        .when(!active, |s| {
            s.hover(move |s| s.cursor_pointer().bg(color.opacity(0.10)))
        })
        .when(active, |s| s.cursor_pointer())
        .child(
            Icon::new(IconName::GripVertical)
                .size(px(16.0))
                .color(color),
        )
}

/// Convenience wrapper for callers that need a boxed element handle.
pub fn drag_handle_element(color: Hsla, active: bool, width: Pixels) -> AnyElement {
    drag_handle(color, active, width).into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Bounds, point, px, size};

    #[test]
    fn reorder_indices_moves_items() {
        let mut items = vec![0, 1, 2, 3];
        assert!(reorder_indices(&mut items, 0, 2));
        assert_eq!(items, vec![1, 2, 0, 3]);
        assert!(reorder_indices(&mut items, 3, 1));
        assert_eq!(items, vec![1, 3, 2, 0]);
    }

    #[test]
    fn drag_state_tracks_axis_offsets() {
        let mut state = DragState::default();
        state.start(2, point(px(10.0), px(20.0)));
        state.update_position(point(px(42.0), px(12.0)));
        assert_eq!(state.offset(DragAxis::Horizontal), (px(32.0), px(0.0)));
        assert_eq!(state.offset(DragAxis::Vertical), (px(0.0), px(-8.0)));
        assert_eq!(state.offset(DragAxis::Free), (px(32.0), px(-8.0)));
    }

    #[test]
    fn drag_state_finishes_with_last_over_index() {
        let mut state = DragState::default();
        state.start(1, point(px(0.0), px(0.0)));
        state.update_position(point(px(20.0), px(0.0)));
        state.move_active_to(3);
        assert_eq!(state.finish(), Some((1, 3)));
        assert_eq!(state.origin_index(), None);
        assert_eq!(state.active_index(), None);
        assert_eq!(state.over_index(), None);
    }

    #[test]
    fn drag_state_keeps_grab_offset_when_slot_moves_backward() {
        let mut state = DragState::default();
        state.start_at(
            3,
            point(px(310.0), px(10.0)),
            Some(Bounds::new(
                point(px(300.0), px(0.0)),
                size(px(100.0), px(40.0)),
            )),
        );
        state.update_position(point(px(250.0), px(10.0)));
        state.move_active_to(2);

        assert_eq!(
            state.offset_from_bounds(
                DragAxis::Horizontal,
                Some(Bounds::new(
                    point(px(200.0), px(0.0)),
                    size(px(100.0), px(40.0)),
                )),
            ),
            (px(40.0), px(0.0))
        );
    }

    #[test]
    fn drag_handle_centers_icon_across_list_orientations() {
        let source = include_str!("draggable.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(!source.contains(".items_start()"));
        assert!(source.contains(".h_full()"));
        assert!(source.contains(".items_center()"));
        assert!(source.contains(".justify_center()"));
    }
}
