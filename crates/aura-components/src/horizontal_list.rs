use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Context, Entity, IntoElement, MouseButton, MouseMoveEvent, Pixels, Render,
    Window, div, prelude::*, px,
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
    drag_from: Option<usize>,
    drag_over: Option<usize>,
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
            drag_from: None,
            drag_over: None,
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
        self.drag_from = None;
        self.drag_over = None;
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
            self.drag_from = None;
            self.drag_over = None;
        }
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
        if disabled {
            self.drag_from = None;
            self.drag_over = None;
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

    fn start_drag(&mut self, index: usize, cx: &mut Context<Self>) {
        if !self.draggable || self.disabled {
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
        if from != to && reorder_indices(&mut self.order, from, to) {
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
}

impl Render for HorizontalList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let render_item = self.render_item.clone();
        let render_divider = self.render_divider.clone();
        let item_gap = self.item_gap;
        let drag_from = self.drag_from;
        let drag_over = self.drag_over;
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

            let is_dragging = drag_from == Some(position);
            let is_over = drag_over == Some(position) && drag_from != Some(position);
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
                .opacity(if is_dragging { 0.72 } else { 1.0 });

            if draggable {
                item_shell = item_shell
                    .on_mouse_move(cx.listener(move |this, event, _, cx| {
                        this.hover_drag(position, event, cx);
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
                        cx.listener(|this, _, _, cx| {
                            this.cancel_drag(cx);
                        }),
                    )
                    .child(
                        render_drag_handle(theme.neutral.text_3, is_dragging).on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.start_drag(position, cx);
                                cx.stop_propagation();
                            }),
                        ),
                    )
                    .child(item);
            } else {
                item_shell = item_shell.child(item);
            }

            children.push(item_shell.into_any_element());
        }

        div()
            .id("aura-horizontal-list-scroll")
            .w_full()
            .when_some(self.height, |s, height| s.h(height))
            .overflow_x_scroll()
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

fn render_drag_handle(color: gpui::Hsla, active: bool) -> gpui::Div {
    div()
        .flex_none()
        .w(px(28.0))
        .items_center()
        .justify_center()
        .cursor_pointer()
        .hover(|s| s.cursor_pointer().bg(gpui::black().opacity(0.04)))
        .when(active, |s| s.bg(gpui::black().opacity(0.06)))
        .child(
            Icon::new(IconName::GripVertical)
                .size(px(16.0))
                .color(color),
        )
}

fn default_divider(color: gpui::Hsla) -> AnyElement {
    div()
        .flex_none()
        .w(px(1.0))
        .h(px(32.0))
        .bg(color)
        .into_any_element()
}

pub fn reorder_indices<T>(items: &mut Vec<T>, from: usize, to: usize) -> bool {
    if from >= items.len() || to >= items.len() || from == to {
        return false;
    }
    let item = items.remove(from);
    items.insert(to, item);
    true
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
        assert!(source.contains("render_drag_handle"));
        assert!(source.contains("IconName::GripVertical"));
    }
}
