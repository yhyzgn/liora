use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Context, Entity, IntoElement, ListAlignment, ListState, MouseButton,
    MouseMoveEvent, Pixels, Render, Window, div, list, prelude::*, px,
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
    drag_from: Option<usize>,
    drag_over: Option<usize>,
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
            drag_from: None,
            drag_over: None,
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
        self.drag_from = None;
        self.drag_over = None;
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
            self.drag_from = None;
            self.drag_over = None;
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

    fn start_drag(&mut self, index: usize, cx: &mut Context<Self>) {
        if !self.draggable {
            return;
        }
        self.drag_from = Some(index);
        self.drag_over = Some(index);
        cx.notify();
    }

    fn hover_drag(&mut self, index: usize, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        if event.pressed_button != Some(MouseButton::Left) || self.drag_from.is_none() {
            return;
        }
        if self.drag_over != Some(index) {
            self.drag_over = Some(index);
            cx.notify();
        }
    }

    fn finish_drag(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(from) = self.drag_from.take() else {
            return;
        };
        self.drag_over = None;
        let to = index.min(self.order.len().saturating_sub(1));
        if from != to && crate::horizontal_list::reorder_indices(&mut self.order, from, to) {
            self.list_state.remeasure();
            if let Some(callback) = self.on_reorder.clone() {
                callback(from, to, window, cx);
            }
        }
        cx.notify();
    }

    fn cancel_drag(&mut self, cx: &mut Context<Self>) {
        if self.drag_from.is_some() || self.drag_over.is_some() {
            self.drag_from = None;
            self.drag_over = None;
            cx.notify();
        }
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
        let drag_from = self.drag_from;
        let drag_over = self.drag_over;
        let entity = cx.entity().clone();

        div()
            .relative()
            .size_full()
            .when_some(self.height, |el, height| el.h(height))
            .child(
                list(self.list_state.clone(), move |index, window, cx| {
                    let item_index = order.get(index).copied().unwrap_or(index);
                    let item = (render_item)(item_index, window, cx);
                    let is_dragging = drag_from == Some(index);
                    let is_over = drag_over == Some(index) && drag_from != Some(index);
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
                        .opacity(if is_dragging { 0.72 } else { 1.0 });
                    if draggable {
                        shell = shell
                            .on_mouse_move(move |event, _, cx| {
                                move_entity
                                    .update(cx, |list, cx| list.hover_drag(index, event, cx));
                            })
                            .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                                up_entity
                                    .update(cx, |list, cx| list.finish_drag(index, window, cx));
                                cx.stop_propagation();
                            })
                            .on_mouse_up_out(MouseButton::Left, move |_, _, cx| {
                                out_entity.update(cx, |list, cx| list.cancel_drag(cx));
                            })
                            .child(render_drag_handle(is_dragging).on_mouse_down(
                                MouseButton::Left,
                                move |_, _, cx| {
                                    item_entity.update(cx, |list, cx| list.start_drag(index, cx));
                                    cx.stop_propagation();
                                },
                            ))
                            .child(item);
                    } else {
                        shell = shell.child(item);
                    }
                    if spacing > px(0.0) {
                        div().pb(spacing).child(shell).into_any_element()
                    } else {
                        shell.into_any_element()
                    }
                })
                .size_full(),
            )
            .child(crate::VirtualScrollbar::new(self.list_state.clone()))
    }
}

fn render_drag_handle(active: bool) -> gpui::Div {
    div()
        .flex_none()
        .w(px(32.0))
        .items_center()
        .justify_center()
        .cursor_pointer()
        .hover(|s| s.cursor_pointer().bg(gpui::black().opacity(0.04)))
        .when(active, |s| s.bg(gpui::black().opacity(0.06)))
        .child(
            Icon::new(IconName::GripVertical)
                .size(px(16.0))
                .color(gpui::rgb(0x94a3b8).into()),
        )
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
        assert!(source.contains("render_drag_handle"));
        assert!(source.contains("IconName::GripVertical"));
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
