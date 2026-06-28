//! List module.
//!
//! `List` renders ordered and unordered content lists with nested item support,
//! per-level marker defaults, and explicit marker overrides for individual
//! levels or rows. It is intended for article bodies, settings explainers,
//! onboarding checklists, release notes, and any UI where a native GPUI app
//! needs richer list semantics than plain markdown rendering.

use crate::Text;
use gpui::{
    AnyElement, App, Component, Hsla, IntoElement, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Controls whether a [`List`] renders bullet markers or generated counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListKind {
    /// Renders unordered bullet markers.
    #[default]
    Unordered,
    /// Renders ordered counters.
    Ordered,
}

/// Marker used by unordered lists and row-level overrides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListMarker {
    /// Filled circular bullet.
    Disc,
    /// Hollow circular bullet.
    Circle,
    /// Filled square bullet.
    Square,
    /// Dash marker.
    Dash,
    /// Check icon marker for checklist-like lists.
    Check,
    /// Star icon marker for highlighted lists.
    Star,
    /// Custom text marker.
    Text(SharedString),
}

impl ListMarker {
    /// Creates a custom text marker.
    pub fn text(value: impl Into<SharedString>) -> Self {
        Self::Text(value.into())
    }
}

/// Counter style used by ordered lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderedCounterStyle {
    /// 1, 2, 3.
    Decimal,
    /// 01, 02, 03.
    DecimalLeadingZero,
    /// a, b, c.
    LowerAlpha,
    /// A, B, C.
    UpperAlpha,
    /// i, ii, iii.
    LowerRoman,
    /// I, II, III.
    UpperRoman,
}

/// Generated marker policy for an ordered list level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderedMarker {
    /// Counter style used to format the numeric value.
    pub style: OrderedCounterStyle,
    /// Display pattern. The `{n}` token is replaced by the formatted counter.
    pub pattern: SharedString,
}

impl OrderedMarker {
    /// Creates an ordered marker policy from a counter style and `{n}` pattern.
    pub fn new(style: OrderedCounterStyle, pattern: impl Into<SharedString>) -> Self {
        Self {
            style,
            pattern: pattern.into(),
        }
    }

    /// Creates a decimal marker such as `1.`.
    pub fn decimal() -> Self {
        Self::new(OrderedCounterStyle::Decimal, "{n}.")
    }

    /// Creates a lower-alpha marker such as `a)`.
    pub fn lower_alpha() -> Self {
        Self::new(OrderedCounterStyle::LowerAlpha, "{n})")
    }

    /// Creates a lower-roman marker such as `(i)`.
    pub fn lower_roman() -> Self {
        Self::new(OrderedCounterStyle::LowerRoman, "({n})")
    }

    /// Formats `value` using this marker policy.
    pub fn format(&self, value: usize) -> SharedString {
        self.pattern
            .replace("{n}", &format_counter(value, self.style))
            .into()
    }
}

/// One row in a [`List`], optionally with nested child rows.
pub struct ListItem {
    content: AnyElement,
    children: Vec<ListItem>,
    marker: Option<ListMarker>,
    ordered_marker: Option<OrderedMarker>,
    marker_color: Option<Hsla>,
}

impl ListItem {
    /// Creates a text list item. Text rows are selectable by default.
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self::element(Text::new(content.into()).selectable(true))
    }

    /// Creates a list item from a custom renderable element.
    ///
    /// Use this when a row needs rich content such as badges, buttons, cards,
    /// or manually configured [`Text`]. Plain string rows should use
    /// [`ListItem::new`] so they stay mouse-selectable by default.
    pub fn element(content: impl IntoElement) -> Self {
        Self {
            content: content.into_any_element(),
            children: Vec::new(),
            marker: None,
            ordered_marker: None,
            marker_color: None,
        }
    }

    /// Adds a nested child item.
    pub fn child(mut self, child: ListItem) -> Self {
        self.children.push(child);
        self
    }

    /// Adds several nested child items.
    pub fn children(mut self, children: impl IntoIterator<Item = ListItem>) -> Self {
        self.children.extend(children);
        self
    }

    /// Overrides the unordered marker for this row.
    pub fn marker(mut self, marker: ListMarker) -> Self {
        self.marker = Some(marker);
        self
    }

    /// Overrides the ordered marker policy for this row.
    pub fn ordered_marker(mut self, marker: OrderedMarker) -> Self {
        self.ordered_marker = Some(marker);
        self
    }

    /// Overrides this row's marker color.
    pub fn marker_color(mut self, color: Hsla) -> Self {
        self.marker_color = Some(color);
        self
    }
}

/// Indentation policy for nested child lists.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ListIndent {
    /// Indent by a number of visual spaces using the current UI font size.
    Spaces(usize),
    /// Indent by an explicit pixel value for advanced layout control.
    Pixels(gpui::Pixels),
}

impl ListIndent {
    fn to_pixels(self, theme: &liora_theme::Theme) -> gpui::Pixels {
        match self {
            Self::Spaces(spaces) => px(spaces as f32 * theme.font_size.md * 0.5),
            Self::Pixels(pixels) => pixels,
        }
    }
}

/// Converts common list spacing inputs into a [`ListIndent`] policy.
pub trait IntoListSpacing {
    /// Converts this value into a list spacing policy.
    fn into_list_spacing(self) -> ListIndent;
}

impl IntoListSpacing for usize {
    fn into_list_spacing(self) -> ListIndent {
        ListIndent::Spaces(self)
    }
}

impl IntoListSpacing for u32 {
    fn into_list_spacing(self) -> ListIndent {
        ListIndent::Spaces(self as usize)
    }
}

impl IntoListSpacing for i32 {
    fn into_list_spacing(self) -> ListIndent {
        ListIndent::Spaces(self.max(0) as usize)
    }
}

impl IntoListSpacing for gpui::Pixels {
    fn into_list_spacing(self) -> ListIndent {
        ListIndent::Pixels(self.max(px(0.0)))
    }
}

/// Native GPUI list component with nested ordered/unordered marker policies.
pub struct List {
    kind: ListKind,
    items: Vec<ListItem>,
    unordered_markers: Vec<ListMarker>,
    ordered_markers: Vec<OrderedMarker>,
    start: usize,
    row_gap: gpui::Pixels,
    indent: ListIndent,
    marker_width: gpui::Pixels,
    marker_gap: ListIndent,
    marker_colors: Vec<Hsla>,
}

impl List {
    /// Creates an unordered list with Liora's default per-level bullet markers.
    pub fn unordered() -> Self {
        Self::new(ListKind::Unordered)
    }

    /// Creates an ordered list with Liora's default per-level counter policies.
    pub fn ordered() -> Self {
        Self::new(ListKind::Ordered)
    }

    /// Creates a list with the supplied kind.
    pub fn new(kind: ListKind) -> Self {
        Self {
            kind,
            items: Vec::new(),
            unordered_markers: default_unordered_markers(),
            ordered_markers: default_ordered_markers(),
            start: 1,
            row_gap: px(8.0),
            indent: ListIndent::Spaces(4),
            marker_width: px(26.0),
            marker_gap: ListIndent::Spaces(2),
            marker_colors: Vec::new(),
        }
    }

    /// Adds one root item.
    pub fn item(mut self, item: ListItem) -> Self {
        self.items.push(item);
        self
    }

    /// Adds several root items.
    pub fn items(mut self, items: impl IntoIterator<Item = ListItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Sets the first ordered counter value.
    pub fn start(mut self, start: usize) -> Self {
        self.start = start.max(1);
        self
    }

    /// Replaces per-level unordered markers. If nesting exceeds this list, markers cycle.
    pub fn unordered_markers(mut self, markers: impl IntoIterator<Item = ListMarker>) -> Self {
        let markers: Vec<ListMarker> = markers.into_iter().collect();
        if !markers.is_empty() {
            self.unordered_markers = markers;
        }
        self
    }

    /// Replaces per-level ordered marker policies. If nesting exceeds this list, policies cycle.
    pub fn ordered_markers(mut self, markers: impl IntoIterator<Item = OrderedMarker>) -> Self {
        let markers: Vec<OrderedMarker> = markers.into_iter().collect();
        if !markers.is_empty() {
            self.ordered_markers = markers;
        }
        self
    }

    /// Sets per-level marker colors. If nesting exceeds this list, colors cycle.
    pub fn marker_colors(mut self, colors: impl IntoIterator<Item = Hsla>) -> Self {
        let colors: Vec<Hsla> = colors.into_iter().collect();
        if !colors.is_empty() {
            self.marker_colors = colors;
        }
        self
    }

    /// Sets vertical spacing between sibling rows.
    pub fn row_gap(mut self, gap: impl Into<gpui::Pixels>) -> Self {
        self.row_gap = gap.into().max(px(0.0));
        self
    }

    /// Sets child-list indentation.
    ///
    /// `indent(4)` uses four visual spaces and is the default. Passing
    /// `indent(px(24.0))` uses an explicit pixel value when aligning with
    /// custom non-text layouts.
    pub fn indent(mut self, spacing: impl IntoListSpacing) -> Self {
        self.indent = spacing.into_list_spacing();
        self
    }

    /// Deprecated compatibility alias for pixel indentation.
    ///
    /// Prefer [`List::indent`] with either a space count or `gpui::px(...)`.
    pub fn level_gap(mut self, gap: impl Into<gpui::Pixels>) -> Self {
        self.indent = gap.into().into_list_spacing();
        self
    }

    /// Sets the reserved marker column width.
    pub fn marker_width(mut self, width: impl Into<gpui::Pixels>) -> Self {
        self.marker_width = width.into().max(px(12.0));
        self
    }

    /// Sets the horizontal gap between the marker column and item content.
    ///
    /// `marker_gap(2)` is the default and means two visual spaces. Passing
    /// `marker_gap(px(8.0))` uses an explicit pixel gap.
    pub fn marker_gap(mut self, spacing: impl IntoListSpacing) -> Self {
        self.marker_gap = spacing.into_list_spacing();
        self
    }
}

impl RenderOnce for List {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        render_items(
            self.items,
            self.kind,
            0,
            self.start,
            &self.unordered_markers,
            &self.ordered_markers,
            &self.marker_colors,
            self.row_gap,
            self.indent.to_pixels(&theme),
            self.marker_width,
            self.marker_gap.to_pixels(&theme),
            &theme,
        )
    }
}

impl IntoElement for List {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn render_items(
    items: Vec<ListItem>,
    kind: ListKind,
    level: usize,
    start: usize,
    unordered_markers: &[ListMarker],
    ordered_markers: &[OrderedMarker],
    marker_colors: &[Hsla],
    row_gap: gpui::Pixels,
    indent_px: gpui::Pixels,
    marker_width: gpui::Pixels,
    marker_gap_px: gpui::Pixels,
    theme: &liora_theme::Theme,
) -> AnyElement {
    let mut root = div().flex().flex_col().gap(row_gap);
    for (index, item) in items.into_iter().enumerate() {
        let marker_color = item
            .marker_color
            .unwrap_or_else(|| marker_color_for_level(level, marker_colors, theme));
        let marker = match kind {
            ListKind::Unordered => render_unordered_marker(
                item.marker
                    .clone()
                    .unwrap_or_else(|| unordered_marker_for_level(level, unordered_markers)),
                marker_width,
                marker_color,
                theme,
            ),
            ListKind::Ordered => render_ordered_marker(
                item.ordered_marker
                    .clone()
                    .unwrap_or_else(|| ordered_marker_for_level(level, ordered_markers))
                    .format(start + index),
                marker_width,
                marker_color,
                theme,
            ),
        };

        let children = item.children;
        let child_block = if children.is_empty() {
            None
        } else {
            Some(render_items(
                children,
                kind,
                level + 1,
                1,
                unordered_markers,
                ordered_markers,
                marker_colors,
                row_gap,
                indent_px,
                marker_width,
                marker_gap_px,
                theme,
            ))
        };

        root = root.child(
            div()
                .flex()
                .flex_col()
                .gap(row_gap)
                .child(
                    div()
                        .flex()
                        .items_start()
                        .gap(marker_gap_px)
                        .child(marker)
                        .child(
                            div()
                                .flex_1()
                                .text_color(theme.neutral.text_1)
                                .child(item.content),
                        ),
                )
                .when_some(child_block, |s, children| {
                    s.child(div().ml(indent_px).child(children))
                }),
        );
    }
    root.into_any_element()
}

fn render_unordered_marker(
    marker: ListMarker,
    marker_width: gpui::Pixels,
    marker_color: Hsla,
    _theme: &liora_theme::Theme,
) -> AnyElement {
    let base = div()
        .w(marker_width)
        .min_w(marker_width)
        .h(px(22.0))
        .flex()
        .items_center()
        .justify_end()
        .text_color(marker_color);

    match marker {
        ListMarker::Disc => base
            .child(div().size(px(6.0)).rounded_full().bg(marker_color))
            .into_any_element(),
        ListMarker::Circle => base
            .child(
                div()
                    .size(px(7.0))
                    .rounded_full()
                    .border_1()
                    .border_color(marker_color),
            )
            .into_any_element(),
        ListMarker::Square => base
            .child(div().size(px(6.0)).rounded(px(1.0)).bg(marker_color))
            .into_any_element(),
        ListMarker::Dash => base.child("—").into_any_element(),
        ListMarker::Check => base
            .child(
                Icon::new(IconName::Check)
                    .size(px(14.0))
                    .color(marker_color),
            )
            .into_any_element(),
        ListMarker::Star => base
            .child(Icon::new(IconName::Star).size(px(13.0)).color(marker_color))
            .into_any_element(),
        ListMarker::Text(text) => base.child(text).into_any_element(),
    }
}

fn render_ordered_marker(
    marker: SharedString,
    marker_width: gpui::Pixels,
    marker_color: Hsla,
    _theme: &liora_theme::Theme,
) -> AnyElement {
    div()
        .w(marker_width)
        .min_w(marker_width)
        .h(px(22.0))
        .flex()
        .items_center()
        .justify_end()
        .text_color(marker_color)
        .text_sm()
        .font_weight(gpui::FontWeight::MEDIUM)
        .child(marker)
        .into_any_element()
}

fn default_unordered_markers() -> Vec<ListMarker> {
    vec![
        ListMarker::Disc,
        ListMarker::Circle,
        ListMarker::Square,
        ListMarker::Dash,
    ]
}

fn default_ordered_markers() -> Vec<OrderedMarker> {
    vec![
        OrderedMarker::new(OrderedCounterStyle::Decimal, "{n}."),
        OrderedMarker::new(OrderedCounterStyle::LowerAlpha, "{n})"),
        OrderedMarker::new(OrderedCounterStyle::LowerRoman, "({n})"),
        OrderedMarker::new(OrderedCounterStyle::DecimalLeadingZero, "Step {n}"),
    ]
}

fn unordered_marker_for_level(level: usize, markers: &[ListMarker]) -> ListMarker {
    markers[level % markers.len()].clone()
}

fn ordered_marker_for_level(level: usize, markers: &[OrderedMarker]) -> OrderedMarker {
    markers[level % markers.len()].clone()
}

fn marker_color_for_level(level: usize, colors: &[Hsla], theme: &liora_theme::Theme) -> Hsla {
    colors
        .get(level % colors.len().max(1))
        .copied()
        .unwrap_or(theme.neutral.text_3)
}

fn format_counter(value: usize, style: OrderedCounterStyle) -> String {
    match style {
        OrderedCounterStyle::Decimal => value.to_string(),
        OrderedCounterStyle::DecimalLeadingZero => format!("{value:02}"),
        OrderedCounterStyle::LowerAlpha => alpha_counter(value, false),
        OrderedCounterStyle::UpperAlpha => alpha_counter(value, true),
        OrderedCounterStyle::LowerRoman => roman_counter(value).to_lowercase(),
        OrderedCounterStyle::UpperRoman => roman_counter(value),
    }
}

fn alpha_counter(mut value: usize, uppercase: bool) -> String {
    if value == 0 {
        return String::new();
    }

    let mut chars = Vec::new();
    while value > 0 {
        value -= 1;
        let base = if uppercase { b'A' } else { b'a' };
        chars.push((base + (value % 26) as u8) as char);
        value /= 26;
    }
    chars.iter().rev().collect()
}

fn roman_counter(mut value: usize) -> String {
    if value == 0 {
        return String::new();
    }
    let table = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut out = String::new();
    for (number, roman) in table {
        while value >= number {
            out.push_str(roman);
            value -= number;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordered_counter_formats_decimal_alpha_and_roman() {
        assert_eq!(format_counter(1, OrderedCounterStyle::Decimal), "1");
        assert_eq!(
            format_counter(2, OrderedCounterStyle::DecimalLeadingZero),
            "02"
        );
        assert_eq!(format_counter(27, OrderedCounterStyle::LowerAlpha), "aa");
        assert_eq!(format_counter(28, OrderedCounterStyle::UpperAlpha), "AB");
        assert_eq!(format_counter(9, OrderedCounterStyle::LowerRoman), "ix");
        assert_eq!(format_counter(44, OrderedCounterStyle::UpperRoman), "XLIV");
    }

    #[test]
    fn ordered_marker_replaces_pattern_token() {
        let marker = OrderedMarker::new(OrderedCounterStyle::UpperAlpha, "[{n}]");
        assert_eq!(marker.format(3).as_ref(), "[C]");
    }

    #[test]
    fn list_supports_nested_markers_and_row_overrides() {
        let source = include_str!("list.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();
        assert!(source.contains("pub enum ListKind"));
        assert!(source.contains("pub enum ListMarker"));
        assert!(source.contains("pub struct OrderedMarker"));
        assert!(source.contains("unordered_markers"));
        assert!(source.contains("ordered_markers"));
        assert!(source.contains("ordered_marker(mut self"));
        assert!(source.contains("pub fn element(content: impl IntoElement)"));
        assert!(source.contains("Text::new(content.into()).selectable(true)"));
        assert!(source.contains("marker(mut self"));
        assert!(source.contains("pub fn marker_color(mut self, color: Hsla)"));
        assert!(source.contains("pub fn marker_colors"));
        assert!(source.contains("marker_color_for_level"));
        assert!(source.contains("level % markers.len()"));
        assert!(source.contains("indent: ListIndent::Spaces(4)"));
        assert!(source.contains("pub fn indent(mut self, spacing: impl IntoListSpacing)"));
        assert!(source.contains("spaces as f32 * theme.font_size.md * 0.5"));
        assert!(source.contains("div().ml(indent_px).child(children)"));
        assert!(source.contains("marker_gap: ListIndent::Spaces(2)"));
        assert!(source.contains("pub fn marker_gap(mut self, spacing: impl IntoListSpacing)"));
        assert!(source.contains("impl IntoListSpacing for gpui::Pixels"));
        assert!(source.contains(".gap(marker_gap_px)"));
        assert!(source.contains(".justify_end()"));
        assert!(!source.contains("level_gap + marker_width"));
    }
}
