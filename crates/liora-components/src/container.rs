//! Container module.
//!
//! This public module implements the Liora application shell layout with header, aside, main, footer, and overlay slots. It keeps the reusable
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

use gpui::{AnyElement, App, Component, IntoElement, Pixels, RenderOnce, Window, prelude::*, px};
use liora_core::stable_unique_id;

/// Fluent native GPUI component for rendering Liora container.
pub struct Container {
    header: Option<AnyElement>,
    aside: Option<AnyElement>,
    aside_right: bool,
    footer: Option<AnyElement>,
    main: Vec<AnyElement>,
    overlays: Vec<AnyElement>,
    header_height: Pixels,
    footer_height: Pixels,
    aside_width: Pixels,
    aside_scroll: bool,
    main_scroll: bool,
    main_padding: Option<Pixels>,
}

impl Container {
    /// Creates `Container` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            header: None,
            aside: None,
            aside_right: false,
            footer: None,
            main: vec![],
            overlays: vec![],
            header_height: px(48.0),
            footer_height: px(48.0),
            aside_width: px(200.0),
            aside_scroll: false,
            main_scroll: false,
            main_padding: None,
        }
    }

    /// Sets the header value used by the component.
    pub fn header(mut self, el: impl IntoElement) -> Self {
        self.header = Some(el.into_any_element());
        self
    }
    /// Sets the aside value used by the component.
    pub fn aside(mut self, el: impl IntoElement) -> Self {
        self.aside = Some(el.into_any_element());
        self
    }
    /// Sets the aside right value used by the component.
    pub fn aside_right(mut self) -> Self {
        self.aside_right = true;
        self
    }
    /// Sets the footer value used by the component.
    pub fn footer(mut self, el: impl IntoElement) -> Self {
        self.footer = Some(el.into_any_element());
        self
    }
    /// Adds a child element to the component body.
    pub fn child(mut self, el: impl IntoElement) -> Self {
        self.main.push(el.into_any_element());
        self
    }
    /// Uses overlay chart rendering where series share the same baseline.
    pub fn overlay(mut self, el: impl IntoElement) -> Self {
        self.overlays.push(el.into_any_element());
        self
    }

    /// Sets the header height value used by the component.
    pub fn header_height(mut self, height: impl Into<Pixels>) -> Self {
        self.header_height = height.into();
        self
    }

    /// Applies the predefined header height lg sizing preset.
    pub fn header_height_lg(self) -> Self {
        self.header_height(px(84.0))
    }

    /// Sets the footer height value used by the component.
    pub fn footer_height(mut self, height: impl Into<Pixels>) -> Self {
        self.footer_height = height.into();
        self
    }

    /// Sets the aside width value used by the component.
    pub fn aside_width(mut self, width: impl Into<Pixels>) -> Self {
        self.aside_width = width.into();
        self
    }

    /// Applies the predefined aside width lg sizing preset.
    pub fn aside_width_lg(self) -> Self {
        self.aside_width(px(280.0))
    }

    /// Sets the aside scroll value used by the component.
    pub fn aside_scroll(mut self) -> Self {
        self.aside_scroll = true;
        self
    }

    /// Sets the main scroll value used by the component.
    pub fn main_scroll(mut self) -> Self {
        self.main_scroll = true;
        self
    }

    /// Sets the main padding value used by the component.
    pub fn main_padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.main_padding = Some(padding.into());
        self
    }

    /// Applies the predefined main padding xl sizing preset.
    pub fn main_padding_xl(self) -> Self {
        self.main_padding(px(32.0))
    }
}

impl RenderOnce for Container {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let aside_id = stable_unique_id("container-aside-scroll", "aside", window, cx);
        let main_id = stable_unique_id("container-main-scroll", "main", window, cx);
        let theme = cx.global::<liora_core::Config>().theme.clone();
        let aside_right = self.aside_right;
        let main_children = self.main;
        let overlays = self.overlays;
        let aside_width = self.aside_width;
        let aside_scroll = self.aside_scroll;
        let main_scroll = self.main_scroll;
        let main_padding = self.main_padding;

        let mut page = gpui::div()
            .flex()
            .flex_col()
            .size_full()
            .relative()
            .bg(theme.neutral.body);

        // Header
        if let Some(h) = self.header {
            page = page.child(
                gpui::div()
                    .flex_none()
                    .h(self.header_height)
                    .w_full()
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .child(h),
            );
        }

        // Body: aside + main
        let main = gpui::div()
            .flex_1()
            .min_h_0()
            .flex()
            .flex_col()
            .h_full()
            .id(main_id)
            .when(main_scroll, |s| s.overflow_y_scroll())
            .when_some(main_padding, |s, padding| s.p(padding))
            .children(main_children);

        let mut body = gpui::div().flex().flex_1().min_h_0().flex_row();
        if let Some(a) = self.aside {
            let aside_el = gpui::div()
                .flex_none()
                .w(aside_width)
                .h_full()
                .border_r_1()
                .border_color(theme.neutral.border)
                .id(aside_id)
                .when(aside_scroll, |s| s.overflow_y_scroll())
                .child(a);
            if aside_right {
                body = body.child(main);
                body = body.child(aside_el);
            } else {
                body = body.child(aside_el);
                body = body.child(main);
            }
        } else {
            body = body.child(main);
        }
        page = page.child(body);

        // Footer
        if let Some(f) = self.footer {
            page = page.child(
                gpui::div()
                    .flex_none()
                    .h(self.footer_height)
                    .w_full()
                    .border_t_1()
                    .border_color(theme.neutral.border)
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .child(f),
            );
        }

        page.children(overlays)
    }
}

impl IntoElement for Container {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_gallery_shell_helpers_track_layout_state() {
        let container = Container::new()
            .header_height_lg()
            .aside_width_lg()
            .aside_scroll()
            .main_scroll()
            .main_padding_xl()
            .overlay("portal");

        assert_eq!(container.header_height, px(84.0));
        assert_eq!(container.aside_width, px(280.0));
        assert!(container.aside_scroll);
        assert!(container.main_scroll);
        assert_eq!(container.main_padding, Some(px(32.0)));
        assert_eq!(container.overlays.len(), 1);
    }

    #[test]
    fn container_scroll_regions_use_distinct_stable_id_keys() {
        let production = include_str!("container.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(
            production.contains(r#"stable_unique_id("container-aside-scroll""#),
            "aside scroll region needs its own stable key"
        );
        assert!(
            production.contains(r#"stable_unique_id("container-main-scroll""#),
            "main scroll region needs its own stable key"
        );
        assert!(
            !production.contains(r#"stable_unique_id("container", "aside""#),
            "aside/main scroll regions must not share the same keyed state"
        );
    }

    #[test]
    fn container_main_scroll_region_is_height_constrained() {
        let production = include_str!("container.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(
            production.contains(".h_full()\n            .id(main_id)"),
            "main scroll region needs h_full before overflow_y_scroll so it forms a bounded viewport"
        );
    }
}
