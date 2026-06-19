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
/// Enumerates the supported timeline modes and options.
pub enum TimelineMode {
    #[default]
    /// Uses the left variant.
    Left,
    /// Uses the right variant.
    Right,
    /// Uses the alternate variant.
    Alternate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerates the supported timeline tone modes and options.
pub enum TimelineTone {
    /// Uses the primary semantic button variant.
    Primary,
    /// Uses the success semantic button variant.
    Success,
    /// Uses the warning semantic button variant.
    Warning,
    /// Uses the danger semantic button variant.
    Danger,
    /// Uses the info semantic button variant.
    Info,
}

/// Public builder and render state for the Liora timeline item component.
pub struct TimelineItem {
    /// Timestamp for this data model.
    pub timestamp: Option<SharedString>,
    /// Content rendered inside the component body.
    pub content: AnyElement,
    /// Color token or explicit color applied to the visual element.
    pub color: Option<Hsla>,
    /// Tone for this data model.
    pub tone: Option<TimelineTone>,
    /// Optional icon rendered with the item.
    pub icon: Option<IconName>,
    /// Hollow for this data model.
    pub hollow: bool,
    /// Hide timestamp for this data model.
    pub hide_timestamp: bool,
    /// Placement for this data model.
    pub placement: TimelinePlacement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Enumerates the supported timeline placement modes and options.
pub enum TimelinePlacement {
    #[default]
    /// Uses the top variant.
    Top,
    /// Uses the bottom variant.
    Bottom,
}

/// Public builder and render state for the Liora timeline component.
pub struct Timeline {
    items: Vec<TimelineItem>,
    reverse: bool,
    mode: TimelineMode,
}

impl TimelineItem {
    /// Creates a new value with the required baseline configuration.
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

    /// Configures the timestamp option.
    pub fn timestamp(mut self, t: impl Into<SharedString>) -> Self {
        self.timestamp = Some(t.into());
        self
    }

    /// Configures the content option.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = content.into_any_element();
        self
    }

    /// Configures the color option.
    pub fn color(mut self, c: Hsla) -> Self {
        self.color = Some(c);
        self.tone = None;
        self
    }

    /// Configures the tone option.
    pub fn tone(mut self, tone: TimelineTone) -> Self {
        self.tone = Some(tone);
        self.color = None;
        self
    }

    /// Configures the primary option.
    pub fn primary(self) -> Self {
        self.tone(TimelineTone::Primary)
    }

    /// Configures the success option.
    pub fn success(self) -> Self {
        self.tone(TimelineTone::Success)
    }

    /// Configures the warning option.
    pub fn warning(self) -> Self {
        self.tone(TimelineTone::Warning)
    }

    /// Configures the danger option.
    pub fn danger(self) -> Self {
        self.tone(TimelineTone::Danger)
    }

    /// Configures the info option.
    pub fn info(self) -> Self {
        self.tone(TimelineTone::Info)
    }

    /// Sets the tray icon configuration value.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Configures the hollow option.
    pub fn hollow(mut self, h: bool) -> Self {
        self.hollow = h;
        self
    }

    /// Configures the placement option.
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
    /// Creates a new value with the required baseline configuration.
    pub fn new() -> Self {
        Self {
            items: vec![],
            reverse: false,
            mode: TimelineMode::Left,
        }
    }

    /// Configures the reverse option.
    pub fn reverse(mut self, r: bool) -> Self {
        self.reverse = r;
        self
    }

    /// Configures the mode option.
    pub fn mode(mut self, m: TimelineMode) -> Self {
        self.mode = m;
        self
    }

    /// Configures the item option.
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
