//! Notification module.
//!
//! This public module implements the Liora notification manager and notification rendering helpers. It keeps the reusable
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
    App, Context, Entity, Global, HoverListenerMode, IntoElement, Render, SharedString,
    Subscription, Task, Window, WindowId, div, prelude::*, px,
};
use liora_core::{Config, push_passive_portal};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control notification type behavior.
pub enum NotificationType {
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
/// Data model used by notification item rendering.
pub struct NotificationItem {
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: usize,
    /// Primary heading or title text displayed by the component.
    pub title: SharedString,
    /// Supporting descriptive text shown near the primary label.
    pub description: Option<SharedString>,
    /// Semantic message type used to choose icon and color tokens.
    pub msg_type: NotificationType,
}

struct ActiveNotification {
    item: NotificationItem,
    remaining: Duration,
    started: Option<Instant>,
    hovered_windows: Vec<WindowId>,
    expiration: Option<Task<()>>,
}

/// Fluent native GPUI component for rendering Liora notification manager.
pub struct NotificationManager {
    notifications: Vec<ActiveNotification>,
    next_id: usize,
    window_closed: Option<Subscription>,
}

/// Fluent native GPUI component for rendering Liora notification manager global.
pub struct NotificationManagerGlobal(pub Entity<NotificationManager>);
impl Global for NotificationManagerGlobal {}

impl NotificationManager {
    /// Creates `NotificationManager` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            notifications: vec![],
            next_id: 0,
            window_closed: None,
        }
    }

    /// Performs the init operation used by this component.
    pub fn init(cx: &mut App) {
        if !cx.has_global::<NotificationManagerGlobal>() {
            let manager = cx.new(|_| Self::new());
            cx.set_global(NotificationManagerGlobal(manager));
        }
    }

    /// Performs the show operation used by this component.
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
            this.notifications.push(ActiveNotification {
                item: NotificationItem {
                    id,
                    title,
                    description,
                    msg_type,
                },
                remaining: Duration::from_secs(4),
                started: None,
                hovered_windows: Vec::new(),
                expiration: None,
            });
            this.next_id += 1;
            this.resume(this.notifications.len() - 1, cx);
            if this.window_closed.is_none() {
                let owner = cx.entity().downgrade();
                this.window_closed = Some(cx.on_window_closed(move |cx, closed| {
                    let _ = owner.update(cx, |this, cx| {
                        for index in 0..this.notifications.len() {
                            let item = &mut this.notifications[index];
                            let hovered = !item.hovered_windows.is_empty();
                            item.hovered_windows.retain(|window| *window != closed);
                            if hovered && item.hovered_windows.is_empty() {
                                this.resume(index, cx);
                            }
                        }
                    });
                }));
            }

            cx.notify();
        });
    }

    fn resume(&mut self, index: usize, cx: &mut Context<Self>) {
        let entry = &mut self.notifications[index];
        let id = entry.item.id;
        let timer = cx.background_executor().timer(entry.remaining);
        entry.started = Some(cx.background_executor().now());
        entry.expiration = Some(cx.spawn(async move |owner, cx| {
            timer.await;
            let _ = owner.update(cx, |this, cx| this.dismiss(id, cx));
        }));
    }

    fn dismiss(&mut self, id: usize, cx: &mut Context<Self>) {
        let Some(index) = self
            .notifications
            .iter()
            .position(|entry| entry.item.id == id)
        else {
            return;
        };
        self.notifications.remove(index);
        cx.notify();
    }

    fn set_hovered(&mut self, id: usize, window: WindowId, hovered: bool, cx: &mut Context<Self>) {
        let Some(index) = self
            .notifications
            .iter()
            .position(|entry| entry.item.id == id)
        else {
            return;
        };
        let entry = &mut self.notifications[index];
        if hovered {
            if entry.hovered_windows.contains(&window) {
                return;
            }
            entry.hovered_windows.push(window);
            if let Some(started) = entry.started.take() {
                let elapsed = cx
                    .background_executor()
                    .now()
                    .saturating_duration_since(started);
                entry.remaining = entry.remaining.saturating_sub(elapsed);
                entry.expiration = None;
            }
        } else {
            let before = entry.hovered_windows.len();
            entry.hovered_windows.retain(|id| *id != window);
            if before > 0 && entry.hovered_windows.is_empty() {
                self.resume(index, cx);
            }
        }
    }
}

impl Render for NotificationManager {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.notifications.is_empty() {
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
            .children(self.notifications.iter().map(|entry| {
                let item = &entry.item;
                let id = item.id;
                let hover_owner = cx.entity().downgrade();
                let close_owner = hover_owner.clone();
                let (color, icon) = match item.msg_type {
                    NotificationType::Info => (theme.primary.base, IconName::Info),
                    NotificationType::Success => (theme.success.base, IconName::Check),
                    NotificationType::Warning => (theme.warning.base, IconName::TriangleAlert),
                    NotificationType::Error => (theme.danger.base, IconName::CircleX),
                };

                pop_in(
                    ("liora-notification", item.id),
                    div()
                        .id(("notification-body", id))
                        .debug_selector(move || format!("notification-{id}"))
                        .hover_listener_mode(HoverListenerMode::InputModalityIndependent)
                        .on_hover(move |hovered, window, cx| {
                            let window = window.window_handle().window_id();
                            let _ = hover_owner.update(cx, |this, cx| {
                                this.set_hovered(id, window, *hovered, cx);
                            });
                        })
                        .w(px(320.0))
                        .bg(theme.neutral.card)
                        .border_1()
                        .border_color(theme.neutral.border)
                        .p_4()
                        .rounded(px(theme.radius.md))
                        .text_color(theme.neutral.text_2)
                        .shadow_lg()
                        .flex()
                        .flex_row()
                        .gap_3()
                        .child(
                            div()
                                .flex_none()
                                .child(Icon::new(icon).size(px(24.0)).color(color)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .min_w(px(0.0))
                                .flex()
                                .flex_col()
                                .gap_1()
                                .child(
                                    div()
                                        .font_weight(gpui::FontWeight::BOLD)
                                        .text_color(theme.neutral.text_1)
                                        .child(item.title.clone()),
                                )
                                .when_some(item.description.clone(), |s, d| {
                                    s.child(
                                        div().text_sm().text_color(theme.neutral.text_3).child(d),
                                    )
                                }),
                        )
                        .child(
                            crate::Button::new("Close notification")
                                .id(("notification-close", id))
                                .icon_only(IconName::X)
                                .text()
                                .small()
                                .on_click(move |_, _, cx| {
                                    let _ = close_owner.update(cx, |this, cx| this.dismiss(id, cx));
                                }),
                        ),
                )
            }))
    }
}

#[cfg(test)]
mod interaction_tests;

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
