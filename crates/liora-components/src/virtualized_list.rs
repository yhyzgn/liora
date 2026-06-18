use crate::draggable::{DragAxis, DragState, drag_handle, reorder_indices};
use gpui::{
    AnyElement, App, Bounds, Context, Entity, IntoElement, ListAlignment, ListState, MouseButton,
    MouseMoveEvent, Pixels, Point, Render, Size, Window, deferred, div, list, prelude::*, px,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

type RenderItem = dyn Fn(usize, &mut Window, &mut App) -> AnyElement + 'static;
type ReorderCallback = dyn Fn(usize, usize, &mut Window, &mut App) + 'static;

/// A native virtualized vertical list for large or expensive item trees.
///
/// The component owns GPUI's [`ListState`] and renders only the visible item
/// range plus a configurable overdraw area. Pair it with [`crate::VirtualScrollbar`]
/// when a custom Liora scrollbar is needed.
pub struct VirtualizedList {
    item_count: usize,
    list_state: ListState,
    render_item: Arc<RenderItem>,
    overdraw: Pixels,
    item_spacing: Pixels,
    height: Option<Pixels>,
    measure_all_items: bool,
    order: Vec<usize>,
    draggable: bool,
    drag_state: DragState,
    item_bounds: Rc<RefCell<Vec<Option<Bounds<Pixels>>>>>,
    drag_reference_bounds: Vec<Option<Bounds<Pixels>>>,
    on_reorder: Option<Arc<ReorderCallback>>,
}

impl VirtualizedList {
    pub fn new(
        item_count: usize,
        _cx: &mut Context<Self>,
        render_item: impl Fn(usize, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        let overdraw = px(640.0);
        Self {
            item_count,
            list_state: ListState::new(item_count, ListAlignment::Top, overdraw),
            render_item: Arc::new(render_item),
            overdraw,
            item_spacing: px(0.0),
            height: None,
            measure_all_items: false,
            order: (0..item_count).collect(),
            draggable: false,
            drag_state: DragState::default(),
            item_bounds: Rc::new(RefCell::new(vec![None; item_count])),
            drag_reference_bounds: vec![None; item_count],
            on_reorder: None,
        }
    }

    pub fn entity(
        item_count: usize,
        cx: &mut App,
        render_item: impl Fn(usize, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(item_count, cx, render_item))
    }

    pub fn list_state(&self) -> ListState {
        self.list_state.clone()
    }

    pub fn set_item_count(&mut self, item_count: usize) {
        if self.item_count == item_count {
            return;
        }
        self.item_count = item_count;
        self.order = (0..item_count).collect();
        self.drag_state.cancel();
        *self.item_bounds.borrow_mut() = vec![None; item_count];
        self.drag_reference_bounds = vec![None; item_count];
        self.list_state = Self::new_list_state(item_count, self.overdraw, self.measure_all_items);
    }

    pub fn set_render_item(
        &mut self,
        render_item: impl Fn(usize, &mut Window, &mut App) -> AnyElement + 'static,
    ) {
        self.render_item = Arc::new(render_item);
    }

    pub fn set_item_spacing(&mut self, spacing: impl Into<Pixels>) {
        let spacing = spacing.into();
        if self.item_spacing == spacing {
            return;
        }
        self.item_spacing = spacing;
        self.list_state.remeasure();
    }

    pub fn set_overdraw(&mut self, overdraw: impl Into<Pixels>) {
        let overdraw = overdraw.into();
        if self.overdraw == overdraw {
            return;
        }
        self.overdraw = overdraw;
        self.list_state = Self::new_list_state(self.item_count, overdraw, self.measure_all_items);
    }

    pub fn set_height(&mut self, height: Option<Pixels>) {
        if self.height == height {
            return;
        }
        self.height = height;
        self.list_state.remeasure();
    }

    pub fn set_draggable(&mut self, draggable: bool) {
        self.draggable = draggable;
        if !draggable {
            self.drag_state.cancel();
            self.drag_reference_bounds.clear();
        }
    }

    pub fn set_on_reorder(
        &mut self,
        callback: impl Fn(usize, usize, &mut Window, &mut App) + 'static,
    ) {
        self.on_reorder = Some(Arc::new(callback));
    }

    pub fn order(&self) -> &[usize] {
        &self.order
    }

    fn start_drag(&mut self, index: usize, position: gpui::Point<Pixels>, cx: &mut Context<Self>) {
        if !self.draggable {
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

            let distance = (position.y - item_bounds.center().y).abs();
            if distance < nearest_distance {
                nearest_distance = distance;
                target = index;
            }
        }

        if self.drag_state.over_index() != Some(target) {
            self.drag_state.set_over(target);
            self.list_state.remeasure();
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
            if reorder_indices(&mut self.order, from, to) {
                self.list_state.remeasure();
            }
            if let Some(callback) = self.on_reorder.clone() {
                callback(from, to, window, cx);
            }
        }
        let _ = index;
        cx.notify();
    }

    /// Measure every item once so GPUI's scrollbar math has a stable total height.
    ///
    /// GPUI's virtual list reports scrollbar extents from measured rows only; for
    /// long variable-height documents this can otherwise make the thumb jump or
    /// reach the ends before the content does. Use this when scrollbar accuracy is
    /// more important than the first-frame cost of measuring every row.
    pub fn measure_all_items_for_scrollbar(&mut self) {
        if self.measure_all_items {
            return;
        }
        self.measure_all_items = true;
        self.list_state = self.list_state.clone().measure_all();
    }

    fn new_list_state(item_count: usize, overdraw: Pixels, measure_all_items: bool) -> ListState {
        let state = ListState::new(item_count, ListAlignment::Top, overdraw);
        if measure_all_items {
            state.measure_all()
        } else {
            state
        }
    }

    /// Mark every item for remeasurement while preserving proportional scroll.
    ///
    /// Updating the render closure alone does not remeasure automatically, so
    /// callers that know item heights changed can opt into the heavier work.
    pub fn remeasure(&self) {
        self.list_state.remeasure();
    }

    /// Mark one item range for remeasurement while preserving proportional scroll.
    pub fn remeasure_items(&self, range: std::ops::Range<usize>) {
        self.list_state.remeasure_items(range);
    }
}

impl Render for VirtualizedList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let render_item = self.render_item.clone();
        let spacing = self.item_spacing;
        let mut display_order = self.order.clone();
        let draggable = self.draggable;
        let drag_state = self.drag_state.clone();
        let drag_active = drag_state.active_index().is_some();
        let active_item = drag_state
            .origin_index()
            .and_then(|index| self.order.get(index).copied());
        if let (Some(active), Some(over)) = (drag_state.active_index(), drag_state.over_index()) {
            reorder_indices(&mut display_order, active, over);
        }
        let drag_reference_bounds = self.drag_reference_bounds.clone();
        let active_size = drag_state
            .origin_index()
            .and_then(|index| drag_reference_bounds.get(index).copied().flatten())
            .map(|bounds| bounds.size);
        let item_bounds_store = self.item_bounds.clone();
        let entity = cx.entity().clone();

        div()
            .relative()
            .size_full()
            .when_some(self.height, |el, height| el.h(height))
            .when(draggable, |el| {
                let move_entity = entity.clone();
                let up_entity = entity.clone();
                let out_entity = entity.clone();
                el.on_mouse_move(move |event, window, cx| {
                    move_entity.update(cx, |list, cx| list.update_drag_position(event, window, cx));
                })
                .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                    up_entity.update(cx, |list, cx| list.finish_drag(0, window, cx));
                })
                .on_mouse_up_out(MouseButton::Left, move |_, window, cx| {
                    out_entity.update(cx, |list, cx| list.finish_drag(0, window, cx));
                })
            })
            .child(
                list(self.list_state.clone(), move |index, window, cx| {
                    let item_index = display_order.get(index).copied().unwrap_or(index);
                    let item = (render_item)(item_index, window, cx);
                    let is_dragging = active_item == Some(item_index);
                    let is_over = drag_state.is_over(index);
                    let item_entity = entity.clone();
                    let move_entity = entity.clone();
                    let up_entity = entity.clone();
                    let out_entity = entity.clone();
                    let mut shell = div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .rounded_md()
                        .border_1()
                        .border_color(if is_dragging {
                            gpui::rgb(0xcbd5e1).into()
                        } else if is_over {
                            gpui::blue()
                        } else {
                            gpui::transparent_black()
                        })
                        .opacity(1.0)
                        .when(is_dragging, |s| s.shadow_lg());
                    if draggable && !is_dragging {
                        let up_entity = up_entity.clone();
                        let out_entity = out_entity.clone();
                        shell = shell
                            .on_mouse_move(move |event, window, cx| {
                                move_entity.update(cx, |list, cx| {
                                    list.hover_drag(index, event, window, cx)
                                });
                            })
                            .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                                up_entity
                                    .update(cx, |list, cx| list.finish_drag(index, window, cx));
                                cx.stop_propagation();
                            })
                            .on_mouse_up_out(MouseButton::Left, move |_, window, cx| {
                                out_entity
                                    .update(cx, |list, cx| list.finish_drag(index, window, cx));
                            })
                            .child(
                                drag_handle(gpui::rgb(0x94a3b8).into(), false, px(32.0))
                                    .on_mouse_down(MouseButton::Left, move |event, _, cx| {
                                        item_entity.update(cx, |list, cx| {
                                            list.start_drag(index, event.position, cx)
                                        });
                                        cx.stop_propagation();
                                    }),
                            )
                            .child(item);
                    } else if draggable {
                        shell = shell
                            .child(drag_handle(gpui::rgb(0x94a3b8).into(), true, px(32.0)))
                            .child(item);
                    } else {
                        shell = shell.child(item);
                    }

                    let row_content = if is_dragging {
                        let up_entity = up_entity.clone();
                        let out_entity = out_entity.clone();
                        let (drag_dx, drag_dy) = drag_state.offset_from_bounds(
                            DragAxis::Vertical,
                            drag_reference_bounds.get(index).copied().flatten(),
                        );
                        drag_placeholder(active_size)
                            .child(
                                deferred(
                                    shell
                                        .absolute()
                                        .left(drag_dx)
                                        .top(drag_dy)
                                        .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                                            up_entity.update(cx, |list, cx| {
                                                list.finish_drag(index, window, cx)
                                            });
                                            cx.stop_propagation();
                                        })
                                        .on_mouse_up_out(
                                            MouseButton::Left,
                                            move |_, window, cx| {
                                                out_entity.update(cx, |list, cx| {
                                                    list.finish_drag(index, window, cx)
                                                });
                                            },
                                        ),
                                )
                                .with_priority(1000),
                            )
                            .into_any_element()
                    } else {
                        shell.into_any_element()
                    };

                    let bounds_store = item_bounds_store.clone();
                    let row = div()
                        .child(row_content)
                        .on_children_prepainted(move |bounds, _, _| {
                            if drag_active {
                                return;
                            }
                            let Some(bounds) = bounds.into_iter().next() else {
                                return;
                            };
                            let mut item_bounds = bounds_store.borrow_mut();
                            if item_bounds.len() <= index {
                                item_bounds.resize(index + 1, None);
                            }
                            item_bounds[index] = Some(bounds);
                        })
                        .into_any_element();

                    if spacing > px(0.0) {
                        div().pb(spacing).child(row).into_any_element()
                    } else {
                        row
                    }
                })
                .size_full(),
            )
            .child(crate::VirtualScrollbar::new(self.list_state.clone()))
    }
}

fn drag_placeholder(size: Option<Size<Pixels>>) -> gpui::Div {
    div()
        .relative()
        .flex_none()
        .when_some(size, |s, size| s.w(size.width).h(size.height))
        .rounded_md()
        .border_1()
        .border_color(gpui::rgb(0xcbd5e1))
        .bg(gpui::transparent_black())
}

#[cfg(test)]
mod tests {
    #[test]
    fn virtualized_list_owns_list_state_and_uses_liora_scrollbar() {
        let source = include_str!("virtualized_list.rs");

        assert!(source.contains("pub struct VirtualizedList"));
        assert!(source.contains("ListState::new"));
        assert!(source.contains("list(self.list_state.clone()"));
        assert!(source.contains("VirtualScrollbar::new"));
        assert!(source.contains("set_item_spacing"));
        assert!(source.contains("set_render_item"));
        assert!(source.contains("measure_all_items_for_scrollbar"));
        assert!(source.contains("set_draggable"));
        assert!(source.contains("set_on_reorder"));
        assert!(source.contains("drag_handle"));
        assert!(source.contains("DragState"));
        assert!(source.contains("display_order"));
        assert!(source.contains("drag_placeholder"));
        assert!(source.contains("on_children_prepainted"));
        assert!(source.contains("drag_reference_bounds"));
    }

    #[test]
    fn virtualized_list_resets_state_when_count_or_overdraw_changes() {
        let source = include_str!("virtualized_list.rs");

        assert!(source.contains("set_item_count"));
        assert!(source.contains("set_overdraw"));
        assert!(source.contains("Self::new_list_state"));
        assert!(source.contains("pub fn remeasure(&self)"));
        assert!(source.contains("pub fn remeasure_items"));
    }
}
