//! Textarea module.
//!
//! This public module implements the Liora multi-line text input component. It keeps the reusable
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

use crate::Input;
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Render, SharedString, Window, prelude::*, px,
};
use liora_core::Config;

/// Public builder and render state for the Liora textarea component.
pub struct Textarea {
    input: Entity<Input>,
    rows: usize,
    max_length: Option<usize>,
    focus_handle: FocusHandle,
}

impl Textarea {
    /// Creates a new value with the required baseline configuration.
    pub fn new(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let value = value.into();
        let rows = 1;
        let input = cx.new(|cx| Input::new(value, cx).min_rows(rows));

        Self {
            input,
            rows,
            max_length: None,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Configures the rows option.
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    /// Configures the placeholder option.
    pub fn placeholder(self, p: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        self.input.update(cx, |input, cx| {
            input.set_placeholder(p, cx);
        });
        self
    }

    /// Configures the disabled option.
    pub fn disabled(self, d: bool, cx: &mut Context<Self>) -> Self {
        self.input.update(cx, |input, cx| {
            input.set_disabled(d, cx);
        });
        self
    }

    /// Configures the max length option.
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }
}

impl Focusable for Textarea {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Textarea {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let value = self.input.read(cx).value();
        let len = value.chars().count();
        let rows = self.rows;

        // Sync rows to inner input if changed
        self.input.update(cx, |input, cx| {
            if input.min_rows != rows {
                input.set_min_rows(rows, cx);
            }
        });

        gpui::div()
            .flex()
            .flex_col()
            .gap_1()
            .child(self.input.clone())
            .when_some(self.max_length, |this, max| {
                this.child(
                    gpui::div()
                        .flex()
                        .justify_end()
                        .px(px(4.0))
                        .text_size(px(theme.font_size.sm))
                        .text_color(if len > max {
                            theme.danger.base
                        } else {
                            theme.neutral.text_3
                        })
                        .child(format!("{}/{}", len, max)),
                )
            })
    }
}
