use crate::Button;
use crate::gpui_compat::element_id;
use crate::motion::{fade_in, pop_in};
use gpui::{
    App, Context, KeyBinding, MouseButton, Pixels, Render, SharedString, Window, actions, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

actions!(tour, [TourClose]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TourPlacement {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
    Center,
}

#[derive(Clone)]
pub struct TourStep {
    pub title: SharedString,
    pub description: SharedString,
    pub target: Option<SharedString>,
    pub placement: TourPlacement,
}

impl TourStep {
    pub fn new(title: impl Into<SharedString>, description: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            target: None,
            placement: TourPlacement::Bottom,
        }
    }

    pub fn target(mut self, target: impl Into<SharedString>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn placement(mut self, placement: TourPlacement) -> Self {
        self.placement = placement;
        self
    }
}

type ChangeCallback = dyn Fn(usize, &mut Window, &mut App) + 'static;
type CloseCallback = dyn Fn(&mut Window, &mut App) + 'static;

pub struct Tour {
    id: SharedString,
    steps: Vec<TourStep>,
    active_index: usize,
    open: bool,
    show_mask: bool,
    show_progress: bool,
    close_on_click_outside: bool,
    close_on_escape: bool,
    card_width: Pixels,
    finish_text: SharedString,
    next_text: SharedString,
    previous_text: SharedString,
    on_change: Option<Arc<ChangeCallback>>,
    on_close: Option<Arc<CloseCallback>>,
    on_finish: Option<Arc<CloseCallback>>,
}

pub struct TourView {
    id: SharedString,
    steps: Vec<TourStep>,
    active_index: usize,
    show_mask: bool,
    show_progress: bool,
    close_on_click_outside: bool,
    close_on_escape: bool,
    card_width: Pixels,
    finish_text: SharedString,
    next_text: SharedString,
    previous_text: SharedString,
    on_change: Option<Arc<ChangeCallback>>,
    on_close: Option<Arc<CloseCallback>>,
    on_finish: Option<Arc<CloseCallback>>,
}

impl Tour {
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", TourClose, None)]);
    }

    pub fn new(steps: Vec<TourStep>) -> Self {
        Self {
            id: liora_core::unique_id("tour"),
            steps,
            active_index: 0,
            open: true,
            show_mask: true,
            show_progress: true,
            close_on_click_outside: false,
            close_on_escape: true,
            card_width: px(360.0),
            finish_text: "Finish".into(),
            next_text: "Next".into(),
            previous_text: "Previous".into(),
            on_change: None,
            on_close: None,
            on_finish: None,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn active_index(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn show_mask(mut self, show: bool) -> Self {
        self.show_mask = show;
        self
    }

    pub fn show_progress(mut self, show: bool) -> Self {
        self.show_progress = show;
        self
    }

    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    pub fn card_width(mut self, width: impl Into<Pixels>) -> Self {
        self.card_width = width.into();
        self
    }

    pub fn finish_text(mut self, text: impl Into<SharedString>) -> Self {
        self.finish_text = text.into();
        self
    }

    pub fn next_text(mut self, text: impl Into<SharedString>) -> Self {
        self.next_text = text.into();
        self
    }

    pub fn previous_text(mut self, text: impl Into<SharedString>) -> Self {
        self.previous_text = text.into();
        self
    }

    pub fn on_change(mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Arc::new(cb));
        self
    }

    pub fn on_close(mut self, cb: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Arc::new(cb));
        self
    }

    pub fn on_finish(mut self, cb: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_finish = Some(Arc::new(cb));
        self
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    pub fn resolved_active_index(&self) -> Option<usize> {
        (!self.steps.is_empty()).then(|| self.active_index.min(self.steps.len() - 1))
    }

    pub fn next_index(&self) -> Option<usize> {
        self.resolved_active_index()
            .and_then(|idx| (idx + 1 < self.steps.len()).then_some(idx + 1))
    }

    pub fn previous_index(&self) -> Option<usize> {
        self.resolved_active_index()
            .and_then(|idx| (idx > 0).then_some(idx - 1))
    }

    /// Show the tour as a top-level modal overlay.
    ///
    /// This is the primary Tour API. It renders above the window through Liora's
    /// active modal layer instead of taking space in the normal page layout.
    pub fn show(self, cx: &mut App) {
        if !self.open || self.steps.is_empty() {
            return;
        }
        let id = self.id;
        let view = cx.new(|_cx| TourView {
            id: id.clone(),
            steps: self.steps,
            active_index: self.active_index,
            show_mask: self.show_mask,
            show_progress: self.show_progress,
            close_on_click_outside: self.close_on_click_outside,
            close_on_escape: self.close_on_escape,
            card_width: self.card_width,
            finish_text: self.finish_text,
            next_text: self.next_text,
            previous_text: self.previous_text,
            on_change: self.on_change,
            on_close: self.on_close,
            on_finish: self.on_finish,
        });
        liora_core::set_active_modal(id, view.into(), cx);
    }

    pub fn close(cx: &mut App) {
        liora_core::clear_active_modal(cx);
    }

    pub fn close_id(id: impl Into<SharedString>, cx: &mut App) {
        liora_core::clear_modal(&id.into(), cx);
    }
}

impl TourView {
    fn resolved_active_index(&self) -> Option<usize> {
        (!self.steps.is_empty()).then(|| self.active_index.min(self.steps.len() - 1))
    }

    fn set_active_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(max_index) = self.steps.len().checked_sub(1) else {
            self.close(window, cx);
            return;
        };
        self.active_index = index.min(max_index);
        if let Some(callback) = self.on_change.clone() {
            callback(self.active_index, window, cx);
        }
        cx.notify();
    }

    fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(callback) = self.on_close.clone() {
            callback(window, cx);
        }
        liora_core::clear_modal(&self.id, cx);
    }

    fn finish(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(callback) = self.on_finish.clone() {
            callback(window, cx);
        }
        liora_core::clear_modal(&self.id, cx);
    }
}

impl Render for TourView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let Some(active_index) = self.resolved_active_index() else {
            return div().into_any_element();
        };
        let step = self.steps[active_index].clone();
        let total = self.steps.len();
        let is_first = active_index == 0;
        let is_last = active_index + 1 == total;
        let entity = cx.entity().clone();
        let close_entity = entity.clone();
        let prev_entity = entity.clone();
        let next_entity = entity.clone();
        let finish_entity = entity.clone();
        let placement = step.placement;
        let id = self.id.clone();
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;

        let card = div()
            .id(element_id(format!("{id}-card")))
            .rounded_lg()
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .shadow_xl()
            .p_4()
            .w(self.card_width)
            .cursor_default()
            .flex()
            .flex_col()
            .gap_3()
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(
                div()
                    .flex()
                    .items_start()
                    .justify_between()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(theme.neutral.text_1)
                                    .child(step.title),
                            )
                            .when_some(step.target.clone(), |s, target| {
                                s.child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.primary.base)
                                        .child(format!("Target: {target}")),
                                )
                            }),
                    )
                    .child(
                        div()
                            .id(element_id(format!("{id}-close")))
                            .cursor_pointer()
                            .child(
                                Icon::new(IconName::X)
                                    .size(px(16.0))
                                    .color(theme.neutral.icon),
                            )
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                close_entity.update(cx, |tour, cx| tour.close(window, cx));
                            }),
                    ),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(theme.neutral.text_2)
                    .child(step.description),
            )
            .when(self.show_progress, |s| {
                s.child(
                    div()
                        .text_xs()
                        .text_color(theme.neutral.text_3)
                        .child(format!(
                            "Step {} / {} · {:?}",
                            active_index + 1,
                            total,
                            placement
                        )),
                )
            })
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new(self.previous_text.clone())
                            .small()
                            .disabled(is_first)
                            .on_click(move |_, window, cx| {
                                if !is_first {
                                    prev_entity.update(cx, |tour, cx| {
                                        tour.set_active_index(active_index - 1, window, cx)
                                    });
                                }
                            }),
                    )
                    .child(if is_last {
                        Button::new(self.finish_text.clone())
                            .small()
                            .primary()
                            .on_click(move |_, window, cx| {
                                finish_entity.update(cx, |tour, cx| tour.finish(window, cx));
                            })
                            .into_any_element()
                    } else {
                        Button::new(self.next_text.clone())
                            .small()
                            .primary()
                            .on_click(move |_, window, cx| {
                                next_entity.update(cx, |tour, cx| {
                                    tour.set_active_index(active_index + 1, window, cx)
                                });
                            })
                            .into_any_element()
                    }),
            );

        let overlay = place_overlay_card(div(), placement)
            .id(id.clone())
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .cursor_default()
            .occlude()
            .bg(if self.show_mask {
                theme.neutral.overlay
            } else {
                gpui::transparent_black()
            })
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .when(close_on_click_outside, |s| {
                let entity = entity.clone();
                s.on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    entity.update(cx, |tour, cx| tour.close(window, cx));
                })
            })
            .when(close_on_escape, |s| {
                let entity = entity.clone();
                s.on_action(move |_: &TourClose, window, cx| {
                    entity.update(cx, |tour, cx| tour.close(window, cx));
                })
            })
            .child(pop_in(element_id(format!("{id}-card-motion")), card));

        fade_in(element_id(format!("{id}-overlay-motion")), overlay).into_any_element()
    }
}

fn place_overlay_card(mut overlay: gpui::Div, placement: TourPlacement) -> gpui::Div {
    overlay = overlay.p_8();
    match placement {
        TourPlacement::Top => overlay.flex().flex_col().items_center().justify_start(),
        TourPlacement::Bottom => overlay.flex().flex_col().items_center().justify_end(),
        TourPlacement::Left => overlay.flex().flex_row().items_center().justify_start(),
        TourPlacement::Right => overlay.flex().flex_row().items_center().justify_end(),
        TourPlacement::Center => overlay.flex().items_center().justify_center(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tour() -> Tour {
        Tour::new(vec![
            TourStep::new("A", "a"),
            TourStep::new("B", "b"),
            TourStep::new("C", "c"),
        ])
    }

    #[test]
    fn tour_resolves_navigation_indices() {
        let tour = tour().active_index(1);
        assert_eq!(tour.previous_index(), Some(0));
        assert_eq!(tour.next_index(), Some(2));
    }

    #[test]
    fn tour_clamps_active_index() {
        let tour = tour().active_index(99);
        assert_eq!(tour.resolved_active_index(), Some(2));
        assert_eq!(tour.next_index(), None);
    }

    #[test]
    fn tour_exposes_modal_overlay_api() {
        let source = include_str!("tour.rs");

        assert!(source.contains("pub fn show(self, cx: &mut App)"));
        assert!(source.contains("set_active_modal"));
        assert!(source.contains("absolute()"));
        assert!(source.contains("size_full()"));
        assert!(source.contains("close_on_escape"));
        assert!(source.contains("close_on_click_outside"));
        assert!(source.contains("when(close_on_click_outside"));
        assert!(source.contains("when(close_on_escape"));
        assert!(source.contains("Tour::show(cx)"));
        let render_once_impl = ["impl RenderOnce", " for Tour"].concat();
        let into_element_impl = ["impl IntoElement", " for Tour"].concat();
        assert!(!source.contains(&render_once_impl));
        assert!(!source.contains(&into_element_impl));
    }
}
