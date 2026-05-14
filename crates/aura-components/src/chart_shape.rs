use gpui::{PathBuilder, Pixels, Point, point, px};

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

pub fn smooth_line_path(
    points: &[Point<Pixels>],
    stroke_width: Pixels,
) -> Option<gpui::Path<Pixels>> {
    let first = *points.first()?;
    let mut builder = PathBuilder::stroke(stroke_width);
    builder.move_to(first);
    if points.len() == 2 {
        builder.line_to(points[1]);
        return builder.build().ok();
    }

    for index in 1..points.len() {
        let current = points[index];
        let prev = points[index.saturating_sub(1)];
        let next = points.get(index + 1).copied().unwrap_or(current);
        let ctrl1 = point(
            prev.x + (current.x - prev.x) * 0.5,
            prev.y + (current.y - prev.y) * 0.5,
        );
        let ctrl2 = point(
            current.x + (next.x - current.x) * 0.5,
            current.y + (next.y - current.y) * 0.5,
        );
        builder.curve_to(ctrl1, current);
        builder.curve_to(ctrl2, current);
    }
    builder.build().ok()
}

pub fn line_path(points: &[Point<Pixels>], stroke_width: Pixels) -> Option<gpui::Path<Pixels>> {
    let first = *points.first()?;
    let mut builder = PathBuilder::stroke(stroke_width);
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
}
