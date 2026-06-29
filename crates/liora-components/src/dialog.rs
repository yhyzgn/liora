//! Dialog module.
//!
//! This public module implements the Liora modal dialog component with controlled close policies. It keeps the reusable
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

use crate::gpui_compat::element_id;
use crate::motion::{fade_in, pop_in};
use gpui::{
    AnyElement, App, Context, FocusHandle, Focusable, IntoElement, KeyBinding, MouseButton, Render,
    SharedString, Window, actions, div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::{collections::HashMap, sync::Arc};

type DialogCloseCallback = Arc<dyn Fn(&mut Window, &mut App) + 'static>;

#[derive(Default)]
struct ActiveDialogRuntime {
    close_on_escape: HashMap<SharedString, bool>,
    on_close: HashMap<SharedString, DialogCloseCallback>,
}

impl gpui::Global for ActiveDialogRuntime {}

struct DialogEscapeInterceptorInstalled;
impl gpui::Global for DialogEscapeInterceptorInstalled {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DialogEscapeDecision {
    Close(SharedString),
    Block,
    Ignore,
}

fn ensure_dialog_runtime(cx: &mut App) {
    if !cx.has_global::<ActiveDialogRuntime>() {
        cx.set_global(ActiveDialogRuntime::default());
    }

    if !cx.has_global::<DialogEscapeInterceptorInstalled>() {
        cx.intercept_keystrokes(|event, window, cx| {
            if event.keystroke.key == "escape" {
                close_top_dialog_from_window(window, cx);
            }
        })
        .detach();
        cx.set_global(DialogEscapeInterceptorInstalled);
    }
}

fn register_dialog_runtime(
    id: SharedString,
    close_on_escape: bool,
    on_close: DialogCloseCallback,
    cx: &mut App,
) {
    ensure_dialog_runtime(cx);
    let runtime = cx.global_mut::<ActiveDialogRuntime>();
    runtime.close_on_escape.insert(id.clone(), close_on_escape);
    runtime.on_close.insert(id, on_close);
}

fn unregister_dialog_runtime(id: &SharedString, cx: &mut App) {
    if !cx.has_global::<ActiveDialogRuntime>() {
        return;
    }
    let runtime = cx.global_mut::<ActiveDialogRuntime>();
    runtime.close_on_escape.remove(id);
    runtime.on_close.remove(id);
}

fn clear_dialog_runtime(cx: &mut App) {
    if !cx.has_global::<ActiveDialogRuntime>() {
        return;
    }
    let runtime = cx.global_mut::<ActiveDialogRuntime>();
    runtime.close_on_escape.clear();
    runtime.on_close.clear();
}

fn active_modal_top_id(cx: &App) -> Option<SharedString> {
    cx.try_global::<liora_core::ActiveModal>()
        .and_then(|modal| modal.0.last().map(|entry| entry.id.clone()))
}

fn dialog_escape_decision_for(
    runtime: Option<&ActiveDialogRuntime>,
    active_modal_id: Option<&SharedString>,
) -> DialogEscapeDecision {
    let Some(runtime) = runtime else {
        return DialogEscapeDecision::Ignore;
    };
    let Some(id) = active_modal_id else {
        return DialogEscapeDecision::Ignore;
    };
    if !runtime.close_on_escape.contains_key(id) && !runtime.on_close.contains_key(id) {
        return DialogEscapeDecision::Ignore;
    }
    if runtime.close_on_escape.get(id).copied().unwrap_or(true) {
        DialogEscapeDecision::Close(id.clone())
    } else {
        DialogEscapeDecision::Block
    }
}

fn dialog_escape_decision(cx: &App) -> DialogEscapeDecision {
    let active_modal_id = active_modal_top_id(cx);
    dialog_escape_decision_for(
        cx.try_global::<ActiveDialogRuntime>(),
        active_modal_id.as_ref(),
    )
}

fn close_dialog_by_id(id: SharedString, window: &mut Window, cx: &mut App) {
    let callback = cx
        .try_global::<ActiveDialogRuntime>()
        .and_then(|runtime| runtime.on_close.get(&id).cloned());
    if let Some(callback) = callback {
        callback(window, cx);
    } else {
        unregister_dialog_runtime(&id, cx);
        liora_core::clear_modal(&id, cx);
        cx.refresh_windows();
    }
}

fn close_top_dialog_from_window(window: &mut Window, cx: &mut App) {
    match dialog_escape_decision(cx) {
        DialogEscapeDecision::Close(id) => {
            close_dialog_by_id(id, window, cx);
            cx.stop_propagation();
        }
        DialogEscapeDecision::Block => {
            cx.stop_propagation();
        }
        DialogEscapeDecision::Ignore => {}
    }
}

fn close_top_dialog_if_escape_enabled(cx: &mut App) {
    match dialog_escape_decision(cx) {
        DialogEscapeDecision::Close(id) => {
            if let Some(window) = cx.active_window() {
                cx.defer(move |cx| {
                    let _ = window.update(cx, |_, window, cx| {
                        close_dialog_by_id(id, window, cx);
                    });
                });
            }
        }
        DialogEscapeDecision::Block => {}
        DialogEscapeDecision::Ignore => cx.propagate(),
    }
}

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
    animated: bool,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

/// Fluent native GPUI component for rendering Liora dialog view.
pub struct DialogView {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    animated: bool,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
    focus_handle: FocusHandle,
    focus_requested: bool,
}

impl DialogView {
    fn new(
        id: SharedString,
        title: SharedString,
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        close_on_click_outside: bool,
        close_on_escape: bool,
        animated: bool,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
        focus_handle: FocusHandle,
        focus_requested: bool,
    ) -> Self {
        Self {
            id,
            title,
            content,
            close_on_click_outside,
            close_on_escape,
            animated,
            on_close: Arc::new(on_close),
            focus_handle,
            focus_requested,
        }
    }
}

impl Focusable for DialogView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DialogView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = self.id.clone();
        let title = self.title.clone();
        let content_fn = self.content.clone();
        let on_close = self.on_close.clone();
        let on_close_for_close_button = on_close.clone();
        let on_close_for_outside_click = on_close.clone();
        let on_close_for_escape = on_close;
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;
        let animated = self.animated;
        let focus_handle = self.focus_handle(cx);
        if !self.focus_requested && animated {
            self.focus_requested = true;
            let focus_handle = focus_handle.clone();
            window.defer(cx, move |window, cx| window.focus(&focus_handle, cx));
        } else if !self.focus_requested {
            self.focus_requested = true;
        }

        let panel = div()
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
                                on_close_for_close_button(window, cx);
                            }),
                    ),
            )
            .child(
                div()
                    .p_4()
                    .min_w(px(0.0))
                    .text_color(theme.neutral.text_2)
                    .overflow_hidden()
                    .child(content_fn(window, cx)),
            );

        let overlay = div()
            .id(id.clone())
            .track_focus(&focus_handle)
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
                    let on_close = on_close_for_outside_click.clone();
                    move |_, window, cx| {
                        on_close(window, cx);
                    }
                })
            })
            .when(close_on_escape, |s| {
                s.on_action(cx.listener({
                    let on_close = on_close_for_escape.clone();
                    move |_, _action: &DialogClose, window, cx| {
                        on_close(window, cx);
                    }
                }))
            });

        if animated {
            fade_in(
                element_id(format!("{id}-overlay-motion")),
                overlay.child(pop_in(element_id(format!("{id}-panel-motion")), panel)),
            )
            .into_any_element()
        } else {
            overlay.child(panel).into_any_element()
        }
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

    #[test]
    fn dialog_can_skip_intro_motion_for_latency_sensitive_surfaces() {
        let source = include_str!("dialog.rs");
        let impl_source = source
            .rsplit("\nimpl Dialog {")
            .next()
            .expect("Dialog builder implementation should exist");

        assert!(source.contains("animated: bool"));
        assert!(source.contains("if animated {"));
        assert!(source.contains("overlay.child(panel).into_any_element()"));
        assert!(impl_source.contains("pub fn animated(mut self, animated: bool) -> Self"));
        assert!(impl_source.contains("pub fn immediate(self) -> Self"));
        assert!(impl_source.contains("self.animated(false)"));
        assert!(
            impl_source.contains("pub fn show_in_window(self, window: &mut Window, cx: &mut App)")
        );
        assert!(impl_source.contains("window.focus(&focus_handle, cx)"));
        assert!(impl_source.contains("window.refresh()"));
    }

    #[test]
    fn dialog_exposes_on_close_for_host_state_cleanup() {
        let source = include_str!("dialog.rs");
        let impl_source = source
            .rsplit("\nimpl Dialog {")
            .next()
            .expect("Dialog builder implementation should exist");

        assert!(source.contains("on_close: Arc<dyn Fn(&mut Window, &mut App)"));
        assert!(impl_source.contains("pub fn on_close("));
        assert!(impl_source.contains("user_on_close(window, cx);"));
    }

    #[test]
    fn dialog_supports_focus_and_global_escape_dismissal() {
        let full_source = include_str!("dialog.rs");
        let source = full_source.split("#[cfg(test)]").next().unwrap();

        assert!(source.contains("impl Focusable for DialogView"));
        assert!(source.contains("focus_handle: FocusHandle"));
        assert!(source.contains(".track_focus(&focus_handle)"));
        assert!(source.contains("!self.focus_requested && animated"));
        assert!(source.contains("window.defer(cx, move |window, cx|"));
        assert!(source.contains("window.focus(&focus_handle, cx)"));
        assert!(source.contains("s.on_action(cx.listener({"));
        assert!(source.contains("struct ActiveDialogRuntime"));
        assert!(source.contains("struct DialogEscapeInterceptorInstalled"));
        assert!(source.contains("cx.intercept_keystrokes"));
        assert!(source.contains(".detach();"));
        assert!(full_source.contains("cx.on_action(|_: &DialogClose, cx|"));
        assert!(source.contains("close_top_dialog_from_window"));
        assert!(source.contains("close_top_dialog_if_escape_enabled"));
        assert!(full_source.contains("unregister_dialog_runtime(&id_for_close, cx)"));
        assert!(full_source.contains("cx.refresh_windows();"));
    }
}

#[cfg(test)]
mod runtime_tests {
    use super::*;

    fn runtime_with(entries: &[(&str, bool)]) -> ActiveDialogRuntime {
        let mut runtime = ActiveDialogRuntime::default();
        for (id, close_on_escape) in entries {
            let id: SharedString = id.to_string().into();
            runtime.close_on_escape.insert(id, *close_on_escape);
        }
        runtime
    }

    #[test]
    fn dialog_escape_decision_ignores_without_active_dialog_match() {
        assert_eq!(
            dialog_escape_decision_for(None, Some(&"dialog-a".into())),
            DialogEscapeDecision::Ignore
        );

        let runtime = runtime_with(&[("dialog-a", true)]);
        assert_eq!(
            dialog_escape_decision_for(Some(&runtime), Some(&"tour-a".into())),
            DialogEscapeDecision::Ignore
        );
        assert_eq!(
            dialog_escape_decision_for(Some(&runtime), None),
            DialogEscapeDecision::Ignore
        );
    }

    #[test]
    fn dialog_escape_decision_respects_top_dialog_close_policy() {
        let runtime = runtime_with(&[("dialog-a", true), ("dialog-b", false)]);
        assert_eq!(
            dialog_escape_decision_for(Some(&runtime), Some(&"dialog-b".into())),
            DialogEscapeDecision::Block
        );
        assert_eq!(
            dialog_escape_decision_for(Some(&runtime), Some(&"dialog-a".into())),
            DialogEscapeDecision::Close("dialog-a".into())
        );

        let runtime = runtime_with(&[("dialog-a", false), ("dialog-b", true)]);
        assert_eq!(
            dialog_escape_decision_for(Some(&runtime), Some(&"dialog-b".into())),
            DialogEscapeDecision::Close("dialog-b".into())
        );
    }

    #[test]
    fn dialog_escape_decision_uses_active_modal_as_source_of_truth() {
        let runtime = runtime_with(&[("dialog-a", true), ("stale-dialog", false)]);

        assert_eq!(
            dialog_escape_decision_for(Some(&runtime), Some(&"dialog-a".into())),
            DialogEscapeDecision::Close("dialog-a".into())
        );
        assert_eq!(
            dialog_escape_decision_for(Some(&runtime), Some(&"tour-a".into())),
            DialogEscapeDecision::Ignore
        );
    }

    #[test]
    fn dialog_escape_interceptor_is_process_once_not_runtime_state() {
        let source = include_str!("dialog.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        let runtime_struct = source
            .split("struct ActiveDialogRuntime")
            .nth(1)
            .and_then(|part| {
                part.split("impl gpui::Global for ActiveDialogRuntime")
                    .next()
            })
            .expect("ActiveDialogRuntime should be defined before its Global impl");

        assert!(source.contains("struct DialogEscapeInterceptorInstalled"));
        assert!(source.contains("!cx.has_global::<DialogEscapeInterceptorInstalled>()"));
        assert!(source.contains("cx.set_global(DialogEscapeInterceptorInstalled)"));
        assert!(source.contains(".detach();"));
        assert!(!runtime_struct.contains("Subscription"));
        assert!(!runtime_struct.contains("escape_interceptor"));
        assert!(!runtime_struct.contains("stack"));
    }
}

impl Dialog {
    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        ensure_dialog_runtime(cx);
        cx.bind_keys([KeyBinding::new("escape", DialogClose, None)]);
        cx.on_action(|_: &DialogClose, cx| {
            close_top_dialog_if_escape_enabled(cx);
        });
    }

    /// Creates `Dialog` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            id: liora_core::unique_id("dialog"),
            title: SharedString::default(),
            content: Arc::new(|_, _| div().child("Dialog Content").into_any_element()),
            close_on_click_outside: true,
            close_on_escape: true,
            animated: true,
            on_close: Arc::new(|_, _| {}),
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

    /// Toggles whether the dialog uses its default intro motion.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Disables intro motion so latency-sensitive host flows can appear immediately.
    pub fn immediate(self) -> Self {
        self.animated(false)
    }

    /// Registers a callback invoked when the dialog is dismissed by its built-in close controls.
    pub fn on_close(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Arc::new(f);
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
        self.show_with_window(None, cx);
    }

    /// Shows the dialog from an existing window and focuses the dialog before the next render pass.
    pub fn show_in_window(self, window: &mut Window, cx: &mut App) {
        self.show_with_window(Some(window), cx);
    }

    fn show_with_window(self, focus_window: Option<&mut Window>, cx: &mut App) {
        let focus_requested = focus_window.is_some() || !self.animated;
        let id = self.id;
        let title = self.title;
        let content = self.content;
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;
        let animated = self.animated;
        let user_on_close = self.on_close;

        let id_for_close = id.clone();
        let close_callback: DialogCloseCallback = Arc::new(move |window, cx| {
            unregister_dialog_runtime(&id_for_close, cx);
            liora_core::clear_modal(&id_for_close, cx);
            user_on_close(window, cx);
            cx.refresh_windows();
        });
        let id_for_view = id.clone();
        let close_callback_for_view = close_callback.clone();
        let view = cx.new(move |cx| {
            let focus_handle = cx.focus_handle();
            DialogView::new(
                id_for_view.clone(),
                title,
                content,
                close_on_click_outside,
                close_on_escape,
                animated,
                move |window, cx| {
                    close_callback_for_view(window, cx);
                },
                focus_handle,
                focus_requested,
            )
        });
        let view_for_focus = view.clone();

        register_dialog_runtime(id.clone(), close_on_escape, close_callback, cx);
        liora_core::set_active_modal(id, view.into(), cx);
        if let Some(window) = focus_window {
            let focus_handle = view_for_focus.read(cx).focus_handle(cx);
            window.focus(&focus_handle, cx);
            window.refresh();
        }
        cx.refresh_windows();
    }

    /// Performs the close operation used by this component.
    pub fn close(cx: &mut App) {
        clear_dialog_runtime(cx);
        liora_core::clear_active_modal(cx);
        cx.refresh_windows();
    }

    /// Performs the close id operation used by this component.
    pub fn close_id(id: impl Into<SharedString>, cx: &mut App) {
        let id = id.into();
        unregister_dialog_runtime(&id, cx);
        liora_core::clear_modal(&id, cx);
        cx.refresh_windows();
    }
}
