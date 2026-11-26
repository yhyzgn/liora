//! Mount `PortalLayer` last at the window root before using this preview.

use gpui::{AnyView, App, Context, Render, Task, Window, div, prelude::*};
use liora_components::{Button, Loading};
use liora_core::push_portal;
use std::time::Duration;

/// A retained, cancelable window-content loading preview, not a keyboard-modal dialog.
pub fn fullscreen_loading(cx: &mut App) -> AnyView {
    cx.new(|_| FullscreenLoading { dismissal: None }).into()
}

struct FullscreenLoading {
    dismissal: Option<Task<()>>,
}

impl FullscreenLoading {
    fn dismiss(&mut self, cx: &mut Context<Self>) {
        self.dismissal = None;
        cx.notify();
    }
}

impl Render for FullscreenLoading {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.dismissal.is_some() {
            let cancel = cx.listener(|this, _, _, cx| this.dismiss(cx));
            push_portal(
                move |_, _| {
                    div()
                        .relative()
                        .size_full()
                        .debug_selector(|| "loading-preview-overlay".into())
                        .child(Loading::new().text("Preparing workspace...").full_screen())
                        .child(
                            div()
                                .absolute()
                                .top_4()
                                .right_4()
                                .debug_selector(|| "loading-preview-cancel".into())
                                .child(Button::new("Cancel preview").on_click(cancel)),
                        )
                        .into_any_element()
                },
                cx,
            );
        }
        div()
            .debug_selector(|| "loading-preview-start".into())
            .child(
                Button::new(if self.dismissal.is_some() {
                    "Cancel preview"
                } else {
                    "Preview full-screen loading"
                })
                .id("loading-preview-start")
                .on_click(cx.listener(|this, _, _, cx| {
                    if this.dismissal.is_some() {
                        this.dismiss(cx);
                        return;
                    }
                    let timer = cx.background_executor().timer(Duration::from_secs(3));
                    this.dismissal = Some(cx.spawn(async move |this, cx| {
                        timer.await;
                        let _ = this.update(cx, Self::dismiss);
                    }));
                    cx.notify();
                })),
            )
    }
}
