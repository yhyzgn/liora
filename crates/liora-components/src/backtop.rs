//! Backtop module.
//!
//! This public module implements the Liora scroll-to-top affordance for long native scroll containers. It keeps the reusable
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

use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use gpui::{
    AnyElement, App, Bounds, Context, ElementId, Entity, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, Render, ScrollHandle, Window, div, point, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Fluent native GPUI component for rendering Liora backtop.
pub struct Backtop {
    id: gpui::SharedString,
    scroll_handle: ScrollHandle,
    visibility_height: Pixels,
    right: Pixels,
    bottom: Pixels,
    is_visible: bool,
    content: Option<Box<dyn Fn(&mut Window, &mut Context<Backtop>) -> AnyElement + 'static>>,
}

impl Backtop {
    /// Creates `Backtop` bound to the provided GPUI scroll handle.
    pub fn new(scroll_handle: ScrollHandle) -> Self {
        Self {
            id: liora_core::unique_id("backtop"),
            scroll_handle,
            visibility_height: px(200.0),
            right: px(40.0),
            bottom: px(40.0),
            is_visible: false,
            content: None,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<gpui::SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the scroll distance required before Backtop becomes visible.
    pub fn visibility_height(mut self, h: impl Into<Pixels>) -> Self {
        self.visibility_height = h.into();
        self
    }

    /// Applies the predefined visibility height sm sizing preset.
    pub fn visibility_height_sm(self) -> Self {
        self.visibility_height(px(100.0))
    }

    /// Applies the right preset.
    pub fn right(mut self, r: impl Into<Pixels>) -> Self {
        self.right = r.into();
        self
    }

    /// Applies the predefined right lg sizing preset.
    pub fn right_lg(self) -> Self {
        self.right(px(100.0))
    }

    /// Applies the bottom preset.
    pub fn bottom(mut self, b: impl Into<Pixels>) -> Self {
        self.bottom = b.into();
        self
    }

    /// Sets the rendered content element or text for this component.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<Backtop>) -> AnyElement + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl Render for Backtop {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let (card, border, hover, primary) = (
            theme.neutral.card,
            theme.neutral.border,
            theme.neutral.hover,
            theme.primary.base,
        );
        let is_visible = self.is_visible;

        let scroll_handle = self.scroll_handle.clone();

        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .child(BacktopVisibilityTracker {
                backtop: cx.entity().clone(),
                scroll_handle: self.scroll_handle.clone(),
                visibility_height: self.visibility_height,
            })
            .when(is_visible, |s| {
                s.child(pop_in(
                    element_id(format!("{}-btn-motion", self.id)),
                    div()
                        .id(element_id(format!("{}-btn", self.id)))
                        .tab_index(0)
                        .role(gpui::accesskit::Role::Button)
                        .aria_label("Back to top")
                        .absolute()
                        .bottom(self.bottom)
                        .right(self.right)
                        .cursor_pointer()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(40.0))
                        .h(px(40.0))
                        .rounded_full()
                        .bg(card)
                        .border_1()
                        .border_color(border)
                        .shadow_lg()
                        .hover(|s| s.cursor_pointer().bg(hover))
                        .focus_visible(|style| {
                            style.shadow(vec![
                                gpui::BoxShadow::new(px(0.0), px(0.0), primary)
                                    .spread_radius(px(2.0)),
                            ])
                        })
                        .on_key_down(|event, window, cx| {
                            let modifiers = event.keystroke.modifiers;
                            if event.keystroke.key == "tab"
                                && !modifiers.control
                                && !modifiers.alt
                                && !modifiers.platform
                                && !modifiers.function
                            {
                                if modifiers.shift {
                                    window.focus_prev(cx);
                                } else {
                                    window.focus_next(cx);
                                }
                                window.prevent_default();
                                cx.stop_propagation();
                            } else if matches!(event.keystroke.key.as_str(), "enter" | "space")
                                && !event.keystroke.modifiers.modified()
                            {
                                cx.stop_propagation();
                            }
                        })
                        .on_click(move |_, window, cx| {
                            scroll_handle.set_offset(point(px(0.0), px(0.0)));
                            window.refresh();
                            cx.stop_propagation();
                        })
                        .child(match &self.content {
                            Some(content_fn) => (content_fn)(_window, cx),
                            None => Icon::new(IconName::ChevronUp)
                                .size(px(20.0))
                                .color(primary)
                                .into_any_element(),
                        }),
                ))
            })
    }
}

struct BacktopVisibilityTracker {
    backtop: Entity<Backtop>,
    scroll_handle: ScrollHandle,
    visibility_height: Pixels,
}

impl gpui::Element for BacktopVisibilityTracker {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = gpui::Style::default();
        style.size.width = px(0.0).into();
        style.size.height = px(0.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let visible = -self.scroll_handle.offset().y >= self.visibility_height;
        self.backtop.update(cx, |this, _| {
            if this.is_visible != visible {
                this.is_visible = visible;
                // Paint-time notifications do not schedule another native frame.
                window.request_animation_frame();
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_return_respects_threshold_and_bound_container()
    -> Result<(), Box<dyn std::error::Error>> {
        struct Page {
            scroll: ScrollHandle,
            other: ScrollHandle,
            backtop: Entity<Backtop>,
        }

        impl Render for Page {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .size_full()
                    .flex()
                    .child(
                        div()
                            .relative()
                            .w(px(240.0))
                            .h(px(200.0))
                            .child(
                                div()
                                    .id("backtop-primary-scroll")
                                    .size_full()
                                    .overflow_y_scroll()
                                    .track_scroll(&self.scroll)
                                    .child(div().h(px(600.0))),
                            )
                            .child(self.backtop.clone()),
                    )
                    .child(
                        div()
                            .id("backtop-unrelated-scroll")
                            .w(px(240.0))
                            .h(px(200.0))
                            .overflow_y_scroll()
                            .track_scroll(&self.other)
                            .child(div().h(px(600.0))),
                    )
            }
        }

        for key in ["enter", "space"] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            app.update(|cx| cx.set_reduce_motion(true));
            let scroll = ScrollHandle::new();
            let other = ScrollHandle::new();
            let handle = app.add_window(|_, cx| Page {
                scroll: scroll.clone(),
                other: other.clone(),
                backtop: cx.new(|_| Backtop::new(scroll.clone()).visibility_height_sm()),
            });
            let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
            window.update(|window, cx| window.draw(cx).clear(cx));
            other.set_offset(point(px(0.0), px(-250.0)));
            for (offset, reachable) in [(-99.0, false), (-100.0, true)] {
                scroll.set_offset(point(px(0.0), px(offset)));
                window.update(|window, cx| {
                    window.refresh();
                    window.draw(cx).clear(cx);
                    if reachable {
                        assert!(
                            window.simulate_next_frame(cx) > 0,
                            "visibility change must request the frame that exposes Backtop"
                        );
                    }
                });
                window.update(|window, cx| {
                    window.draw(cx).clear(cx);
                    window.blur(cx);
                    window.focus_next(cx);
                    assert_eq!(
                        window.focused(cx).is_some(),
                        reachable,
                        "Backtop keyboard reachability at offset {offset}"
                    );
                    window.draw(cx).clear(cx);
                    assert_eq!(
                        window.simulate_next_frame(cx),
                        0,
                        "stable visibility must not keep requesting frames"
                    );
                });
            }
            window.simulate_keystrokes(key);
            assert_eq!(
                scroll.offset().y,
                px(-100.0),
                "keydown must not return to top"
            );
            window.update(|window, cx| {
                window.refresh();
                window.draw(cx).clear(cx);
                let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os)?;
                window.dispatch_event(
                    gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                    cx,
                );
                Ok::<_, Box<dyn std::error::Error>>(())
            })?;
            assert_eq!(
                scroll.offset().y,
                px(0.0),
                "{key} must return its container to top"
            );
            assert_eq!(
                other.offset().y,
                px(-250.0),
                "unrelated container must not move"
            );
            window.update(|window, cx| {
                window.draw(cx).clear(cx);
                assert!(
                    window.simulate_next_frame(cx) > 0,
                    "return must request the frame that removes Backtop"
                );
            });
            window.update(|window, cx| {
                window.draw(cx).clear(cx);
                window.blur(cx);
                window.focus_next(cx);
                assert!(
                    window.focused(cx).is_none(),
                    "hidden Backtop must not remain a tab stop"
                );
                assert_eq!(
                    window.simulate_next_frame(cx),
                    0,
                    "hidden Backtop must settle without another frame"
                );
            });
            app.quit();
        }
        Ok(())
    }

    #[test]
    fn tab_navigation_preserves_parent_shortcuts() -> Result<(), Box<dyn std::error::Error>> {
        use std::{cell::Cell, rc::Rc};

        struct Page {
            before: gpui::FocusHandle,
            after: gpui::FocusHandle,
            scroll: ScrollHandle,
            backtop: Entity<Backtop>,
            parent_shortcuts: Rc<Cell<usize>>,
        }

        impl Render for Page {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let parent_shortcuts = self.parent_shortcuts.clone();
                div()
                    .relative()
                    .flex()
                    .flex_col()
                    .w(px(240.0))
                    .h(px(200.0))
                    .on_key_down(move |event, window, cx| {
                        if event.keystroke.key == "tab" {
                            let modifiers = event.keystroke.modifiers;
                            if modifiers.control
                                || modifiers.alt
                                || modifiers.platform
                                || modifiers.function
                            {
                                parent_shortcuts.set(parent_shortcuts.get() + 1);
                            } else if modifiers.shift {
                                window.focus_prev(cx);
                            } else {
                                window.focus_next(cx);
                            }
                        }
                    })
                    .child(
                        div()
                            .id("before")
                            .size(px(20.0))
                            .tab_index(0)
                            .track_focus(&self.before),
                    )
                    .child(
                        div()
                            .id("scroll")
                            .h(px(200.0))
                            .overflow_y_scroll()
                            .track_scroll(&self.scroll)
                            .child(div().h(px(600.0))),
                    )
                    .child(self.backtop.clone())
                    .child(
                        div()
                            .id("after")
                            .size(px(20.0))
                            .tab_index(0)
                            .track_focus(&self.after),
                    )
            }
        }

        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let before = app.update(|cx| cx.focus_handle().tab_stop(true));
        let after = app.update(|cx| cx.focus_handle().tab_stop(true));
        let scroll = ScrollHandle::new();
        let parent_shortcuts = Rc::new(Cell::new(0));
        let handle = app.add_window(|_, cx| Page {
            before: before.clone(),
            after: after.clone(),
            scroll: scroll.clone(),
            backtop: cx.new(|_| Backtop::new(scroll.clone()).visibility_height_sm()),
            parent_shortcuts: parent_shortcuts.clone(),
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        scroll.set_offset(point(px(0.0), px(-200.0)));
        window.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
            window.draw(cx).clear(cx);
            before.focus(window, cx);
            window.draw(cx).clear(cx);
        });
        window.simulate_keystrokes("tab");
        let focused = window
            .update(|window, cx| window.focused(cx))
            .ok_or("Backtop focus missing")?;
        assert_ne!(focused, before);
        assert_ne!(focused, after);
        for (index, shortcut) in ["ctrl-tab", "ctrl-shift-tab", "alt-tab", "cmd-tab", "fn-tab"]
            .iter()
            .enumerate()
        {
            window.simulate_keystrokes(shortcut);
            window.update(|window, cx| assert_eq!(window.focused(cx), Some(focused.clone())));
            assert_eq!(
                parent_shortcuts.get(),
                index + 1,
                "parent shortcut {shortcut}"
            );
        }
        window.simulate_keystrokes("tab");
        window.update(|window, _| assert!(after.is_focused(window)));
        window.simulate_keystrokes("shift-tab");
        window.update(|window, cx| assert_eq!(window.focused(cx), Some(focused.clone())));
        window.simulate_keystrokes("shift-tab");
        window.update(|window, _| assert!(before.is_focused(window)));
        assert_eq!(
            scroll.offset().y,
            px(-200.0),
            "Tab must not activate Backtop"
        );
        app.quit();
        Ok(())
    }
}
