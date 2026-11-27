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
use liora_core::{ActiveModal, Config};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

struct DialogKeyBindingsInstalled;
impl gpui::Global for DialogKeyBindingsInstalled {}

fn ensure_dialog_key_bindings(cx: &mut App) {
    if !cx.has_global::<DialogKeyBindingsInstalled>() {
        cx.bind_keys([
            KeyBinding::new("escape", DialogClose, Some("Dialog")),
            KeyBinding::new("escape", DialogClose, None),
        ]);
        cx.on_action(|_: &DialogClose, cx| close_top_dialog_if_escape_enabled(cx));
        cx.set_global(DialogKeyBindingsInstalled);
    }
}

fn active_modal_for_window(cx: &App, window: gpui::AnyWindowHandle) -> Option<gpui::AnyView> {
    cx.try_global::<ActiveModal>()?
        .0
        .iter()
        .rev()
        .find(|entry| {
            liora_core::active_modal_owner(&entry.id, cx).is_none_or(|owner| owner == window)
        })
        .map(|entry| entry.view.clone())
}

fn close_top_dialog_from_window(window: &mut Window, cx: &mut App) {
    let Some(modal) = active_modal_for_window(cx, window.window_handle()) else {
        cx.propagate();
        return;
    };
    let Ok(dialog) = modal.downcast::<DialogView>() else {
        cx.propagate();
        return;
    };
    let dialog = dialog.read(cx);
    if dialog.close_on_escape {
        let on_close = dialog.on_close.clone();
        on_close(window, cx);
    }
    cx.stop_propagation();
}

fn close_top_dialog_if_escape_enabled(cx: &mut App) {
    let Some(window) = cx.active_window() else {
        cx.propagate();
        return;
    };
    let Some(modal) = active_modal_for_window(cx, window) else {
        cx.propagate();
        return;
    };
    let Ok(dialog) = modal.downcast::<DialogView>() else {
        cx.propagate();
        return;
    };
    let dialog = dialog.read(cx);
    if dialog.close_on_escape {
        let on_close = dialog.on_close.clone();
        cx.defer(move |cx| {
            let _ = window.update(cx, |_, window, cx| on_close(window, cx));
        });
    }
    cx.stop_propagation();
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
    close_id: SharedString,
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
            close_id: liora_core::unique_id("dialog-close"),
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
        let on_close_for_close_button = self.on_close.clone();
        let on_close_for_outside_click = self.on_close.clone();
        let close_on_click_outside = self.close_on_click_outside;
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
                            .id(element_id(self.close_id.clone()))
                            .accessibility_id(self.close_id.clone())
                            .debug_selector(|| "dialog-title-close".to_string())
                            .tab_index(0)
                            .role(gpui::Role::Button)
                            .aria_label("Dismiss dialog")
                            .border_2()
                            .border_color(theme.primary.base.opacity(0.0))
                            .focus_visible(move |style| style.border_color(theme.primary.base))
                            .cursor_pointer()
                            .child(
                                Icon::new(IconName::X)
                                    .size(px(16.0))
                                    .color(theme.neutral.icon),
                            )
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
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
                                } else if !modifiers.modified()
                                    && matches!(event.keystroke.key.as_str(), "enter" | "space")
                                {
                                    cx.stop_propagation();
                                }
                            })
                            .on_click(move |_, window, cx| {
                                on_close_for_close_button(window, cx);
                                cx.stop_propagation();
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
            .key_context("Dialog")
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
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                cx.stop_propagation();
                if close_on_click_outside {
                    on_close_for_outside_click(window, cx);
                }
            })
            .on_action(|_: &DialogClose, window, cx| {
                close_top_dialog_from_window(window, cx);
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
}

#[cfg(test)]
mod runtime_tests {
    use super::*;
    use gpui::{Context, Render, px};

    fn active_empty_window(app: &mut gpui::TestAppContext) -> &mut gpui::VisualTestContext {
        // No portal here: exercise the global fallback, which uses App::active_window().
        let window = app.add_empty_window();
        window.update(|window, _| window.activate_window());
        window.run_until_parked();
        assert!(window.update(|window, cx| cx.active_window() == Some(window.window_handle())));
        window
    }

    fn escape_propagates(window: &mut gpui::VisualTestContext) -> bool {
        window.update(|window, cx| {
            let keystroke =
                gpui::Keystroke::parse_with_os("escape", cx.platform_info().os).unwrap();
            let down = window.dispatch_event(
                gpui::PlatformInput::KeyDown(gpui::KeyDownEvent {
                    keystroke: keystroke.clone(),
                    is_held: false,
                    prefer_character_input: false,
                }),
                cx,
            );
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
            down.propagate
        })
    }

    #[test]
    fn escape_respects_the_top_dialog_close_policy() {
        let mut app = gpui::TestAppContext::single();
        let window = active_empty_window(&mut app);
        window.update(|_, cx| {
            liora_core::init_liora(cx, liora_theme::Theme::light());
            Dialog::new().id("underlying").show(cx);
            Dialog::new().id("top").close_on_escape(false).show(cx);
        });

        assert!(
            !escape_propagates(window),
            "a disabled top dialog still owns Escape"
        );
        window.update(|_, cx| {
            assert_eq!(cx.global::<ActiveModal>().0.len(), 2);
            Dialog::close_id("top", cx);
        });

        assert!(!escape_propagates(window));
        window.update(|_, cx| assert!(cx.global::<ActiveModal>().0.is_empty()));
        app.quit();
    }

    #[test]
    fn escape_does_not_close_a_replacement_modal_with_the_same_id() {
        let mut app = gpui::TestAppContext::single();
        let window = active_empty_window(&mut app);
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

        assert!(escape_propagates(window), "the replacement is not a Dialog");

        assert_eq!(close_count.get(), 0, "dismissed the replaced dialog");
        window.update(|_, cx| {
            let top = cx.global::<ActiveModal>().0.last().unwrap();
            assert_eq!(top.view.entity_id(), replacement_id);
        });
        app.quit();
    }

    #[test]
    fn dialog_escape_bindings_install_once_and_show_needs_only_core_init() {
        let mut app = gpui::TestAppContext::single();
        let window = active_empty_window(&mut app);
        let calls = std::rc::Rc::new(std::cell::Cell::new(0));
        window.update(|_, cx| {
            liora_core::init_liora(cx, liora_theme::Theme::light());
            let calls = calls.clone();
            Dialog::new()
                .on_close(move |_, _| calls.set(calls.get() + 1))
                .show(cx);
        });
        assert!(!escape_propagates(window));
        assert_eq!(
            calls.get(),
            1,
            "show must install Escape without components::init"
        );
        window.update(|_, cx| {
            let calls = calls.clone();
            Dialog::new()
                .on_close(move |_, _| calls.set(calls.get() + 1))
                .show(cx);
            Dialog::register_key_bindings(cx);
            Dialog::register_key_bindings(cx);
            assert_eq!(
                cx.key_bindings()
                    .borrow()
                    .bindings_for_action(&DialogClose)
                    .count(),
                2,
                "register only the scoped and fallback bindings, once each"
            );
        });
        assert!(!escape_propagates(window));
        assert_eq!(calls.get(), 2);
        window.update(|_, cx| assert!(cx.global::<ActiveModal>().0.is_empty()));
        assert!(escape_propagates(window), "no Dialog remains to own Escape");
        assert_eq!(
            calls.get(),
            2,
            "no stale callback after the second dialog closes"
        );
        app.quit();
    }

    struct ModalSurface;

    impl Render for ModalSurface {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            liora_core::render_active_modal_in_window(window, cx);
            div().relative().size_full().child(crate::PortalLayer)
        }
    }

    #[test]
    fn manual_close_removes_only_top_dialog() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, _| ModalSurface);
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        let close_callback_count = std::rc::Rc::new(std::cell::Cell::new(0));

        window.update(|window, cx| {
            Dialog::close(cx);
            assert!(cx.global::<ActiveModal>().0.is_empty());

            Dialog::new().id("parent").show(cx);
            Dialog::new()
                .id("child")
                .on_close({
                    let close_callback_count = close_callback_count.clone();
                    move |_, _| close_callback_count.set(close_callback_count.get() + 1)
                })
                .content(|_, _| {
                    div()
                        .debug_selector(|| "dialog-manual-close".into())
                        .child(crate::Button::new("Close").on_click(|_, _, cx| Dialog::close(cx)))
                })
                .show(cx);
            let modals = &cx.global::<ActiveModal>().0;
            assert_eq!(modals.len(), 2);
            assert_eq!(modals[0].id, "parent");
            assert_eq!(modals[1].id, "child");
            window.draw(cx).clear(cx);
        });

        let close_bounds = window
            .debug_bounds("dialog-manual-close")
            .expect("manual close button must render");
        window.simulate_click(close_bounds.center(), gpui::Modifiers::none());

        window.update(|_, cx| {
            let modals = &cx.global::<ActiveModal>().0;
            assert_eq!(modals.len(), 1);
            assert_eq!(modals[0].id, "parent");
        });

        assert_eq!(close_callback_count.get(), 0);

        window.update(|_, cx| {
            crate::MessageBox::new("Message box", "Dismiss me").alert(cx);
            crate::MessageBox::close(cx);
            let modals = &cx.global::<ActiveModal>().0;
            assert_eq!(modals.len(), 1);
            assert_eq!(modals[0].id, "parent");
        });
        assert_eq!(close_callback_count.get(), 0);
        app.quit();
    }

    struct KeyboardModalSurface(std::rc::Rc<std::cell::Cell<usize>>);

    impl Render for KeyboardModalSurface {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            liora_core::render_active_modal_in_window(window, cx);
            let key_actions = self.0.clone();
            let clicks = self.0.clone();
            crate::AppWindowFrame::new(
                "Dialog keyboard test",
                div()
                    .id("dialog-keyboard-parent")
                    .size_full()
                    .on_key_down(move |event, _, _| {
                        if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                            key_actions.set(key_actions.get() + 1);
                        }
                    })
                    .on_click(move |_, _, _| clicks.set(clicks.get() + 1))
                    .child(crate::PortalLayer),
            )
        }
    }

    #[test]
    fn title_close_is_tab_reachable_and_activates_once_without_bubbling() {
        use std::{cell::Cell, rc::Rc};

        for key in ["enter", "space", "pointer"] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let parent_actions = Rc::new(Cell::new(0));
            let root_actions = parent_actions.clone();
            let handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, _| {
                KeyboardModalSurface(root_actions)
            });
            let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
            let calls = Rc::new(Cell::new(0));
            window.update(|window, cx| {
                Dialog::new()
                    .id("parent")
                    .animated(false)
                    .show_in_window(window, cx);
                Dialog::new()
                    .id("keyboard-child")
                    .animated(false)
                    .close_on_escape(false)
                    .close_on_click_outside(false)
                    .on_close({
                        let calls = calls.clone();
                        move |_, _| calls.set(calls.get() + 1)
                    })
                    .content(|_, _| crate::Button::new("Other action"))
                    .show_in_window(window, cx);
                window.draw(cx).clear(cx);
            });
            let bounds = window
                .debug_bounds("dialog-title-close")
                .expect("title close bounds");
            window.simulate_keystrokes("escape");
            window.simulate_click(gpui::point(px(4.0), px(4.0)), gpui::Modifiers::none());
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert_eq!(calls.get(), 0, "disabled Escape/outside must not close");
            window.update(|_, cx| {
                let modals = &cx.global::<ActiveModal>().0;
                assert_eq!(
                    modals.len(),
                    2,
                    "disabled outside must preserve both dialogs"
                );
                assert_eq!(modals[0].id, "parent");
                assert_eq!(modals[1].id, "keyboard-child");
            });
            window.simulate_keystrokes("tab");
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert_eq!(window.debug_bounds("dialog-title-close"), Some(bounds));
            let close_focus = window
                .update(|window, cx| window.focused(cx))
                .expect("Tab must focus the close control");
            window.simulate_keystrokes("tab");
            window.update(|window, cx| {
                window.draw(cx).clear(cx);
                assert!(window.focused(cx).is_some());
                assert!(
                    window.focused(cx) != Some(close_focus.clone()),
                    "Tab must leave close"
                );
            });
            window.simulate_keystrokes("shift-tab");
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert_eq!(window.debug_bounds("dialog-title-close"), Some(bounds));
            window.update(|window, cx| {
                assert!(
                    window.focused(cx) == Some(close_focus.clone()),
                    "Shift+Tab must return to close"
                );
            });
            if key == "pointer" {
                window.simulate_click(bounds.center(), gpui::Modifiers::none());
            } else {
                window.simulate_keystrokes(key);
                assert_eq!(calls.get(), 0, "{key}: key-down must not close");
                window.update(|window, cx| {
                    let keystroke =
                        gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
                    window.dispatch_event(
                        gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                        cx,
                    );
                    window.draw(cx).clear(cx);
                });
            }
            assert_eq!(
                calls.get(),
                1,
                "{key}: Tab must reach the title close control"
            );
            assert_eq!(parent_actions.get(), 0, "activation must not bubble");
            window.update(|_, cx| {
                let modals = &cx.global::<ActiveModal>().0;
                assert_eq!(modals.len(), 1);
                assert_eq!(modals[0].id, "parent");
            });
            app.quit();
        }
    }

    #[test]
    fn outside_click_is_consumed_by_only_the_top_dialog() {
        use std::{cell::Cell, rc::Rc};

        for child_closes_on_outside in [false, true] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, _| {
                KeyboardModalSurface(Rc::new(Cell::new(0)))
            });
            let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
            let parent_calls = Rc::new(Cell::new(0));
            let child_calls = Rc::new(Cell::new(0));
            window.update(|window, cx| {
                Dialog::new()
                    .id("outside-parent")
                    .animated(false)
                    .on_close({
                        let calls = parent_calls.clone();
                        move |_, _| calls.set(calls.get() + 1)
                    })
                    .show_in_window(window, cx);
                Dialog::new()
                    .id("outside-child")
                    .animated(false)
                    .close_on_click_outside(child_closes_on_outside)
                    .on_close({
                        let calls = child_calls.clone();
                        move |_, _| calls.set(calls.get() + 1)
                    })
                    .show_in_window(window, cx);
                window.draw(cx).clear(cx);
            });
            window.simulate_click(gpui::point(px(4.0), px(4.0)), gpui::Modifiers::none());
            assert_eq!(parent_calls.get(), 0, "outside must never reach the parent");
            assert_eq!(child_calls.get(), usize::from(child_closes_on_outside));
            window.update(|_, cx| {
                let modals = &cx.global::<ActiveModal>().0;
                assert_eq!(modals.len(), if child_closes_on_outside { 1 } else { 2 });
                assert_eq!(modals[0].id, "outside-parent");
                if !child_closes_on_outside {
                    assert_eq!(modals[1].id, "outside-child");
                }
            });
            app.quit();
        }
    }
    #[test]
    fn modal_escape_and_rendering_follow_their_window_owner() {
        use std::{cell::Cell, rc::Rc};

        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let owner_handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, _| ModalSurface);
        let other_handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, _| ModalSurface);
        let mut owner = gpui::VisualTestContext::from_window(owner_handle.into(), &app);
        let mut other = gpui::VisualTestContext::from_window(other_handle.into(), &app);
        let owner_closes = Rc::new(Cell::new(0));
        let other_closes = Rc::new(Cell::new(0));

        owner.update(|window, cx| {
            Dialog::new()
                .id("owner-dialog")
                .animated(false)
                .content(|_, _| {
                    gpui::div()
                        .debug_selector(|| "owner-dialog-content".to_string())
                        .child("A")
                })
                .on_close({
                    let closes = owner_closes.clone();
                    move |_, _| closes.set(closes.get() + 1)
                })
                .show_in_window(window, cx);
            window.draw(cx).clear(cx);
        });
        other.update(|window, cx| {
            Dialog::new()
                .id("other-dialog")
                .animated(false)
                .content(|_, _| {
                    gpui::div()
                        .debug_selector(|| "other-dialog-content".to_string())
                        .child("B")
                })
                .on_close({
                    let closes = other_closes.clone();
                    move |_, _| closes.set(closes.get() + 1)
                })
                .show_in_window(window, cx);
            window.draw(cx).clear(cx);
        });

        assert!(owner.debug_bounds("owner-dialog-content").is_some());
        assert_eq!(owner.debug_bounds("other-dialog-content"), None);
        assert!(other.debug_bounds("other-dialog-content").is_some());
        assert_eq!(
            other.debug_bounds("owner-dialog-content"),
            None,
            "the other window must not render the owner's modal"
        );

        owner.update(|window, _| window.activate_window());
        assert!(
            !escape_propagates(&mut owner),
            "Escape must close A's modal rather than ignore B's top entry"
        );
        assert_eq!(owner_closes.get(), 1);
        assert_eq!(other_closes.get(), 0);

        owner.update(|window, cx| {
            Dialog::new()
                .id("owner-disabled-dialog")
                .animated(false)
                .close_on_escape(false)
                .show_in_window(window, cx);
            window.draw(cx).clear(cx);
            window.activate_window();
        });
        assert!(
            !escape_propagates(&mut owner),
            "a disabled top Dialog must own Escape"
        );
        assert_eq!(owner_closes.get(), 1);
        assert_eq!(other_closes.get(), 0);

        owner.update(|window, cx| {
            let blocker = cx.new(|_| gpui::Empty);
            liora_core::set_active_modal_in_window(
                "owner-non-dialog".into(),
                blocker.into(),
                Some(window.window_handle()),
                cx,
            );
            window.draw(cx).clear(cx);
            window.activate_window();
        });
        assert!(
            escape_propagates(&mut owner),
            "a non-Dialog top modal keeps its own Escape policy"
        );
        owner.update(|_, cx| {
            let ids: Vec<_> = cx
                .global::<ActiveModal>()
                .0
                .iter()
                .map(|entry| entry.id.as_ref())
                .collect();
            assert!(ids.contains(&"owner-disabled-dialog"));
            assert!(ids.contains(&"owner-non-dialog"));
            assert!(ids.contains(&"other-dialog"));
        });

        other.update(|window, _| window.activate_window());
        assert!(
            !escape_propagates(&mut other),
            "owner A's non-Dialog must not block owner B"
        );
        assert_eq!(other_closes.get(), 1);
        assert_eq!(owner_closes.get(), 1);

        owner.update(|window, _| window.remove_window());
        other.update(|window, cx| {
            window.activate_window();
            window.draw(cx).clear(cx);
            assert!(
                cx.global::<ActiveModal>().0.is_empty(),
                "closing owner A must clear its modal and window-scoped Escape blocker"
            );
        });
        assert!(
            escape_propagates(&mut other),
            "closed owners must not leave stale modal bindings"
        );
        app.quit();
    }

    #[test]
    fn manual_close_targets_the_active_windows_top_modal() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let first_handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, _| ModalSurface);
        let second_handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, _| ModalSurface);
        let mut first = gpui::VisualTestContext::from_window(first_handle.into(), &app);
        let mut second = gpui::VisualTestContext::from_window(second_handle.into(), &app);
        first.update(|window, cx| {
            Dialog::new()
                .id("first-parent")
                .immediate()
                .show_in_window(window, cx);
            Dialog::new()
                .id("first-child")
                .immediate()
                .content(|_, _| {
                    div()
                        .debug_selector(|| "first-window-close".into())
                        .child(crate::Button::new("Close").on_click(|_, _, cx| Dialog::close(cx)))
                })
                .show_in_window(window, cx);
            window.draw(cx).clear(cx);
        });
        second.update(|window, cx| {
            Dialog::new()
                .id("second-dialog")
                .immediate()
                .show_in_window(window, cx);
            window.draw(cx).clear(cx);
        });
        first.update(|window, _| window.activate_window());
        first.run_until_parked();
        first.update(|window, cx| assert_eq!(cx.active_window(), Some(window.window_handle())));
        let close = first
            .debug_bounds("first-window-close")
            .expect("first window's business Close");
        first.simulate_click(close.center(), gpui::Modifiers::none());
        first.update(|_, cx| {
            let ids: Vec<_> = cx
                .global::<ActiveModal>()
                .0
                .iter()
                .map(|entry| entry.id.as_ref())
                .collect();
            assert_eq!(
                ids,
                ["first-parent", "second-dialog"],
                "business Close in A must not close B's newer modal"
            );
        });
        app.quit();
    }
}

impl Dialog {
    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        ensure_dialog_key_bindings(cx);
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
        let owner_window = focus_window
            .as_ref()
            .map(|window| window.window_handle())
            .or_else(|| cx.active_window());
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

        ensure_dialog_key_bindings(cx);
        liora_core::set_active_modal_in_window(id, view.into(), owner_window, cx);
        if let Some(window) = focus_window {
            let focus_handle = view_for_focus.read(cx).focus_handle(cx);
            window.focus(&focus_handle, cx);
            window.refresh();
        }
        cx.refresh_windows();
    }

    /// Closes the active window's topmost modal without invoking its built-in `on_close` callback.
    /// Ownerless modals remain eligible; without an active window, closes the app-wide top modal.
    pub fn close(cx: &mut App) {
        let active_window = cx.active_window();
        if let Some(id) = cx
            .try_global::<ActiveModal>()
            .and_then(|modal| {
                modal.0.iter().rev().find(|entry| {
                    active_window.is_none_or(|window| {
                        liora_core::active_modal_owner(&entry.id, cx)
                            .is_none_or(|owner| owner == window)
                    })
                })
            })
            .map(|entry| entry.id.clone())
        {
            liora_core::clear_modal(&id, cx);
        }
        cx.refresh_windows();
    }

    /// Performs the close id operation used by this component.
    pub fn close_id(id: impl Into<SharedString>, cx: &mut App) {
        let id = id.into();
        liora_core::clear_modal(&id, cx);
        cx.refresh_windows();
    }
}
