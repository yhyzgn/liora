//! Anchor module.
//!
//! This public module implements the Liora in-page anchor navigation component for documentation and long views. It keeps the reusable
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
    AnyElement, App, Bounds, Context, ElementId, Entity, EntityId, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, Pixels, Render, ScrollHandle, SharedString,
    WeakEntity, Window, div, point, prelude::*, px,
};
use liora_core::Config;
use std::collections::HashMap;

/// Fluent native GPUI component for rendering Liora anchor link.
pub struct AnchorLink {
    /// Primary heading or title text displayed by the component.
    pub title: SharedString,
    /// Destination URL or anchor fragment used by this navigation item.
    pub href: SharedString,
    /// Nested child items rendered beneath this item.
    pub children: Vec<AnchorLink>,
}

/// Fluent native GPUI component for rendering Liora anchor.
pub struct Anchor {
    scroll_handle: ScrollHandle,
    active_link: Option<SharedString>,
    links: Vec<AnchorLink>,
    offset: Pixels,
    targets_bounds:
        HashMap<SharedString, (Bounds<Pixels>, WeakEntity<AnchorTargetLease>, EntityId)>,
}

impl AnchorLink {
    /// Creates `AnchorLink` initialized from the supplied title, and href.
    pub fn new(title: impl Into<SharedString>, href: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            href: href.into(),
            children: vec![],
        }
    }

    /// Adds a child element to the component body.
    pub fn child(mut self, link: AnchorLink) -> Self {
        self.children.push(link);
        self
    }
}

impl Anchor {
    /// Creates `Anchor` bound to the provided GPUI scroll handle.
    pub fn new(scroll_handle: ScrollHandle) -> Self {
        Self {
            scroll_handle,
            active_link: None,
            links: vec![],
            offset: px(0.0),
            targets_bounds: HashMap::new(),
        }
    }

    /// Sets the pixel offset used when positioning the component.
    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }

    /// Applies the predefined offset sm sizing preset.
    pub fn offset_sm(self) -> Self {
        self.offset(px(20.0))
    }

    /// Adds a navigation link item to the anchor list.
    pub fn link(mut self, link: AnchorLink) -> Self {
        self.links.push(link);
        self
    }
    /// Scrolls to a painted target in this handle's viewport; false if it is absent.
    pub fn scroll_to_target(&self, id: &str) -> bool {
        let Some((bounds, lease, _)) = self.targets_bounds.get(id) else {
            return false;
        };
        if lease.upgrade().is_none() {
            return false;
        }
        let jump = self.scroll_handle.offset().y + self.scroll_handle.bounds().top() + self.offset
            - bounds.top();
        self.scroll_handle.set_offset(point(px(0.0), jump));
        true
    }

    fn update_target_bounds(
        &mut self,
        id: SharedString,
        bounds: Bounds<Pixels>,
        lease: WeakEntity<AnchorTargetLease>,
        generation: EntityId,
        cx: &mut Context<Self>,
    ) {
        self.targets_bounds.insert(id, (bounds, lease, generation));
        self.detect_active_link(cx);
    }

    fn remove_target(&mut self, id: &str, generation: EntityId, cx: &mut Context<Self>) {
        if self
            .targets_bounds
            .get(id)
            .is_some_and(|(_, _, current)| *current == generation)
        {
            self.targets_bounds.remove(id);
            self.detect_active_link(cx);
        }
    }

    fn detect_active_link(&mut self, cx: &mut Context<Self>) {
        self.targets_bounds
            .retain(|_, (_, lease, _)| lease.upgrade().is_some());
        let viewport_top = self.scroll_handle.bounds().top();
        let threshold = viewport_top + self.offset + px(10.0);
        let mut best_link = None;
        let mut min_dist = f32::MAX;

        for (id, (bounds, _, _)) in &self.targets_bounds {
            let dist = (f32::from(bounds.top()) - f32::from(viewport_top + self.offset)).abs();
            if dist < min_dist && bounds.top() <= threshold {
                min_dist = dist;
                best_link = Some(id.clone());
            }
        }

        if best_link != self.active_link {
            self.active_link = best_link;
            cx.notify();
        }
    }

    fn render_link(
        &self,
        link: &AnchorLink,
        depth: u32,
        theme: &liora_theme::Theme,
        anchor_entity: Entity<Anchor>,
        window: &mut Window,
        cx: &Context<Self>,
    ) -> AnyElement {
        let is_active = self.active_link.as_ref() == Some(&link.href);
        let href = link.href.clone();
        let click_anchor_entity = anchor_entity.clone();
        let focus = window.with_global_id(href.clone().into(), |global_id, window| {
            window.with_element_state::<gpui::FocusHandle, _>(global_id, |previous, _| {
                let handle =
                    previous.unwrap_or_else(|| cx.focus_handle().tab_stop(true).tab_index(0));
                (handle.clone(), handle)
            })
        });
        let paint_focus = focus.clone();
        let focus_color = theme.primary.base;
        let outline = gpui::canvas(
            |_, _, _| (),
            move |bounds, (), window, _| {
                if paint_focus.is_focused(window) && window.last_input_was_keyboard() {
                    window.paint_quad(
                        gpui::outline(
                            bounds.dilate(px(2.0)),
                            focus_color,
                            gpui::BorderStyle::default(),
                        )
                        .border_widths(px(2.0))
                        .corner_radii(px(2.0)),
                    );
                }
            },
        )
        .absolute()
        .inset(px(0.0));

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .id(href.clone())
                    .track_focus(&focus)
                    .tab_index(0)
                    .role(gpui::Role::Link)
                    .aria_label(link.title.clone())
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .h(px(32.0))
                    .pl(px(16.0 + depth as f32 * 16.0))
                    .text_color(if is_active {
                        theme.primary.base
                    } else {
                        theme.neutral.text_2
                    })
                    .hover(|s| s.text_color(theme.primary.base))
                    .capture_key_down(|event, window, _| {
                        // Keep Space from arming GPUI's shared Enter/Space click marker.
                        if event.keystroke.key == "space" && !event.keystroke.modifiers.modified() {
                            window.prevent_default();
                        }
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
                            cx.stop_propagation();
                        } else if event.keystroke.key == "enter" && !modifiers.modified() {
                            cx.stop_propagation();
                        }
                    })
                    .on_click(move |event, window, cx| {
                        if let gpui::ClickEvent::Keyboard(click) = event {
                            if click.button != gpui::KeyboardButton::Enter {
                                return;
                            }
                            cx.stop_propagation();
                        }
                        if click_anchor_entity.update(cx, |this, _| this.scroll_to_target(&href)) {
                            window.refresh();
                        }
                    })
                    .child(div().text_sm().child(link.title.clone()))
                    .child(outline),
            )
            .children(link.children.iter().map(|child| {
                self.render_link(child, depth + 1, theme, anchor_entity.clone(), window, cx)
            }))
            .into_any_element()
    }
}

impl Render for Anchor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let anchor_entity = cx.entity().clone();

        div()
            .flex()
            .flex_col()
            .w_full()
            .relative()
            .child(
                // Vertical line
                div()
                    .absolute()
                    .left_0()
                    .top_0()
                    .bottom_0()
                    .w(px(2.0))
                    .bg(theme.neutral.border),
            )
            .children(
                self.links.iter().map(|link| {
                    self.render_link(link, 0, theme, anchor_entity.clone(), window, cx)
                }),
            )
    }
}

/// Fluent native GPUI component for rendering Liora anchor target.
pub struct AnchorTarget {
    id: SharedString,
    anchor: Entity<Anchor>,
    child: AnyElement,
}

impl AnchorTarget {
    /// Creates `AnchorTarget` with default theme-driven styling and no optional callbacks attached.
    pub fn new(
        id: impl Into<SharedString>,
        anchor: Entity<Anchor>,
        child: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            anchor,
            child: child.into_any_element(),
        }
    }
}

struct AnchorTargetLease;

impl gpui::Element for AnchorTarget {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(ElementId::NamedInteger(
            self.id.clone(),
            self.anchor.entity_id().as_u64(),
        ))
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
        global_id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _rl: &mut (),
        _ps: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        // GPUI retains the lease through cached paints and releases it when the target is omitted.
        window.with_element_state::<Entity<AnchorTargetLease>, _>(
            global_id.unwrap(),
            |state, _| {
                let lease = state.unwrap_or_else(|| {
                    let lease = cx.new(|_| AnchorTargetLease);
                    let anchor = self.anchor.downgrade();
                    let id = self.id.clone();
                    let generation = lease.entity_id();
                    cx.observe_release(&lease, move |_, cx| {
                        let _ = anchor.update(cx, |anchor, cx| {
                            anchor.remove_target(&id, generation, cx);
                        });
                    })
                    .detach();
                    lease
                });
                let generation = lease.entity_id();
                let _ = self.anchor.update(cx, |this, cx| {
                    this.update_target_bounds(
                        self.id.clone(),
                        bounds,
                        lease.downgrade(),
                        generation,
                        cx,
                    );
                });
                ((), lease)
            },
        );
        self.child.paint(window, cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    #[test]
    fn removed_target_cannot_scroll_retained_anchor() {
        struct TargetView {
            anchor: Entity<Anchor>,
            renders: Rc<Cell<usize>>,
        }

        impl Render for TargetView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                self.renders.set(self.renders.get() + 1);
                AnchorTarget::new("details", self.anchor.clone(), div().h(px(40.0)))
            }
        }

        struct Workbench {
            anchor: Entity<Anchor>,
            target: Entity<TargetView>,
            scroll: ScrollHandle,
            show_target: bool,
            generation: u64,
        }

        impl Render for Workbench {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div().size_full().child(self.anchor.clone()).child(
                    div()
                        .id("anchor-scroll")
                        .w(px(240.0))
                        .h(px(200.0))
                        .overflow_y_scroll()
                        .track_scroll(&self.scroll)
                        .when(self.show_target, |view| {
                            view.child(div().id(self.generation).child(
                                self.target.clone().cached(
                                    gpui::StyleRefinement::default().w(px(240.0)).h(px(40.0)),
                                ),
                            ))
                        })
                        .child(div().h(px(500.0))),
                )
            }
        }

        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let scroll = ScrollHandle::new();
        let renders = Rc::new(Cell::new(0));
        let handle = app.add_window(|_, cx| {
            let anchor =
                cx.new(|_| Anchor::new(scroll.clone()).link(AnchorLink::new("Details", "details")));
            Workbench {
                target: cx.new(|_| TargetView {
                    anchor: anchor.clone(),
                    renders: renders.clone(),
                }),
                anchor,
                scroll: scroll.clone(),
                show_target: true,
                generation: 0,
            }
        });
        let anchor = handle
            .update(&mut app, |view, _, _| view.anchor.clone())
            .unwrap();
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(anchor.read_with(&app, |anchor, _| anchor.scroll_to_target("details")));
        assert!(
            anchor.read_with(&app, |anchor, _| anchor.active_link.as_deref()
                == Some("details"))
        );
        let rendered = renders.get();
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            renders.get(),
            rendered,
            "retained target must use cached paint"
        );
        assert!(anchor.read_with(&app, |anchor, _| anchor.scroll_to_target("details")));

        handle
            .update(&mut app, |view, _, cx| {
                view.generation += 1;
                cx.notify();
            })
            .unwrap();
        window.update(|window, cx| window.draw(cx).clear(cx));
        app.update(|_| {});
        assert!(
            anchor.read_with(&app, |anchor, _| anchor.scroll_to_target("details")),
            "releasing the old lease must preserve the replacement"
        );

        handle
            .update(&mut app, |view, _, cx| {
                view.show_target = false;
                cx.notify();
            })
            .unwrap();
        window.update(|window, cx| window.draw(cx).clear(cx));
        app.update(|_| {});
        scroll.set_offset(point(px(0.0), px(-17.0)));
        let offset = scroll.offset();
        assert!(anchor.read_with(&app, |anchor, _| anchor.links[0].href.as_ref() == "details"));
        assert!(!anchor.read_with(&app, |anchor, _| anchor.scroll_to_target("details")));
        assert_eq!(
            scroll.offset(),
            offset,
            "an absent target must not change the offset"
        );
        assert!(
            anchor.read_with(&app, |anchor, _| anchor.active_link.is_none()),
            "releasing the last target must clear its active highlight"
        );
        app.quit();
    }

    #[test]
    fn keyboard_enter_scrolls_to_painted_target_after_redraw()
    -> Result<(), Box<dyn std::error::Error>> {
        struct Page {
            anchor: Entity<Anchor>,
            scroll: ScrollHandle,
        }

        impl Render for Page {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div().size_full().child(self.anchor.clone()).child(
                    div()
                        .id("anchor-keyboard-scroll")
                        .w(px(240.0))
                        .h(px(200.0))
                        .overflow_y_scroll()
                        .track_scroll(&self.scroll)
                        .child(AnchorTarget::new(
                            "details",
                            self.anchor.clone(),
                            div().h(px(40.0)),
                        ))
                        .child(div().h(px(500.0))),
                )
            }
        }

        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let scroll = ScrollHandle::new();
        let handle = app.add_window(|_, cx| Page {
            anchor: cx
                .new(|_| Anchor::new(scroll.clone()).link(AnchorLink::new("Details", "details"))),
            scroll: scroll.clone(),
        });
        let anchor = handle.update(&mut app, |page, _, _| page.anchor.clone())?;
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        scroll.set_offset(point(px(0.0), px(-17.0)));
        window.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
            window.focus_next(cx);
            window.draw(cx).clear(cx);
        });
        assert_eq!(scroll.offset().y, px(-17.0));
        let before_top = anchor.read_with(&app, |anchor, _| anchor.targets_bounds["details"].0.top());
        for (down, up) in [
            (None, "enter"),
            (Some("space"), "space"),
            (Some("space"), "enter"),
            (Some("ctrl-enter"), "ctrl-enter"),
            (Some("alt-enter"), "alt-enter"),
            (Some("shift-enter"), "shift-enter"),
        ] {
            if let Some(down) = down {
                window.simulate_keystrokes(down);
            }
            window.update(|window, cx| {
                let keystroke = gpui::Keystroke::parse_with_os(up, cx.platform_info().os)?;
                window.dispatch_event(
                    gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                    cx,
                );
                Ok::<_, Box<dyn std::error::Error>>(())
            })?;
            assert_eq!(
                scroll.offset().y,
                px(-17.0),
                "unexpected navigation from {down:?} -> {up}"
            );
        }
        window.simulate_keystrokes("enter");
        assert_eq!(scroll.offset().y, px(-17.0), "keydown must not navigate");
        window.update(|window, cx| {
            window.refresh();
            window.draw(cx).clear(cx);
            let keystroke = gpui::Keystroke::parse_with_os("enter", cx.platform_info().os)?;
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
            Ok::<_, Box<dyn std::error::Error>>(())
        })?;
        assert_eq!(
            scroll.offset().y,
            px(0.0),
            "Enter release must navigate to Details"
        );
        window.update(|window, cx| window.draw(cx).clear(cx));
        let after_top = anchor.read_with(&app, |anchor, _| anchor.targets_bounds["details"].0.top());
        assert_eq!(
            after_top - before_top,
            px(17.0),
            "target must repaint at the new offset"
        );
        app.quit();
        Ok(())
    }
}
