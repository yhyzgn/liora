//! Backtop module.
//!
//! This public module implements the Liora scroll-to-top affordance for long native scroll containers. It keeps the reusable
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

use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use gpui::{
    AnyElement, App, Bounds, Context, ElementId, Entity, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, Render, ScrollHandle, Window, div, point, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Public builder and render state for the Liora backtop component.
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
    /// Creates a new value with the required baseline configuration.
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

    /// Returns the stable tray command identifier used for menu event routing.
    pub fn id(mut self, id: impl Into<gpui::SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Configures the visibility height option.
    pub fn visibility_height(mut self, h: impl Into<Pixels>) -> Self {
        self.visibility_height = h.into();
        self
    }

    /// Applies the predefined visibility height sm sizing preset.
    pub fn visibility_height_sm(self) -> Self {
        self.visibility_height(px(100.0))
    }

    /// Configures the right option.
    pub fn right(mut self, r: impl Into<Pixels>) -> Self {
        self.right = r.into();
        self
    }

    /// Applies the predefined right lg sizing preset.
    pub fn right_lg(self) -> Self {
        self.right(px(100.0))
    }

    /// Configures the bottom option.
    pub fn bottom(mut self, b: impl Into<Pixels>) -> Self {
        self.bottom = b.into();
        self
    }

    /// Configures the content option.
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
        let theme = cx.global::<Config>().theme.clone();
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
                        .bg(theme.neutral.card)
                        .border_1()
                        .border_color(theme.neutral.border)
                        .shadow_lg()
                        .hover(|s| s.cursor_pointer().bg(theme.neutral.hover))
                        .on_click(move |_, _, _| {
                            scroll_handle.set_offset(point(px(0.0), px(0.0)));
                        })
                        .child(match &self.content {
                            Some(content_fn) => (content_fn)(_window, cx),
                            None => Icon::new(IconName::ChevronUp)
                                .size(px(20.0))
                                .color(theme.primary.base)
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

impl IntoElement for BacktopVisibilityTracker {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
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
        _window: &mut Window,
        cx: &mut App,
    ) {
        let visible = -self.scroll_handle.offset().y >= self.visibility_height;
        self.backtop.update(cx, |this, cx| {
            if this.is_visible != visible {
                this.is_visible = visible;
                cx.notify();
            }
        });
    }
}
