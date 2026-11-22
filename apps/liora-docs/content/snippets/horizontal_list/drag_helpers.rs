//! Compose a native reorder surface without HorizontalList.
//! GPUI begins dragging after its movement threshold; offset starts at activation.
//! Left release on a target row commits only while your own drag is the live
//! native drag; right or outside release cancels via native preview cleanup.
//! Cleanup may wait for transient frame references to drain.
use gpui::{App, Context, Entity, MouseButton, Render, Window, div, prelude::*, px};
use liora_components::Text;
use liora_components::draggable::{DragAxis, DragState, drag_handle, reorder_indices};
use liora_core::Config;

pub fn drag_helpers(cx: &mut App) -> Entity<ReorderList> {
    cx.new(|_| ReorderList {
        items: vec!["Design", "Build", "Review"],
        drag: DragState::default(),
        session: None,
    })
}

pub struct ReorderList {
    items: Vec<&'static str>,
    drag: DragState,
    session: Option<gpui::EntityId>,
}

impl ReorderList {
    fn drop_at(&mut self, index: Option<usize>) -> bool {
        self.session = None;
        let Some(index) = index else {
            self.drag.cancel();
            return false;
        };
        self.drag.set_over(index);
        self.drag
            .finish()
            .is_some_and(|(from, to)| reorder_indices(&mut self.items, from, to))
    }
}
struct HelperDrag {
    owner: gpui::EntityId,
    index: usize,
    label: &'static str,
}

// GPUI owns this session-only preview even when the originating list stops painting.
struct HelperDragPreview(&'static str);

impl Render for HelperDragPreview {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        // Opaque popover surface so the floating preview does not show the
        // underlying row's label and highlight through its glyphs.
        div()
            .bg(theme.neutral.card)
            .border_1()
            .border_color(theme.neutral.border)
            .rounded_sm()
            .px_1()
            .child(Text::new(self.0))
    }
}

impl Render for ReorderList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let color = cx.global::<Config>().theme.neutral.text_2;
        let (_, dy) = self.drag.offset(DragAxis::Vertical);
        div()
            .flex()
            .flex_col()
            .child(Text::new(format!("Vertical drag: {dy:?}")))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .children(self.items.iter().copied().enumerate().map(|(index, item)| {
                        div()
                            .flex()
                            .items_center()
                            .h(px(36.0))
                            .on_drag_move::<HelperDrag>(cx.listener(
                                move |this, event: &gpui::DragMoveEvent<HelperDrag>, _, cx| {
                                    if event.drag(cx).owner == cx.entity_id()
                                        && event.bounds.contains(&event.event.position)
                                    {
                                        this.drag.update_position(event.event.position);
                                        this.drag.set_over(index);
                                        cx.notify();
                                    }
                                },
                            ))
                            // Commit only while our own gesture is the live native drag;
                            // stop propagation only on a real reorder so other clicks bubble.
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, _: &gpui::MouseUpEvent, _, cx| {
                                    if this.session.is_some()
                                        && cx.has_active_drag()
                                        && this.drag.active_index().is_some()
                                        && this.drop_at(Some(index))
                                    {
                                        cx.stop_propagation();
                                    }
                                    cx.notify();
                                }),
                            )
                            .child(
                                drag_handle(color, self.drag.is_active(index), px(32.0))
                                    .id(("drag-helper", index))
                                    .on_drag(
                                        HelperDrag {
                                            owner: cx.entity_id(),
                                            index,
                                            label: item,
                                        },
                                        {
                                            let owner = cx.weak_entity();
                                            move |payload, _, window, cx| {
                                                let preview =
                                                    cx.new(|_| HelperDragPreview(payload.label));
                                                let session = preview.entity_id();
                                                owner
                                                    .update(cx, |this, cx| {
                                                        this.session = Some(session);
                                                        this.drag.start(
                                                            payload.index,
                                                            window.mouse_position(),
                                                        );
                                                        cx.observe_release(
                                                            &preview,
                                                            move |this, _, cx| {
                                                                if this.session == Some(session) {
                                                                    this.drop_at(None);
                                                                    cx.notify();
                                                                }
                                                            },
                                                        )
                                                        .detach();
                                                        cx.notify();
                                                    })
                                                    .ok();
                                                preview
                                            }
                                        },
                                    ),
                            )
                            .child(Text::new(item))
                    })),
            )
    }
}
