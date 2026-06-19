//! Steps module.
//!
//! This public module implements the Liora steps/progress navigation component. It keeps the reusable
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

use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control steps direction behavior.
pub enum StepsDirection {
    #[default]
    /// Lays out content in the horizontal direction.
    Horizontal,
    /// Lays out content in the vertical direction.
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control step status behavior.
pub enum StepStatus {
    /// Marks the step as pending.
    Wait,
    /// Marks the step as currently active.
    Process,
    /// Marks the step as completed.
    Finish,
    /// Reports a error failure.
    Error,
}

/// Data model used by step item rendering.
pub struct StepItem {
    /// Primary heading or title text displayed by the component.
    pub title: SharedString,
    /// Supporting descriptive text shown near the primary label.
    pub description: Option<SharedString>,
    /// Optional icon rendered with the item.
    pub icon: Option<IconName>,
    /// Current lifecycle status shown by this item.
    pub status: Option<StepStatus>,
}

/// Fluent native GPUI component for rendering Liora steps.
pub struct Steps {
    active: usize,
    direction: StepsDirection,
    items: Vec<StepItem>,
}

impl StepItem {
    /// Creates `StepItem` initialized from the supplied title.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
            status: None,
        }
    }

    /// Sets secondary descriptive text shown below the primary label.
    pub fn description(mut self, d: impl Into<SharedString>) -> Self {
        self.description = Some(d.into());
        self
    }

    /// Sets the tray icon configuration value.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the status value used by the component.
    pub fn status(mut self, s: StepStatus) -> Self {
        self.status = Some(s);
        self
    }
}

impl Steps {
    /// Creates `Steps` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            active: 0,
            direction: StepsDirection::Horizontal,
            items: vec![],
        }
    }

    /// Sets the current active state.
    pub fn active(mut self, active: usize) -> Self {
        self.active = active;
        self
    }

    /// Selects the layout or animation direction.
    pub fn direction(mut self, d: StepsDirection) -> Self {
        self.direction = d;
        self
    }

    /// Sets the increment used by numeric or time controls.
    pub fn step(mut self, item: StepItem) -> Self {
        self.items.push(item);
        self
    }
}

impl RenderOnce for Steps {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let items_count = self.items.len();
        let active = self.active;
        let direction = self.direction;
        let is_vertical = direction == StepsDirection::Vertical;

        div()
            .flex()
            .when(!is_vertical, |s| s.flex_row().w_full())
            .when(is_vertical, |s| s.flex_col().h_full())
            .children(self.items.into_iter().enumerate().map(|(i, item)| {
                let is_last = i == items_count - 1;
                let status = item.status.unwrap_or_else(|| {
                    if i < active {
                        StepStatus::Finish
                    } else if i == active {
                        StepStatus::Process
                    } else {
                        StepStatus::Wait
                    }
                });

                let color = match status {
                    StepStatus::Finish => theme.primary.base,
                    StepStatus::Process => theme.neutral.text_1,
                    StepStatus::Wait => theme.neutral.text_3,
                    StepStatus::Error => theme.danger.base,
                };

                let icon_bg = match status {
                    StepStatus::Finish => gpui::transparent_black(),
                    StepStatus::Process => theme.primary.base,
                    StepStatus::Wait => gpui::transparent_black(),
                    StepStatus::Error => theme.danger.base,
                };

                let icon_border = match status {
                    StepStatus::Finish => theme.primary.base,
                    StepStatus::Process => theme.primary.base,
                    StepStatus::Wait => theme.neutral.border,
                    StepStatus::Error => theme.danger.base,
                };

                let icon_color = match status {
                    StepStatus::Finish => theme.primary.base,
                    StepStatus::Process => theme.neutral.card,
                    StepStatus::Wait => theme.neutral.text_3,
                    StepStatus::Error => theme.neutral.card,
                };

                div()
                    .flex()
                    .when(!is_vertical, |s| s.flex_1().flex_row().items_center())
                    .when(is_vertical, |s| s.flex_col())
                    .child(
                        div()
                            .flex()
                            .when(!is_vertical, |s| s.flex_row().items_center().gap_2())
                            .when(is_vertical, |s| s.flex_col().items_start().gap_2())
                            .child(
                                // Icon/Number container
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(24.0))
                                    .h(px(24.0))
                                    .rounded_full()
                                    .border_1()
                                    .border_color(icon_border)
                                    .bg(icon_bg)
                                    .child(match item.icon {
                                        Some(icon) => Icon::new(icon)
                                            .size(px(14.0))
                                            .color(icon_color)
                                            .into_any_element(),
                                        None => {
                                            if status == StepStatus::Finish {
                                                Icon::new(IconName::Check)
                                                    .size(px(14.0))
                                                    .color(icon_color)
                                                    .into_any_element()
                                            } else {
                                                div()
                                                    .text_xs()
                                                    .text_color(icon_color)
                                                    .child((i + 1).to_string())
                                                    .into_any_element()
                                            }
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(gpui::FontWeight::BOLD)
                                            .text_color(color)
                                            .child(item.title),
                                    )
                                    .when_some(item.description, |s, d| {
                                        s.child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.neutral.text_3)
                                                .child(d),
                                        )
                                    }),
                            ),
                    )
                    .when(!is_last, |s| {
                        s.child(
                            // Line
                            div()
                                .flex_1()
                                .when(!is_vertical, |s| {
                                    s.mx_4().h(px(1.0)).bg(if i < active {
                                        theme.primary.base
                                    } else {
                                        theme.neutral.border
                                    })
                                })
                                .when(is_vertical, |s| {
                                    s.ml(px(12.0)).my_2().w(px(1.0)).min_h(px(40.0)).bg(
                                        if i < active {
                                            theme.primary.base
                                        } else {
                                            theme.neutral.border
                                        },
                                    )
                                }),
                        )
                    })
            }))
    }
}

impl IntoElement for Steps {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
