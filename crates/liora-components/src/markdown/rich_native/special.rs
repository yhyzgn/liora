//! Inline assets and scripts share one piece layout with document selection and IME.
use super::*;
use gpui::{
    AvailableSpace, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, LayoutId,
    Pixels, Point, ShapedLine, Style, TextRun, fill, point, size,
};
use std::{cell::RefCell, sync::Arc};
use unicode_segmentation::UnicodeSegmentation;

struct Piece {
    range: Range<usize>,
    origin: Point<Pixels>,
    width: Pixels,
    height: Pixels,
    shaped: ShapedLine,
    scale_x: f32,
    shift: Pixels,
    image: Option<Arc<gpui::RenderImage>>,
}
pub(super) struct SpecialLayout {
    pieces: Vec<Piece>,
    height: Pixels,
}
impl SpecialLayout {
    pub(super) fn position(&self, offset: usize) -> Point<Pixels> {
        let piece = self
            .pieces
            .iter()
            .find(|piece| piece.range.start <= offset && offset < piece.range.end)
            .or_else(|| self.pieces.last());
        let Some(piece) = piece else {
            return Point::default();
        };
        let index = offset
            .saturating_sub(piece.range.start)
            .min(piece.shaped.len());
        point(
            piece.origin.x + piece.shaped.x_for_index(index) * piece.scale_x,
            piece.origin.y,
        )
    }
    pub(super) fn index(&self, position: Point<Pixels>) -> usize {
        let piece = self.pieces.iter().min_by(|a, b| {
            let distance = |piece: &Piece| {
                let y = if position.y < piece.origin.y {
                    piece.origin.y - position.y
                } else if position.y >= piece.origin.y + piece.height {
                    position.y - piece.origin.y - piece.height + px(1.)
                } else {
                    px(0.)
                };
                let x = if position.x < piece.origin.x {
                    piece.origin.x - position.x
                } else if position.x > piece.origin.x + piece.width {
                    position.x - piece.origin.x - piece.width
                } else {
                    px(0.)
                };
                y * 10000. + x
            };
            distance(a)
                .partial_cmp(&distance(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let Some(piece) = piece else {
            return 0;
        };
        piece.range.start
            + piece
                .shaped
                .closest_index_for_x((position.x - piece.origin.x) / piece.scale_x)
                .min(piece.range.len())
    }
}

pub(super) struct SpecialElement {
    id: u64,
    text: SharedString,
    range: Range<usize>,
    spans: Vec<(Range<usize>, super::super::model::Style)>,
    editor: Entity<MarkdownEditor>,
}
impl SpecialElement {
    pub(super) fn new(block: &Block, editor: Entity<MarkdownEditor>) -> Self {
        Self {
            id: block.id,
            text: block.text.clone(),
            range: block.source_range.clone(),
            spans: block
                .spans
                .iter()
                .map(|span| (span.range.clone(), span.style.clone()))
                .collect(),
            editor,
        }
    }
    pub(super) fn shape(&self, width: Pixels, window: &Window, cx: &mut App) -> Rc<SpecialLayout> {
        let theme = cx.global::<liora_core::Config>().theme.clone();
        let mut base = window.text_style().font();
        if let Some(family) = liora_core::ui_font_family(cx) {
            base.family = family;
        }
        let font_size = window.text_style().font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();
        let mut pieces = Vec::new();
        let mut x = px(0.);
        let mut y = px(0.);
        let mut row_height = line_height;
        for (range, style) in &self.spans {
            let Some(text) = self.text.get(range.clone()) else {
                continue;
            };
            let ranges: Vec<_> = if style.math.is_some() {
                vec![0..text.len()]
            } else {
                text.split_word_bound_indices()
                    .map(|(start, word)| start..start + word.len())
                    .collect()
            };
            let mut remaining: std::collections::VecDeque<_> = ranges.into();
            while let Some(part) = remaining.pop_front() {
                let value = &text[part.clone()];
                if value == "\n" || value == "\r\n" {
                    pieces.push(Piece {
                        range: range.start + part.start..range.start + part.end,
                        origin: point(x, y),
                        width: px(0.),
                        height: row_height,
                        shaped: window
                            .text_system()
                            .shape_line("".into(), font_size, &[], None),
                        scale_x: 1.,
                        shift: px(0.),
                        image: None,
                    });
                    x = px(0.);
                    y += row_height;
                    row_height = line_height;
                    continue;
                }
                let mut font = base.clone();
                if style.strong {
                    font.weight = gpui::FontWeight::BOLD;
                }
                if style.emphasis {
                    font.style = gpui::FontStyle::Italic;
                }
                if style.code {
                    font.family = liora_core::code_font_family(cx);
                }
                let safe_link = style.link.as_deref().is_some_and(|link| {
                    super::super::model::is_safe_markdown_link(link) || link.starts_with('#')
                });
                let color = if safe_link || style.footnote.is_some() {
                    theme.primary.base
                } else {
                    theme.neutral.text_1
                };
                let run = TextRun {
                    len: value.len(),
                    font,
                    color,
                    background_color: (style.code || style.highlight)
                        .then_some(theme.neutral.hover),
                    underline: safe_link.then_some(gpui::UnderlineStyle {
                        thickness: px(1.),
                        color: Some(color),
                        wavy: false,
                    }),
                    strikethrough: style.strike.then_some(gpui::StrikethroughStyle {
                        thickness: px(1.),
                        color: Some(color),
                    }),
                };
                let script = style.superscript || style.subscript;
                let shaped = window.text_system().shape_line(
                    value.to_owned().into(),
                    font_size * if script { 0.75 } else { 1. },
                    std::slice::from_ref(&run),
                    None,
                );
                if style.math.is_none() && shaped.width > width {
                    let graphemes: Vec<_> = value.grapheme_indices(true).collect();
                    if graphemes.len() > 1 {
                        let mut end = graphemes[1].0;
                        for &(index, _) in graphemes.iter().skip(2) {
                            if shaped.x_for_index(index) > width {
                                break;
                            }
                            end = index;
                        }
                        remaining.push_front(part.start + end..part.end);
                        remaining.push_front(part.start..part.start + end);
                        continue;
                    }
                }
                let mut piece_width = shaped.width;
                let mut height = line_height;
                let mut image = None;
                if let Some(display) = style.math {
                    let rgba = gpui::Rgba::from(color);
                    let foreground =
                        [rgba.r, rgba.g, rgba.b, rgba.a].map(|c| (c * 255.).round() as u8);
                    let assets = self.editor.read(cx).assets.clone();
                    match assets.render(assets::AssetKind::Math(display, foreground), value) {
                        Ok(rendered) => {
                            let image_size = rendered.size(0);
                            let image_width = i32::from(image_size.width).max(1) as f32;
                            let image_height = i32::from(image_size.height).max(1) as f32;
                            let scale = (f32::from(width) / image_width).min(1.);
                            piece_width = px(image_width * scale);
                            height = px(image_height * scale).max(line_height);
                            image = Some(rendered);
                        }
                        Err(error) => {
                            self.editor.update(cx, |editor, _| {
                                editor.error =
                                    Some(format!("Native formula render failed: {error}").into())
                            });
                        }
                    }
                }
                if x > px(0.) && x + piece_width > width {
                    x = px(0.);
                    y += row_height;
                    row_height = line_height;
                }
                row_height = row_height.max(height);
                let shift = if style.superscript {
                    -line_height * 0.3
                } else if style.subscript {
                    line_height * 0.2
                } else {
                    px(0.)
                };
                let scale_x = if image.is_some() && shaped.width > px(0.) {
                    piece_width / shaped.width
                } else {
                    1.
                };
                pieces.push(Piece {
                    range: range.start + part.start..range.start + part.end,
                    origin: point(x, y),
                    width: piece_width,
                    height,
                    shaped,
                    scale_x,
                    shift,
                    image,
                });
                x += piece_width;
            }
        }
        pieces.push(Piece {
            range: self.text.len()..self.text.len(),
            origin: point(x, y),
            width: px(0.),
            height: row_height,
            shaped: window
                .text_system()
                .shape_line("".into(), font_size, &[], None),
            scale_x: 1.,
            shift: px(0.),
            image: None,
        });
        Rc::new(SpecialLayout {
            pieces,
            height: y + row_height,
        })
    }
}
type Cache = Rc<RefCell<Option<(Pixels, Rc<SpecialLayout>)>>>;
impl Element for SpecialElement {
    type RequestLayoutState = Cache;
    type PrepaintState = (Rc<SpecialLayout>, Rc<text::TextLayout>);
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Cache) {
        let cache: Cache = Rc::default();
        let measured = cache.clone();
        let element = Self {
            id: self.id,
            text: self.text.clone(),
            range: self.range.clone(),
            spans: self.spans.clone(),
            editor: self.editor.clone(),
        };
        let mut style = Style::default();
        style.size.width = gpui::relative(1.).into();
        style.min_size.width = px(0.).into();
        let id = window.request_measured_layout(style, move |known, available, window, cx| {
            let width = known
                .width
                .unwrap_or_else(|| match available.width {
                    AvailableSpace::Definite(width) => width,
                    _ => px(640.),
                })
                .max(px(1.));
            let previous = measured
                .borrow()
                .as_ref()
                .filter(|(old, _)| *old == width)
                .map(|(_, layout)| layout.clone());
            let layout = previous.unwrap_or_else(|| element.shape(width, window, cx));
            let height = layout.height;
            *measured.borrow_mut() = Some((width, layout));
            size(width, height)
        });
        let _ = cx;
        (id, cache)
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        cache: &mut Cache,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let previous = cache
            .borrow()
            .as_ref()
            .filter(|(width, _)| *width == bounds.size.width)
            .map(|(_, layout)| layout.clone());
        let special =
            previous.unwrap_or_else(|| self.shape(bounds.size.width.max(px(1.)), window, cx));
        let layout = Rc::new(text::TextLayout::special(
            self.id,
            bounds,
            window.line_height(),
            special.clone(),
        ));
        self.editor.update(cx, |editor, cx| {
            editor.resolve_vertical(&layout, window, cx);
            if editor.reveal_pending {
                if let Some((id, offset)) = editor
                    .engine
                    .display_position(editor.engine.selection().head)
                {
                    if id == self.id {
                        window.request_autoscroll(layout.range_bounds(offset..offset));
                        editor.reveal_pending = false;
                    }
                }
            }
        });
        (special, layout)
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Cache,
        state: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (special, layout) = state;
        self.editor.update(cx, |editor, cx| {
            editor.layouts.insert(self.id, layout.clone());
            if editor
                .engine
                .display_position(editor.engine.selection().head)
                .is_some_and(|(id, _)| id == self.id)
            {
                if let Some((down, extend)) = editor.pending_vertical_retry.take() {
                    editor.vertical(down, extend, cx);
                }
            }
        });
        let editor = self.editor.read(cx);
        let selection = editor.engine.selection();
        let marked = editor.marked.clone();
        let focus =
            editor.focus.is_focused(window) && editor.cursor_visible && selection.is_empty();
        let caret = editor
            .engine
            .display_position(selection.head)
            .filter(|(id, _)| *id == self.id)
            .map(|(_, offset)| offset);
        let selected = |range: Range<usize>| {
            let start = if range.start <= self.range.start {
                0
            } else {
                editor
                    .engine
                    .display_position(range.start)
                    .filter(|(id, _)| *id == self.id)
                    .map_or(self.text.len(), |(_, offset)| offset)
            };
            let end = if range.end >= self.range.end {
                self.text.len()
            } else {
                editor
                    .engine
                    .display_position(range.end)
                    .filter(|(id, _)| *id == self.id)
                    .map_or(0, |(_, offset)| offset)
            };
            start..end
        };
        let selected_range = selected(selection.range());
        let marked_range = marked.map(selected);
        let theme = cx.global::<liora_core::Config>().theme.clone();
        let mut paint_error = None;
        for piece in &special.pieces {
            let origin = bounds.origin + piece.origin;
            if let Some(image) = &piece.image {
                let image_bounds = Bounds::new(origin, size(piece.width, piece.height));
                if let Err(error) = window.paint_image(
                    image_bounds,
                    image_bounds,
                    gpui::Corners::all(px(0.)),
                    image.clone(),
                    0,
                    false,
                ) {
                    paint_error = Some(format!("Native formula paint failed: {error}"));
                }
            } else {
                let _ = piece.shaped.paint_background(
                    point(origin.x, origin.y + piece.shift),
                    piece.height,
                    gpui::TextAlign::Left,
                    None,
                    window,
                    cx,
                );
            }
            for (range, underline) in [
                (Some(selected_range.clone()), false),
                (marked_range.clone(), true),
            ] {
                if let Some(range) = range {
                    let start = range.start.max(piece.range.start);
                    let end = range.end.min(piece.range.end);
                    if start < end {
                        let x = piece.shaped.x_for_index(start - piece.range.start) * piece.scale_x;
                        let right =
                            piece.shaped.x_for_index(end - piece.range.start) * piece.scale_x;
                        window.paint_quad(fill(
                            Bounds::new(
                                point(
                                    origin.x + x,
                                    origin.y
                                        + if underline {
                                            piece.height - px(2.)
                                        } else {
                                            px(0.)
                                        },
                                ),
                                size(
                                    (right - x).max(px(1.)),
                                    if underline { px(2.) } else { piece.height },
                                ),
                            ),
                            theme
                                .primary
                                .base
                                .opacity(if underline { 0.9 } else { 0.26 }),
                        ));
                    }
                }
            }
            if piece.image.is_none() {
                let _ = piece.shaped.paint(
                    point(origin.x, origin.y + piece.shift),
                    piece.height,
                    gpui::TextAlign::Left,
                    None,
                    window,
                    cx,
                );
            }
        }
        if focus {
            if let Some(offset) = caret {
                window.paint_quad(fill(
                    layout.range_bounds(offset..offset),
                    theme.primary.base,
                ));
            }
        }
        if let Some(error) = paint_error {
            self.editor.update(cx, |editor, cx| {
                let error: SharedString = error.into();
                if editor.error.as_ref() != Some(&error) {
                    editor.error = Some(error);
                    cx.notify();
                }
            });
        }
    }
}
