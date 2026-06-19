//! Anchor module.
//!
//! This public module implements the Liora in-page anchor navigation component for documentation and long views. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
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
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use gpui::{
    AnyElement, App, Bounds, Context, ElementId, Entity, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, Render, ScrollHandle, SharedString, Window, div, point,
    prelude::*, px,
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
    targets_bounds: HashMap<SharedString, Bounds<Pixels>>,
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

    fn update_target_bounds(
        &mut self,
        id: SharedString,
        bounds: Bounds<Pixels>,
        cx: &mut Context<Self>,
    ) {
        self.targets_bounds.insert(id, bounds);
        // Detect active link in next frame or here
        self.detect_active_link(cx);
    }

    fn detect_active_link(&mut self, cx: &mut Context<Self>) {
        let viewport_top = self.scroll_handle.bounds().top();
        let threshold = viewport_top + self.offset + px(10.0);
        let mut best_link = None;
        let mut min_dist = f32::MAX;

        for (id, bounds) in &self.targets_bounds {
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
        cx: &Context<Self>,
    ) -> AnyElement {
        let is_active = self.active_link.as_ref() == Some(&link.href);
        let href = link.href.clone();
        let scroll_handle = self.scroll_handle.clone();
        let offset = self.offset;
        let click_anchor_entity = anchor_entity.clone();

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .id(href.clone())
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
                    .on_click(move |_, _, cx| {
                        let target_bounds = click_anchor_entity
                            .update(cx, |this, _| this.targets_bounds.get(&href).copied());
                        if let Some(bounds) = target_bounds {
                            let current_offset = scroll_handle.offset();
                            let viewport_top = scroll_handle.bounds().top();
                            let jump = current_offset.y + viewport_top + offset - bounds.top();
                            scroll_handle.set_offset(point(px(0.0), jump));
                        }
                    })
                    .child(div().text_sm().child(link.title.clone())),
            )
            .children(
                link.children.iter().map(|child| {
                    self.render_link(child, depth + 1, theme, anchor_entity.clone(), cx)
                }),
            )
            .into_any_element()
    }
}

impl Render for Anchor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
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
            .when_some(self.active_link.clone(), |s, _active| {
                // We'd need to know the position of the active link to render the indicator
                // For simplicity, we'll just style the link itself
                s
            })
            .children(
                self.links
                    .iter()
                    .map(|link| self.render_link(link, 0, &theme, anchor_entity.clone(), cx)),
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

impl IntoElement for AnchorTarget {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for AnchorTarget {
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
        bounds: Bounds<Pixels>,
        _rl: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> () {
        self.child.prepaint_at(bounds.origin, window, cx);
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
        let id = self.id.clone();
        let _ = self.anchor.update(cx, |this, cx| {
            this.update_target_bounds(id, bounds, cx);
        });
        self.child.paint(window, cx);
    }
}
