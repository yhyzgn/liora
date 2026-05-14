use gpui::{PathBuilder, Pixels, Point, point, px};

pub fn finite_line_points(points: impl IntoIterator<Item = (f32, f32)>) -> Vec<Point<Pixels>> {
    points
        .into_iter()
        .filter(|(x, y)| x.is_finite() && y.is_finite())
        .map(|(x, y)| point(px(x), px(y)))
        .collect()
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
    fn line_path_requires_at_least_one_point() {
        assert!(line_path(&[], px(2.0)).is_none());
        assert!(line_path(&[point(px(0.0), px(0.0)), point(px(1.0), px(1.0))], px(2.0)).is_some());
    }
}
