//! Message Box module.
//!
//! This public module implements the Liora message-box confirmation/dialog helpers. It keeps the reusable
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

use crate::{Button, Dialog};
use gpui::{App, SharedString, Window, div, prelude::*};
use std::sync::Arc;

/// Fluent native GPUI component for rendering Liora message box.
pub struct MessageBox {
    title: SharedString,
    content: SharedString,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl MessageBox {
    /// Creates `MessageBox` initialized from the supplied title, and content.
    pub fn new(title: impl Into<SharedString>, content: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    /// Toggles whether the popup closes when click outside occurs.
    pub fn close_on_click_outside(mut self, c: bool) -> Self {
        self.close_on_click_outside = c;
        self
    }

    /// Toggles whether the popup closes when escape occurs.
    pub fn close_on_escape(mut self, c: bool) -> Self {
        self.close_on_escape = c;
        self
    }

    /// Performs the alert operation used by this component.
    pub fn alert(self, cx: &mut App) {
        let content = self.content.clone();
        Dialog::new()
            .title(self.title)
            .close_on_click_outside(self.close_on_click_outside)
            .close_on_escape(self.close_on_escape)
            .content(move |_, _| {
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(content.clone())
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .child(Button::new("OK").primary().on_click(|_, _, cx| {
                                Dialog::close(cx);
                            })),
                    )
            })
            .show(cx);
    }

    /// Performs the confirm operation used by this component.
    pub fn confirm(self, on_confirm: impl Fn(&mut Window, &mut App) + 'static, cx: &mut App) {
        let content = self.content.clone();
        let on_confirm = Arc::new(on_confirm);

        Dialog::new()
            .title(self.title)
            .close_on_click_outside(self.close_on_click_outside)
            .close_on_escape(self.close_on_escape)
            .content(move |_window, _cx| {
                let on_confirm = on_confirm.clone();
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(content.clone())
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .gap_2()
                            .child(Button::new("Cancel").on_click(|_, _, cx| {
                                Dialog::close(cx);
                            }))
                            .child(Button::new("Confirm").primary().on_click(
                                move |_, window, cx| {
                                    on_confirm(window, cx);
                                    Dialog::close(cx);
                                },
                            )),
                    )
            })
            .show(cx);
    }

    /// Performs the close operation used by this component.
    pub fn close(cx: &mut App) {
        Dialog::close(cx);
    }
}

/// Performs the close operation used by this component.
pub fn close(cx: &mut App) {
    MessageBox::close(cx);
}

/// Performs the alert operation used by this component.
pub fn alert(title: impl Into<SharedString>, content: impl Into<SharedString>, cx: &mut App) {
    MessageBox::new(title, content).alert(cx);
}

/// Performs the confirm operation used by this component.
pub fn confirm(
    title: impl Into<SharedString>,
    content: impl Into<SharedString>,
    on_confirm: impl Fn(&mut Window, &mut App) + 'static,
    cx: &mut App,
) {
    MessageBox::new(title, content).confirm(on_confirm, cx);
}
