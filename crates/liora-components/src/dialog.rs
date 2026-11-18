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
    AnyElement, App, Context, Entity, FocusHandle, Focusable, IntoElement, KeyBinding, MouseButton,
    Render, SharedString, Window, actions, div, prelude::*, px,
};
use liora_core::{ActiveModal, Config};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

struct DialogEscapeInterceptorInstalled;
impl gpui::Global for DialogEscapeInterceptorInstalled {}

fn ensure_dialog_escape_interceptor(cx: &mut App) {
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

fn active_dialog(cx: &App) -> Option<Entity<DialogView>> {
    // A modal ID can be reused by another overlay; only the current view owns its policy.
    cx.try_global::<ActiveModal>()?
        .0
        .last()?
        .view
        .clone()
        .downcast::<DialogView>()
        .ok()
}

fn close_top_dialog_from_window(window: &mut Window, cx: &mut App) {
    if let Some(dialog) = active_dialog(cx) {
        let dialog = dialog.read(cx);
        if dialog.close_on_escape {
            let on_close = dialog.on_close.clone();
            on_close(window, cx);
        }
        cx.stop_propagation();
    }
}

fn close_top_dialog_if_escape_enabled(cx: &mut App) {
    let Some(dialog) = active_dialog(cx) else {
        cx.propagate();
        return;
    };
    let Some(window) = cx.active_window() else {
        return;
    };
    let dialog = dialog.read(cx);
    if dialog.close_on_escape {
        let on_close = dialog.on_close.clone();
        cx.defer(move |cx| {
            let _ = window.update(cx, |_, window, cx| on_close(window, cx));
        });
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
        assert!(source.contains("struct DialogEscapeInterceptorInstalled"));
        assert!(source.contains("cx.intercept_keystrokes"));
        assert!(source.contains(".detach();"));
        assert!(full_source.contains("cx.on_action(|_: &DialogClose, cx|"));
        assert!(source.contains("close_top_dialog_from_window"));
        assert!(source.contains("close_top_dialog_if_escape_enabled"));
        assert!(full_source.contains("cx.refresh_windows();"));
    }
}

#[cfg(test)]
mod runtime_tests {
    use super::*;

    #[test]
    fn escape_respects_the_top_dialog_close_policy() {
        let mut app = gpui::TestAppContext::single();
        let window = app.add_empty_window();
        window.update(|_, cx| {
            liora_core::init_liora(cx, liora_theme::Theme::light());
            Dialog::new().id("underlying").show(cx);
            Dialog::new().id("top").close_on_escape(false).show(cx);
        });

        window.simulate_keystrokes("escape");
        window.update(|_, cx| {
            assert_eq!(cx.global::<ActiveModal>().0.len(), 2);
            Dialog::close_id("top", cx);
        });

        window.simulate_keystrokes("escape");
        window.update(|_, cx| assert!(cx.global::<ActiveModal>().0.is_empty()));
        app.quit();
    }

    #[test]
    fn escape_does_not_close_a_replacement_modal_with_the_same_id() {
        let mut app = gpui::TestAppContext::single();
        let window = app.add_empty_window();
        let close_count = std::rc::Rc::new(std::cell::Cell::new(0));
        let replacement_id = window.update(|_, cx| {
            liora_core::init_liora(cx, liora_theme::Theme::light());
            Dialog::new()
                .id("shared-modal")
                .on_close({
                    let close_count = close_count.clone();
                    move |_, _| close_count.set(close_count.get() + 1)
                })
                .show(cx);

            let replacement = cx.new(|_| gpui::Empty);
            let entity_id = replacement.entity_id();
            liora_core::set_active_modal("shared-modal".into(), replacement.into(), cx);
            entity_id
        });

        window.simulate_keystrokes("escape");

        assert_eq!(close_count.get(), 0, "dismissed the replaced dialog");
        window.update(|_, cx| {
            let top = cx.global::<ActiveModal>().0.last().unwrap();
            assert_eq!(top.view.entity_id(), replacement_id);
        });
        app.quit();
    }

    #[test]
    fn dialog_escape_interceptor_is_process_once_not_runtime_state() {
        let source = include_str!("dialog.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        assert!(source.contains("struct DialogEscapeInterceptorInstalled"));
        assert!(source.contains("!cx.has_global::<DialogEscapeInterceptorInstalled>()"));
        assert!(source.contains("cx.set_global(DialogEscapeInterceptorInstalled)"));
        assert!(source.contains(".detach();"));
    }
}

impl Dialog {
    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        ensure_dialog_escape_interceptor(cx);
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

        let id_for_view = id.clone();
        let view = cx.new(move |cx| {
            let focus_handle = cx.focus_handle();
            let view_id = cx.entity_id();
            let id_for_close = id_for_view.clone();
            DialogView::new(
                id_for_view,
                title,
                content,
                close_on_click_outside,
                close_on_escape,
                animated,
                move |window, cx| {
                    // A handler from an older frame may outlive its modal.
                    if !cx.try_global::<ActiveModal>().is_some_and(|modal| {
                        modal.0.iter().any(|entry| {
                            entry.id == id_for_close && entry.view.entity_id() == view_id
                        })
                    }) {
                        return;
                    }
                    liora_core::clear_modal(&id_for_close, cx);
                    user_on_close(window, cx);
                    cx.refresh_windows();
                },
                focus_handle,
                focus_requested,
            )
        });
        let view_for_focus = view.clone();

        ensure_dialog_escape_interceptor(cx);
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
        liora_core::clear_active_modal(cx);
        cx.refresh_windows();
    }

    /// Performs the close id operation used by this component.
    pub fn close_id(id: impl Into<SharedString>, cx: &mut App) {
        let id = id.into();
        liora_core::clear_modal(&id, cx);
        cx.refresh_windows();
    }
}
