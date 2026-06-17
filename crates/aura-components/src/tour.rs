use crate::Button;
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TourPlacement {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
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

pub struct Tour {
    steps: Vec<TourStep>,
    active_index: usize,
    open: bool,
    show_mask: bool,
    show_progress: bool,
    finish_text: SharedString,
    next_text: SharedString,
    previous_text: SharedString,
    on_change: Option<Arc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
    on_close: Option<Arc<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_finish: Option<Arc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Tour {
    pub fn new(steps: Vec<TourStep>) -> Self {
        Self {
            steps,
            active_index: 0,
            open: true,
            show_mask: true,
            show_progress: true,
            finish_text: "Finish".into(),
            next_text: "Next".into(),
            previous_text: "Previous".into(),
            on_change: None,
            on_close: None,
            on_finish: None,
        }
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
}

impl RenderOnce for Tour {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        if !self.open {
            return div().into_any_element();
        }
        let Some(active_index) = self.resolved_active_index() else {
            return div().into_any_element();
        };
        let step = self.steps[active_index].clone();
        let total = self.steps.len();
        let is_first = active_index == 0;
        let is_last = active_index + 1 == total;
        let on_close = self.on_close.clone();
        let on_finish = self.on_finish.clone();
        let on_change_prev = self.on_change.clone();
        let on_change_next = self.on_change.clone();

        let card = div()
            .rounded_lg()
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .shadow_xl()
            .p_4()
            .w(px(360.0))
            .flex()
            .flex_col()
            .gap_3()
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
                            .id("tour-close")
                            .cursor_pointer()
                            .child(
                                Icon::new(IconName::X)
                                    .size(px(16.0))
                                    .color(theme.neutral.icon),
                            )
                            .on_click(move |_, window, cx| {
                                if let Some(cb) = on_close.clone() {
                                    cb(window, cx);
                                }
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
                            step.placement
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
                        Button::new(self.previous_text)
                            .small()
                            .disabled(is_first)
                            .on_click(move |_, window, cx| {
                                if !is_first && let Some(cb) = on_change_prev.clone() {
                                    cb(active_index - 1, window, cx);
                                }
                            }),
                    )
                    .child(if is_last {
                        Button::new(self.finish_text)
                            .small()
                            .primary()
                            .on_click(move |_, window, cx| {
                                if let Some(cb) = on_finish.clone() {
                                    cb(window, cx);
                                }
                            })
                            .into_any_element()
                    } else {
                        Button::new(self.next_text)
                            .small()
                            .primary()
                            .on_click(move |_, window, cx| {
                                if let Some(cb) = on_change_next.clone() {
                                    cb(active_index + 1, window, cx);
                                }
                            })
                            .into_any_element()
                    }),
            );

        div()
            .id(aura_core::unique_id("tour"))
            .relative()
            .rounded_lg()
            .border_1()
            .border_color(if self.show_mask {
                theme.primary.base.opacity(0.28)
            } else {
                theme.neutral.border
            })
            .bg(if self.show_mask {
                theme.primary.base.opacity(0.04)
            } else {
                gpui::transparent_black()
            })
            .p_4()
            .child(card)
            .into_any_element()
    }
}

impl IntoElement for Tour {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
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
}
