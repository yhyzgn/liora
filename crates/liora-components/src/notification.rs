//! Notification module.
//!
//! This public module implements the Liora notification manager and notification rendering helpers. It keeps the reusable
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

use crate::motion::pop_in;
use gpui::{
    App, Context, Entity, Global, IntoElement, Render, SharedString, Window, div, prelude::*, px,
};
use liora_core::{Config, push_passive_portal};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerates the supported notification type modes and options.
pub enum NotificationType {
    /// Uses the info semantic button variant.
    Info,
    /// Uses the success semantic button variant.
    Success,
    /// Uses the warning semantic button variant.
    Warning,
    /// Reports a error failure.
    Error,
}

#[derive(Clone)]
/// Public builder and render state for the Liora notification item component.
pub struct NotificationItem {
    /// Stable identifier used to connect rendered UI, callbacks, and external state.
    pub id: usize,
    /// Primary heading or title text displayed by the component.
    pub title: SharedString,
    /// Supporting descriptive text shown near the primary label.
    pub description: Option<SharedString>,
    /// Msg type for this data model.
    pub msg_type: NotificationType,
}

/// Public builder and render state for the Liora notification manager component.
pub struct NotificationManager {
    notifications: Vec<NotificationItem>,
    next_id: usize,
}

/// Public builder and render state for the Liora notification manager global component.
pub struct NotificationManagerGlobal(pub Entity<NotificationManager>);
impl Global for NotificationManagerGlobal {}

impl NotificationManager {
    /// Creates a new value with the required baseline configuration.
    pub fn new() -> Self {
        Self {
            notifications: vec![],
            next_id: 0,
        }
    }

    /// Configures the init option.
    pub fn init(cx: &mut App) {
        if !cx.has_global::<NotificationManagerGlobal>() {
            let manager = cx.new(|_| Self::new());
            cx.set_global(NotificationManagerGlobal(manager));
        }
    }

    /// Configures the show option.
    pub fn show(
        title: impl Into<SharedString>,
        description: Option<SharedString>,
        msg_type: NotificationType,
        cx: &mut App,
    ) {
        Self::init(cx);
        let manager = cx.global::<NotificationManagerGlobal>().0.clone();
        let title = title.into();

        manager.update(cx, |this, cx| {
            let id = this.next_id;
            this.notifications.push(NotificationItem {
                id,
                title: title.clone(),
                description: description.clone(),
                msg_type,
            });
            this.next_id += 1;

            let async_cx = cx.to_async();
            let executor = cx.background_executor().clone();
            cx.foreground_executor()
                .spawn(async move {
                    executor.timer(Duration::from_secs(4)).await;
                    let _ = async_cx.update(|cx| {
                        if cx.has_global::<NotificationManagerGlobal>() {
                            let manager = cx.global::<NotificationManagerGlobal>().0.clone();
                            manager.update(cx, |this, cx| {
                                this.notifications.retain(|n| n.id != id);
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

impl Render for NotificationManager {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items = self.notifications.clone();
        if items.is_empty() {
            return div();
        }

        let theme = cx.global::<Config>().theme.clone();

        div()
            .absolute()
            .top_8()
            .right_8()
            .flex()
            .flex_col()
            .items_end()
            .gap_4()
            .children(items.into_iter().map(|item| {
                let (color, icon) = match item.msg_type {
                    NotificationType::Info => (theme.primary.base, IconName::Info),
                    NotificationType::Success => (theme.success.base, IconName::Check),
                    NotificationType::Warning => (theme.warning.base, IconName::TriangleAlert),
                    NotificationType::Error => (theme.danger.base, IconName::CircleX),
                };

                pop_in(
                    ("liora-notification", item.id),
                    div()
                        .w(px(320.0))
                        .bg(theme.neutral.card)
                        .border_1()
                        .border_color(theme.neutral.border)
                        .p_4()
                        .rounded(px(theme.radius.md))
                        .shadow_lg()
                        .flex()
                        .flex_row()
                        .gap_3()
                        .child(Icon::new(icon).size(px(24.0)).color(color))
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(div().font_weight(gpui::FontWeight::BOLD).child(item.title))
                                .when_some(item.description, |s, d| {
                                    s.child(
                                        div().text_sm().text_color(theme.neutral.text_3).child(d),
                                    )
                                }),
                        ),
                )
            }))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn notifications_use_liora_motion() {
        let source = include_str!("notification.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("pop_in("));
        assert!(source.contains("liora-notification"));
    }
}

/// Configures whether notification is visible in the rendered component.
pub fn show_notification(
    title: impl Into<SharedString>,
    description: Option<SharedString>,
    msg_type: NotificationType,
    cx: &mut App,
) {
    NotificationManager::show(title, description, msg_type, cx);
}

/// Renders the render notifications layer into native GPUI elements.
pub fn render_notifications(cx: &mut App) {
    if cx.has_global::<NotificationManagerGlobal>() {
        let manager = cx.global::<NotificationManagerGlobal>().0.clone();
        if !manager.read(cx).notifications.is_empty() {
            push_passive_portal(move |_window, _cx| manager.clone().into_any_element(), cx);
        }
    }
}
