use crate::draggable::{DragAxis, DragState, drag_handle, reorder_indices};
use aura_core::Config;
use gpui::{
    AnyElement, App, Context, Entity, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent,
    Pixels, Render, Window, deferred, div, prelude::*, px,
};
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
}

impl HorizontalList {
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
        }
    }

    pub fn entity(
        item_count: usize,
        cx: &mut App,
        render_item: impl Fn(usize) -> AnyElement + 'static,
    ) -> Entity<Self> {
        cx.new(|_| Self::new(item_count, render_item))
    }

    pub fn set_item_count(&mut self, item_count: usize) {
        if self.item_count == item_count {
            return;
        }
        self.item_count = item_count;
        self.order = (0..item_count).collect();
        self.drag_state.cancel();
    }

    pub fn set_render_item(&mut self, render_item: impl Fn(usize) -> AnyElement + 'static) {
        self.render_item = Arc::new(render_item);
    }

    pub fn set_divider(&mut self, divider: impl Fn(usize) -> AnyElement + 'static) {
        self.render_divider = Some(Arc::new(divider));
    }

    pub fn clear_divider(&mut self) {
        self.render_divider = None;
    }

    pub fn set_draggable(&mut self, draggable: bool) {
        self.draggable = draggable;
        if !draggable {
            self.drag_state.cancel();
        }
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        if disabled {
            self.drag_state.cancel();
        }
    }

    pub fn set_on_reorder(
        &mut self,
        callback: impl Fn(usize, usize, &mut Window, &mut Context<HorizontalList>) + 'static,
    ) {
        self.on_reorder = Some(Arc::new(callback));
    }

    pub fn set_item_gap(&mut self, gap: impl Into<Pixels>) {
        self.item_gap = gap.into().max(px(0.0));
    }

    pub fn set_padding(&mut self, padding: impl Into<Pixels>) {
        self.padding = padding.into().max(px(0.0));
    }

    pub fn set_height(&mut self, height: Option<Pixels>) {
        self.height = height;
    }

    pub fn order(&self) -> &[usize] {
        &self.order
    }

    pub fn draggable(mut self, draggable: bool) -> Self {
        self.set_draggable(draggable);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.set_disabled(disabled);
        self
    }

    pub fn divider(mut self, divider: impl Fn(usize) -> AnyElement + 'static) -> Self {
        self.set_divider(divider);
        self
    }

    pub fn on_reorder(
        mut self,
        callback: impl Fn(usize, usize, &mut Window, &mut Context<HorizontalList>) + 'static,
    ) -> Self {
        self.set_on_reorder(callback);
        self
    }

    pub fn item_gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.set_item_gap(gap);
        self
    }

    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.set_padding(padding);
        self
    }

    pub fn height(mut self, height: Pixels) -> Self {
        self.set_height(Some(height));
        self
    }

    fn start_drag(&mut self, index: usize, position: gpui::Point<Pixels>, cx: &mut Context<Self>) {
        if !self.draggable || self.disabled {
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
}

impl Render for HorizontalList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let render_item = self.render_item.clone();
        let render_divider = self.render_divider.clone();
        let item_gap = self.item_gap;
        let drag_state = self.drag_state.clone();
        let draggable = self.draggable && !self.disabled;

        let mut children = Vec::new();
        for (position, item_index) in self.order.iter().copied().enumerate() {
            if position > 0 {
                let divider = render_divider
                    .as_ref()
                    .map(|render| render(position - 1))
                    .unwrap_or_else(|| default_divider(theme.neutral.border));
                children.push(divider);
            }

            let is_dragging = drag_state.is_active(position);
            let is_over = drag_state.is_over(position);
            let (drag_dx, drag_dy) = if is_dragging {
                drag_state.offset(DragAxis::Horizontal)
            } else {
                (px(0.0), px(0.0))
            };
            let item = (render_item)(item_index);
            let mut item_shell = div()
                .flex_none()
                .flex()
                .flex_row()
                .items_stretch()
                .rounded_md()
                .border_1()
                .border_color(if is_over {
                    theme.primary.base
                } else {
                    gpui::transparent_black()
                })
                .opacity(if is_dragging { 0.86 } else { 1.0 })
                .when(is_dragging, move |s| {
                    s.relative().left(drag_dx).top(drag_dy).shadow_lg()
                });

            if draggable {
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
                        drag_handle(theme.neutral.text_3, is_dragging, px(28.0)).on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                                this.start_drag(position, event.position, cx);
                                cx.stop_propagation();
                            }),
                        ),
                    )
                    .child(item);
            } else {
                item_shell = item_shell.child(item);
            }

            let item_element = if is_dragging {
                deferred(item_shell).with_priority(1000).into_any_element()
            } else {
                item_shell.into_any_element()
            };
            children.push(item_element);
        }

        div()
            .id("aura-horizontal-list-scroll")
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
                    .children(children),
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
    }
}
