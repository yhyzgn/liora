//! Flex module.
//!
//! This public module implements the Liora flexbox layout wrapper with Liora-friendly builder helpers. It keeps the reusable
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
    AnyElement, App, Component, ElementId, Hsla, IntoElement, Pixels, RenderOnce, ScrollHandle,
    Window, div, prelude::*, px,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlexDirection {
    Row,
    Column,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CrossAlign {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MainAlign {
    Start,
    Center,
    End,
    Between,
}

/// Public builder and render state for the Liora flex component.
pub struct Flex {
    children: Vec<AnyElement>,
    direction: Option<FlexDirection>,
    wrap: bool,
    gap: Option<Pixels>,
    padding: Option<Pixels>,
    padding_x: Option<Pixels>,
    padding_y: Option<Pixels>,
    margin_y: Option<Pixels>,
    height: Option<Pixels>,
    width: Option<Pixels>,
    width_percent: Option<f32>,
    h_full: bool,
    w_full: bool,
    size_full: bool,
    flex_1: bool,
    flex_none: bool,
    min_h_0: bool,
    bg: Option<Hsla>,
    text_color: Option<Hsla>,
    text_size: Option<Pixels>,
    bold: bool,
    border: bool,
    border_color: Option<Hsla>,
    rounded: Option<Pixels>,
    relative: bool,
    overflow_hidden: bool,
    overflow_y_scroll: bool,
    id: Option<ElementId>,
    scroll_handle: Option<ScrollHandle>,
    align: Option<CrossAlign>,
    justify: Option<MainAlign>,
}

impl Flex {
    /// Creates a new value with the required baseline configuration.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            direction: None,
            wrap: false,
            gap: None,
            padding: None,
            padding_x: None,
            padding_y: None,
            margin_y: None,
            height: None,
            width: None,
            width_percent: None,
            h_full: false,
            w_full: false,
            size_full: false,
            flex_1: false,
            flex_none: false,
            min_h_0: false,
            bg: None,
            text_color: None,
            text_size: None,
            bold: false,
            border: false,
            border_color: None,
            rounded: None,
            relative: false,
            overflow_hidden: false,
            overflow_y_scroll: false,
            id: None,
            scroll_handle: None,
            align: None,
            justify: None,
        }
    }

    /// Configures the row option.
    pub fn row(mut self) -> Self {
        self.direction = Some(FlexDirection::Row);
        self
    }

    /// Configures the column option.
    pub fn column(mut self) -> Self {
        self.direction = Some(FlexDirection::Column);
        self
    }

    /// Configures the wrap option.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }

    /// Configures the gap px option.
    pub fn gap_px(mut self, gap: f32) -> Self {
        self.gap = Some(px(gap));
        self
    }

    /// Applies the predefined gap sm sizing preset.
    pub fn gap_sm(self) -> Self {
        self.gap_px(8.0)
    }

    /// Applies the predefined gap md sizing preset.
    pub fn gap_md(self) -> Self {
        self.gap_px(12.0)
    }

    /// Applies the predefined gap lg sizing preset.
    pub fn gap_lg(self) -> Self {
        self.gap_px(16.0)
    }

    /// Applies the predefined gap xl sizing preset.
    pub fn gap_xl(self) -> Self {
        self.gap_px(24.0)
    }

    /// Configures the padding px option.
    pub fn padding_px(mut self, padding: f32) -> Self {
        self.padding = Some(px(padding));
        self
    }

    /// Applies the predefined padding sm sizing preset.
    pub fn padding_sm(self) -> Self {
        self.padding_px(8.0)
    }

    /// Applies the predefined padding md sizing preset.
    pub fn padding_md(self) -> Self {
        self.padding_px(16.0)
    }

    /// Applies the predefined padding lg sizing preset.
    pub fn padding_lg(self) -> Self {
        self.padding_px(24.0)
    }

    /// Configures the padding x px option.
    pub fn padding_x_px(mut self, padding: f32) -> Self {
        self.padding_x = Some(px(padding));
        self
    }

    /// Configures the padding x units option.
    pub fn padding_x_units(self, padding: f32) -> Self {
        self.padding_x_px(padding)
    }

    /// Configures the padding y px option.
    pub fn padding_y_px(mut self, padding: f32) -> Self {
        self.padding_y = Some(px(padding));
        self
    }

    /// Configures the margin y px option.
    pub fn margin_y_px(mut self, margin: f32) -> Self {
        self.margin_y = Some(px(margin));
        self
    }

    /// Configures the margin y units option.
    pub fn margin_y_units(self, margin: f32) -> Self {
        self.margin_y_px(margin)
    }

    /// Applies the predefined height px sizing preset.
    pub fn height_px(mut self, height: f32) -> Self {
        self.height = Some(px(height));
        self
    }

    /// Applies the predefined height units sizing preset.
    pub fn height_units(self, height: f32) -> Self {
        self.height_px(height)
    }

    /// Applies the predefined width px sizing preset.
    pub fn width_px(mut self, width: f32) -> Self {
        self.width = Some(px(width));
        self
    }

    /// Applies the predefined width percent sizing preset.
    pub fn width_percent(mut self, percent: f32) -> Self {
        self.width_percent = Some((percent / 100.0).clamp(0.0, 1.0));
        self
    }

    /// Configures the h full option.
    pub fn h_full(mut self) -> Self {
        self.h_full = true;
        self
    }

    /// Configures the w full option.
    pub fn w_full(mut self) -> Self {
        self.w_full = true;
        self
    }

    /// Applies the predefined size full sizing preset.
    pub fn size_full(mut self) -> Self {
        self.size_full = true;
        self
    }

    /// Configures the flex 1 option.
    pub fn flex_1(mut self) -> Self {
        self.flex_1 = true;
        self
    }

    /// Configures the flex none option.
    pub fn flex_none(mut self) -> Self {
        self.flex_none = true;
        self
    }

    /// Configures the min h 0 option.
    pub fn min_h_0(mut self) -> Self {
        self.min_h_0 = true;
        self
    }

    /// Configures the bg option.
    pub fn bg(mut self, color: Hsla) -> Self {
        self.bg = Some(color);
        self
    }

    /// Configures the text color option.
    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Configures the text size px option.
    pub fn text_size_px(mut self, size: f32) -> Self {
        self.text_size = Some(px(size));
        self
    }

    /// Applies the predefined `text_xs` sizing preset.
    pub fn text_xs(self) -> Self {
        self.text_size_px(12.0)
    }

    /// Applies the predefined text sm sizing preset.
    pub fn text_sm(self) -> Self {
        self.text_size_px(14.0)
    }

    /// Configures the bold option.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Configures the border option.
    pub fn border(mut self) -> Self {
        self.border = true;
        self
    }

    /// Configures the border color option.
    pub fn border_color(mut self, color: Hsla) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Configures the rounded px option.
    pub fn rounded_px(mut self, radius: f32) -> Self {
        self.rounded = Some(px(radius));
        self
    }

    /// Configures the rounded units option.
    pub fn rounded_units(self, radius: f32) -> Self {
        self.rounded_px(radius)
    }

    /// Applies the predefined rounded md sizing preset.
    pub fn rounded_md(mut self) -> Self {
        self.rounded = Some(px(8.0));
        self
    }

    /// Configures the rounded pill option.
    pub fn rounded_pill(mut self) -> Self {
        self.rounded = Some(px(999.0));
        self
    }

    /// Configures the relative option.
    pub fn relative(mut self) -> Self {
        self.relative = true;
        self
    }

    /// Configures the overflow hidden option.
    pub fn overflow_hidden(mut self) -> Self {
        self.overflow_hidden = true;
        self
    }

    /// Configures the overflow y scroll option.
    pub fn overflow_y_scroll(mut self) -> Self {
        self.overflow_y_scroll = true;
        self
    }

    /// Returns the stable tray command identifier used for menu event routing.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Configures the track scroll option.
    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.scroll_handle = Some(handle.clone());
        self
    }

    /// Configures the align start option.
    pub fn align_start(mut self) -> Self {
        self.align = Some(CrossAlign::Start);
        self
    }

    /// Configures the align center option.
    pub fn align_center(mut self) -> Self {
        self.align = Some(CrossAlign::Center);
        self
    }

    /// Configures the align end option.
    pub fn align_end(mut self) -> Self {
        self.align = Some(CrossAlign::End);
        self
    }

    /// Configures the justify start option.
    pub fn justify_start(mut self) -> Self {
        self.justify = Some(MainAlign::Start);
        self
    }

    /// Configures the justify center option.
    pub fn justify_center(mut self) -> Self {
        self.justify = Some(MainAlign::Center);
        self
    }

    /// Configures the justify end option.
    pub fn justify_end(mut self) -> Self {
        self.justify = Some(MainAlign::End);
        self
    }

    /// Configures the justify between option.
    pub fn justify_between(mut self) -> Self {
        self.justify = Some(MainAlign::Between);
        self
    }

    /// Configures the center option.
    pub fn center(self) -> Self {
        self.align_center().justify_center()
    }

    /// Configures the child option.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Configures the children option.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|child| child.into_any_element()));
        self
    }
}

impl RenderOnce for Flex {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut el = div();

        if self.direction.is_some()
            || self.align.is_some()
            || self.justify.is_some()
            || self.gap.is_some()
        {
            el = el.flex();
        }

        match self.direction {
            Some(FlexDirection::Row) => el = el.flex_row(),
            Some(FlexDirection::Column) => el = el.flex_col(),
            None => {}
        }

        if self.wrap {
            el = el.flex_wrap();
        }
        if let Some(gap) = self.gap {
            el = el.gap(gap);
        }
        if let Some(padding) = self.padding {
            el = el.p(padding);
        }
        if let Some(padding_x) = self.padding_x {
            el = el.px(padding_x);
        }
        if let Some(padding_y) = self.padding_y {
            el = el.py(padding_y);
        }
        if let Some(margin_y) = self.margin_y {
            el = el.my(margin_y);
        }
        if let Some(height) = self.height {
            el = el.h(height);
        }
        if let Some(width) = self.width {
            el = el.w(width);
        }
        if let Some(width_percent) = self.width_percent {
            el = el.w(gpui::relative(width_percent));
        }
        if self.h_full {
            el = el.h_full();
        }
        if self.w_full {
            el = el.w_full();
        }
        if self.size_full {
            el = el.size_full();
        }
        if self.flex_1 {
            el = el.flex_1();
        }
        if self.flex_none {
            el = el.flex_none();
        }
        if self.min_h_0 {
            el = el.min_h_0();
        }
        if let Some(bg) = self.bg {
            el = el.bg(bg);
        }
        if let Some(color) = self.text_color {
            el = el.text_color(color);
        }
        if let Some(size) = self.text_size {
            el = el.text_size(size);
        }
        if self.bold {
            el = el.font_weight(gpui::FontWeight::BOLD);
        }
        if self.border {
            el = el.border_1();
        }
        if let Some(color) = self.border_color {
            el = el.border_color(color);
        }
        if let Some(radius) = self.rounded {
            el = el.rounded(radius);
        }
        if self.relative {
            el = el.relative();
        }
        if self.overflow_hidden {
            el = el.overflow_hidden();
        }

        match self.align {
            Some(CrossAlign::Start) => el = el.items_start(),
            Some(CrossAlign::Center) => el = el.items_center(),
            Some(CrossAlign::End) => el = el.items_end(),
            None => {}
        }
        match self.justify {
            Some(MainAlign::Start) => el = el.justify_start(),
            Some(MainAlign::Center) => el = el.justify_center(),
            Some(MainAlign::End) => el = el.justify_end(),
            Some(MainAlign::Between) => el = el.justify_between(),
            None => {}
        }

        if let Some(id) = self.id {
            let mut stateful = el.id(id);
            if self.overflow_y_scroll {
                stateful = stateful.overflow_y_scroll();
            }
            if let Some(scroll_handle) = self.scroll_handle {
                stateful = stateful.track_scroll(&scroll_handle);
            }
            stateful.children(self.children).into_any_element()
        } else {
            el.children(self.children).into_any_element()
        }
    }
}

impl IntoElement for Flex {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flex_tracks_scroll_container_configuration() {
        let handle = ScrollHandle::new();
        let flex = Flex::new()
            .column()
            .height_px(320.0)
            .id("test-scroll")
            .overflow_y_scroll()
            .track_scroll(&handle);

        assert_eq!(flex.direction, Some(FlexDirection::Column));
        assert_eq!(flex.height, Some(px(320.0)));
        assert!(flex.overflow_y_scroll);
        assert!(flex.scroll_handle.is_some());
    }

    #[test]
    fn flex_tracks_visual_box_configuration() {
        let flex = Flex::new()
            .row()
            .wrap()
            .gap_lg()
            .padding_md()
            .width_percent(75.0)
            .rounded_md()
            .border();

        assert_eq!(flex.direction, Some(FlexDirection::Row));
        assert!(flex.wrap);
        assert_eq!(flex.gap, Some(px(16.0)));
        assert_eq!(flex.padding, Some(px(16.0)));
        assert_eq!(flex.width_percent, Some(0.75));
        assert_eq!(flex.rounded, Some(px(8.0)));
        assert!(flex.border);
    }
}
