//! Top-level portal host for Drawer, Modal, Popover, and other overlays.

use gpui::{App, IntoElement, RenderOnce, ViewElement as Component, Window, div, prelude::*};
use liora_core::{PassivePortal, Portal};

/// Consumes Liora's global portal queues and paints them above application content.
/// Place after content at the window root; queues are consumed during prepaint so
/// virtual-list children can register overlays in the same frame.
pub struct PortalLayer;

impl IntoElement for PortalLayer {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for PortalLayer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        gpui::canvas(
            |bounds, window, cx| {
                // List items render in prepaint; draining during request_layout misses them.
                let mut root = portal_root(window, cx);
                root.prepaint_as_root(bounds.origin, bounds.size.into(), window, cx);
                root
            },
            |_, mut root, window, cx| root.paint(window, cx),
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full()
    }
}

fn portal_root(window: &mut Window, cx: &mut App) -> gpui::AnyElement {
    let passive = if cx.has_global::<PassivePortal>() {
        std::mem::take(&mut cx.global_mut::<PassivePortal>().entries)
    } else {
        Vec::new()
    };
    let active = if cx.has_global::<Portal>() {
        std::mem::take(&mut cx.global_mut::<Portal>().entries)
    } else {
        Vec::new()
    };
    let viewport = window.viewport_size();
    let mut root = div().relative().w(viewport.width).h(viewport.height);
    if !passive.is_empty() {
        let mut layer = div()
            .id("liora-passive-portal-layer")
            .absolute()
            .top_0()
            .left_0()
            .size_full();
        for entry in passive {
            layer = layer.child((entry.render)(window, cx));
        }
        root = root.child(layer);
    }
    if !active.is_empty() {
        let mut layer = div()
            .id("liora-portal-layer")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .cursor_default()
            .occlude()
            .bg(gpui::transparent_black())
            .on_hover(|_, _, cx| cx.stop_propagation())
            .on_mouse_move(|_, _, cx| cx.stop_propagation());
        for entry in active {
            layer = layer.child((entry.render)(window, cx));
        }
        root = root.child(layer);
    }
    root.into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, Context, Entity, Render, px};
    use std::{cell::Cell, rc::Rc};

    struct VirtualSelectSurface(Entity<crate::VirtualizedList>);

    impl Render for VirtualSelectSurface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .relative()
                .size_full()
                .child(self.0.clone())
                .child(PortalLayer)
        }
    }

    fn click(window: &mut Window, cx: &mut App, position: gpui::Point<gpui::Pixels>) {
        window.dispatch_event(
            gpui::PlatformInput::MouseMove(gpui::MouseMoveEvent {
                position,
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_event(
            gpui::PlatformInput::MouseDown(gpui::MouseDownEvent {
                position,
                button: gpui::MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_event(
            gpui::PlatformInput::MouseUp(gpui::MouseUpEvent {
                position,
                button: gpui::MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
    }

    #[test]
    fn virtualized_select_selects_and_dismisses_without_queued_ghosts() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let select =
            app.new(|cx| crate::Select::new(vec!["Apple", "Banana", "Orange"], Some(0), cx));
        let trigger = Rc::new(Cell::new(None));
        let list = app.new(|cx| {
            let select = select.clone();
            let trigger = trigger.clone();
            crate::VirtualizedList::new(1, cx, move |_, _, _| {
                let trigger = trigger.clone();
                div()
                    .w(px(200.0))
                    .child(select.clone())
                    .on_children_prepainted(move |bounds, _, _| {
                        trigger.set(bounds.into_iter().next())
                    })
                    .into_any_element()
            })
        });
        let window = app.open_window(gpui::size(px(400.0), px(300.0)), |_, _| {
            VirtualSelectSurface(list)
        });
        app.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            let bounds = trigger.get().expect("virtualized Select must be laid out");
            click(window, cx, bounds.center());
            window.draw(cx).clear(cx);
            assert!(
                cx.global::<Portal>().entries.is_empty(),
                "same-frame popup queue must be drained"
            );
            let second_option =
                gpui::point(bounds.left() + px(20.0), bounds.bottom() + px(4.0 + 50.0));
            click(window, cx, second_option);
            assert_eq!(
                select.read(cx).selected_index(),
                Some(1),
                "portal option must receive native dispatch"
            );
            for _ in 0..3 {
                window.refresh();
                window.draw(cx).clear(cx);
                assert!(
                    cx.global::<Portal>().entries.is_empty(),
                    "closed popup must not leave stale entries"
                );
            }
            click(window, cx, bounds.center());
            window.draw(cx).clear(cx);
            click(
                window,
                cx,
                gpui::point(bounds.left() + px(20.0), bounds.bottom() + px(4.0 + 85.0)),
            );
            assert_eq!(
                select.read(cx).selected_index(),
                Some(2),
                "reopen after redraw must use fresh options"
            );
            window.draw(cx).clear(cx);
            assert!(cx.global::<Portal>().entries.is_empty());
        })
        .unwrap();
        app.quit();
    }
}
