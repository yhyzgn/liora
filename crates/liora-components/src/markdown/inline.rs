//! Native word-flow for spans whose metrics cannot be expressed by GPUI TextRun.
use super::*;

// Keep every byte, including whitespace, in the selection scope. Newlines are
// separate full-width flex items rather than relying on flex to interpret text.
fn pieces(text: &str) -> impl Iterator<Item = Range<usize>> + '_ {
    let mut start = 0;
    std::iter::from_fn(move || {
        if start == text.len() {
            return None;
        }
        let first = text[start..].chars().next()?;
        let end = if first == '\n' {
            start + 1
        } else if matches!(first, '\u{00a0}' | '\u{202f}' | '\u{2011}') {
            text[start..]
                .char_indices()
                .find_map(|(offset, ch)| {
                    (ch == ' ' || ch == '\n' || ch.is_ascii_punctuation()).then_some(start + offset)
                })
                .unwrap_or(text.len())
        } else if first.is_ascii_alphanumeric()
            || matches!(first as u32, 0x00c0..=0x024f | 0x0400..=0x052f | 0x1e00..=0x1eff | 0x0300..=0x036f)
        {
            text[start..].char_indices().find_map(|(offset, ch)| {
                if offset == 0 { None } else if !(ch.is_ascii_alphanumeric() || ch.is_numeric() || matches!(ch, '\u{00a0}' | '\u{202f}' | '\u{2011}' | '_' | '-' | '\'' | '’') || matches!(ch as u32, 0x00c0..=0x024f | 0x0400..=0x052f | 0x1e00..=0x1eff | 0x0300..=0x036f)) { Some(start + offset) } else { None }
            }).unwrap_or(text.len())
        } else if first.is_whitespace() {
            start + first.len_utf8()
        } else {
            start + first.len_utf8()
        };
        let range = start..end;
        start = end;
        Some(range)
    })
}

fn leaf(
    options: SelectableTextOptions,
    fonts: &DocumentFonts,
    scope: &SharedString,
    last: bool,
    edit: Option<(u64, usize)>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    if let Some((id, start)) = edit {
        if let Some(activate) = &fonts.on_edit {
            let activate = activate.clone();
            return SelectableText::view_with_pointer_handler(
                options,
                Some(Rc::new(move |offset, window, cx| {
                    activate(id, start + offset, window, cx);
                })),
                window,
                cx,
            );
        }
    }
    let order = fonts.order.get();
    fonts.order.set(order + 1);
    SelectableText::view_in_scope(
        options,
        scope.clone(),
        order,
        if last { "\n".into() } else { "".into() },
        window,
        cx,
    )
}

pub(super) fn formula(
    mut options: SelectableTextOptions,
    display: bool,
    fonts: &DocumentFonts,
    scope: &SharedString,
    last: bool,
    edit: Option<(u64, usize)>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let diagnostic_id = options.id.clone();
    let source = options.text.clone();
    options.fill_width = false;
    options.wrap = crate::SelectableTextWrap::NoWrap;
    options.runs.truncate(1);
    if options.runs.is_empty() {
        options
            .runs
            .push(gpui::TextStyle::default().to_run(source.len()));
        options.runs[0].color = options.text_color;
    }
    options.runs[0].len = source.len();
    let foreground = gpui::Rgba::from(options.runs[0].color);
    let foreground = [foreground.r, foreground.g, foreground.b, foreground.a]
        .map(|channel| (channel * 255.0).round() as u8);
    match fonts.assets.render(
        super::super::assets::AssetKind::Math(display, foreground),
        &source,
    ) {
        Ok(image) => {
            let (mut width, mut height) = super::image_box_size(&image);
            let available_width = (window.viewport_size().width - px(32.0)).max(px(1.0));
            let scale = (available_width / width).min(1.0);
            width *= scale;
            height *= scale;
            // Select the original source over the image without painting TeX twice.
            options.text_color = gpui::transparent_black();
            let run = &mut options.runs[0];
            run.color = gpui::transparent_black();
            run.background_color = None;
            run.underline = None;
            run.strikethrough = None;
            let measured = window
                .text_system()
                .shape_text(source.clone(), options.font_size, &options.runs, None, None)
                .map(|lines| {
                    lines
                        .iter()
                        .map(|line| line.width())
                        .fold(px(0.0), |a, b| a.max(b))
                })
                .unwrap_or(px(0.0));
            if measured > px(0.0) {
                options.font_size = options.font_size * (width / measured);
            }
            options.line_height = height / source.split('\n').count() as f32;
            let selection = leaf(options, fonts, scope, last, edit, window, cx);
            div()
                .relative()
                .flex_none()
                .w_full()
                .max_w(width)
                .h(height)
                .child(
                    img(image)
                        .absolute()
                        .size_full()
                        .object_fit(ObjectFit::Contain),
                )
                .child(selection)
                .into_any_element()
        }
        Err(error) => {
            let source = leaf(options, fonts, scope, last, edit, window, cx);
            div()
                .flex()
                .flex_col()
                .child(source)
                .child(
                    crate::Text::new(format!("Native render failed: {error}"))
                        .id(format!("{diagnostic_id}-diagnostic"))
                        .text_color(cx.global::<Config>().theme.danger.base),
                )
                .into_any_element()
        }
    }
}

fn script_metrics(style: &super::super::model::Style, font_size: Pixels) -> (Pixels, Pixels) {
    if style.superscript {
        (font_size * 0.75, font_size * -0.3)
    } else if style.subscript {
        (font_size * 0.75, font_size * 0.2)
    } else {
        (font_size, px(0.0))
    }
}

pub(super) fn render(
    block: &Block,
    mut options: SelectableTextOptions,
    fonts: &DocumentFonts,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let scope = fonts
        .selection_scope
        .clone()
        .unwrap_or_else(|| SharedString::from(format!("markdown-inline:{:?}", options.id)));
    let mut flow = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .min_w_0()
        .w_full();
    flow = match options.text_align {
        gpui::TextAlign::Center => flow.justify_center(),
        gpui::TextAlign::Right => flow.justify_end(),
        _ => flow,
    };
    let runs = std::mem::take(&mut options.runs);
    let can_edit =
        matches!(block.kind, Kind::Paragraph | Kind::Heading(_)) && block.images.is_empty();
    let mut previous_newline = true;
    for (span_index, span) in block.spans.iter().enumerate() {
        let source = &block.text[span.range.clone()];
        let mut base = options.clone();
        base.fill_width = false;
        base.wrap = crate::SelectableTextWrap::NoWrap;
        base.text_align = gpui::TextAlign::Left;
        base.runs = vec![runs[span_index].clone()];
        if let Some(display) = span.style.math {
            base.id = SharedString::from(format!("{:?}-math-{span_index}", options.id)).into();
            base.text = source.to_owned().into();
            let last = span.range.end == block.text.len();
            previous_newline = false;
            let child = formula(
                base,
                display,
                fonts,
                &scope,
                last,
                can_edit.then_some((block.id, span.range.start)),
                window,
                cx,
            );
            if display {
                flow = flow.child(div().w_full().flex().justify_center().child(child));
            } else {
                flow = flow.child(child);
            }
            continue;
        }
        let (font_size, shift) = script_metrics(&span.style, options.font_size);
        base.font_size = font_size;
        for range in pieces(source) {
            let text = &source[range.clone()];
            let mut part = base.clone();
            part.id = SharedString::from(format!(
                "{:?}-span-{span_index}-{}",
                options.id, range.start
            ))
            .into();
            part.text = text.to_owned().into();
            part.runs[0].len = text.len();
            let last = span.range.start + range.end == block.text.len();
            let child = leaf(
                part,
                fonts,
                &scope,
                last,
                can_edit.then_some((block.id, span.range.start + range.start)),
                window,
                cx,
            );
            if text == "\n" {
                let height = if previous_newline {
                    options.line_height
                } else {
                    px(0.0)
                };
                previous_newline = true;
                flow = flow.child(div().w_full().h(height).overflow_hidden().child(child));
            } else {
                previous_newline = false;
                flow = flow.child(
                    div()
                        .relative()
                        .flex_none()
                        .h(options.line_height)
                        .child(div().relative().top(shift).child(child)),
                );
            }
        }
    }
    flow.into_any_element()
}

#[cfg(test)]
mod tests {
    #[test]
    fn cjk_has_breaks_between_characters_but_nbsp_stays_glued() {
        let text = "中文 prose\u{00a0}stays";
        let parts: Vec<_> = pieces(text).map(|range| &text[range]).collect();
        assert_eq!(parts, ["中", "文", " ", "prose\u{00a0}stays"]);
    }
    use super::{pieces, script_metrics};
    use crate::markdown::{MarkdownDocument, model::Style};
    use gpui::px;

    #[test]
    fn word_flow_preserves_all_unicode_whitespace_and_newlines() {
        let text = "前é  after\tword\n\nlast\r\n";
        let ranges: Vec<_> = pieces(text).collect();
        assert_eq!(
            ranges
                .iter()
                .map(|range| &text[range.clone()])
                .collect::<String>(),
            text
        );
        assert_eq!(
            ranges
                .iter()
                .filter(|range| &text[(*range).clone()] == "\n")
                .count(),
            3
        );
    }

    #[test]
    fn word_flow_has_no_span_or_word_limit() {
        let text = "x ".repeat(300);
        let ranges: Vec<_> = pieces(&text).collect();
        assert!(ranges.len() >= 300);
        assert_eq!(ranges.last().unwrap().end, text.len());
    }

    #[test]
    fn scripts_have_smaller_type_and_opposite_baseline_offsets() {
        let normal = script_metrics(&Style::default(), px(20.0));
        let superscript = script_metrics(
            &Style {
                superscript: true,
                ..Style::default()
            },
            px(20.0),
        );
        let subscript = script_metrics(
            &Style {
                subscript: true,
                ..Style::default()
            },
            px(20.0),
        );
        assert_eq!(normal, (px(20.0), px(0.0)));
        assert_eq!(superscript, (px(15.0), px(-6.0)));
        assert_eq!(subscript, (px(15.0), px(4.0)));
    }

    #[test]
    fn mixed_formula_script_flow_keeps_every_source_byte_past_128_spans() {
        let source = format!("{}tail", "before $x$ x^2^ H~2~O ".repeat(150));
        let document = MarkdownDocument::new(source);
        let block = &document.blocks[0];
        let mut reconstructed = String::new();
        let mut formulas = 0;
        for span in &block.spans {
            let text = &block.text[span.range.clone()];
            if span.style.math.is_some() {
                formulas += 1;
                reconstructed.push_str(text);
            } else {
                for range in pieces(text) {
                    reconstructed.push_str(&text[range]);
                }
            }
        }
        assert_eq!(formulas, 150);
        assert_eq!(reconstructed, block.text.as_ref());
        assert!(reconstructed.ends_with("tail"));
    }
}
