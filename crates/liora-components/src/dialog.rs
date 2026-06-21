//! Dialog module.
//!
//! This public module implements the Liora modal dialog component with controlled close policies. It keeps the reusable
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
use crate::motion::{fade_in, pop_in};
use gpui::{
    AnyElement, App, Context, IntoElement, KeyBinding, MouseButton, Render, SharedString, Window,
    actions, div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

actions!(
    dialog,
    [
        #[doc = "Keyboard action that closes the active dialog when dismissal is allowed."]
        DialogClose
    ]
);

/// Fluent native GPUI component for rendering Liora dialog.
pub struct Dialog {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<DialogView>) -> AnyElement + 'static>,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

/// Fluent native GPUI component for rendering Liora dialog view.
pub struct DialogView {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl DialogView {
    fn new(
        id: SharedString,
        title: SharedString,
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        close_on_click_outside: bool,
        close_on_escape: bool,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id,
            title,
            content,
            close_on_click_outside,
            close_on_escape,
            on_close: Arc::new(on_close),
        }
    }
}

impl Render for DialogView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = self.id.clone();
        let title = self.title.clone();
        let content_fn = self.content.clone();
        let on_close = self.on_close.clone();
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;

        fade_in(
            element_id(format!("{id}-overlay-motion")),
            div()
                .id(id.clone())
                .absolute()
                .size_full()
                .cursor_default()
                .bg(theme.neutral.overlay)
                .flex()
                .items_center()
                .justify_center()
                .on_mouse_move(|_, _, cx| {
                    cx.stop_propagation();
                })
                .when(close_on_click_outside, |s| {
                    s.on_mouse_down(MouseButton::Left, {
                        let on_close = on_close.clone();
                        move |_, window, cx| {
                            on_close(window, cx);
                        }
                    })
                })
                .when(close_on_escape, |s| {
                    s.on_action(cx.listener({
                        let on_close = on_close.clone();
                        move |_, _action: &DialogClose, window, cx| {
                            on_close(window, cx);
                        }
                    }))
                })
                .child(pop_in(
                    element_id(format!("{id}-panel-motion")),
                    div()
                        .w_full()
                        .max_w(px(420.0))
                        .min_w(px(0.0))
                        .mx_4()
                        .bg(theme.neutral.card)
                        .cursor_default()
                        .rounded(px(theme.radius.md))
                        .shadow_xl()
                        .on_mouse_move(|_, _, cx| {
                            cx.stop_propagation();
                        })
                        .on_mouse_down(MouseButton::Left, |_, _, cx| {
                            cx.stop_propagation();
                        }) // Consume click so it doesn't trigger the background
                        .child(
                            div()
                                .p_4()
                                .min_w(px(0.0))
                                .border_b_1()
                                .border_color(theme.neutral.border)
                                .flex()
                                .justify_between()
                                .items_center()
                                .child(
                                    div()
                                        .min_w(px(0.0))
                                        .flex_1()
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .text_color(theme.neutral.text_1)
                                        .whitespace_normal()
                                        .child(title),
                                )
                                .child(
                                    div()
                                        .id(element_id(format!("{id}-close-btn")))
                                        .cursor_pointer()
                                        .child(
                                            Icon::new(IconName::X)
                                                .size(px(16.0))
                                                .color(theme.neutral.icon),
                                        )
                                        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                            on_close(window, cx);
                                        }),
                                ),
                        )
                        .child(
                            div()
                                .p_4()
                                .min_w(px(0.0))
                                .text_color(theme.neutral.text_2)
                                .overflow_hidden()
                                .child(content_fn(_window, cx)),
                        ),
                )),
        )
    }
}

#[cfg(test)]
mod motion_tests {
    #[test]
    fn dialog_panel_is_responsive_and_text_wraps() {
        let source = include_str!("dialog.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains(".w_full()"));
        assert!(source.contains(".max_w(px(420.0))"));
        assert!(source.contains(".min_w(px(0.0))"));
        assert!(source.contains(".mx_4()"));
        assert!(source.contains(".overflow_hidden()"));
        assert!(source.contains(".whitespace_normal()"));
    }

    #[test]
    fn dialog_uses_liora_motion_on_overlay_and_panel() {
        let source = include_str!("dialog.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("fade_in("));
        assert!(source.contains("pop_in("));
        assert!(source.contains("panel-motion"));
    }
}

impl Dialog {
    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", DialogClose, None)]);
    }

    /// Creates `Dialog` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            id: liora_core::unique_id("dialog"),
            title: SharedString::default(),
            content: Arc::new(|_, _| div().child("Dialog Content").into_any_element()),
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the primary title text displayed by the component.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
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

    /// Sets the rendered content element or text for this component.
    pub fn content<F, E>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<DialogView>) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }

    /// Performs the show operation used by this component.
    pub fn show(self, cx: &mut App) {
        let id = self.id;
        let title = self.title;
        let content = self.content;
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;

        let id_for_close = id.clone();
        let view = cx.new(|_cx| {
            DialogView::new(
                id.clone(),
                title,
                content,
                close_on_click_outside,
                close_on_escape,
                move |_window, _cx| {
                    liora_core::clear_modal(&id_for_close, _cx);
                },
            )
        });

        liora_core::set_active_modal(id, view.into(), cx);
    }

    /// Performs the close operation used by this component.
    pub fn close(cx: &mut App) {
        liora_core::clear_active_modal(cx);
    }

    /// Performs the close id operation used by this component.
    pub fn close_id(id: impl Into<SharedString>, cx: &mut App) {
        let id = id.into();
        liora_core::clear_modal(&id, cx);
    }
}
