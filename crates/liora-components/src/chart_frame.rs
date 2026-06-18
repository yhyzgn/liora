use crate::chart::{ChartAxisLabel, ChartOptions, ChartPalette, default_y_format};
use crate::chart_scale::{ScaleLinear, ScalePoint};
use gpui::{
    App, Background, Hsla, Pixels, SharedString, TextAlign, TextRun, Window, fill, point, px, size,
};

#[allow(clippy::too_many_arguments)]
pub fn paint_chart_frame(
    left: Pixels,
    top: Pixels,
    width: Pixels,
    height: Pixels,
    labels: &[ChartAxisLabel],
    x: &ScalePoint,
    y: &ScaleLinear,
    palette: &ChartPalette,
    options: &ChartOptions,
    window: &mut Window,
    cx: &mut App,
) {
    let y_format = options.y_format.unwrap_or(default_y_format);
    for (value, y_pos) in y.ticks(options.y_tick_count) {
        let y_abs = top + px(y_pos);
        if options.show_grid {
            window.paint_quad(fill(
                gpui::Bounds::new(point(left, y_abs), size(width, px(1.0))),
                Background::from(palette.grid),
            ));
        }
        if options.show_axis {
            paint_chart_label(
                y_format(value),
                point(left - px(38.0), y_abs - px(7.0)),
                palette.label,
                window,
                cx,
            );
        }
    }

    if options.show_axis {
        window.paint_quad(fill(
            gpui::Bounds::new(point(left, top + height), size(width, px(1.0))),
            Background::from(palette.axis),
        ));
        window.paint_quad(fill(
            gpui::Bounds::new(point(left, top), size(px(1.0), height)),
            Background::from(palette.axis),
        ));

        for label in labels {
            if let Some(x_pos) = x.tick_index(label.index) {
                paint_chart_label(
                    label.label.clone(),
                    point(left + px(x_pos) - px(14.0), top + height + px(8.0)),
                    palette.label,
                    window,
                    cx,
                );
            }
        }
    }
}

pub fn paint_chart_label(
    text: SharedString,
    origin: gpui::Point<Pixels>,
    color: Hsla,
    window: &mut Window,
    cx: &mut App,
) {
    paint_chart_label_aligned(text, origin, color, TextAlign::Left, None, window, cx);
}

pub fn paint_chart_label_aligned(
    text: SharedString,
    origin: gpui::Point<Pixels>,
    color: Hsla,
    align: TextAlign,
    align_width: Option<Pixels>,
    window: &mut Window,
    cx: &mut App,
) {
    let run = TextRun {
        len: text.len(),
        font: window.text_style().font(),
        color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let line = window
        .text_system()
        .shape_line(text, px(11.0), &[run], None);
    let _ = line.paint(origin, px(14.0), align, align_width, window, cx);
}
