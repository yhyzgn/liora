use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyElement, Div, Hsla, Pixels, Point, div, prelude::*, px};

/// Axis used by Aura's native drag helpers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragAxis {
    Horizontal,
    Vertical,
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
}

impl DragState {
    pub fn start(&mut self, index: usize, position: Point<Pixels>) {
        self.origin_index = Some(index);
        self.active_index = Some(index);
        self.over_index = Some(index);
        self.start_position = Some(position);
        self.current_position = Some(position);
    }

    pub fn update_position(&mut self, position: Point<Pixels>) {
        if self.active_index.is_some() {
            self.current_position = Some(position);
        }
    }

    pub fn set_over(&mut self, index: usize) {
        if self.active_index.is_some() {
            self.over_index = Some(index);
        }
    }

    pub fn move_active_to(&mut self, index: usize) {
        self.active_index = Some(index);
        self.over_index = Some(index);
        self.start_position = self.current_position;
    }

    pub fn finish(&mut self) -> Option<(usize, usize)> {
        let origin = self.origin_index.take()?;
        let active = self.active_index.take()?;
        let target = self.over_index.take().unwrap_or(active);
        self.start_position = None;
        self.current_position = None;
        Some((origin, target))
    }

    pub fn cancel(&mut self) {
        self.origin_index = None;
        self.active_index = None;
        self.over_index = None;
        self.start_position = None;
        self.current_position = None;
    }

    pub fn active_index(&self) -> Option<usize> {
        self.active_index
    }

    pub fn origin_index(&self) -> Option<usize> {
        self.origin_index
    }

    pub fn over_index(&self) -> Option<usize> {
        self.over_index
    }

    pub fn is_active(&self, index: usize) -> bool {
        self.active_index == Some(index)
    }

    pub fn is_over(&self, index: usize) -> bool {
        self.over_index == Some(index) && self.active_index != Some(index)
    }

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
        .hover(|s| s.cursor_pointer().bg(gpui::black().opacity(0.04)))
        .when(active, |s| s.bg(gpui::black().opacity(0.06)))
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
    use gpui::{point, px};

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
}
