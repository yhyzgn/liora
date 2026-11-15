//! A native copy control with brief, local confirmation feedback.

use std::time::Duration;

use gpui::{
    Context, ElementId, FocusHandle, Focusable, IntoElement, KeyDownEvent, Render, SharedString,
    Task, Window, div, prelude::*,
};
use liora_icons_lucide::IconName;

use crate::{button::Button, clipboard::write_text_to_clipboard};

const COPIED_DURATION: Duration = Duration::from_secs(2);

#[derive(Default)]
struct CopyFeedback {
    copied: bool,
    generation: u64,
}

impl CopyFeedback {
    fn mark_copied(&mut self) -> u64 {
        self.generation = self.generation.wrapping_add(1);
        self.copied = true;
        self.generation
    }

    fn expire(&mut self, generation: u64) -> bool {
        if self.generation != generation {
            return false;
        }
        self.copied = false;
        true
    }
}

/// A keyboard-accessible copy control. Keep it in an entity so confirmation survives renders.
pub struct CopyButton {
    id: SharedString,
    text: SharedString,
    disabled: bool,
    feedback: CopyFeedback,
    focus_handle: FocusHandle,
    expiration: Option<Task<()>>,
}

impl CopyButton {
    /// Creates a copy control with a stable element ID and the text to copy.
    pub fn new(
        id: impl Into<SharedString>,
        text: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            disabled: false,
            feedback: CopyFeedback::default(),
            focus_handle: cx.focus_handle(),
            expiration: None,
        }
    }

    /// Disables or enables copying by pointer and keyboard.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Replaces the text copied by future activations.
    pub fn set_text(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.text = text.into();
        cx.notify();
    }

    fn copy(&mut self, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        write_text_to_clipboard(cx, self.text.to_string());
        let generation = self.feedback.mark_copied();
        // Replacing the GPUI task cancels an earlier expiration on repeated activation.
        let executor = cx.background_executor().clone();
        self.expiration = Some(cx.spawn(async move |this, cx| {
            executor.timer(COPIED_DURATION).await;
            let _ = this.update(cx, |button, cx| {
                if !button.feedback.expire(generation) {
                    return;
                }
                cx.notify();
            });
        }));
        cx.notify();
    }

    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled
            && self.focus_handle.is_focused(window)
            && !event.keystroke.modifiers.modified()
            && matches!(event.keystroke.key.as_str(), "enter" | "space")
        {
            self.copy(cx);
            cx.stop_propagation();
        }
    }
}

impl Focusable for CopyButton {
    fn focus_handle(&self, _: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CopyButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();
        div()
            .id(ElementId::from(SharedString::from(format!(
                "{}-focus",
                self.id
            ))))
            .tab_index(0)
            .aria_label(if self.feedback.copied {
                "Copied"
            } else {
                "Copy"
            })
            .track_focus(&self.focus_handle)
            .tab_stop(!self.disabled)
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    if !this.disabled {
                        window.focus(&this.focus_handle, cx);
                    }
                }),
            )
            .on_key_down(cx.listener(Self::key_down))
            .child(
                Button::new(if self.feedback.copied {
                    "Copied"
                } else {
                    "Copy"
                })
                .id(self.id.clone())
                .small()
                .icon_start(if self.feedback.copied {
                    IconName::Check
                } else {
                    IconName::Copy
                })
                .disabled(self.disabled)
                .on_click(move |_, _, cx| {
                    let _ = entity.update(cx, |button, cx| button.copy(cx));
                }),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_copies_replace_feedback_expiration() {
        let mut state = CopyFeedback::default();
        assert!(!state.copied);
        let first = state.mark_copied();
        let second = state.mark_copied();
        assert!(!state.expire(first));
        assert!(state.copied);
        assert!(state.expire(second));
        assert!(!state.copied);
    }
}
