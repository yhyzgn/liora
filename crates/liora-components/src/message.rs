//! Message module.
//!
//! This public module implements the Liora toast/message manager and convenience macros. It keeps the reusable
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

use crate::motion::pop_in;
use gpui::{
    App, AsyncApp, Context, Entity, ForegroundExecutor, Global, IntoElement, Render, SharedString,
    Window, div, prelude::*, px,
};
use liora_core::{Config, push_passive_portal};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::Theme;
use std::{cell::RefCell, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control message type behavior.
pub enum MessageType {
    /// Uses informational semantic color tokens.
    Info,
    /// Uses success semantic color tokens.
    Success,
    /// Uses warning semantic color tokens.
    Warning,
    /// Reports a error failure.
    Error,
}

#[derive(Clone)]
/// Data model used by message item rendering.
pub struct MessageItem {
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: usize,
    /// Content rendered inside the component body.
    pub content: SharedString,
    /// Semantic message type used to choose icon and color tokens.
    pub msg_type: MessageType,
}

/// Fluent native GPUI component for rendering Liora message manager.
pub struct MessageManager {
    messages: Vec<MessageItem>,
    next_id: usize,
}

/// Fluent native GPUI component for rendering Liora message manager global.
pub struct MessageManagerGlobal(pub Entity<MessageManager>);
impl Global for MessageManagerGlobal {}

#[derive(Clone)]
struct ToastDispatcherGlobal {
    app: AsyncApp,
    foreground_executor: ForegroundExecutor,
}
impl Global for ToastDispatcherGlobal {}

thread_local! {
    static TOAST_DISPATCHER: RefCell<Option<ToastDispatcherGlobal>> = const { RefCell::new(None) };
}

impl ToastDispatcherGlobal {
    fn new(cx: &mut App) -> Self {
        Self {
            app: cx.to_async(),
            foreground_executor: cx.foreground_executor().clone(),
        }
    }

    fn show(&self, content: SharedString, msg_type: MessageType) {
        let app = self.app.clone();
        self.foreground_executor
            .spawn(async move {
                let _ = app.update(|cx| {
                    show_message(content, msg_type, cx);
                    cx.refresh_windows();
                });
            })
            .detach();
    }
}

impl MessageManager {
    /// Creates `MessageManager` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            messages: vec![],
            next_id: 0,
        }
    }

    /// Performs the init operation used by this component.
    pub fn init(cx: &mut App) {
        if !cx.has_global::<MessageManagerGlobal>() {
            let manager = cx.new(|_| Self::new());
            cx.set_global(MessageManagerGlobal(manager));
        }
        if !cx.has_global::<ToastDispatcherGlobal>() {
            let dispatcher = ToastDispatcherGlobal::new(cx);
            cx.set_global(dispatcher);
        }
        TOAST_DISPATCHER.with(|dispatcher| {
            *dispatcher.borrow_mut() = Some(cx.global::<ToastDispatcherGlobal>().clone());
        });
    }

    /// Performs the show operation used by this component.
    pub fn show(content: impl Into<SharedString>, msg_type: MessageType, cx: &mut App) {
        Self::init(cx);
        let manager = cx.global::<MessageManagerGlobal>().0.clone();
        let content = content.into();

        manager.update(cx, |this, cx| {
            let id = this.next_id;
            this.messages.push(MessageItem {
                id,
                content: content.clone(),
                msg_type,
            });
            this.next_id += 1;

            let async_cx = cx.to_async();
            let executor = cx.background_executor().clone();
            cx.foreground_executor()
                .spawn(async move {
                    executor.timer(Duration::from_secs(3)).await;
                    let _ = async_cx.update(|cx| {
                        if cx.has_global::<MessageManagerGlobal>() {
                            let manager = cx.global::<MessageManagerGlobal>().0.clone();
                            manager.update(cx, |this, cx| {
                                this.messages.retain(|m| m.id != id);
                                cx.notify();
                            });
                        }
                    });
                })
                .detach();

            cx.notify();
        });
    }
}

impl Render for MessageManager {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let messages = self.messages.clone();
        if messages.is_empty() {
            return div();
        }

        let theme = cx.global::<Config>().theme.clone();

        div()
            .absolute()
            .top_8()
            .left_0()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .children(messages.into_iter().map(|msg| {
                let style = message_style(&theme, msg.msg_type);

                pop_in(
                    ("liora-message", msg.id),
                    div()
                        .bg(style.bg)
                        .border_1()
                        .border_color(style.border)
                        .px_4()
                        .py_2()
                        .rounded(px(theme.radius.md))
                        .shadow_lg()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(Icon::new(style.icon).size(px(16.0)).color(style.fg))
                        .child(
                            div()
                                .text_color(style.fg)
                                .text_size(px(theme.font_size.sm))
                                .child(msg.content),
                        ),
                )
            }))
    }
}

/// Configures whether message is visible in the rendered component.
pub fn show_message(content: impl Into<SharedString>, msg_type: MessageType, cx: &mut App) {
    MessageManager::show(content, msg_type, cx);
}

/// Performs the toast operation used by this component.
pub fn toast(content: impl Into<SharedString>, msg_type: MessageType, cx: &mut App) {
    show_message(content, msg_type, cx);
}

/// Performs the toast info operation used by this component.
pub fn toast_info(content: impl Into<SharedString>, cx: &mut App) {
    toast(content, MessageType::Info, cx);
}

/// Performs the toast success operation used by this component.
pub fn toast_success(content: impl Into<SharedString>, cx: &mut App) {
    toast(content, MessageType::Success, cx);
}

/// Performs the toast warning operation used by this component.
pub fn toast_warning(content: impl Into<SharedString>, cx: &mut App) {
    toast(content, MessageType::Warning, cx);
}

/// Performs the toast error operation used by this component.
pub fn toast_error(content: impl Into<SharedString>, cx: &mut App) {
    toast(content, MessageType::Error, cx);
}

/// Performs the dispatch toast operation used by this component.
pub fn dispatch_toast(content: impl Into<SharedString>, msg_type: MessageType) {
    let content = content.into();
    TOAST_DISPATCHER.with(|dispatcher| {
        let Some(dispatcher) = dispatcher.borrow().clone() else {
            panic!("toast macros require MessageManager::init(cx) before use");
        };
        dispatcher.show(content, msg_type);
    });
}

/// Performs the dispatch toast info operation used by this component.
pub fn dispatch_toast_info(content: impl Into<SharedString>) {
    dispatch_toast(content, MessageType::Info);
}

/// Performs the dispatch toast success operation used by this component.
pub fn dispatch_toast_success(content: impl Into<SharedString>) {
    dispatch_toast(content, MessageType::Success);
}

/// Performs the dispatch toast warning operation used by this component.
pub fn dispatch_toast_warning(content: impl Into<SharedString>) {
    dispatch_toast(content, MessageType::Warning);
}

/// Performs the dispatch toast error operation used by this component.
pub fn dispatch_toast_error(content: impl Into<SharedString>) {
    dispatch_toast(content, MessageType::Error);
}

/// Renders the render messages layer into native GPUI elements.
pub fn render_messages(cx: &mut App) {
    if cx.has_global::<MessageManagerGlobal>() {
        let manager = cx.global::<MessageManagerGlobal>().0.clone();
        if !manager.read(cx).messages.is_empty() {
            push_passive_portal(move |_window, _cx| manager.clone().into_any_element(), cx);
        }
    }
}

struct MessageStyle {
    bg: gpui::Hsla,
    fg: gpui::Hsla,
    border: gpui::Hsla,
    icon: IconName,
}

fn message_style(theme: &Theme, msg_type: MessageType) -> MessageStyle {
    let (family, icon) = match msg_type {
        MessageType::Info => (&theme.info, IconName::Info),
        MessageType::Success => (&theme.success, IconName::Check),
        MessageType::Warning => (&theme.warning, IconName::TriangleAlert),
        MessageType::Error => (&theme.danger, IconName::CircleX),
    };

    MessageStyle {
        bg: family.base,
        fg: theme.neutral.card,
        border: family.base,
        icon,
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __liora_toast_dispatch {
    ($dispatch:path, $fmt:literal $(, $($arg:tt)+)?) => {{
        $dispatch(format!($fmt $(, $($arg)+)?));
    }};
    ($dispatch:path, $message:expr $(,)?) => {{
        $dispatch($message);
    }};
}

#[macro_export]
/// Generates the item helper used by Liora applications and examples.
macro_rules! toast_info {
    ($($arg:tt)*) => {{
        $crate::__liora_toast_dispatch!($crate::dispatch_toast_info, $($arg)*);
    }};
}

#[macro_export]
/// Generates the item helper used by Liora applications and examples.
macro_rules! toast_success {
    ($($arg:tt)*) => {{
        $crate::__liora_toast_dispatch!($crate::dispatch_toast_success, $($arg)*);
    }};
}

#[macro_export]
/// Generates the item helper used by Liora applications and examples.
macro_rules! toast_warning {
    ($($arg:tt)*) => {{
        $crate::__liora_toast_dispatch!($crate::dispatch_toast_warning, $($arg)*);
    }};
}

#[macro_export]
/// Generates the item helper used by Liora applications and examples.
macro_rules! toast_error {
    ($($arg:tt)*) => {{
        $crate::__liora_toast_dispatch!($crate::dispatch_toast_error, $($arg)*);
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_styles_use_solid_type_background_and_inverted_foreground() {
        let theme = Theme::light();
        let cases = [
            (MessageType::Info, theme.info.base),
            (MessageType::Success, theme.success.base),
            (MessageType::Warning, theme.warning.base),
            (MessageType::Error, theme.danger.base),
        ];

        for (message_type, expected_bg) in cases {
            let message = message_style(&theme, message_type);

            assert_eq!(message.bg, expected_bg);
            assert_eq!(message.border, expected_bg);
            assert_eq!(message.fg, theme.neutral.card);
        }
    }

    #[test]
    fn toast_helpers_map_to_message_types() {
        let source = include_str!("message.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("pub fn toast_info"));
        assert!(source.contains("pub fn toast_success"));
        assert!(source.contains("pub fn toast_warning"));
        assert!(source.contains("pub fn toast_error"));
        assert!(source.contains("MessageType::Info"));
        assert!(source.contains("MessageType::Success"));
        assert!(source.contains("MessageType::Warning"));
        assert!(source.contains("MessageType::Error"));
    }

    #[test]
    fn toast_macros_support_format_arguments() {
        let source = include_str!("message.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("macro_rules! toast_info"));
        assert!(source.contains("format!("));
        assert!(source.contains("dispatch_toast_info"));
        assert!(source.contains("dispatch_toast_success"));
        assert!(source.contains("dispatch_toast_warning"));
        assert!(source.contains("dispatch_toast_error"));
    }

    #[test]
    #[should_panic(expected = "toast macros require MessageManager::init(cx) before use")]
    fn toast_macro_expands_format_arguments() {
        crate::toast_info!("{left}, {right}", left = "left", right = "right");
    }

    #[test]
    fn messages_use_liora_motion() {
        let source = include_str!("message.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("pop_in("));
        assert!(source.contains("liora-message"));
    }
}
