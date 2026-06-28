//! Status bar module.
//!
//! `StatusBar` renders a native GPUI desktop status strip with left, center,
//! and right regions. It is intended for application shells that need to show
//! connection state, background tasks, cursor position, current page, version,
//! shortcuts, or update status without coupling that logic to Gallery or Docs.

use crate::{Spinner, Text};
use gpui::{
    AnyElement, App, Component, Hsla, IntoElement, MouseButton, Pixels, RenderOnce, SharedString,
    Window, div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

type StatusBarClickCallback = dyn Fn(&mut Window, &mut App) + 'static;

/// Visual tone applied to a [`StatusBarItem`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusBarTone {
    /// Neutral item tone for ordinary information.
    #[default]
    Neutral,
    /// Primary tone for active scope or selected context.
    Primary,
    /// Success tone for healthy or connected states.
    Success,
    /// Warning tone for degraded or pending states.
    Warning,
    /// Danger tone for error or disconnected states.
    Danger,
    /// Info tone for informational task state.
    Info,
}

/// One item rendered inside a status bar region.
pub struct StatusBarItem {
    id: Option<SharedString>,
    label: Option<SharedString>,
    detail: Option<SharedString>,
    icon: Option<IconName>,
    tone: StatusBarTone,
    loading: bool,
    pill: bool,
    compact: bool,
    custom: Option<AnyElement>,
    separator: bool,
    spacer: bool,
    dot: bool,
    min_width: Option<Pixels>,
    background: Option<Hsla>,
    text_color: Option<Hsla>,
    on_click: Option<Arc<StatusBarClickCallback>>,
}

impl StatusBarItem {
    /// Creates a text status item.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            id: None,
            label: Some(label.into()),
            detail: None,
            icon: None,
            tone: StatusBarTone::Neutral,
            loading: false,
            pill: false,
            compact: false,
            custom: None,
            separator: false,
            spacer: false,
            dot: false,
            min_width: None,
            background: None,
            text_color: None,
            on_click: None,
        }
    }

    /// Creates a custom status item from any GPUI element.
    pub fn custom(content: impl IntoElement) -> Self {
        Self {
            id: None,
            label: None,
            detail: None,
            icon: None,
            tone: StatusBarTone::Neutral,
            loading: false,
            pill: false,
            compact: false,
            custom: Some(content.into_any_element()),
            separator: false,
            spacer: false,
            dot: false,
            min_width: None,
            background: None,
            text_color: None,
            on_click: None,
        }
    }

    /// Creates a vertical separator item between dense status clusters.
    pub fn separator() -> Self {
        Self {
            id: None,
            label: None,
            detail: None,
            icon: None,
            tone: StatusBarTone::Neutral,
            loading: false,
            pill: false,
            compact: true,
            custom: None,
            separator: true,
            spacer: false,
            dot: false,
            min_width: None,
            background: None,
            text_color: None,
            on_click: None,
        }
    }

    /// Creates a flexible spacer item for advanced region composition.
    pub fn spacer() -> Self {
        let mut item = Self::separator();
        item.separator = false;
        item.spacer = true;
        item
    }

    /// Assigns a stable element id used by GPUI state and tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Adds secondary detail text.
    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Adds a leading icon.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Applies a semantic tone.
    pub fn tone(mut self, tone: StatusBarTone) -> Self {
        self.tone = tone;
        self
    }

    /// Applies primary tone.
    pub fn primary(self) -> Self {
        self.tone(StatusBarTone::Primary)
    }

    /// Applies success tone.
    pub fn success(self) -> Self {
        self.tone(StatusBarTone::Success)
    }

    /// Applies warning tone.
    pub fn warning(self) -> Self {
        self.tone(StatusBarTone::Warning)
    }

    /// Applies danger tone.
    pub fn danger(self) -> Self {
        self.tone(StatusBarTone::Danger)
    }

    /// Applies info tone.
    pub fn info(self) -> Self {
        self.tone(StatusBarTone::Info)
    }

    /// Shows a spinner before the label.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Shows a small semantic status dot before icon/text.
    pub fn dot(mut self) -> Self {
        self.dot = true;
        self
    }

    /// Sets a minimum width for aligned status clusters.
    pub fn min_width(mut self, width: impl Into<Pixels>) -> Self {
        self.min_width = Some(width.into());
        self
    }

    /// Overrides the item text/icon color.
    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Overrides the item background color.
    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    /// Registers a click callback and renders the item as an interactive affordance.
    pub fn on_click(mut self, callback: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Arc::new(callback));
        self
    }

    /// Renders the item with pill background and border.
    pub fn pill(mut self) -> Self {
        self.pill = true;
        self
    }

    /// Uses compact horizontal padding.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    fn colors(&self, theme: &liora_theme::Theme) -> (Hsla, Hsla) {
        match self.tone {
            StatusBarTone::Neutral => (theme.neutral.text_2, theme.neutral.hover),
            StatusBarTone::Primary => (theme.primary.base, theme.primary.hover.opacity(0.16)),
            StatusBarTone::Success => (theme.success.base, theme.success.hover.opacity(0.18)),
            StatusBarTone::Warning => (theme.warning.base, theme.warning.hover.opacity(0.20)),
            StatusBarTone::Danger => (theme.danger.base, theme.danger.hover.opacity(0.18)),
            StatusBarTone::Info => (theme.info.base, theme.info.hover.opacity(0.18)),
        }
    }

    fn render(self, theme: &liora_theme::Theme) -> AnyElement {
        if self.separator {
            return div()
                .w(px(1.0))
                .h(px(16.0))
                .mx_1()
                .bg(theme.neutral.divider)
                .into_any_element();
        }
        if self.spacer {
            return div().flex_1().min_w(px(8.0)).into_any_element();
        }
        if let Some(custom) = self.custom {
            return custom;
        }
        let (tone_fg, tone_bg) = self.colors(theme);
        let fg = self.text_color.unwrap_or(tone_fg);
        let bg = self.background.unwrap_or(tone_bg);
        let click = self.on_click.clone();
        let mut item = div()
            .flex()
            .items_center()
            .gap_1()
            .min_h(px(22.0))
            .px(if self.compact { px(4.0) } else { px(7.0) })
            .text_color(fg)
            .text_size(px(theme.font_size.sm))
            .when_some(self.min_width, |s, width| s.min_w(width))
            .when(click.is_some(), |s| {
                s.cursor_pointer().hover(|s| s.bg(bg)).on_mouse_up(
                    MouseButton::Left,
                    move |_, window, cx| {
                        if let Some(callback) = &click {
                            callback(window, cx);
                        }
                    },
                )
            })
            .when(self.pill, |s| {
                s.rounded_full()
                    .bg(bg)
                    .border_1()
                    .border_color(theme.neutral.border)
            });

        if self.dot {
            item = item.child(div().size(px(7.0)).rounded_full().bg(fg));
        }

        if self.loading {
            item = item.child(Spinner::new().small().color(fg));
        } else if let Some(icon) = self.icon {
            item = item.child(Icon::new(icon).size(px(13.0)).color(fg));
        }

        if let Some(label) = self.label {
            item = item.child(Text::new(label).sm().nowrap().text_color(fg));
        }
        if let Some(detail) = self.detail {
            item = item
                .child(Text::new("·").xs().text_color(theme.neutral.text_3))
                .child(
                    Text::new(detail)
                        .xs()
                        .nowrap()
                        .text_color(theme.neutral.text_3),
                );
        }

        item.into_any_element()
    }
}

/// Fluent native GPUI component for rendering a shell status bar.
pub struct StatusBar {
    id: SharedString,
    left: Vec<StatusBarItem>,
    center: Vec<StatusBarItem>,
    right: Vec<StatusBarItem>,
    height: Pixels,
    background: Option<Hsla>,
    border_color: Option<Hsla>,
    show_top_border: bool,
    padding_x: Pixels,
    gap: Pixels,
}

impl StatusBar {
    /// Creates an empty status bar with left/center/right regions.
    pub fn new() -> Self {
        Self {
            id: "liora-status-bar".into(),
            left: Vec::new(),
            center: Vec::new(),
            right: Vec::new(),
            height: px(32.0),
            background: None,
            border_color: None,
            show_top_border: true,
            padding_x: px(10.0),
            gap: px(8.0),
        }
    }

    /// Assigns a stable element id used by GPUI state and tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Appends an item to the left region.
    pub fn left_item(mut self, item: StatusBarItem) -> Self {
        self.left.push(item);
        self
    }

    /// Appends an item to the center region.
    pub fn center_item(mut self, item: StatusBarItem) -> Self {
        self.center.push(item);
        self
    }

    /// Appends an item to the right region.
    pub fn right_item(mut self, item: StatusBarItem) -> Self {
        self.right.push(item);
        self
    }

    /// Replaces the left region.
    pub fn left_items(mut self, items: Vec<StatusBarItem>) -> Self {
        self.left = items;
        self
    }

    /// Replaces the center region.
    pub fn center_items(mut self, items: Vec<StatusBarItem>) -> Self {
        self.center = items;
        self
    }

    /// Replaces the right region.
    pub fn right_items(mut self, items: Vec<StatusBarItem>) -> Self {
        self.right = items;
        self
    }

    /// Sets the status bar height.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into().max(px(24.0));
        self
    }

    /// Overrides the background color.
    pub fn background(mut self, background: Hsla) -> Self {
        self.background = Some(background);
        self
    }

    /// Overrides the top border color.
    pub fn border_color(mut self, border_color: Hsla) -> Self {
        self.border_color = Some(border_color);
        self
    }

    /// Hides the top border.
    pub fn borderless(mut self) -> Self {
        self.show_top_border = false;
        self
    }

    /// Sets horizontal padding.
    pub fn padding_x(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding_x = padding.into();
        self
    }

    /// Sets gap between items inside each region.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into();
        self
    }

    /// Returns the number of items in the three regions.
    pub fn region_counts(&self) -> (usize, usize, usize) {
        (self.left.len(), self.center.len(), self.right.len())
    }

    fn render_region(
        items: Vec<StatusBarItem>,
        theme: &liora_theme::Theme,
        gap: Pixels,
    ) -> AnyElement {
        div()
            .flex()
            .items_center()
            .gap(gap)
            .min_w(px(0.0))
            .children(items.into_iter().map(|item| item.render(theme)))
            .into_any_element()
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for StatusBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let border = self.border_color.unwrap_or(theme.neutral.border);
        div()
            .id(self.id)
            .w_full()
            .h(self.height)
            .flex()
            .items_center()
            .justify_between()
            .gap_3()
            .px(self.padding_x)
            .bg(self.background.unwrap_or(theme.neutral.card))
            .when(self.show_top_border, |s| {
                s.border_t_1().border_color(border)
            })
            .child(Self::render_region(self.left, &theme, self.gap))
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Self::render_region(self.center, &theme, self.gap)),
            )
            .child(Self::render_region(self.right, &theme, self.gap))
    }
}

impl IntoElement for StatusBar {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_bar_tracks_region_counts() {
        let bar = StatusBar::new()
            .left_item(StatusBarItem::new("Ready"))
            .center_item(StatusBarItem::new("main.rs"))
            .right_item(StatusBarItem::new("v0.1.15"))
            .right_item(StatusBarItem::new("UTF-8"));

        assert_eq!(bar.region_counts(), (1, 1, 2));
    }

    #[test]
    fn status_bar_item_builders_track_state() {
        let item = StatusBarItem::new("Connected")
            .success()
            .icon(IconName::Wifi)
            .detail("42ms")
            .pill()
            .compact();

        assert_eq!(item.tone, StatusBarTone::Success);
        assert_eq!(item.icon, Some(IconName::Wifi));
        assert!(item.pill);
        assert!(item.compact);
        assert_eq!(item.detail.as_deref(), Some("42ms"));
    }

    #[test]
    fn status_bar_item_supports_custom_interactive_layout_affordances() {
        let source = include_str!("status_bar.rs");
        let item = StatusBarItem::new("Build")
            .dot()
            .min_width(px(88.0))
            .background(gpui::rgb(0x111827).into())
            .text_color(gpui::white())
            .on_click(|_, _| {});

        assert!(item.dot);
        assert_eq!(item.min_width, Some(px(88.0)));
        assert!(item.background.is_some());
        assert!(item.text_color.is_some());
        assert!(item.on_click.is_some());
        assert!(source.contains("StatusBarItem::separator"));
        assert!(source.contains("StatusBarItem::spacer"));
        assert!(source.contains("cursor_pointer"));
    }
}
