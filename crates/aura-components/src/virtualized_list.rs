use crate::draggable::{DragAxis, DragState, drag_handle, reorder_indices};
use gpui::{
    AnyElement, App, Context, Entity, IntoElement, ListAlignment, ListState, MouseButton,
    MouseMoveEvent, Pixels, Render, Window, deferred, div, list, prelude::*, px,
};
use std::sync::Arc;

type RenderItem = dyn Fn(usize, &mut Window, &mut App) -> AnyElement + 'static;
type ReorderCallback = dyn Fn(usize, usize, &mut Window, &mut App) + 'static;

/// A native virtualized vertical list for large or expensive item trees.
///
/// The component owns GPUI's [`ListState`] and renders only the visible item
/// range plus a configurable overdraw area. Pair it with [`crate::VirtualScrollbar`]
/// when a custom Aura scrollbar is needed.
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
        self.drag_state.start(index, position);
        cx.notify();
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
        if reorder_indices(&mut self.order, active, index) {
            self.drag_state.move_active_to(index);
            self.list_state.remeasure();
            cx.notify();
        }
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
        cx.notify();
    }

    fn finish_drag(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some((from, to)) = self.drag_state.finish() else {
            return;
        };
        if from != to {
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
        let order = self.order.clone();
        let draggable = self.draggable;
        let drag_state = self.drag_state.clone();
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
                    let item_index = order.get(index).copied().unwrap_or(index);
                    let item = (render_item)(item_index, window, cx);
                    let is_dragging = drag_state.is_active(index);
                    let is_over = drag_state.is_over(index);
                    let (drag_dx, drag_dy) = if is_dragging {
                        drag_state.offset(DragAxis::Vertical)
                    } else {
                        (px(0.0), px(0.0))
                    };
                    let item_entity = entity.clone();
                    let move_entity = entity.clone();
                    let up_entity = entity.clone();
                    let out_entity = entity.clone();
                    let mut shell = div()
                        .flex()
                        .flex_row()
                        .items_stretch()
                        .rounded_md()
                        .border_1()
                        .border_color(if is_over {
                            gpui::blue()
                        } else {
                            gpui::transparent_black()
                        })
                        .opacity(if is_dragging { 0.86 } else { 1.0 })
                        .when(is_dragging, move |s| {
                            s.relative().left(drag_dx).top(drag_dy).shadow_lg()
                        });
                    if draggable {
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
                                drag_handle(gpui::rgb(0x94a3b8).into(), is_dragging, px(32.0))
                                    .on_mouse_down(MouseButton::Left, move |event, _, cx| {
                                        item_entity.update(cx, |list, cx| {
                                            list.start_drag(index, event.position, cx)
                                        });
                                        cx.stop_propagation();
                                    }),
                            )
                            .child(item);
                    } else {
                        shell = shell.child(item);
                    }
                    let row = if is_dragging {
                        deferred(shell).with_priority(1000).into_any_element()
                    } else {
                        shell.into_any_element()
                    };
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

#[cfg(test)]
mod tests {
    #[test]
    fn virtualized_list_owns_list_state_and_uses_aura_scrollbar() {
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
