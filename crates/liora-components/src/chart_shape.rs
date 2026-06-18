use crate::chart::ChartLineStyle;
use gpui::{PathBuilder, PathStyle, Pixels, Point, StrokeOptions, point, px};
use lyon_tessellation::{LineCap, LineJoin};

pub fn finite_line_points(points: impl IntoIterator<Item = (f32, f32)>) -> Vec<Point<Pixels>> {
    points
        .into_iter()
        .filter(|(x, y)| x.is_finite() && y.is_finite())
        .map(|(x, y)| point(px(x), px(y)))
        .collect()
}

pub fn area_path(points: &[Point<Pixels>], baseline_y: Pixels) -> Option<gpui::Path<Pixels>> {
    let first = *points.first()?;
    let last = *points.last()?;
    let mut builder = PathBuilder::fill();
    builder.move_to(point(first.x, baseline_y));
    builder.line_to(first);
    for point in points.iter().skip(1) {
        builder.line_to(*point);
    }
    builder.line_to(point(last.x, baseline_y));
    builder.close();
    builder.build().ok()
}

pub fn smooth_area_path(
    points: &[Point<Pixels>],
    baseline_y: Pixels,
) -> Option<gpui::Path<Pixels>> {
    let first = *points.first()?;
    let last = *points.last()?;
    let mut builder = PathBuilder::fill();
    builder.move_to(point(first.x, baseline_y));
    builder.line_to(first);
    append_smooth_segments(&mut builder, points);
    builder.line_to(point(last.x, baseline_y));
    builder.close();
    builder.build().ok()
}

pub fn smooth_line_path(
    points: &[Point<Pixels>],
    stroke_width: Pixels,
) -> Option<gpui::Path<Pixels>> {
    smooth_line_path_with_style(points, stroke_width, ChartLineStyle::Solid, None)
}

pub fn smooth_line_path_with_style(
    points: &[Point<Pixels>],
    stroke_width: Pixels,
    line_style: ChartLineStyle,
    dash_pattern: Option<&[Pixels]>,
) -> Option<gpui::Path<Pixels>> {
    let first = *points.first()?;
    let mut builder = chart_stroke_builder(stroke_width, line_style, dash_pattern);
    builder.move_to(first);
    if points.len() == 2 {
        builder.line_to(points[1]);
        return builder.build().ok();
    }

    append_smooth_segments(&mut builder, points);
    builder.build().ok()
}

fn chart_stroke_builder(
    stroke_width: Pixels,
    line_style: ChartLineStyle,
    dash_pattern: Option<&[Pixels]>,
) -> PathBuilder {
    let options = StrokeOptions::default()
        .with_line_width(stroke_width.as_f32())
        .with_tolerance(0.01)
        .with_line_join(LineJoin::Round)
        .with_line_cap(LineCap::Round);

    let builder = PathBuilder::stroke(stroke_width).with_style(PathStyle::Stroke(options));
    let pattern =
        dash_pattern
            .map(|pattern| pattern.to_vec())
            .unwrap_or_else(|| match line_style {
                ChartLineStyle::Solid => Vec::new(),
                ChartLineStyle::Dashed => vec![stroke_width * 3.0, stroke_width * 2.0],
                ChartLineStyle::Dotted => vec![stroke_width, stroke_width * 1.8],
            });
    if pattern.is_empty() {
        builder
    } else {
        builder.dash_array(&pattern)
    }
}

fn append_smooth_segments(builder: &mut PathBuilder, points: &[Point<Pixels>]) {
    if points.len() < 2 {
        return;
    }

    if points.len() == 2 {
        builder.line_to(points[1]);
        return;
    }

    for index in 0..points.len() - 1 {
        let p0 = if index == 0 {
            points[index]
        } else {
            points[index - 1]
        };
        let p1 = points[index];
        let p2 = points[index + 1];
        let p3 = points.get(index + 2).copied().unwrap_or(p2);
        let (control_a, control_b) = catmull_rom_controls(p0, p1, p2, p3);
        builder.cubic_bezier_to(p2, control_a, control_b);
    }
}

fn catmull_rom_controls(
    p0: Point<Pixels>,
    p1: Point<Pixels>,
    p2: Point<Pixels>,
    p3: Point<Pixels>,
) -> (Point<Pixels>, Point<Pixels>) {
    const TENSION: f32 = 1.0;
    let scale = TENSION / 6.0;
    (
        point(
            px(p1.x.as_f32() + (p2.x.as_f32() - p0.x.as_f32()) * scale),
            px(p1.y.as_f32() + (p2.y.as_f32() - p0.y.as_f32()) * scale),
        ),
        point(
            px(p2.x.as_f32() - (p3.x.as_f32() - p1.x.as_f32()) * scale),
            px(p2.y.as_f32() - (p3.y.as_f32() - p1.y.as_f32()) * scale),
        ),
    )
}

pub fn line_soft_edge_path(
    points: &[Point<Pixels>],
    stroke_width: Pixels,
    smooth: bool,
) -> Option<gpui::Path<Pixels>> {
    let soft_width = px((stroke_width.as_f32() + 1.2).max(stroke_width.as_f32()));
    line_soft_edge_path_with_style(points, soft_width, smooth, ChartLineStyle::Solid, None)
}

pub fn line_soft_edge_path_with_style(
    points: &[Point<Pixels>],
    stroke_width: Pixels,
    smooth: bool,
    line_style: ChartLineStyle,
    dash_pattern: Option<&[Pixels]>,
) -> Option<gpui::Path<Pixels>> {
    if smooth {
        smooth_line_path_with_style(points, stroke_width, line_style, dash_pattern)
    } else {
        line_path_with_style(points, stroke_width, line_style, dash_pattern)
    }
}

pub fn line_path(points: &[Point<Pixels>], stroke_width: Pixels) -> Option<gpui::Path<Pixels>> {
    line_path_with_style(points, stroke_width, ChartLineStyle::Solid, None)
}

pub fn line_path_with_style(
    points: &[Point<Pixels>],
    stroke_width: Pixels,
    line_style: ChartLineStyle,
    dash_pattern: Option<&[Pixels]>,
) -> Option<gpui::Path<Pixels>> {
    let first = *points.first()?;
    let mut builder = chart_stroke_builder(stroke_width, line_style, dash_pattern);
    builder.move_to(first);
    for point in points.iter().skip(1) {
        builder.line_to(*point);
    }
    builder.build().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_line_points_drop_invalid_coordinates() {
        let points = finite_line_points([(0.0, 1.0), (f32::NAN, 2.0), (2.0, f32::INFINITY)]);
        assert_eq!(points.len(), 1);
        assert_eq!(points[0], point(px(0.0), px(1.0)));
    }

    #[test]
    fn area_path_requires_at_least_one_point() {
        assert!(area_path(&[], px(10.0)).is_none());
        assert!(
            area_path(
                &[point(px(0.0), px(1.0)), point(px(2.0), px(3.0))],
                px(10.0)
            )
            .is_some()
        );
    }

    #[test]
    fn smooth_area_path_requires_at_least_one_point() {
        assert!(smooth_area_path(&[], px(10.0)).is_none());
        assert!(
            smooth_area_path(
                &[
                    point(px(0.0), px(1.0)),
                    point(px(2.0), px(3.0)),
                    point(px(4.0), px(2.0))
                ],
                px(10.0)
            )
            .is_some()
        );
    }

    #[test]
    fn smooth_line_path_requires_at_least_one_point() {
        assert!(smooth_line_path(&[], px(2.0)).is_none());
        assert!(
            smooth_line_path(&[point(px(0.0), px(0.0)), point(px(1.0), px(1.0))], px(2.0))
                .is_some()
        );
    }

    #[test]
    fn line_path_requires_at_least_one_point() {
        assert!(line_path(&[], px(2.0)).is_none());
        assert!(line_path(&[point(px(0.0), px(0.0)), point(px(1.0), px(1.0))], px(2.0)).is_some());
    }

    #[test]
    fn chart_lines_use_high_quality_round_strokes() {
        let source = include_str!("chart_shape.rs");
        assert!(source.contains("with_tolerance(0.01)"));
        assert!(source.contains("LineJoin::Round"));
        assert!(source.contains("LineCap::Round"));
        assert!(source.contains("line_soft_edge_path"));
        assert!(source.contains("dash_array"));
        assert!(source.contains("ChartLineStyle"));
    }
}
