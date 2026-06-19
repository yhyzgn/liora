//! Form module.
//!
//! This public module implements the Liora form and form-item layout primitives. It keeps the reusable
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

use gpui::{AnyElement, App, Component, Pixels, SharedString, Window, div, prelude::*, px};
use liora_core::Config;

/// Public builder and render state for the Liora form component.
pub struct Form {
    _label_width: Option<Pixels>,
    inline: bool,
    children: Vec<AnyElement>,
}

impl Form {
    /// Creates a new value with the required baseline configuration.
    pub fn new() -> Self {
        Self {
            _label_width: None,
            inline: false,
            children: Vec::new(),
        }
    }

    /// Configures the label width option.
    pub fn label_width(mut self, width: impl Into<Pixels>) -> Self {
        self._label_width = Some(width.into());
        self
    }
    /// Configures the inline option.
    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }
    /// Configures the child option.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for Form {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for Form {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .when(self.inline, |s| s.flex_row().gap_4().flex_wrap())
            .when(!self.inline, |s| s.flex_col().gap_4())
            .children(self.children)
    }
}

/// Public builder and render state for the Liora form item component.
pub struct FormItem {
    label: Option<SharedString>,
    label_width: Option<Pixels>,
    required: bool,
    error: Option<SharedString>,
    content: Option<AnyElement>,
}

impl FormItem {
    /// Creates a new value with the required baseline configuration.
    pub fn new() -> Self {
        Self {
            label: None,
            label_width: None,
            required: false,
            error: None,
            content: None,
        }
    }

    /// Returns the stable user-facing label for this value.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Configures the label width option.
    pub fn label_width(mut self, width: impl Into<Pixels>) -> Self {
        self.label_width = Some(width.into());
        self
    }
    /// Configures the required option.
    pub fn required(mut self, r: bool) -> Self {
        self.required = r;
        self
    }
    /// Configures the error option.
    pub fn error(mut self, e: impl Into<SharedString>) -> Self {
        self.error = Some(e.into());
        self
    }

    /// Configures the child option.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.content = Some(child.into_any_element());
        self
    }
}

impl IntoElement for FormItem {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for FormItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(div().flex().flex_row().items_center().gap_1().when_some(
                self.label,
                |this, label| {
                    this.child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .when_some(self.label_width, |s, w| s.w(w))
                            .child(
                                div()
                                    .text_size(px(theme.font_size.md))
                                    .text_color(theme.neutral.text_1)
                                    .child(label),
                            )
                            .when(self.required, |this| {
                                this.child(div().text_color(theme.danger.base).child("*"))
                            }),
                    )
                },
            ))
            .when_some(self.content, |this, content| this.child(content))
            .when_some(self.error, |this, error| {
                this.child(
                    div()
                        .text_size(px(theme.font_size.sm))
                        .text_color(theme.danger.base)
                        .child(error),
                )
            })
    }
}
