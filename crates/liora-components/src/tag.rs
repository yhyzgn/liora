//! Tag module.
//!
//! This public module implements the Liora tag/chip component for labels, status, and closable tokens. It keeps the reusable
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

use crate::gpui_compat::element_id;
use gpui::{
    AnyElement, App, Component, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Enumerates the supported tag type modes and options.
pub enum TagType {
    #[default]
    /// Uses the info semantic button variant.
    Info,
    /// Uses the success semantic button variant.
    Success,
    /// Uses the warning semantic button variant.
    Warning,
    /// Uses the danger semantic button variant.
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Enumerates the supported tag size modes and options.
pub enum TagSize {
    /// Uses the small component size preset.
    Small,
    #[default]
    /// Uses the default semantic button variant.
    Default,
    /// Uses the large component size preset.
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Enumerates the supported tag effect modes and options.
pub enum TagEffect {
    /// Uses the dark theme mode.
    Dark,
    #[default]
    /// Uses the light theme mode.
    Light,
    /// Uses the plain variant.
    Plain,
}

/// Public builder and render state for the Liora tag component.
pub struct Tag {
    label: SharedString,
    tag_type: TagType,
    size: TagSize,
    effect: TagEffect,
    closable: bool,
    round: bool,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Tag {
    /// Creates a new value with the required baseline configuration.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            tag_type: TagType::Info,
            size: TagSize::Default,
            effect: TagEffect::Light,
            closable: false,
            round: false,
            on_close: None,
        }
    }

    /// Configures the tag type option.
    pub fn tag_type(mut self, t: TagType) -> Self {
        self.tag_type = t;
        self
    }

    /// Configures the success option.
    pub fn success(mut self) -> Self {
        self.tag_type = TagType::Success;
        self
    }

    /// Configures the warning option.
    pub fn warning(mut self) -> Self {
        self.tag_type = TagType::Warning;
        self
    }

    /// Configures the danger option.
    pub fn danger(mut self) -> Self {
        self.tag_type = TagType::Danger;
        self
    }

    /// Configures the info option.
    pub fn info(mut self) -> Self {
        self.tag_type = TagType::Info;
        self
    }

    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, s: TagSize) -> Self {
        self.size = s;
        self
    }

    /// Configures the small option.
    pub fn small(mut self) -> Self {
        self.size = TagSize::Small;
        self
    }

    /// Configures the large option.
    pub fn large(mut self) -> Self {
        self.size = TagSize::Large;
        self
    }

    /// Configures the effect option.
    pub fn effect(mut self, e: TagEffect) -> Self {
        self.effect = e;
        self
    }

    /// Builds the complete dark theme token set.
    pub fn dark(mut self) -> Self {
        self.effect = TagEffect::Dark;
        self
    }

    /// Configures the plain option.
    pub fn plain(mut self) -> Self {
        self.effect = TagEffect::Plain;
        self
    }

    /// Configures the closable option.
    pub fn closable(mut self, c: bool) -> Self {
        self.closable = c;
        self
    }

    /// Configures the round option.
    pub fn round(mut self, r: bool) -> Self {
        self.round = r;
        self
    }

    /// Registers a callback that runs when close occurs.
    pub fn on_close(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(f));
        self
    }
}

impl RenderOnce for Tag {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let on_close = self.on_close;

        let color = match self.tag_type {
            TagType::Info => theme.primary.base,
            TagType::Success => theme.success.base,
            TagType::Warning => theme.warning.base,
            TagType::Danger => theme.danger.base,
        };

        let (bg, border, text_color) = match self.effect {
            TagEffect::Light => (color.opacity(0.1), color.opacity(0.2), color),
            TagEffect::Dark => (color, color, theme.neutral.text_1.opacity(1.0)),
            TagEffect::Plain => (theme.neutral.body, color.opacity(0.4), color),
        };

        let actual_text_color = if self.effect == TagEffect::Dark {
            theme.neutral.inverted
        } else {
            text_color
        };

        let (padding_x, height, text_size) = match self.size {
            TagSize::Small => (px(8.0), px(20.0), px(11.0)),
            TagSize::Default => (px(10.0), px(24.0), px(12.0)),
            TagSize::Large => (px(12.0), px(32.0), px(14.0)),
        };

        let radius = if self.round {
            height / 2.0
        } else {
            px(theme.radius.sm)
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .h(height)
            .px(padding_x)
            .bg(bg)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .text_size(text_size)
            .text_color(actual_text_color)
            .child(div().child(self.label.clone()))
            .when(self.closable, |s| {
                let label = self.label.clone();
                s.child(
                    div()
                        .id(element_id(format!("close-btn-{}", label)))
                        .ml_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .child(
                            Icon::new(IconName::X)
                                .size(px(10.0))
                                .color(actual_text_color),
                        )
                        .hover(|s| s.bg(actual_text_color.opacity(0.2)).rounded(px(2.0)))
                        .on_click(move |_, window, cx| {
                            if let Some(ref f) = on_close {
                                f(window, cx);
                            }
                        }),
                )
            })
    }
}

impl IntoElement for Tag {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Enumerates the supported tag flow align modes and options.
pub enum TagFlowAlign {
    #[default]
    /// Uses the start variant.
    Start,
    /// Uses the center variant.
    Center,
    /// Uses the end variant.
    End,
}

/// Public builder and render state for the Liora tag flow component.
pub struct TagFlow {
    tags: Vec<AnyElement>,
    gap: Pixels,
    align: TagFlowAlign,
    max_rows: Option<usize>,
    estimated_items_per_row: usize,
    collapsed: bool,
    overflow_indicator: Option<SharedString>,
}

impl TagFlow {
    /// Creates a new value with the required baseline configuration.
    pub fn new(tags: impl IntoIterator<Item = Tag>) -> Self {
        Self {
            tags: tags.into_iter().map(|tag| tag.into_any_element()).collect(),
            gap: px(8.0),
            align: TagFlowAlign::Start,
            max_rows: None,
            estimated_items_per_row: 4,
            collapsed: false,
            overflow_indicator: None,
        }
    }

    /// Creates this value from elements.
    pub fn from_elements(tags: impl IntoIterator<Item = impl IntoElement>) -> Self {
        Self {
            tags: tags.into_iter().map(|tag| tag.into_any_element()).collect(),
            gap: px(8.0),
            align: TagFlowAlign::Start,
            max_rows: None,
            estimated_items_per_row: 4,
            collapsed: false,
            overflow_indicator: None,
        }
    }

    /// Configures the gap option.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().max(px(0.0));
        self
    }

    /// Configures the align option.
    pub fn align(mut self, align: TagFlowAlign) -> Self {
        self.align = align;
        self
    }

    /// Configures the center option.
    pub fn center(self) -> Self {
        self.align(TagFlowAlign::Center)
    }

    /// Configures the end option.
    pub fn end(self) -> Self {
        self.align(TagFlowAlign::End)
    }

    /// Configures the max rows option.
    pub fn max_rows(mut self, rows: usize) -> Self {
        self.max_rows = Some(rows.max(1));
        self.collapsed = true;
        self
    }

    /// Configures the estimated items per row option.
    pub fn estimated_items_per_row(mut self, count: usize) -> Self {
        self.estimated_items_per_row = count.max(1);
        self
    }

    /// Configures the collapsed option.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Configures the expanded option.
    pub fn expanded(self) -> Self {
        self.collapsed(false)
    }

    /// Configures the overflow indicator option.
    pub fn overflow_indicator(mut self, label: impl Into<SharedString>) -> Self {
        self.overflow_indicator = Some(label.into());
        self
    }

    fn visible_count(&self) -> usize {
        if !self.collapsed {
            return self.tags.len();
        }
        self.max_rows
            .map(|rows| rows.saturating_mul(self.estimated_items_per_row))
            .unwrap_or(self.tags.len())
            .min(self.tags.len())
    }
}

impl RenderOnce for TagFlow {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let visible_count = self.visible_count();
        let hidden_count = self.tags.len().saturating_sub(visible_count);
        let overflow_label = self
            .overflow_indicator
            .clone()
            .unwrap_or_else(|| format!("+{hidden_count}").into());
        let tags = self
            .tags
            .into_iter()
            .take(visible_count)
            .chain((hidden_count > 0).then(|| {
                Tag::new(overflow_label)
                    .plain()
                    .round(true)
                    .into_any_element()
            }));

        div()
            .flex()
            .flex_wrap()
            .gap(self.gap)
            .when(self.align == TagFlowAlign::Center, |s| s.justify_center())
            .when(self.align == TagFlowAlign::End, |s| s.justify_end())
            .children(tags)
    }
}

impl IntoElement for TagFlow {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_flow_tracks_gap_and_alignment() {
        let flow = TagFlow::new([Tag::new("A"), Tag::new("B")])
            .gap(px(12.0))
            .center();

        assert_eq!(flow.gap, px(12.0));
        assert_eq!(flow.align, TagFlowAlign::Center);
        assert_eq!(flow.tags.len(), 2);
    }

    #[test]
    fn tag_flow_tracks_collapse_options() {
        let flow = TagFlow::new([
            Tag::new("A"),
            Tag::new("B"),
            Tag::new("C"),
            Tag::new("D"),
            Tag::new("E"),
        ])
        .max_rows(2)
        .estimated_items_per_row(2)
        .overflow_indicator("more");

        assert_eq!(flow.visible_count(), 4);
        assert_eq!(flow.max_rows, Some(2));
        assert_eq!(flow.estimated_items_per_row, 2);
        assert!(flow.collapsed);
    }
}
