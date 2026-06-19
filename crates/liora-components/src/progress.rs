//! Progress module.
//!
//! This public module implements the Liora linear and circular progress indicators. It keeps the reusable
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

use crate::gpui_compat::PixelsExt;
use crate::gpui_compat::element_id;
use crate::motion::{MotionDuration, MotionEasing, motion_animation};
use gpui::{
    AnimationExt, App, FillOptions, FontWeight, Hsla, IntoElement, ParentElement, PathBuilder,
    PathStyle, Pixels, Point, RenderOnce, SharedString, Styled, Window, canvas, div,
    linear_color_stop, linear_gradient, point, prelude::*, px,
};
use liora_core::{Config, stable_unique_id};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Enumerates the supported progress type modes and options.
pub enum ProgressType {
    #[default]
    /// Uses the line variant.
    Line,
    /// Uses the circle variant.
    Circle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerates the supported progress status modes and options.
pub enum ProgressStatus {
    /// Uses the success semantic button variant.
    Success,
    /// Uses the warning semantic button variant.
    Warning,
    /// Uses the exception variant.
    Exception,
}

/// Public builder and render state for the Liora progress component.
pub struct Progress {
    percentage: f32,
    type_: ProgressType,
    stroke_width: Pixels,
    status: Option<ProgressStatus>,
    color: Option<Hsla>,
    gradient: Option<Vec<Hsla>>,
    complete_color: Option<Hsla>,
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
    /// Creates a new value with the required baseline configuration.
    pub fn new(percentage: f32) -> Self {
        Self {
            percentage: percentage.clamp(0.0, 100.0),
            type_: ProgressType::Line,
            stroke_width: px(6.0),
            status: None,
            color: None,
            gradient: None,
            complete_color: None,
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

    /// Configures the type option.
    pub fn type_(mut self, t: ProgressType) -> Self {
        self.type_ = t;
        self
    }

    /// Configures the line option.
    pub fn line(mut self) -> Self {
        self.type_ = ProgressType::Line;
        self
    }

    /// Configures the circle option.
    pub fn circle(mut self) -> Self {
        self.type_ = ProgressType::Circle;
        self.stroke_width = px(8.0);
        self
    }

    /// Configures the stroke width option.
    pub fn stroke_width(mut self, w: impl Into<Pixels>) -> Self {
        self.stroke_width = w.into();
        self
    }

    /// Configures the ring width option.
    pub fn ring_width(self, width: impl Into<Pixels>) -> Self {
        self.stroke_width(width)
    }

    /// Configures the thick option.
    pub fn thick(self) -> Self {
        self.stroke_width(px(20.0))
    }

    /// Configures the status option.
    pub fn status(mut self, s: ProgressStatus) -> Self {
        self.status = Some(s);
        self
    }

    /// Configures the color option.
    pub fn color(mut self, c: Hsla) -> Self {
        self.color = Some(c);
        self.gradient = None;
        self.complete_color = None;
        self
    }

    /// Configures the primary option.
    pub fn primary(mut self) -> Self {
        self.color = None;
        self.gradient = None;
        self.complete_color = None;
        self.status = None;
        self
    }

    /// Configures the gradient option.
    pub fn gradient(mut self, colors: Vec<Hsla>) -> Self {
        self.gradient = if colors.is_empty() {
            None
        } else {
            Some(colors)
        };
        self.color = None;
        self
    }

    /// Configures the complete color option.
    pub fn complete_color(mut self, color: Hsla) -> Self {
        self.complete_color = Some(color);
        self
    }

    /// Configures whether text is visible in the rendered component.
    pub fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }

    /// Configures the text inside option.
    pub fn text_inside(mut self, inside: bool) -> Self {
        self.text_inside = inside;
        self
    }

    /// Configures the text inside center option.
    pub fn text_inside_center(mut self, center: bool) -> Self {
        self.text_inside_center = center;
        self
    }

    /// Configures the text inside centered option.
    pub fn text_inside_centered(mut self) -> Self {
        self.text_inside = true;
        self.text_inside_center = true;
        self
    }

    /// Configures the animated option.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Configures the circle size option.
    pub fn circle_size(mut self, size: impl Into<Pixels>) -> Self {
        self.circle_size = size.into();
        self
    }

    /// Configures the track color option.
    pub fn track_color(mut self, color: Hsla) -> Self {
        self.track_color = Some(color);
        self
    }

    /// Configures the ring color option.
    pub fn ring_color(self, color: Hsla) -> Self {
        self.track_color(color)
    }

    /// Configures the progress color option.
    pub fn progress_color(self, color: Hsla) -> Self {
        self.color(color)
    }

    /// Configures the circle inner color option.
    pub fn circle_inner_color(mut self, color: Hsla) -> Self {
        self.circle_inner_color = Some(color);
        self
    }

    /// Configures the inner color option.
    pub fn inner_color(self, color: Hsla) -> Self {
        self.circle_inner_color(color)
    }

    /// Configures the text option.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Configures the center text option.
    pub fn center_text(self, text: impl Into<SharedString>) -> Self {
        self.text(text)
    }

    /// Configures the text color option.
    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Configures the text size option.
    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    /// Configures the text weight option.
    pub fn text_weight(mut self, weight: FontWeight) -> Self {
        self.text_weight = weight;
        self
    }
}

fn render_gradient_segments(
    mut bar: gpui::Div,
    colors: Vec<Hsla>,
    complete_color: Option<Hsla>,
    progress: f32,
) -> gpui::Div {
    let mut colors = colors;
    if progress >= 0.999 {
        if let Some(color) = complete_color.or_else(|| colors.last().copied()) {
            if let Some(last) = colors.last_mut() {
                *last = color;
            } else {
                colors.push(color);
            }
        }
    }

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
                "liora-progress:{:?}:{:.3}:{:.3}:{:.3}:{:?}:{:?}:{:?}:{}:{}:{}:{}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}:{:?}",
                self.type_,
                self.percentage,
                self.stroke_width.as_f32(),
                self.circle_size.as_f32(),
                self.status,
                self.color,
                self.gradient,
                self.show_text,
                self.text_inside,
                self.text_inside_center,
                self.animated,
                self.text,
                self.text_size,
                self.text_color,
                self.text_weight,
                self.track_color,
                self.circle_inner_color,
                self.complete_color,
            ),
            "liora-progress",
            window,
            cx,
        );

        if self.type_ == ProgressType::Line {
            let target = self.percentage / 100.0;
            let inside_center = self.show_text && self.text_inside && self.text_inside_center;
            let center_text_color = if self.percentage >= 50.0 {
                theme.neutral.inverted
            } else {
                theme.neutral.text_2
            };
            let mut bar = div()
                .h_full()
                .rounded_full()
                .overflow_hidden()
                .when(gradient.is_none(), |s| s.bg(status_color))
                .when_some(gradient, |s, colors| {
                    render_gradient_segments(s, colors, self.complete_color, target)
                })
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
                                    .text_color(theme.neutral.inverted)
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
                    element_id(format!("{}-line-fill", id)),
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
                let progress_color = resolved_progress_color(
                    status_color,
                    gradient.as_deref(),
                    self.complete_color,
                    target,
                );
                let gradient = gradient.clone();
                let complete_color = self.complete_color;
                let center_text = percent_text.clone();
                base.with_animation(
                    element_id(format!("{}-circle-fill", id)),
                    motion_animation(MotionDuration::Normal, MotionEasing::EaseOut),
                    move |base, delta| {
                        let progress = target * delta.clamp(0.0, 1.0);
                        let base = base.child(render_circle_canvas(
                            progress,
                            circle_size,
                            stroke_width,
                            track_color,
                            progress_color,
                            gradient.clone(),
                            complete_color,
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
                    resolved_progress_color(
                        status_color,
                        gradient.as_deref(),
                        self.complete_color,
                        target,
                    ),
                    gradient,
                    self.complete_color,
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
    gradient: Option<Vec<Hsla>>,
    complete_color: Option<Hsla>,
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
            if let Some(colors) = gradient.as_deref() {
                paint_gradient_annular_sector(
                    window,
                    center,
                    outer_radius,
                    inner_radius,
                    progress,
                    colors,
                    complete_color,
                );
            } else {
                paint_smooth_annular_sector(
                    window,
                    center,
                    outer_radius,
                    inner_radius,
                    0.0,
                    progress,
                    progress_color,
                );
            }

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

fn resolved_progress_color(
    fallback: Hsla,
    gradient: Option<&[Hsla]>,
    complete_color: Option<Hsla>,
    target: f32,
) -> Hsla {
    if target >= 0.999 {
        complete_color
            .or_else(|| gradient.and_then(|colors| colors.last().copied()))
            .unwrap_or(fallback)
    } else {
        gradient
            .and_then(|colors| colors.first().copied())
            .unwrap_or(fallback)
    }
}

fn paint_gradient_annular_sector(
    window: &mut Window,
    center: Point<Pixels>,
    outer_radius: f32,
    inner_radius: f32,
    progress: f32,
    colors: &[Hsla],
    complete_color: Option<Hsla>,
) {
    let progress = progress.clamp(0.0, 1.0);
    if progress <= f32::EPSILON || colors.is_empty() {
        return;
    }
    if colors.len() == 1 {
        let color = if progress >= 0.999 {
            complete_color.unwrap_or(colors[0])
        } else {
            colors[0]
        };
        paint_smooth_annular_sector(
            window,
            center,
            outer_radius,
            inner_radius,
            0.0,
            progress,
            color,
        );
        return;
    }
    let segment_count = colors.len().saturating_sub(1).max(1);
    for index in 0..segment_count {
        let start = index as f32 / segment_count as f32;
        let end = (index + 1) as f32 / segment_count as f32;
        if start >= progress {
            break;
        }
        let segment_end = end.min(progress);
        let color = if progress >= 0.999 && index + 1 == segment_count {
            complete_color.unwrap_or(colors[index + 1])
        } else {
            colors[index].blend(colors[index + 1].opacity(0.62))
        };
        paint_smooth_annular_sector(
            window,
            center,
            outer_radius,
            inner_radius,
            start,
            segment_end,
            color,
        );
    }
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

    // Use lyon's native arc commands with a tighter tessellation tolerance instead
    // of a hand-sampled polygon. This keeps the ring boundary curved at the GPU
    // geometry level and avoids visible segment stair-steps on the circular edge.
    if let Some(path) = annular_sector_arc_path(center, outer_radius, inner_radius, start, end) {
        window.paint_path(path, color);
    }

    // A very thin translucent fringe blends the final raster edge into the
    // surrounding pixels. It is intentionally tiny so it smooths without making
    // the ring look blurry or changing the requested ring width.
    let feather = 0.45;
    if let Some(path) = annular_sector_arc_path(
        center,
        outer_radius + feather,
        outer_radius.max(inner_radius + 0.1),
        start,
        end,
    ) {
        window.paint_path(path, color.opacity(0.16));
    }
    if inner_radius > feather {
        if let Some(path) = annular_sector_arc_path(
            center,
            inner_radius,
            (inner_radius - feather).max(0.0),
            start,
            end,
        ) {
            window.paint_path(path, color.opacity(0.10));
        }
    }
}

fn paint_smooth_circle(window: &mut Window, center: Point<Pixels>, radius: f32, color: Hsla) {
    if let Some(path) = circle_fill_path(center, radius) {
        window.paint_path(path, color);
    }
    if let Some(path) = annular_sector_arc_path(center, radius + 0.45, radius, 0.0, 1.0) {
        window.paint_path(path, color.opacity(0.24));
    }
}

fn annular_sector_arc_path(
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

    let start_deg = -90.0 + start_progress.clamp(0.0, 1.0) * 360.0;
    let end_deg = -90.0 + end_progress.clamp(0.0, 1.0) * 360.0;
    let sweep_deg = (end_deg - start_deg).clamp(0.0, 360.0);
    if sweep_deg <= f32::EPSILON {
        return None;
    }

    if sweep_deg >= 359.999 {
        return ring_fill_path(center, outer_radius, inner_radius);
    }

    let outer_start = polar_degrees(center, outer_radius, start_deg);
    let outer_end = polar_degrees(center, outer_radius, end_deg);
    let inner_start = polar_degrees(center, inner_radius, start_deg);
    let inner_end = polar_degrees(center, inner_radius, end_deg);
    let large_arc = sweep_deg > 180.0;
    let mut builder = high_quality_fill_builder();
    builder.move_to(outer_start);
    builder.arc_to(
        point(px(outer_radius), px(outer_radius)),
        px(0.0),
        large_arc,
        true,
        outer_end,
    );
    builder.line_to(inner_end);
    builder.arc_to(
        point(px(inner_radius), px(inner_radius)),
        px(0.0),
        large_arc,
        false,
        inner_start,
    );
    builder.close();
    builder.build().ok()
}

fn ring_fill_path(
    center: Point<Pixels>,
    outer_radius: f32,
    inner_radius: f32,
) -> Option<gpui::Path<Pixels>> {
    if outer_radius <= 0.0 || inner_radius < 0.0 || outer_radius <= inner_radius {
        return None;
    }

    let outer_top = polar_degrees(center, outer_radius, -90.0);
    let outer_bottom = polar_degrees(center, outer_radius, 90.0);
    let inner_top = polar_degrees(center, inner_radius, -90.0);
    let inner_bottom = polar_degrees(center, inner_radius, 90.0);

    let mut builder = high_quality_fill_builder();
    builder.move_to(outer_top);
    builder.arc_to(
        point(px(outer_radius), px(outer_radius)),
        px(0.0),
        false,
        true,
        outer_bottom,
    );
    builder.arc_to(
        point(px(outer_radius), px(outer_radius)),
        px(0.0),
        false,
        true,
        outer_top,
    );
    builder.line_to(inner_top);
    builder.arc_to(
        point(px(inner_radius), px(inner_radius)),
        px(0.0),
        false,
        false,
        inner_bottom,
    );
    builder.arc_to(
        point(px(inner_radius), px(inner_radius)),
        px(0.0),
        false,
        false,
        inner_top,
    );
    builder.close();
    builder.build().ok()
}

fn circle_fill_path(center: Point<Pixels>, radius: f32) -> Option<gpui::Path<Pixels>> {
    if radius <= 0.0 || !radius.is_finite() {
        return None;
    }

    let top = polar_degrees(center, radius, -90.0);
    let bottom = polar_degrees(center, radius, 90.0);
    let mut builder = high_quality_fill_builder();
    builder.move_to(top);
    builder.arc_to(point(px(radius), px(radius)), px(0.0), false, true, bottom);
    builder.arc_to(point(px(radius), px(radius)), px(0.0), false, true, top);
    builder.close();
    builder.build().ok()
}

fn high_quality_fill_builder() -> PathBuilder {
    PathBuilder::fill().with_style(PathStyle::Fill(FillOptions::default().with_tolerance(0.01)))
}

fn polar_degrees(center: Point<Pixels>, radius: f32, degrees: f32) -> Point<Pixels> {
    let radians = degrees.to_radians();
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
            .inner_color(gpui::white().opacity(0.5))
            .gradient(vec![gpui::blue(), gpui::green()])
            .complete_color(gpui::green());
        assert_eq!(progress.type_, ProgressType::Circle);
        assert_eq!(progress.circle_size, px(144.0));
        assert_eq!(progress.stroke_width, px(12.0));
        assert_eq!(progress.track_color, Some(gpui::black()));
        assert_eq!(progress.gradient, Some(vec![gpui::blue(), gpui::green()]));
        assert_eq!(progress.complete_color, Some(gpui::green()));
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
    fn progress_clamps_percentage_to_valid_range() {
        assert_eq!(Progress::new(-12.0).percentage, 0.0);
        assert_eq!(Progress::new(128.0).percentage, 100.0);
    }

    #[test]
    fn progress_gradient_complete_color_resolution_matches_completion_state() {
        let fallback = gpui::black();
        let colors = [gpui::blue(), gpui::green()];
        assert_eq!(
            resolved_progress_color(fallback, Some(&colors), Some(gpui::red()), 1.0),
            gpui::red()
        );
        assert_eq!(
            resolved_progress_color(fallback, Some(&colors), None, 1.0),
            gpui::green()
        );
        assert_eq!(
            resolved_progress_color(fallback, Some(&colors), Some(gpui::red()), 0.5),
            gpui::blue()
        );
    }

    #[test]
    fn progress_custom_text_tracks_style() {
        let progress = Progress::new(86.0)
            .circle()
            .center_text("Deploy")
            .text_color(gpui::white())
            .text_size(px(22.0))
            .text_weight(FontWeight::NORMAL);
        assert_eq!(
            progress.text.as_ref().map(|text| text.as_ref()),
            Some("Deploy")
        );
        assert_eq!(progress.text_size, Some(px(22.0)));
        assert_eq!(progress.text_weight, FontWeight::NORMAL);
    }

    #[test]
    fn progress_uses_native_paths_and_animation() {
        let source = include_str!("progress.rs");
        assert!(source.contains("PathBuilder::fill"));
        assert!(source.contains("arc_to("));
        assert!(source.contains("high_quality_fill_builder"));
        assert!(source.contains("paint_smooth_annular_sector"));
        assert!(source.contains("with_animation("));
        assert!(source.contains("render_circle_canvas"));
        assert!(source.contains("paint_gradient_annular_sector"));
        assert!(source.contains("complete_color"));
        assert!(
            source.contains("render_gradient_segments(s, colors, self.complete_color, target)")
        );
    }
}
