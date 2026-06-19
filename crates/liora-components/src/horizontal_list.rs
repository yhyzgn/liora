//! Horizontal List module.
//!
//! This public module implements the Liora horizontally scrollable and optionally draggable list component. It keeps the reusable
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

use crate::draggable::{DragAxis, DragState, drag_handle, reorder_indices};
use gpui::{
    AnyElement, App, Bounds, Context, Entity, IntoElement, MouseButton, MouseDownEvent,
    MouseMoveEvent, Pixels, Point, Render, Size, Window, deferred, div, prelude::*, px,
};
use liora_core::Config;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

type RenderItem = dyn Fn(usize) -> AnyElement + 'static;
type RenderDivider = dyn Fn(usize) -> AnyElement + 'static;
type ReorderCallback = dyn Fn(usize, usize, &mut Window, &mut Context<HorizontalList>) + 'static;

/// Horizontal scroll list with custom item/divider rendering and native item reordering.
///
/// Items are rendered from an internal order vector so drag-to-reorder works even when
/// the caller supplies a purely functional item renderer. The reorder callback receives
/// display positions (`from_index`, `to_index`) after the internal order has changed.
pub struct HorizontalList {
    item_count: usize,
    order: Vec<usize>,
    render_item: Arc<RenderItem>,
    render_divider: Option<Arc<RenderDivider>>,
    on_reorder: Option<Arc<ReorderCallback>>,
    draggable: bool,
    disabled: bool,
    item_gap: Pixels,
    padding: Pixels,
    height: Option<Pixels>,
    drag_state: DragState,
    item_bounds: Rc<RefCell<Vec<Option<Bounds<Pixels>>>>>,
    drag_reference_bounds: Vec<Option<Bounds<Pixels>>>,
}

impl HorizontalList {
    /// Creates `HorizontalList` initialized from the supplied item count, and render item.
    pub fn new(item_count: usize, render_item: impl Fn(usize) -> AnyElement + 'static) -> Self {
        Self {
            item_count,
            order: (0..item_count).collect(),
            render_item: Arc::new(render_item),
            render_divider: None,
            on_reorder: None,
            draggable: false,
            disabled: false,
            item_gap: px(8.0),
            padding: px(4.0),
            height: None,
            drag_state: DragState::default(),
            item_bounds: Rc::new(RefCell::new(vec![None; item_count])),
            drag_reference_bounds: vec![None; item_count],
        }
    }

    /// Creates a GPUI entity that owns this component state across render passes.
    pub fn entity(
        item_count: usize,
        cx: &mut App,
        render_item: impl Fn(usize) -> AnyElement + 'static,
    ) -> Entity<Self> {
        cx.new(|_| Self::new(item_count, render_item))
    }

    /// Updates the stored item count value and keeps the existing component identity.
    pub fn set_item_count(&mut self, item_count: usize) {
        if self.item_count == item_count {
            return;
        }
        self.item_count = item_count;
        self.order = (0..item_count).collect();
        self.drag_state.cancel();
        *self.item_bounds.borrow_mut() = vec![None; item_count];
        self.drag_reference_bounds = vec![None; item_count];
    }

    /// Updates the stored render item value and keeps the existing component identity.
    pub fn set_render_item(&mut self, render_item: impl Fn(usize) -> AnyElement + 'static) {
        self.render_item = Arc::new(render_item);
    }

    /// Updates the stored divider value and keeps the existing component identity.
    pub fn set_divider(&mut self, divider: impl Fn(usize) -> AnyElement + 'static) {
        self.render_divider = Some(Arc::new(divider));
    }

    /// Clears the current divider state.
    pub fn clear_divider(&mut self) {
        self.render_divider = None;
    }

    /// Updates the stored draggable value and keeps the existing component identity.
    pub fn set_draggable(&mut self, draggable: bool) {
        self.draggable = draggable;
        if !draggable {
            self.drag_state.cancel();
            self.drag_reference_bounds.clear();
        }
    }

    /// Updates the stored disabled value and keeps the existing component identity.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        if disabled {
            self.drag_state.cancel();
            self.drag_reference_bounds.clear();
        }
    }

    /// Updates the stored on reorder value and keeps the existing component identity.
    pub fn set_on_reorder(
        &mut self,
        callback: impl Fn(usize, usize, &mut Window, &mut Context<HorizontalList>) + 'static,
    ) {
        self.on_reorder = Some(Arc::new(callback));
    }

    /// Updates the stored item gap value and keeps the existing component identity.
    pub fn set_item_gap(&mut self, gap: impl Into<Pixels>) {
        self.item_gap = gap.into().max(px(0.0));
    }

    /// Updates the stored padding value and keeps the existing component identity.
    pub fn set_padding(&mut self, padding: impl Into<Pixels>) {
        self.padding = padding.into().max(px(0.0));
    }

    /// Updates the stored height value and keeps the existing component identity.
    pub fn set_height(&mut self, height: Option<Pixels>) {
        self.height = height;
    }

    /// Performs the order operation used by this component.
    pub fn order(&self) -> &[usize] {
        &self.order
    }

    /// Sets the draggable value used by the component.
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.set_draggable(draggable);
        self
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.set_disabled(disabled);
        self
    }

    /// Sets the divider value used by the component.
    pub fn divider(mut self, divider: impl Fn(usize) -> AnyElement + 'static) -> Self {
        self.set_divider(divider);
        self
    }

    /// Registers a callback that runs when reorder occurs.
    pub fn on_reorder(
        mut self,
        callback: impl Fn(usize, usize, &mut Window, &mut Context<HorizontalList>) + 'static,
    ) -> Self {
        self.set_on_reorder(callback);
        self
    }

    /// Sets the item gap value used by the component.
    pub fn item_gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.set_item_gap(gap);
        self
    }

    /// Sets inner padding on all sides of the component.
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.set_padding(padding);
        self
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.set_height(Some(height.into()));
        self
    }

    fn start_drag(&mut self, index: usize, position: gpui::Point<Pixels>, cx: &mut Context<Self>) {
        if !self.draggable || self.disabled {
            return;
        }
        let bounds = self.item_bounds.borrow().get(index).copied().flatten();
        self.drag_reference_bounds = self.item_bounds.borrow().clone();
        self.drag_state.start_at(index, position, bounds);
        cx.notify();
    }

    fn update_drag_target_from_position(
        &mut self,
        position: Point<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let Some(active) = self.drag_state.active_index() else {
            return;
        };
        if self.drag_reference_bounds.is_empty() {
            return;
        }

        let mut target = active.min(self.drag_reference_bounds.len().saturating_sub(1));
        let mut nearest_distance = Pixels::MAX;
        for (index, item_bounds) in self.drag_reference_bounds.iter().enumerate() {
            let Some(item_bounds) = item_bounds else {
                continue;
            };
            if item_bounds.contains(&position) {
                target = index;
                break;
            }

            let distance = (position.x - item_bounds.center().x).abs();
            if distance < nearest_distance {
                nearest_distance = distance;
                target = index;
            }
        }

        if self.drag_state.over_index() != Some(target) {
            self.drag_state.set_over(target);
            cx.notify();
        }
    }

    fn hover_drag(
        &mut self,
        index: usize,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.drag_state.update_position(event.position);
        let Some(active) = self.drag_state.active_index() else {
            return;
        };
        if event.pressed_button != Some(MouseButton::Left) {
            return;
        }
        if index >= self.order.len() || index == active {
            return;
        }
        self.update_drag_target_from_position(event.position, cx);
    }

    fn update_drag_position(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.drag_state.active_index().is_none() {
            return;
        }
        if event.pressed_button != Some(MouseButton::Left) {
            self.finish_drag(0, window, cx);
            return;
        }
        self.drag_state.update_position(event.position);
        self.update_drag_target_from_position(event.position, cx);
        cx.notify();
    }

    fn finish_drag(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some((from, to)) = self.drag_state.finish() else {
            return;
        };
        self.drag_reference_bounds.clear();
        if from != to {
            reorder_indices(&mut self.order, from, to);
            if let Some(callback) = self.on_reorder.clone() {
                callback(from, to, window, cx);
            }
        }
        let _ = index;
        cx.notify();
    }
}

impl Render for HorizontalList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let render_item = self.render_item.clone();
        let render_divider = self.render_divider.clone();
        let item_gap = self.item_gap;
        let drag_state = self.drag_state.clone();
        let draggable = self.draggable && !self.disabled;
        let drag_active = drag_state.active_index().is_some();
        let active_item = drag_state
            .origin_index()
            .and_then(|index| self.order.get(index).copied());
        let mut display_order = self.order.clone();
        if let (Some(active), Some(over)) = (drag_state.active_index(), drag_state.over_index()) {
            reorder_indices(&mut display_order, active, over);
        }
        let drag_reference_bounds = self.drag_reference_bounds.clone();
        let item_bounds_store = self.item_bounds.clone();
        let active_size = drag_state
            .origin_index()
            .and_then(|index| drag_reference_bounds.get(index).copied().flatten())
            .map(|bounds| bounds.size);

        let mut children = Vec::new();
        let mut child_positions = Vec::new();
        for (position, item_index) in display_order.iter().copied().enumerate() {
            if position > 0 {
                let divider = render_divider
                    .as_ref()
                    .map(|render| render(position - 1))
                    .unwrap_or_else(|| default_divider(theme.neutral.border));
                children.push(divider);
                child_positions.push(None);
            }

            let is_dragging = active_item == Some(item_index);
            let is_over = drag_state.over_index() == Some(position) && !is_dragging;
            let item = (render_item)(item_index);
            let mut item_shell = div()
                .flex_none()
                .flex()
                .flex_row()
                .items_start()
                .rounded_md()
                .border_1()
                .border_color(if is_over {
                    theme.primary.base
                } else {
                    gpui::transparent_black()
                })
                .opacity(if is_dragging { 0.94 } else { 1.0 });

            if draggable && !is_dragging {
                item_shell = item_shell
                    .on_mouse_move(cx.listener(move |this, event, window, cx| {
                        this.hover_drag(position, event, window, cx);
                    }))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(move |this, _, window, cx| {
                            this.finish_drag(position, window, cx);
                            cx.stop_propagation();
                        }),
                    )
                    .on_mouse_up_out(
                        MouseButton::Left,
                        cx.listener(move |this, _, window, cx| {
                            this.finish_drag(position, window, cx);
                        }),
                    )
                    .child(
                        drag_handle(theme.neutral.text_3, false, px(28.0)).on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                                this.start_drag(position, event.position, cx);
                                cx.stop_propagation();
                            }),
                        ),
                    )
                    .child(item);
            } else if draggable {
                item_shell = item_shell
                    .child(drag_handle(theme.neutral.text_3, true, px(28.0)))
                    .child(item);
            } else {
                item_shell = item_shell.child(item);
            }

            let item_element = if is_dragging {
                let (drag_dx, drag_dy) = drag_state.offset_from_bounds(
                    DragAxis::Horizontal,
                    drag_reference_bounds.get(position).copied().flatten(),
                );
                let placeholder = drag_placeholder(active_size)
                    .border_color(theme.primary.base)
                    .child(
                        deferred(
                            item_shell
                                .absolute()
                                .left(drag_dx)
                                .top(drag_dy)
                                .shadow_lg()
                                .on_mouse_up(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, window, cx| {
                                        this.finish_drag(position, window, cx);
                                        cx.stop_propagation();
                                    }),
                                )
                                .on_mouse_up_out(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, window, cx| {
                                        this.finish_drag(position, window, cx);
                                    }),
                                ),
                        )
                        .with_priority(1000),
                    )
                    .into_any_element();
                placeholder
            } else {
                item_shell.into_any_element()
            };
            children.push(item_element);
            child_positions.push(Some(position));
        }

        div()
            .id("liora-horizontal-list-scroll")
            .w_full()
            .when_some(self.height, |s, height| s.h(height))
            .overflow_x_scroll()
            .when(draggable, |s| {
                s.on_mouse_move(cx.listener(|this, event, window, cx| {
                    this.update_drag_position(event, window, cx);
                }))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        this.finish_drag(0, window, cx);
                    }),
                )
                .on_mouse_up_out(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        this.finish_drag(0, window, cx);
                    }),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(item_gap)
                    .p(self.padding)
                    .children(children)
                    .on_children_prepainted(move |bounds, _, _| {
                        if drag_active {
                            return;
                        }
                        let mut item_bounds = vec![None; display_order.len()];
                        for (child_index, bounds) in bounds.into_iter().enumerate() {
                            let Some(Some(position)) = child_positions.get(child_index) else {
                                continue;
                            };
                            if *position < item_bounds.len() {
                                item_bounds[*position] = Some(bounds);
                            }
                        }
                        *item_bounds_store.borrow_mut() = item_bounds;
                    }),
            )
    }
}

fn default_divider(color: gpui::Hsla) -> AnyElement {
    div()
        .flex_none()
        .w(px(1.0))
        .h(px(32.0))
        .bg(color)
        .into_any_element()
}

fn drag_placeholder(size: Option<Size<Pixels>>) -> gpui::Div {
    div()
        .relative()
        .flex_none()
        .when_some(size, |s, size| s.w(size.width).h(size.height))
        .rounded_md()
        .border_1()
        .bg(gpui::black().opacity(0.018))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reorder_indices_moves_items() {
        let mut items = vec![0, 1, 2, 3];
        assert!(reorder_indices(&mut items, 0, 2));
        assert_eq!(items, vec![1, 2, 0, 3]);
        assert!(reorder_indices(&mut items, 3, 1));
        assert_eq!(items, vec![1, 3, 2, 0]);
    }

    #[test]
    fn reorder_indices_rejects_invalid_moves() {
        let mut items = vec![0, 1, 2];
        assert!(!reorder_indices(&mut items, 1, 1));
        assert!(!reorder_indices(&mut items, 4, 1));
        assert!(!reorder_indices(&mut items, 1, 4));
        assert_eq!(items, vec![0, 1, 2]);
    }

    #[test]
    fn horizontal_list_exposes_drag_and_custom_rendering_api() {
        let source = include_str!("horizontal_list.rs");
        assert!(source.contains("pub struct HorizontalList"));
        assert!(source.contains("set_divider"));
        assert!(source.contains("set_on_reorder"));
        assert!(source.contains("draggable"));
        assert!(source.contains("on_mouse_down"));
        assert!(source.contains("on_mouse_move"));
        assert!(source.contains("on_mouse_up"));
        assert!(source.contains("drag_handle"));
        assert!(source.contains("DragState"));
        assert!(source.contains("set_over"));
        assert!(source.contains("display_order"));
        assert!(source.contains("on_children_prepainted"));
    }
}
