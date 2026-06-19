//! Carousel module.
//!
//! This public module implements the Liora carousel component for cycling through visual or content panels. It keeps the reusable
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
    AnyElement, App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control carousel direction behavior.
pub enum CarouselDirection {
    #[default]
    /// Lays out content in the horizontal direction.
    Horizontal,
    /// Lays out content in the vertical direction.
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control carousel indicator position behavior.
pub enum CarouselIndicatorPosition {
    #[default]
    /// Places indicators inside the carousel frame.
    Inside,
    /// Places indicators outside the carousel frame.
    Outside,
    /// Disables the optional visual affordance.
    None,
}

/// Data model used by carousel item rendering.
pub struct CarouselItem {
    title: SharedString,
    description: Option<SharedString>,
    accent: Option<Hsla>,
    content: Option<AnyElement>,
}

impl CarouselItem {
    /// Creates `CarouselItem` initialized from the supplied title.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            accent: None,
            content: None,
        }
    }

    /// Sets secondary descriptive text shown below the primary label.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the accent used by the rendered component.
    pub fn accent(mut self, color: Hsla) -> Self {
        self.accent = Some(color);
        self
    }

    /// Sets the rendered content element or text for this component.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

/// Fluent native GPUI component for rendering Liora carousel.
pub struct Carousel {
    items: Vec<CarouselItem>,
    active_index: usize,
    direction: CarouselDirection,
    indicator_position: CarouselIndicatorPosition,
    height: Pixels,
    autoplay: bool,
    interval_ms: u64,
    show_arrows: bool,
    pause_on_hover: bool,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl Carousel {
    /// Creates `Carousel` that renders the supplied items collection.
    pub fn new(items: Vec<CarouselItem>) -> Self {
        Self {
            items,
            active_index: 0,
            direction: CarouselDirection::Horizontal,
            indicator_position: CarouselIndicatorPosition::Inside,
            height: px(220.0),
            autoplay: false,
            interval_ms: 3000,
            show_arrows: true,
            pause_on_hover: true,
            on_change: None,
        }
    }

    /// Sets the current active index state.
    pub fn active_index(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }
    /// Selects the layout or animation direction.
    pub fn direction(mut self, direction: CarouselDirection) -> Self {
        self.direction = direction;
        self
    }
    /// Uses vertical orientation or gradient direction.
    pub fn vertical(self) -> Self {
        self.direction(CarouselDirection::Vertical)
    }
    /// Uses horizontal orientation or gradient direction.
    pub fn horizontal(self) -> Self {
        self.direction(CarouselDirection::Horizontal)
    }
    /// Sets the indicator position value used by the component.
    pub fn indicator_position(mut self, position: CarouselIndicatorPosition) -> Self {
        self.indicator_position = position;
        self
    }
    /// Places carousel indicators outside the item frame.
    pub fn indicators_outside(self) -> Self {
        self.indicator_position(CarouselIndicatorPosition::Outside)
    }
    /// Configures whether indicators is hidden in the rendered component.
    pub fn hide_indicators(self) -> Self {
        self.indicator_position(CarouselIndicatorPosition::None)
    }
    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into();
        self
    }
    /// Enables automatic carousel advancement.
    pub fn autoplay(mut self, enabled: bool) -> Self {
        self.autoplay = enabled;
        self
    }
    /// Sets the interval ms value used by the component.
    pub fn interval_ms(mut self, ms: u64) -> Self {
        self.interval_ms = ms.max(250);
        self
    }
    /// Configures whether arrows is visible in the rendered component.
    pub fn show_arrows(mut self, show: bool) -> Self {
        self.show_arrows = show;
        self
    }
    /// Pauses automatic behavior while the pointer is hovering.
    pub fn pause_on_hover(mut self, pause: bool) -> Self {
        self.pause_on_hover = pause;
        self
    }
    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }
    /// Performs the item count operation used by this component.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    /// Returns the active carousel index after clamping it to available items.
    pub fn resolved_active_index(&self) -> Option<usize> {
        (!self.items.is_empty()).then(|| self.active_index.min(self.items.len() - 1))
    }
    /// Returns the carousel index reached by moving one item forward.
    pub fn next_index(&self) -> Option<usize> {
        self.resolved_active_index()
            .map(|idx| (idx + 1) % self.items.len())
    }
    /// Returns the carousel index reached by moving one item backward.
    pub fn previous_index(&self) -> Option<usize> {
        self.resolved_active_index().map(|idx| {
            if idx == 0 {
                self.items.len() - 1
            } else {
                idx - 1
            }
        })
    }
}

impl RenderOnce for Carousel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let active_index = self.resolved_active_index();
        let count = self.items.len();
        let mut items = self.items;
        let active_item =
            active_index.and_then(|idx| (idx < items.len()).then(|| items.remove(idx)));
        let accent = active_item
            .as_ref()
            .and_then(|item| item.accent)
            .unwrap_or(theme.primary.base);
        let empty = active_item.is_none();
        let mut frame = div()
            .id(liora_core::unique_id("carousel"))
            .relative()
            .overflow_hidden()
            .rounded_lg()
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .h(self.height)
            .w_full();

        frame = frame.child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .bottom_0()
                .bg(accent.opacity(0.12)),
        );

        if let Some(item) = active_item {
            frame = frame.child(
                div()
                    .relative()
                    .size_full()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .gap_3()
                    .p_6()
                    .text_color(theme.neutral.text_1)
                    .when_some(item.content, |s, content| s.child(content))
                    .child(
                        div()
                            .text_size(px(30.0))
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(item.title),
                    )
                    .when_some(item.description, |s, description| {
                        s.child(
                            div()
                                .max_w(px(560.0))
                                .text_size(px(15.0))
                                .text_color(theme.neutral.text_2)
                                .child(description),
                        )
                    }),
            );
        } else {
            frame = frame.child(
                div()
                    .relative()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(theme.neutral.text_3)
                    .child("No carousel items"),
            );
        }

        if self.show_arrows && count > 1 {
            let arrow = |icon| {
                div()
                    .w(px(34.0))
                    .h(px(34.0))
                    .rounded_full()
                    .bg(theme.neutral.card.opacity(0.82))
                    .border_1()
                    .border_color(theme.neutral.border)
                    .shadow_sm()
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|s| s.bg(theme.neutral.hover))
                    .child(Icon::new(icon).size(px(18.0)).color(theme.neutral.text_1))
            };
            frame = frame
                .child(
                    div()
                        .absolute()
                        .left(px(14.0))
                        .top_1_2()
                        .child(arrow(IconName::ChevronLeft)),
                )
                .child(
                    div()
                        .absolute()
                        .right(px(14.0))
                        .top_1_2()
                        .child(arrow(IconName::ChevronRight)),
                );
        }

        let make_dots = || {
            div()
                .flex()
                .items_center()
                .justify_center()
                .gap_2()
                .children((0..count).map(|idx| {
                    let active_dot = Some(idx) == active_index;
                    div()
                        .w(if active_dot { px(22.0) } else { px(7.0) })
                        .h(px(7.0))
                        .rounded_full()
                        .bg(if active_dot {
                            accent
                        } else {
                            theme.neutral.border
                        })
                        .into_any_element()
                }))
        };

        let caption = if self.autoplay {
            format!(
                "auto {}ms · {:?} · pause_on_hover={}",
                self.interval_ms, self.direction, self.pause_on_hover
            )
        } else {
            format!("manual · {:?}", self.direction)
        };

        let mut body = div().flex().flex_col().gap_2().child(frame);
        if !empty && self.indicator_position == CarouselIndicatorPosition::Outside {
            body = body.child(make_dots());
        }
        if !empty && self.indicator_position == CarouselIndicatorPosition::Inside {
            body = body.child(div().mt(px(-34.0)).pb_3().relative().child(make_dots()));
        }
        body.child(
            div()
                .text_xs()
                .text_color(theme.neutral.text_3)
                .child(caption),
        )
    }
}

impl IntoElement for Carousel {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::rgb;

    fn items() -> Vec<CarouselItem> {
        vec![
            CarouselItem::new("A"),
            CarouselItem::new("B"),
            CarouselItem::new("C").accent(rgb(0x16a34a).into()),
        ]
    }

    #[test]
    fn carousel_wraps_next_and_previous_indices() {
        let carousel = Carousel::new(items()).active_index(2);
        assert_eq!(carousel.resolved_active_index(), Some(2));
        assert_eq!(carousel.next_index(), Some(0));
        assert_eq!(carousel.previous_index(), Some(1));
    }

    #[test]
    fn carousel_tracks_display_options() {
        let carousel = Carousel::new(items())
            .vertical()
            .indicators_outside()
            .autoplay(true)
            .interval_ms(1200)
            .pause_on_hover(false);
        assert_eq!(carousel.direction, CarouselDirection::Vertical);
        assert_eq!(
            carousel.indicator_position,
            CarouselIndicatorPosition::Outside
        );
        assert!(carousel.autoplay);
        assert_eq!(carousel.interval_ms, 1200);
        assert!(!carousel.pause_on_hover);
    }
}
