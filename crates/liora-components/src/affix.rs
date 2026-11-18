//! Affix module.
//!
//! This public module implements the Liora affixed helper that pins lightweight content to viewport or container edges. It keeps the reusable
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
    /// Places the overlay above the anchor.
    Top,
    /// Places the overlay below the anchor.
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
        let content_fn = self.content.clone();
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
                let fixed_content = content_fn(_window, cx);

                push_passive_portal(
                    move |_, _| {
                        div()
                            .absolute()
                            .top(fixed_top)
                            .left(fixed_left)
                            .w(fixed_width)
                            .child(fixed_content)
                            .into_any_element()
                    },
                    cx,
                );
            }
        }

        let flow_content = if is_fixed {
            match last_bounds {
                Some(bounds) => div()
                    .w(bounds.size.width)
                    .h(bounds.size.height)
                    .into_any_element(),
                None => div().h(px(40.0)).into_any_element(),
            }
        } else {
            content_fn(_window, cx)
        };

        div().relative().child(BoundsTracker {
            child: flow_content,
            on_bounds_change: Box::new(move |bounds, window, cx| {
                let (offset, position, current_fixed) =
                    affix_handle.update(cx, |this, _| (this.offset, this.position, this.is_fixed));

                let should_be_fixed = match position {
                    AffixPosition::Top => bounds.top() <= offset,
                    AffixPosition::Bottom => {
                        let viewport_h = window.viewport_size().height;
                        bounds.bottom() >= viewport_h - offset
                    }
                };

                affix_handle.update(cx, |this, _| {
                    this.last_bounds = Some(bounds);
                });

                if should_be_fixed != current_fixed {
                    affix_handle.update(cx, |this, cx| {
                        this.is_fixed = should_be_fixed;
                        if let Some(ref on_change) = this.on_change {
                            (on_change)(should_be_fixed, window, cx);
                        }
                        cx.notify();
                    });
                }
            }),
        })
    }
}

struct BoundsTracker {
    child: AnyElement,
    on_bounds_change: Box<dyn Fn(Bounds<Pixels>, &mut Window, &mut App)>,
}

impl gpui::Element for BoundsTracker {
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
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn affix_nested_geometry_tracks_scroll_and_fixed_transition() {
        struct Workbench {
            affix: Entity<Affix>,
            scroll: ScrollHandle,
        }

        impl Render for Workbench {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div().size_full().pl(px(23.0)).pt(px(17.0)).child(
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
            }
        }

        let mut app = gpui::TestAppContext::single();
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
        });
        let affix = handle.update(&mut app, |view, _, _| view.affix.clone()).unwrap();
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        let expected = Bounds::new(point(px(23.0), px(117.0)), size(px(120.0), px(30.0)));
        assert_eq!(window.debug_bounds("affix-child"), Some(expected));
        assert_eq!(affix.read_with(&app, |affix, _| affix.last_bounds), Some(expected));
        assert!(!affix.read_with(&app, |affix, _| affix.is_fixed));

        scroll.set_offset(point(px(0.0), px(-20.0)));
        window.update(|window, cx| { window.refresh(); window.draw(cx).clear(cx); });
        let scrolled = Bounds::new(point(px(23.0), px(97.0)), expected.size);
        assert_eq!(window.debug_bounds("affix-child"), Some(scrolled));
        assert_eq!(affix.read_with(&app, |affix, _| affix.last_bounds), Some(scrolled));

        scroll.set_offset(point(px(0.0), px(-60.0)));
        window.update(|window, cx| { window.refresh(); window.draw(cx).clear(cx); });
        assert!(affix.read_with(&app, |affix, _| affix.is_fixed));
        window.update(|window, cx| window.draw(cx).clear(cx));
        let slot = window.debug_bounds("affix-slot").unwrap();
        let tracked = affix.read_with(&app, |affix, _| affix.last_bounds).unwrap();
        assert_eq!(tracked.origin, slot.origin);
        assert_eq!(tracked.size, expected.size);

        scroll.set_offset(point(px(0.0), px(0.0)));
        window.update(|window, cx| { window.refresh(); window.draw(cx).clear(cx); });
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(window.debug_bounds("affix-child"), Some(expected));
        assert_eq!(*changes.borrow(), [true, false]);
        app.quit();
    }
}
