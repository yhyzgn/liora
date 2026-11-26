//! Affix module.
//!
//! This public module implements the Liora affixed helper that pins lightweight content to window viewport edges. It keeps the reusable
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

use gpui::{
    AnyElement, App, Bounds, Context, ElementId, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, Pixels, Render, Window, div, prelude::*, px,
};
use liora_core::push_passive_portal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control affix position behavior.
pub enum AffixPosition {
    #[default]
    /// Pins content to the top of the window viewport.
    Top,
    /// Pins content to the bottom of the window viewport.
    Bottom,
}

/// Fluent native GPUI component for rendering Liora affix.
pub struct Affix {
    offset: Pixels,
    position: AffixPosition,
    is_fixed: bool,
    last_bounds: Option<Bounds<Pixels>>,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    content: Arc<dyn Fn(&mut Window, &mut Context<Affix>) -> AnyElement + 'static>,
}

use std::sync::Arc;

impl Affix {
    /// Creates `Affix` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            offset: px(0.0),
            position: AffixPosition::Top,
            is_fixed: false,
            last_bounds: None,
            on_change: None,
            content: Arc::new(|_, _| div().into_any_element()),
        }
    }

    /// Sets the pixel offset used when positioning the component.
    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }

    /// Applies the predefined offset md sizing preset.
    pub fn offset_md(self) -> Self {
        self.offset(px(20.0))
    }

    /// Applies the predefined offset lg sizing preset.
    pub fn offset_lg(self) -> Self {
        self.offset(px(80.0))
    }

    /// Selects the edge or anchor position used by layout.
    pub fn position(mut self, pos: AffixPosition) -> Self {
        self.position = pos;
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    /// Sets the rendered content element or text for this component.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<Affix>) -> AnyElement + 'static,
    {
        self.content = Arc::new(f);
        self
    }
}

impl Render for Affix {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_fixed = self.is_fixed;
        let offset = self.offset;
        let content_fn = &self.content;
        let affix_handle = cx.entity().clone();
        let last_bounds = self.last_bounds;

        if is_fixed {
            if let Some(bounds) = last_bounds {
                let fixed_top = match self.position {
                    AffixPosition::Top => offset,
                    AffixPosition::Bottom => {
                        _window.viewport_size().height - offset - bounds.size.height
                    }
                };
                let fixed_left = bounds.left();
                let fixed_width = bounds.size.width;
                let fixed_handle = affix_handle.downgrade();
                let fixed_content = BoundsTracker {
                    child: div()
                        .absolute()
                        .top(fixed_top)
                        .left(fixed_left)
                        .w(fixed_width)
                        .child(content_fn(_window, cx))
                        .into_any_element(),
                    on_bounds_change: move |bounds, window, cx| {
                        let Some(affix) = fixed_handle.upgrade() else {
                            return;
                        };
                        affix.update(cx, |this, cx| {
                            let Some(anchor) = this.last_bounds.as_mut() else {
                                return;
                            };
                            if anchor.size.height == bounds.size.height {
                                return;
                            }
                            anchor.size.height = bounds.size.height;
                            // The portal paints under its host, so notify Affix explicitly.
                            let entity = cx.entity_id();
                            window.on_next_frame(move |_, cx| cx.notify(entity));
                        });
                    },
                };

                push_passive_portal(move |_, _| fixed_content.into_any_element(), cx);
            }
        }

        let flow_content = if is_fixed {
            match last_bounds {
                Some(bounds) => div()
                    .w(bounds.size.width)
                    .max_w_full()
                    .h(bounds.size.height)
                    .into_any_element(),
                None => div().h(px(40.0)).into_any_element(),
            }
        } else {
            content_fn(_window, cx)
        };

        BoundsTracker {
            child: div().relative().child(flow_content).into_any_element(),
            on_bounds_change: move |bounds, window, cx| {
                affix_handle.update(cx, |this, cx| {
                    let should_be_fixed = match this.position {
                        AffixPosition::Top => bounds.top() <= this.offset,
                        AffixPosition::Bottom => {
                            bounds.bottom() >= window.viewport_size().height - this.offset
                        }
                    };
                    let state_changed = should_be_fixed != this.is_fixed;
                    let geometry_changed = should_be_fixed
                        && this.last_bounds.is_some_and(|previous| {
                            previous.left() != bounds.left() || previous.size != bounds.size
                        });
                    this.last_bounds = Some(bounds);

                    if state_changed {
                        this.is_fixed = should_be_fixed;
                        if let Some(on_change) = &this.on_change {
                            (on_change)(should_be_fixed, window, cx);
                        }
                    }
                    if state_changed || geometry_changed {
                        // Paint-time notification cannot schedule the portal correction frame.
                        window.request_animation_frame();
                    }
                });
            },
        }
    }
}

struct BoundsTracker<F: Fn(Bounds<Pixels>, &mut Window, &mut App)> {
    child: AnyElement,
    on_bounds_change: F,
}

impl<F: Fn(Bounds<Pixels>, &mut Window, &mut App) + 'static> gpui::Element for BoundsTracker<F> {
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
    ) -> (LayoutId, ()) {
        (self.child.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _rl: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> () {
        self.child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _rl: &mut (),
        _ps: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        (self.on_bounds_change)(bounds, window, cx);
        self.child.paint(window, cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Entity, ScrollHandle, point, size};
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

    #[test]
    fn affix_nested_geometry_tracks_scroll_and_fixed_transition()
    -> Result<(), Box<dyn std::error::Error>> {
        struct Workbench {
            affix: Entity<Affix>,
            scroll: ScrollHandle,
            left_padding: Pixels,
        }

        impl Render for Workbench {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .size_full()
                    .pl(self.left_padding)
                    .pt(px(17.0))
                    .child(
                        div()
                            .id("affix-scroll")
                            .w(px(240.0))
                            .h(px(200.0))
                            .overflow_y_scroll()
                            .track_scroll(&self.scroll)
                            .child(div().h(px(100.0)))
                            .child(
                                div()
                                    .id("affix-slot")
                                    .debug_selector(|| "affix-slot".into())
                                    .child(self.affix.clone()),
                            )
                            .child(div().h(px(500.0))),
                    )
                    .child(crate::PortalLayer)
            }
        }

        let mut app = gpui::TestAppContext::single();
        app.update(|cx| cx.set_reduce_motion(true));
        let changes = Rc::new(RefCell::new(Vec::new()));
        let scroll = ScrollHandle::new();
        let handle = app.add_window(|_, cx| Workbench {
            affix: cx.new(|_| {
                Affix::new()
                    .offset(px(70.0))
                    .content(|_, _| {
                        div()
                            .id("affix-child")
                            .debug_selector(|| "affix-child".into())
                            .w(px(120.0))
                            .h(px(30.0))
                            .into_any_element()
                    })
                    .on_change({
                        let changes = changes.clone();
                        move |fixed, _, _| changes.borrow_mut().push(fixed)
                    })
            }),
            scroll: scroll.clone(),
            left_padding: px(23.0),
        });
        let affix = handle.update(&mut app, |view, _, _| view.affix.clone())?;
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        let expected = Bounds::new(point(px(23.0), px(117.0)), size(px(120.0), px(30.0)));
        assert_eq!(window.debug_bounds("affix-child"), Some(expected));
        assert!(!affix.read_with(&app, |affix, _| affix.is_fixed));

        scroll.set_offset(point(px(0.0), px(-20.0)));
        window.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
        });
        let scrolled = Bounds::new(point(px(23.0), px(97.0)), expected.size);
        assert_eq!(window.debug_bounds("affix-child"), Some(scrolled));

        scroll.set_offset(point(px(0.0), px(-60.0)));
        window.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
            assert!(
                window.simulate_next_frame(cx) > 0,
                "fixing Affix must schedule the frame that displays its portal"
            );
        });
        assert!(affix.read_with(&app, |affix, _| affix.is_fixed));
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(window.simulate_next_frame(cx), 0, "fixed Affix must settle");
        });
        assert_eq!(
            window.debug_bounds("affix-child"),
            Some(Bounds::new(point(px(23.0), px(70.0)), expected.size))
        );
        handle.update(&mut app, |view, _, cx| {
            view.left_padding = px(63.0);
            cx.notify();
        })?;
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert!(
                window.simulate_next_frame(cx) > 0,
                "a fixed Affix must correct changed anchor geometry"
            );
        });
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(window.simulate_next_frame(cx), 0, "reflow must settle");
        });
        assert_eq!(
            window.debug_bounds("affix-child"),
            Some(Bounds::new(point(px(63.0), px(70.0)), expected.size))
        );
        scroll.set_offset(point(px(0.0), px(-80.0)));
        window.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
            assert_eq!(
                window.simulate_next_frame(cx),
                0,
                "vertical-only scrolling must not request a geometry correction frame"
            );
        });
        assert_eq!(
            window.debug_bounds("affix-child"),
            Some(Bounds::new(point(px(63.0), px(70.0)), expected.size))
        );
        let slot = window
            .debug_bounds("affix-slot")
            .ok_or("Affix slot missing")?;
        assert_eq!(
            slot,
            Bounds::new(point(px(63.0), px(37.0)), size(px(240.0), px(30.0)))
        );

        scroll.set_offset(point(px(0.0), px(0.0)));
        window.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
            assert!(
                window.simulate_next_frame(cx) > 0,
                "restoring Affix must schedule the frame that removes its portal"
            );
        });
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(
                window.simulate_next_frame(cx),
                0,
                "restored Affix must settle"
            );
        });
        assert_eq!(
            window.debug_bounds("affix-child"),
            Some(Bounds::new(point(px(63.0), px(117.0)), expected.size))
        );
        assert_eq!(*changes.borrow(), [true, false]);
        app.quit();
        Ok(())
    }

    #[test]
    fn bottom_affix_tracks_fixed_content_height() -> Result<(), Box<dyn std::error::Error>> {
        struct Workbench(Entity<Affix>);

        impl Render for Workbench {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .relative()
                    .size_full()
                    .child(
                        div()
                            .absolute()
                            .debug_selector(|| "bottom-affix-slot".into())
                            .top(px(180.0))
                            .left(px(23.0))
                            .child(self.0.clone()),
                    )
                    .child(crate::PortalLayer)
            }
        }

        let mut app = gpui::TestAppContext::single();
        app.update(|cx| cx.set_reduce_motion(true));
        let height = Rc::new(Cell::new(px(30.0)));
        let changes = Rc::new(RefCell::new(Vec::new()));
        let handle = app.open_window(size(px(300.0), px(200.0)), |_, cx| {
            Workbench(cx.new(|_| {
                Affix::new()
                    .position(AffixPosition::Bottom)
                    .offset(px(20.0))
                    .content({
                        let height = height.clone();
                        move |_, _| {
                            div()
                                .debug_selector(|| "bottom-affix-child".into())
                                .w(px(120.0))
                                .mt(px(10.0))
                                .h(height.get())
                                .into_any_element()
                        }
                    })
                    .on_change({
                        let changes = changes.clone();
                        move |fixed, _, _| changes.borrow_mut().push(fixed)
                    })
            }))
        });
        let affix = handle.update(&mut app, |view, _, _| view.0.clone())?;
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert!(window.simulate_next_frame(cx) > 0);
        });
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(window.simulate_next_frame(cx), 0);
        });
        let original = Bounds::new(point(px(23.0), px(150.0)), size(px(120.0), px(30.0)));
        assert_eq!(window.debug_bounds("bottom-affix-child"), Some(original));

        height.set(px(60.0));
        affix.update(&mut app, |_, cx| cx.notify());
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert!(
                window.simulate_next_frame(cx) > 0,
                "changed fixed content height must correct portal and placeholder"
            );
        });
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(
                window.simulate_next_frame(cx),
                0,
                "content reflow must settle"
            );
        });
        let resized = Bounds::new(point(px(23.0), px(120.0)), size(px(120.0), px(60.0)));
        assert_eq!(window.debug_bounds("bottom-affix-child"), Some(resized));
        assert_eq!(
            window.debug_bounds("bottom-affix-slot"),
            Some(Bounds::new(
                point(px(23.0), px(180.0)),
                size(px(120.0), px(70.0))
            ))
        );
        assert_eq!(*changes.borrow(), [true]);
        app.quit();
        Ok(())
    }

    #[test]
    fn fixed_affix_follows_container_width_in_both_directions()
    -> Result<(), Box<dyn std::error::Error>> {
        struct Workbench {
            affix: Entity<Affix>,
            width: Pixels,
        }

        impl Render for Workbench {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .size_full()
                    .child(div().w(self.width).child(self.affix.clone()))
                    .child(crate::PortalLayer)
            }
        }

        let mut app = gpui::TestAppContext::single();
        app.update(|cx| cx.set_reduce_motion(true));
        let changes = Rc::new(RefCell::new(Vec::new()));
        let handle = app.open_window(size(px(300.0), px(200.0)), |_, cx| Workbench {
            affix: cx.new(|_| {
                Affix::new()
                    .offset(px(70.0))
                    .content(|_, _| {
                        div()
                            .debug_selector(|| "responsive-affix-child".into())
                            .h(px(30.0))
                            .into_any_element()
                    })
                    .on_change({
                        let changes = changes.clone();
                        move |fixed, _, _| changes.borrow_mut().push(fixed)
                    })
            }),
            width: px(240.0),
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            assert!(window.simulate_next_frame(cx) > 0);
            window.draw(cx).clear(cx);
            assert_eq!(window.simulate_next_frame(cx), 0);
        });
        assert_eq!(
            window.debug_bounds("responsive-affix-child"),
            Some(Bounds::new(
                point(px(0.0), px(70.0)),
                size(px(240.0), px(30.0))
            ))
        );

        for width in [px(160.0), px(240.0)] {
            handle.update(&mut app, |view, _, cx| {
                view.width = width;
                cx.notify();
            })?;
            window.update(|window, cx| {
                window.draw(cx).clear(cx);
                assert!(
                    window.simulate_next_frame(cx) > 0,
                    "fixed Affix must follow both narrower and wider containers"
                );
                window.draw(cx).clear(cx);
                assert_eq!(window.simulate_next_frame(cx), 0, "resize must settle");
            });
            assert_eq!(
                window.debug_bounds("responsive-affix-child"),
                Some(Bounds::new(point(px(0.0), px(70.0)), size(width, px(30.0))))
            );
        }
        assert_eq!(*changes.borrow(), [true]);
        app.quit();
        Ok(())
    }
}
