use crate::motion::{MotionDuration, MotionEasing, motion_animation};
use aura_core::{Config, stable_unique_id};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnimationExt, App, FontWeight, Hsla, IntoElement, ParentElement, PathBuilder, Pixels, Point,
    RenderOnce, SharedString, Styled, Window, canvas, div, linear_color_stop, linear_gradient,
    point, prelude::*, px,
};
use std::f32::consts::TAU;

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
    circle_inner_color: Option<Hsla>,
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
            circle_inner_color: None,
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

    pub fn ring_width(self, width: impl Into<Pixels>) -> Self {
        self.stroke_width(width)
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

    pub fn ring_color(self, color: Hsla) -> Self {
        self.track_color(color)
    }

    pub fn progress_color(self, color: Hsla) -> Self {
        self.color(color)
    }

    pub fn circle_inner_color(mut self, color: Hsla) -> Self {
        self.circle_inner_color = Some(color);
        self
    }

    pub fn inner_color(self, color: Hsla) -> Self {
        self.circle_inner_color(color)
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
                "aura-progress:{:?}:{:.3}:{:.3}:{:?}:{:?}:{}:{}:{}:{:?}:{:?}:{:?}:{:?}",
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
                self.circle_inner_color,
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
            let inner_color = self.circle_inner_color.unwrap_or(theme.neutral.card);
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
                            inner_color,
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
                    inner_color,
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
    inner_color: Hsla,
) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            let width = bounds.right() - bounds.left();
            let height = bounds.bottom() - bounds.top();
            let outer_radius = (width.min(height).as_f32() / 2.0).max(1.0);
            let ring_width = stroke_width.as_f32().clamp(1.0, outer_radius);
            let inner_radius = (outer_radius - ring_width).max(0.0);
            let center = point(bounds.left() + width / 2.0, bounds.top() + height / 2.0);

            // Fill annular geometry instead of stroking an arc.  GPUI path strokes
            // can expose hard pixel stair-steps on circular caps; polygonal ring
            // fills with sub-pixel feather layers produce noticeably smoother edges
            // while staying fully native to the GPUI renderer.
            paint_smooth_annular_sector(
                window,
                center,
                outer_radius,
                inner_radius,
                0.0,
                1.0,
                track_color,
            );
            paint_smooth_annular_sector(
                window,
                center,
                outer_radius,
                inner_radius,
                0.0,
                progress,
                progress_color,
            );

            if inner_radius > 0.0 {
                paint_smooth_circle(window, center, inner_radius, inner_color);
            }
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .w(size)
    .h(size)
}

fn paint_smooth_annular_sector(
    window: &mut Window,
    center: Point<Pixels>,
    outer_radius: f32,
    inner_radius: f32,
    start_progress: f32,
    end_progress: f32,
    color: Hsla,
) {
    let start = start_progress.clamp(0.0, 1.0);
    let end = end_progress.clamp(0.0, 1.0);
    if end <= start || outer_radius <= 0.0 || outer_radius <= inner_radius {
        return;
    }

    if let Some(path) = annular_sector_path(center, outer_radius, inner_radius, start, end) {
        window.paint_path(path, color);
    }

    // Feather the outside/inside edges with translucent 1px bands to soften the
    // native raster boundary without switching away from GPUI path rendering.
    let feather = 0.75;
    if let Some(path) = annular_sector_path(
        center,
        outer_radius + feather,
        outer_radius.max(inner_radius + 0.1),
        start,
        end,
    ) {
        window.paint_path(path, color.opacity(0.18));
    }
    if inner_radius > feather {
        if let Some(path) = annular_sector_path(
            center,
            inner_radius,
            (inner_radius - feather).max(0.0),
            start,
            end,
        ) {
            window.paint_path(path, color.opacity(0.14));
        }
    }
}

fn paint_smooth_circle(window: &mut Window, center: Point<Pixels>, radius: f32, color: Hsla) {
    if let Some(path) = circle_fill_path(center, radius) {
        window.paint_path(path, color);
    }
    if let Some(path) = annular_sector_path(center, radius + 0.75, radius, 0.0, 1.0) {
        window.paint_path(path, color.opacity(0.34));
    }
}

fn annular_sector_path(
    center: Point<Pixels>,
    outer_radius: f32,
    inner_radius: f32,
    start_progress: f32,
    end_progress: f32,
) -> Option<gpui::Path<Pixels>> {
    if !outer_radius.is_finite()
        || !inner_radius.is_finite()
        || outer_radius <= 0.0
        || inner_radius < 0.0
        || outer_radius <= inner_radius
    {
        return None;
    }

    let start_angle = -std::f32::consts::FRAC_PI_2 + start_progress * TAU;
    let end_angle = -std::f32::consts::FRAC_PI_2 + end_progress * TAU;
    let sweep = (end_angle - start_angle).clamp(0.0, TAU);
    if sweep <= f32::EPSILON {
        return None;
    }

    let segments = ring_segments(outer_radius, sweep);
    let mut outer_points = Vec::with_capacity(segments + 1);
    let mut inner_points = Vec::with_capacity(segments + 1);

    for index in 0..=segments {
        let t = index as f32 / segments as f32;
        let angle = start_angle + sweep * t;
        outer_points.push(polar_radians(center, outer_radius, angle));
        inner_points.push(polar_radians(center, inner_radius, angle));
    }

    let mut builder = PathBuilder::fill();
    builder.move_to(*outer_points.first()?);
    for point in outer_points.iter().skip(1) {
        builder.line_to(*point);
    }
    for point in inner_points.iter().rev() {
        builder.line_to(*point);
    }
    builder.close();
    builder.build().ok()
}

fn circle_fill_path(center: Point<Pixels>, radius: f32) -> Option<gpui::Path<Pixels>> {
    if radius <= 0.0 || !radius.is_finite() {
        return None;
    }

    let segments = ring_segments(radius, TAU);
    let start = polar_radians(center, radius, -std::f32::consts::FRAC_PI_2);
    let mut builder = PathBuilder::fill();
    builder.move_to(start);
    for index in 1..=segments {
        let angle = -std::f32::consts::FRAC_PI_2 + TAU * index as f32 / segments as f32;
        builder.line_to(polar_radians(center, radius, angle));
    }
    builder.close();
    builder.build().ok()
}

fn ring_segments(radius: f32, sweep_radians: f32) -> usize {
    // Roughly one segment per 0.55px of arc length, bounded to avoid oversized
    // paths on large charts while keeping small progress rings smooth.
    ((radius * sweep_radians.abs()) / 0.55)
        .ceil()
        .clamp(64.0, 960.0) as usize
}

fn polar_radians(center: Point<Pixels>, radius: f32, radians: f32) -> Point<Pixels> {
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
    fn progress_circle_builder_tracks_shape_size_and_ring_styles() {
        let progress = Progress::new(42.0)
            .circle()
            .circle_size(px(144.0))
            .ring_width(px(12.0))
            .ring_color(gpui::black())
            .progress_color(gpui::white())
            .inner_color(gpui::white().opacity(0.5));
        assert_eq!(progress.type_, ProgressType::Circle);
        assert_eq!(progress.circle_size, px(144.0));
        assert_eq!(progress.stroke_width, px(12.0));
        assert_eq!(progress.track_color, Some(gpui::black()));
        assert_eq!(progress.color, Some(gpui::white()));
        assert_eq!(
            progress.circle_inner_color,
            Some(gpui::white().opacity(0.5))
        );
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
        assert!(source.contains("PathBuilder::fill"));
        assert!(source.contains("paint_smooth_annular_sector"));
        assert!(source.contains("with_animation("));
        assert!(source.contains("render_circle_canvas"));
    }
}
