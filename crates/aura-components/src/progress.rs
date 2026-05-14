use crate::motion::{MotionDuration, MotionEasing, motion_animation};
use aura_core::{Config, stable_unique_id};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnimationExt, App, FontWeight, Hsla, IntoElement, ParentElement, PathBuilder, Pixels, Point,
    RenderOnce, SharedString, Styled, Window, canvas, div, linear_color_stop, linear_gradient,
    point, prelude::*, px,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressType {
    #[default]
    Line,
    Circle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressStatus {
    Success,
    Warning,
    Exception,
}

pub struct Progress {
    percentage: f32,
    type_: ProgressType,
    stroke_width: Pixels,
    status: Option<ProgressStatus>,
    color: Option<Hsla>,
    gradient: Option<Vec<Hsla>>,
    show_text: bool,
    text_inside: bool,
    text_inside_center: bool,
    animated: bool,
    circle_size: Pixels,
    track_color: Option<Hsla>,
    text: Option<SharedString>,
    text_color: Option<Hsla>,
    text_size: Option<Pixels>,
    text_weight: FontWeight,
}

impl Progress {
    pub fn new(percentage: f32) -> Self {
        Self {
            percentage: percentage.clamp(0.0, 100.0),
            type_: ProgressType::Line,
            stroke_width: px(6.0),
            status: None,
            color: None,
            gradient: None,
            show_text: true,
            text_inside: false,
            text_inside_center: false,
            animated: true,
            circle_size: px(120.0),
            track_color: None,
            text: None,
            text_color: None,
            text_size: None,
            text_weight: FontWeight::BOLD,
        }
    }

    pub fn type_(mut self, t: ProgressType) -> Self {
        self.type_ = t;
        self
    }

    pub fn line(mut self) -> Self {
        self.type_ = ProgressType::Line;
        self
    }

    pub fn circle(mut self) -> Self {
        self.type_ = ProgressType::Circle;
        self.stroke_width = px(8.0);
        self
    }

    pub fn stroke_width(mut self, w: impl Into<Pixels>) -> Self {
        self.stroke_width = w.into();
        self
    }

    pub fn thick(self) -> Self {
        self.stroke_width(px(20.0))
    }

    pub fn status(mut self, s: ProgressStatus) -> Self {
        self.status = Some(s);
        self
    }

    pub fn color(mut self, c: Hsla) -> Self {
        self.color = Some(c);
        self.gradient = None;
        self
    }

    pub fn primary(mut self) -> Self {
        self.color = None;
        self.gradient = None;
        self.status = None;
        self
    }

    pub fn gradient(mut self, colors: Vec<Hsla>) -> Self {
        self.gradient = if colors.is_empty() {
            None
        } else {
            Some(colors)
        };
        self.color = None;
        self
    }

    pub fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }

    pub fn text_inside(mut self, inside: bool) -> Self {
        self.text_inside = inside;
        self
    }

    pub fn text_inside_center(mut self, center: bool) -> Self {
        self.text_inside_center = center;
        self
    }

    pub fn text_inside_centered(mut self) -> Self {
        self.text_inside = true;
        self.text_inside_center = true;
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn circle_size(mut self, size: impl Into<Pixels>) -> Self {
        self.circle_size = size.into();
        self
    }

    pub fn track_color(mut self, color: Hsla) -> Self {
        self.track_color = Some(color);
        self
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn center_text(self, text: impl Into<SharedString>) -> Self {
        self.text(text)
    }

    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    pub fn text_weight(mut self, weight: FontWeight) -> Self {
        self.text_weight = weight;
        self
    }
}

fn render_gradient_segments(mut bar: gpui::Div, colors: Vec<Hsla>) -> gpui::Div {
    if colors.len() == 1 {
        return bar.bg(colors[0]);
    }

    bar = bar.flex().flex_row();
    for pair in colors.windows(2) {
        bar = bar.child(div().h_full().flex_1().bg(linear_gradient(
            90.0,
            linear_color_stop(pair[0], 0.0),
            linear_color_stop(pair[1], 1.0),
        )));
    }
    bar
}

impl RenderOnce for Progress {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        let status_color = match self.status {
            Some(ProgressStatus::Success) => theme.success.base,
            Some(ProgressStatus::Warning) => theme.warning.base,
            Some(ProgressStatus::Exception) => theme.danger.base,
            None => self.color.unwrap_or(theme.primary.base),
        };
        let gradient = if self.status.is_none() {
            self.gradient.clone()
        } else {
            None
        };
        let percent_text = self
            .text
            .clone()
            .unwrap_or_else(|| format!("{}%", self.percentage.round() as i32).into());
        let id = stable_unique_id(
            format!(
                "aura-progress:{:?}:{:.3}:{:.3}:{:?}:{:?}:{}:{}:{}:{:?}:{:?}:{:?}",
                self.type_,
                self.percentage,
                self.stroke_width.as_f32(),
                self.status,
                self.color,
                self.show_text,
                self.text_inside,
                self.text_inside_center,
                self.text,
                self.text_size,
                self.text_color,
            ),
            "aura-progress",
            window,
            cx,
        );

        if self.type_ == ProgressType::Line {
            let target = self.percentage / 100.0;
            let inside_center = self.show_text && self.text_inside && self.text_inside_center;
            let center_text_color = if self.percentage >= 50.0 {
                gpui::white()
            } else {
                theme.neutral.text_2
            };
            let mut bar = div()
                .h_full()
                .rounded_full()
                .overflow_hidden()
                .when(gradient.is_none(), |s| s.bg(status_color))
                .when_some(gradient, |s, colors| render_gradient_segments(s, colors))
                .when(
                    self.show_text
                        && self.text_inside
                        && !self.text_inside_center
                        && self.percentage > 0.0,
                    |s| {
                        s.min_w(px(36.0))
                            .flex()
                            .items_center()
                            .justify_end()
                            .px_2()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(gpui::white())
                                    .whitespace_nowrap()
                                    .child(percent_text.clone()),
                            )
                    },
                );

            if !self.animated {
                bar = bar.w(gpui::relative(target));
            }

            let bar = if self.animated {
                bar.with_animation(
                    format!("{}-line-fill", id),
                    motion_animation(MotionDuration::Normal, MotionEasing::EaseOut),
                    move |bar, delta| bar.w(gpui::relative(target * delta.clamp(0.0, 1.0))),
                )
                .into_any_element()
            } else {
                bar.into_any_element()
            };

            let track = div()
                .relative()
                .flex_1()
                .h(self.stroke_width)
                .bg(self.track_color.unwrap_or(theme.neutral.hover))
                .rounded_full()
                .overflow_hidden()
                .child(bar)
                .when(inside_center, |s| {
                    s.child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_xs()
                            .text_color(center_text_color)
                            .whitespace_nowrap()
                            .child(percent_text.clone()),
                    )
                });

            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .w_full()
                .child(track)
                .when(self.show_text && !self.text_inside, |s| {
                    s.child(
                        div()
                            .flex()
                            .items_center()
                            .justify_start()
                            .w(px(40.0))
                            .child(match self.status {
                                Some(ProgressStatus::Success) => Icon::new(IconName::CircleCheck)
                                    .size(px(16.0))
                                    .color(theme.success.base)
                                    .into_any_element(),
                                Some(ProgressStatus::Exception) => Icon::new(IconName::CircleX)
                                    .size(px(16.0))
                                    .color(theme.danger.base)
                                    .into_any_element(),
                                _ => div()
                                    .text_xs()
                                    .text_color(theme.neutral.text_2)
                                    .child(percent_text)
                                    .into_any_element(),
                            }),
                    )
                })
                .into_any_element()
        } else {
            let target = self.percentage / 100.0;
            let track_color = self.track_color.unwrap_or(theme.neutral.hover);
            let text_color = self.text_color.unwrap_or(theme.neutral.text_1);
            let text_size = self.text_size.unwrap_or(px(theme.font_size.xl));
            let show_text = self.show_text;
            let text_weight = self.text_weight;

            let base = div()
                .relative()
                .flex_none()
                .w(self.circle_size)
                .h(self.circle_size);

            if self.animated {
                let circle_size = self.circle_size;
                let stroke_width = self.stroke_width;
                let progress_color = status_color;
                let center_text = percent_text.clone();
                base.with_animation(
                    format!("{}-circle-fill", id),
                    motion_animation(MotionDuration::Normal, MotionEasing::EaseOut),
                    move |base, delta| {
                        let progress = target * delta.clamp(0.0, 1.0);
                        let base = base.child(render_circle_canvas(
                            progress,
                            circle_size,
                            stroke_width,
                            track_color,
                            progress_color,
                        ));
                        if show_text {
                            base.child(render_circle_center_text(
                                center_text.clone(),
                                text_color,
                                text_size,
                                text_weight,
                            ))
                        } else {
                            base
                        }
                    },
                )
                .into_any_element()
            } else {
                let mut base = base.child(render_circle_canvas(
                    target,
                    self.circle_size,
                    self.stroke_width,
                    track_color,
                    status_color,
                ));
                if show_text {
                    base = base.child(render_circle_center_text(
                        percent_text,
                        text_color,
                        text_size,
                        text_weight,
                    ));
                }
                base.into_any_element()
            }
        }
    }
}

fn render_circle_center_text(
    text: SharedString,
    text_color: Hsla,
    text_size: Pixels,
    text_weight: FontWeight,
) -> impl IntoElement {
    div()
        .absolute()
        .top_0()
        .left_0()
        .size_full()
        .flex()
        .items_center()
        .justify_center()
        .text_color(text_color)
        .text_size(text_size)
        .font_weight(text_weight)
        .whitespace_nowrap()
        .child(text)
}

fn render_circle_canvas(
    progress: f32,
    size: Pixels,
    stroke_width: Pixels,
    track_color: Hsla,
    progress_color: Hsla,
) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            let width = bounds.right() - bounds.left();
            let height = bounds.bottom() - bounds.top();
            let radius = ((width.min(height).as_f32() - stroke_width.as_f32()) / 2.0).max(1.0);
            let center = point(bounds.left() + width / 2.0, bounds.top() + height / 2.0);

            if let Some(path) = circle_stroke_path(center, radius, stroke_width) {
                window.paint_path(path, track_color);
            }

            if let Some(path) = progress_arc_path(center, radius, stroke_width, progress) {
                window.paint_path(path, progress_color);
            }
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .w(size)
    .h(size)
}

fn circle_stroke_path(
    center: Point<Pixels>,
    radius: f32,
    stroke_width: Pixels,
) -> Option<gpui::Path<Pixels>> {
    if radius <= 0.0 || !radius.is_finite() {
        return None;
    }

    let start = polar_point(center, radius, -90.0);
    let mid = polar_point(center, radius, 90.0);
    let mut builder = PathBuilder::stroke(stroke_width);
    builder.move_to(start);
    builder.arc_to(point(px(radius), px(radius)), px(0.0), false, true, mid);
    builder.arc_to(point(px(radius), px(radius)), px(0.0), false, true, start);
    builder.build().ok()
}

fn progress_arc_path(
    center: Point<Pixels>,
    radius: f32,
    stroke_width: Pixels,
    progress: f32,
) -> Option<gpui::Path<Pixels>> {
    let progress = progress.clamp(0.0, 1.0);
    if progress <= 0.0 || radius <= 0.0 || !radius.is_finite() {
        return None;
    }
    if progress >= 0.9999 {
        return circle_stroke_path(center, radius, stroke_width);
    }

    let start_deg = -90.0;
    let end_deg = start_deg + progress * 360.0;
    let start = polar_point(center, radius, start_deg);
    let end = polar_point(center, radius, end_deg);
    let mut builder = PathBuilder::stroke(stroke_width);
    builder.move_to(start);
    builder.arc_to(
        point(px(radius), px(radius)),
        px(0.0),
        progress > 0.5,
        true,
        end,
    );
    builder.build().ok()
}

fn polar_point(center: Point<Pixels>, radius: f32, deg: f32) -> Point<Pixels> {
    let radians = deg.to_radians();
    point(
        center.x + px(radius * radians.cos()),
        center.y + px(radius * radians.sin()),
    )
}

impl IntoElement for Progress {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_thick_sets_stroke_width() {
        assert_eq!(Progress::new(42.0).thick().stroke_width, px(20.0));
    }

    #[test]
    fn progress_circle_builder_tracks_shape_and_size() {
        let progress = Progress::new(42.0).circle().circle_size(px(144.0));
        assert_eq!(progress.type_, ProgressType::Circle);
        assert_eq!(progress.circle_size, px(144.0));
        assert_eq!(progress.stroke_width, px(8.0));
    }

    #[test]
    fn progress_animation_defaults_on_and_can_disable() {
        assert!(Progress::new(42.0).animated);
        assert!(!Progress::new(42.0).animated(false).animated);
    }

    #[test]
    fn progress_custom_text_tracks_style() {
        let progress = Progress::new(86.0)
            .circle()
            .center_text("Deploy")
            .text_color(gpui::white())
            .text_size(px(22.0))
            .text_weight(FontWeight::NORMAL);
        assert_eq!(progress.text.as_deref(), Some("Deploy"));
        assert_eq!(progress.text_size, Some(px(22.0)));
        assert_eq!(progress.text_weight, FontWeight::NORMAL);
    }

    #[test]
    fn progress_uses_native_paths_and_animation() {
        let source = include_str!("progress.rs");
        assert!(source.contains("PathBuilder::stroke"));
        assert!(source.contains("with_animation("));
        assert!(source.contains("render_circle_canvas"));
    }
}
