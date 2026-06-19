//! Timeline module.
//!
//! This public module implements the Liora timeline component for chronological event display. It keeps the reusable
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

use gpui::{
    AnyElement, App, Hsla, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control timeline mode behavior.
pub enum TimelineMode {
    #[default]
    /// Places the overlay to the left of the anchor.
    Left,
    /// Places the overlay to the right of the anchor.
    Right,
    /// Uses alternate rendering for `TimelineMode`.
    Alternate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control timeline tone behavior.
pub enum TimelineTone {
    /// Uses the primary brand-accent treatment.
    Primary,
    /// Uses success semantic color tokens.
    Success,
    /// Uses warning semantic color tokens.
    Warning,
    /// Uses danger semantic color tokens.
    Danger,
    /// Uses informational semantic color tokens.
    Info,
}

/// Data model used by timeline item rendering.
pub struct TimelineItem {
    /// Timestamp text displayed alongside a timeline item.
    pub timestamp: Option<SharedString>,
    /// Content rendered inside the component body.
    pub content: AnyElement,
    /// Color token or explicit color applied to the visual element.
    pub color: Option<Hsla>,
    /// Semantic tone used for item marker and accent colors.
    pub tone: Option<TimelineTone>,
    /// Optional icon rendered with the item.
    pub icon: Option<IconName>,
    /// Whether the timeline marker is rendered as a hollow dot.
    pub hollow: bool,
    /// Whether timestamp text is hidden for this item.
    pub hide_timestamp: bool,
    /// Preferred placement relative to the trigger or anchor.
    pub placement: TimelinePlacement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control timeline placement behavior.
pub enum TimelinePlacement {
    #[default]
    /// Places the overlay above the anchor.
    Top,
    /// Places the overlay below the anchor.
    Bottom,
}

/// Fluent native GPUI component for rendering Liora timeline.
pub struct Timeline {
    items: Vec<TimelineItem>,
    reverse: bool,
    mode: TimelineMode,
}

impl TimelineItem {
    /// Creates `TimelineItem` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            timestamp: None,
            content: div().into_any_element(),
            color: None,
            tone: None,
            icon: None,
            hollow: false,
            hide_timestamp: false,
            placement: TimelinePlacement::Bottom,
        }
    }

    /// Sets the timestamp value used by the component.
    pub fn timestamp(mut self, t: impl Into<SharedString>) -> Self {
        self.timestamp = Some(t.into());
        self
    }

    /// Sets the rendered content element or text for this component.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = content.into_any_element();
        self
    }

    /// Applies an explicit color instead of the theme-derived default.
    pub fn color(mut self, c: Hsla) -> Self {
        self.color = Some(c);
        self.tone = None;
        self
    }

    /// Sets the tone value used by the component.
    pub fn tone(mut self, tone: TimelineTone) -> Self {
        self.tone = Some(tone);
        self.color = None;
        self
    }

    /// Applies the primary semantic visual variant.
    pub fn primary(self) -> Self {
        self.tone(TimelineTone::Primary)
    }

    /// Applies the success semantic visual variant.
    pub fn success(self) -> Self {
        self.tone(TimelineTone::Success)
    }

    /// Applies the warning semantic visual variant.
    pub fn warning(self) -> Self {
        self.tone(TimelineTone::Warning)
    }

    /// Applies the danger semantic visual variant.
    pub fn danger(self) -> Self {
        self.tone(TimelineTone::Danger)
    }

    /// Applies the informational semantic visual variant.
    pub fn info(self) -> Self {
        self.tone(TimelineTone::Info)
    }

    /// Sets the tray icon configuration value.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the hollow value used by the component.
    pub fn hollow(mut self, h: bool) -> Self {
        self.hollow = h;
        self
    }

    /// Selects the popup, label, or overlay placement.
    pub fn placement(mut self, p: TimelinePlacement) -> Self {
        self.placement = p;
        self
    }

    /// Configures whether timestamp is hidden in the rendered component.
    pub fn hide_timestamp(mut self, hide: bool) -> Self {
        self.hide_timestamp = hide;
        self
    }
}

impl Timeline {
    /// Creates `Timeline` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            items: vec![],
            reverse: false,
            mode: TimelineMode::Left,
        }
    }

    /// Sets the reverse value used by the component.
    pub fn reverse(mut self, r: bool) -> Self {
        self.reverse = r;
        self
    }

    /// Selects the rendering mode used by this component.
    pub fn mode(mut self, m: TimelineMode) -> Self {
        self.mode = m;
        self
    }

    /// Adds the supplied item to the component.
    pub fn item(mut self, item: TimelineItem) -> Self {
        self.items.push(item);
        self
    }
}

impl RenderOnce for Timeline {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let mut items = self.items;
        if self.reverse {
            items.reverse();
        }
        let items_count = items.len();

        div()
            .flex()
            .flex_col()
            .w_full()
            .children(items.into_iter().enumerate().map(|(i, item)| {
                let is_last = i == items_count - 1;
                let dot_color = item.color.unwrap_or_else(|| match item.tone {
                    Some(TimelineTone::Primary) => theme.primary.base,
                    Some(TimelineTone::Success) => theme.success.base,
                    Some(TimelineTone::Warning) => theme.warning.base,
                    Some(TimelineTone::Danger) => theme.danger.base,
                    Some(TimelineTone::Info) => theme.info.base,
                    None => theme.neutral.border,
                });
                let text_color = theme.neutral.text_2;
                let timestamp_color = theme.neutral.text_3;

                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .relative()
                    .child(
                        // Left: Axis & Node
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .w(px(20.0))
                            .child(
                                // Node
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .mt(px(4.0))
                                    .rounded_full()
                                    .bg(if item.hollow {
                                        theme.neutral.card
                                    } else {
                                        dot_color
                                    })
                                    .border_2()
                                    .border_color(dot_color)
                                    .when_some(item.icon, |s, icon| {
                                        // If icon, use icon instead of dot
                                        s.size(px(20.0))
                                            .mt(px(0.0))
                                            .bg(gpui::transparent_black())
                                            .border_0()
                                            .child(Icon::new(icon).size(px(14.0)).color(dot_color))
                                    }),
                            )
                            .when(!is_last, |s| {
                                s.child(
                                    // Vertical Line
                                    div().flex_1().w(px(2.0)).bg(theme.neutral.border),
                                )
                            }),
                    )
                    .child(
                        // Right: Content & Timestamp
                        div()
                            .flex()
                            .flex_col()
                            .pb_6()
                            .flex_1()
                            .when(
                                item.placement == TimelinePlacement::Top && !item.hide_timestamp,
                                |s| {
                                    s.when_some(item.timestamp.clone(), |s, t| {
                                        s.child(
                                            div()
                                                .text_xs()
                                                .text_color(timestamp_color)
                                                .mb_1()
                                                .child(t),
                                        )
                                    })
                                },
                            )
                            .child(div().text_sm().text_color(text_color).child(item.content))
                            .when(
                                item.placement == TimelinePlacement::Bottom && !item.hide_timestamp,
                                |s| {
                                    s.when_some(item.timestamp, |s, t| {
                                        s.child(
                                            div()
                                                .text_xs()
                                                .text_color(timestamp_color)
                                                .mt_2()
                                                .child(t),
                                        )
                                    })
                                },
                            ),
                    )
            }))
    }
}

impl IntoElement for Timeline {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeline_tone_helpers_track_semantic_tone() {
        let item = TimelineItem::new().success();
        assert_eq!(item.tone, Some(TimelineTone::Success));
        assert!(item.color.is_none());

        let custom = TimelineItem::new().success().color(gpui::red());
        assert_eq!(custom.tone, None);
        assert_eq!(custom.color, Some(gpui::red()));
    }
}
