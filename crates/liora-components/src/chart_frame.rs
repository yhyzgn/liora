//! Chart Frame module.
//!
//! This crate-internal module implements the Liora internal chart frame renderer shared by chart components. It keeps the reusable
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

use crate::chart::{ChartAxisLabel, ChartOptions, ChartPalette, default_y_format};
use crate::chart_scale::{ScaleLinear, ScalePoint};
use gpui::{
    App, Background, Hsla, Pixels, SharedString, TextAlign, TextRun, Window, fill, point, px, size,
};
use liora_core::ui_font_family;

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
    _align: TextAlign,
    _align_width: Option<Pixels>,
    window: &mut Window,
    cx: &mut App,
) {
    let mut style = window.text_style();
    if let Some(family) = ui_font_family(cx) {
        style.font_family = family;
    }
    let run = TextRun {
        len: text.len(),
        font: style.font(),
        color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let line = window
        .text_system()
        .shape_line(text, px(11.0), &[run], None);
    let _ = line.paint(origin, px(14.0), gpui::TextAlign::Left, None, window, cx);
}
