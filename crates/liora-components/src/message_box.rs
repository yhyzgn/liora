//! Message Box module.
//!
//! This public module implements the Liora message-box confirmation/dialog helpers. It keeps the reusable
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

use crate::{Button, Dialog};
use gpui::{App, Window, div, prelude::*};
use liora_core::{LocalizedText, locales};
use std::sync::Arc;

/// Fluent native GPUI component for rendering Liora message box.
pub struct MessageBox {
    title: LocalizedText,
    content: LocalizedText,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl MessageBox {
    /// Creates `MessageBox` initialized from the supplied title, and content.
    pub fn new(title: impl Into<LocalizedText>, content: impl Into<LocalizedText>) -> Self {
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
            .title(self.title.resolve(cx))
            .close_on_click_outside(self.close_on_click_outside)
            .close_on_escape(self.close_on_escape)
            .content(move |_, cx| {
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(content.resolve(cx))
                    .child(
                        div().flex().justify_end().child(
                            Button::new(locales::message_box::ok)
                                .primary()
                                .on_click(|_, _, cx| {
                                    Dialog::close(cx);
                                }),
                        ),
                    )
            })
            .show(cx);
    }

    /// Performs the confirm operation used by this component.
    pub fn confirm(self, on_confirm: impl Fn(&mut Window, &mut App) + 'static, cx: &mut App) {
        let content = self.content.clone();
        let on_confirm = Arc::new(on_confirm);

        Dialog::new()
            .title(self.title.resolve(cx))
            .close_on_click_outside(self.close_on_click_outside)
            .close_on_escape(self.close_on_escape)
            .content(move |_window, cx| {
                let on_confirm = on_confirm.clone();
                let confirm = Button::new(locales::message_box::confirm)
                    .primary()
                    .on_click(move |_, window, cx| {
                        Dialog::close(cx);
                        on_confirm(window, cx);
                    });
                #[cfg(test)]
                let confirm = div()
                    .debug_selector(|| "message-box-confirm".into())
                    .child(confirm);
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(content.resolve(cx))
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .gap_2()
                            .child(Button::new(locales::message_box::cancel).on_click(
                                |_, _, cx| {
                                    Dialog::close(cx);
                                },
                            ))
                            .child(confirm),
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
pub fn alert(title: impl Into<LocalizedText>, content: impl Into<LocalizedText>, cx: &mut App) {
    MessageBox::new(title, content).alert(cx);
}

/// Performs the confirm operation used by this component.
pub fn confirm(
    title: impl Into<LocalizedText>,
    content: impl Into<LocalizedText>,
    on_confirm: impl Fn(&mut Window, &mut App) + 'static,
    cx: &mut App,
) {
    MessageBox::new(title, content).confirm(on_confirm, cx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Context, Render, px};
    use liora_core::ActiveModal;

    struct ModalSurface;

    impl Render for ModalSurface {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            liora_core::render_active_modal_in_window(window, cx);
            div().relative().size_full().child(crate::PortalLayer)
        }
    }

    #[test]
    fn confirm_callback_can_open_follow_up_dialog() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, _| ModalSurface);
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| {
            confirm(
                "Confirm",
                "Proceed?",
                |_, cx| {
                    assert!(cx.global::<ActiveModal>().0.is_empty());
                    Dialog::new().id("follow-up").title("Follow-up").show(cx);
                },
                cx,
            );
            window.draw(cx).clear(cx);
        });
        let bounds = window
            .debug_bounds("message-box-confirm")
            .expect("confirm button must render");
        window.simulate_click(bounds.center(), gpui::Modifiers::none());
        window.update(|_, cx| {
            let modals = &cx.global::<ActiveModal>().0;
            assert_eq!(modals.len(), 1);
            assert_eq!(modals[0].id, "follow-up");
        });
        app.quit();
    }
}
