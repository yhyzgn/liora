//! Shaped lines are shared by paint, pointer selection, movement, and IME queries.
use super::*;
use gpui::{
    AvailableSpace, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, LayoutId,
    Pixels, Point, Style, TextRun, WrappedLine, fill, point, size,
};
use std::cell::RefCell;

pub(super) fn is_special_text(block: &Block) -> bool {
    !block.text.is_empty()
        && (!block.images.is_empty()
            || block.spans.iter().any(|span| {
                let style = &span.style;
                style.math.is_some()
                    || style.footnote.is_some()
                    || style.image.is_some()
                    || style.superscript
                    || style.subscript
                    || style.link.is_some()
            })
            || matches!(block.kind, Kind::Math(_) | Kind::Metadata(_))
            || matches!(&block.kind, Kind::Code(language) if language.as_ref() == "mermaid"))
}

pub(super) fn is_native(block: &Block) -> bool {
    matches!(
        block.kind,
        Kind::Paragraph
            | Kind::Heading(_)
            | Kind::Code(_)
            | Kind::DefinitionTitle
            | Kind::DefinitionDefinition
    ) && !is_special_text(block)
}

pub(super) struct ShapedText {
    lines: Vec<WrappedLine>,
    starts: Vec<usize>,
    height: Pixels,
}

pub(super) struct TextLayout {
    pub id: u64,
    pub bounds: Bounds<Pixels>,
    pub line_height: Pixels,
    text: Rc<ShapedText>,
    special: Option<Rc<super::special::SpecialLayout>>,
    len: usize,
}
impl TextLayout {
    pub(super) fn special(
        id: u64,
        bounds: Bounds<Pixels>,
        line_height: Pixels,
        special: Rc<super::special::SpecialLayout>,
    ) -> Self {
        Self {
            id,
            bounds,
            line_height,
            len: 0,
            text: Rc::new(ShapedText {
                lines: Vec::new(),
                starts: Vec::new(),
                height: bounds.size.height,
            }),
            special: Some(special),
        }
    }
    pub fn position(&self, offset: usize) -> Point<Pixels> {
        if let Some(special) = &self.special {
            return self.bounds.origin + special.position(offset);
        }
        let mut y = self.bounds.top();
        for (line, start) in self.text.lines.iter().zip(&self.text.starts) {
            if offset <= start + line.len() {
                let index = offset.saturating_sub(*start).min(line.len());
                let mut local = line
                    .position_for_index(index, self.line_height)
                    .unwrap_or_default();
                for (row, boundary) in line.wrap_boundaries.iter().enumerate() {
                    if line.unwrapped_layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix].index
                        == index
                    {
                        local = point(px(0.), self.line_height * (row + 1) as f32);
                        break;
                    }
                }
                return point(self.bounds.left() + local.x, y + local.y);
            }
            y += line.size(self.line_height).height;
        }
        point(
            self.bounds.left(),
            (y - self.line_height).max(self.bounds.top()),
        )
    }
    pub fn index(&self, position: Point<Pixels>) -> usize {
        if let Some(special) = &self.special {
            return special.index(position - self.bounds.origin);
        }
        let mut y = self.bounds.top();
        for (line, start) in self.text.lines.iter().zip(&self.text.starts) {
            let height = line.size(self.line_height).height;
            if position.y < y + height {
                return (start
                    + line
                        .closest_index_for_position(
                            point(
                                position.x - self.bounds.left(),
                                (position.y - y).max(px(0.)),
                            ),
                            self.line_height,
                        )
                        .unwrap_or_else(|index| index))
                .min(self.len);
            }
            y += height;
        }
        self.len
    }
    pub fn range_bounds(&self, range: Range<usize>) -> Bounds<Pixels> {
        let start = self.position(range.start);
        let end = self.position(range.end);
        let right = if start.y == end.y {
            end.x
        } else {
            self.bounds.right()
        };
        Bounds::new(
            start,
            size((right - start.x).max(px(1.5)), self.line_height),
        )
    }
}

type Measured = Rc<RefCell<Option<(Pixels, Rc<ShapedText>)>>>;
pub(super) struct TextElement {
    id: u64,
    text: SharedString,
    source_range: Range<usize>,
    styles: Vec<(Range<usize>, super::super::model::Style)>,
    heading: Option<u8>,
    code: bool,
    editor: Entity<MarkdownEditor>,
}
impl TextElement {
    pub fn empty(editor: Entity<MarkdownEditor>) -> Self {
        Self {
            id: 0,
            text: SharedString::default(),
            source_range: 0..0,
            styles: Vec::new(),
            heading: None,
            code: false,
            editor,
        }
    }
    pub fn new(block: &Block, editor: Entity<MarkdownEditor>) -> Self {
        Self {
            id: block.id,
            text: block.text.clone(),
            source_range: block.source_range.clone(),
            styles: block
                .spans
                .iter()
                .map(|span| (span.range.clone(), span.style.clone()))
                .collect(),
            heading: if let Kind::Heading(level) = block.kind {
                Some(level)
            } else {
                None
            },
            code: matches!(
                block.kind,
                Kind::Code(_) | Kind::Math(_) | Kind::Metadata(_)
            ),
            editor,
        }
    }
}

fn shape(
    text: SharedString,
    runs: &[TextRun],
    font_size: Pixels,
    line_height: Pixels,
    width: Pixels,
    window: &Window,
) -> Rc<ShapedText> {
    let lines = window
        .text_system()
        .shape_text(text, font_size, runs, Some(width.max(px(1.))), None)
        .expect("native Markdown text shaping failed")
        .into_vec();
    let mut offset = 0;
    let mut height = px(0.);
    let starts = lines
        .iter()
        .map(|line| {
            let start = offset;
            offset += line.len() + 1;
            height += line.size(line_height).height;
            start
        })
        .collect();
    Rc::new(ShapedText {
        lines,
        starts,
        height: height.max(line_height),
    })
}

impl Element for TextElement {
    type RequestLayoutState = (Measured, Vec<TextRun>, Pixels, Pixels);
    type PrepaintState = Rc<TextLayout>;
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
    ) -> (LayoutId, Self::RequestLayoutState) {
        let theme = &cx.global::<liora_core::Config>().theme;
        let mut font = window.text_style().font();
        if let Some(family) = if self.code {
            Some(liora_core::code_font_family(cx))
        } else {
            liora_core::ui_font_family(cx)
        } {
            font.family = family;
        }
        if self.heading.is_some() {
            font.weight = gpui::FontWeight::BOLD;
        }
        let scale = self.heading.map_or(1., |level| match level {
            1 => 1.8,
            2 => 1.5,
            3 => 1.25,
            _ => 1.1,
        });
        let font_size = window.text_style().font_size.to_pixels(window.rem_size()) * scale;
        let line_height = window.line_height() * scale;
        let base = TextRun {
            len: self.text.len(),
            font,
            color: theme.neutral.text_1,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let mut boundaries = vec![0, self.text.len()];
        for (range, _) in &self.styles {
            boundaries.extend([
                range.start.min(self.text.len()),
                range.end.min(self.text.len()),
            ]);
        }
        boundaries.sort_unstable();
        boundaries.dedup();
        let mut runs = Vec::new();
        for pair in boundaries.windows(2) {
            if pair[0] == pair[1] {
                continue;
            }
            let mut run = base.clone();
            run.len = pair[1] - pair[0];
            for (_, style) in self
                .styles
                .iter()
                .filter(|(range, _)| range.start <= pair[0] && pair[1] <= range.end)
            {
                if style.strong {
                    run.font.weight = gpui::FontWeight::BOLD;
                }
                if style.emphasis {
                    run.font.style = gpui::FontStyle::Italic;
                }
                if style.code {
                    run.font.family = liora_core::code_font_family(cx);
                    run.background_color = Some(theme.neutral.hover);
                }
                if style.highlight {
                    run.background_color = Some(theme.primary.base.opacity(0.18));
                }
                if style.strike {
                    run.strikethrough = Some(gpui::StrikethroughStyle {
                        thickness: px(1.),
                        color: Some(run.color),
                    });
                }
                if style.link.is_some() {
                    run.color = theme.primary.base;
                }
            }
            runs.push(run);
        }
        if runs.is_empty() {
            runs.push(base);
        }
        let cache: Measured = Rc::default();
        let measured = cache.clone();
        let source = self.text.clone();
        let measured_runs = runs.clone();
        let mut style = Style::default();
        style.size.width = gpui::relative(1.).into();
        style.min_size.width = px(0.).into();
        let layout = window.request_measured_layout(style, move |known, available, window, _| {
            let width = known
                .width
                .unwrap_or_else(|| match available.width {
                    AvailableSpace::Definite(width) => width,
                    _ => px(640.),
                })
                .max(px(1.));
            let cached = measured
                .borrow()
                .as_ref()
                .filter(|(old, _)| *old == width)
                .map(|(_, text)| text.clone());
            let shaped = cached.unwrap_or_else(|| {
                shape(
                    source.clone(),
                    &measured_runs,
                    font_size,
                    line_height,
                    width,
                    window,
                )
            });
            let height = shaped.height;
            *measured.borrow_mut() = Some((width, shaped));
            size(width, height)
        });
        (layout, (cache, runs, font_size, line_height))
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        state: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let (cache, runs, font_size, line_height) = state;
        let width = bounds.size.width.max(px(1.));
        let cached = cache
            .borrow()
            .as_ref()
            .filter(|(old, _)| *old == width)
            .map(|(_, text)| text.clone());
        let text = cached.unwrap_or_else(|| {
            shape(
                self.text.clone(),
                runs,
                *font_size,
                *line_height,
                width,
                window,
            )
        });
        let layout = Rc::new(TextLayout {
            id: self.id,
            bounds,
            line_height: *line_height,
            text,
            len: self.text.len(),
            special: None,
        });
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
        layout
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        layout: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
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
        let focus =
            editor.focus.is_focused(window) && editor.cursor_visible && selection.is_empty();
        let caret = if self.id == 0 {
            Some(0)
        } else {
            editor
                .engine
                .display_position(selection.head)
                .filter(|(id, _)| *id == self.id)
                .map(|(_, offset)| offset)
        };
        let display_range = |range: Range<usize>| -> Option<Range<usize>> {
            if range.end <= self.source_range.start || range.start >= self.source_range.end {
                return None;
            }
            let start = if range.start <= self.source_range.start {
                0
            } else {
                editor
                    .engine
                    .display_position(range.start)
                    .filter(|(id, _)| *id == self.id)
                    .map_or(0, |(_, offset)| offset)
            };
            let end = if range.end >= self.source_range.end {
                self.text.len()
            } else {
                editor
                    .engine
                    .display_position(range.end)
                    .filter(|(id, _)| *id == self.id)
                    .map_or(self.text.len(), |(_, offset)| offset)
            };
            (start < end).then_some(start..end)
        };
        let ranges: Vec<_> = display_range(selection.range()).into_iter().collect();
        let marked: Vec<_> = editor
            .marked
            .clone()
            .and_then(display_range)
            .into_iter()
            .collect();
        let theme = cx.global::<liora_core::Config>().theme.clone();
        let mut y = bounds.top();
        for (line, start) in layout.text.lines.iter().zip(&layout.text.starts) {
            let mut row_start = 0;
            let ends = line
                .wrap_boundaries
                .iter()
                .map(|boundary| {
                    line.unwrapped_layout.runs[boundary.run_ix].glyphs[boundary.glyph_ix].index
                })
                .chain(std::iter::once(line.len()));
            for (row, end) in ends.enumerate() {
                let origin = line.unwrapped_layout.x_for_index(row_start);
                for (set, underline) in [(&ranges, false), (&marked, true)] {
                    for range in set {
                        let from = range.start.max(start + row_start);
                        let to = range.end.min(start + end);
                        if from >= to {
                            continue;
                        }
                        let x = line.unwrapped_layout.x_for_index(from - start) - origin;
                        let right = line.unwrapped_layout.x_for_index(to - start) - origin;
                        window.paint_quad(fill(
                            Bounds::new(
                                point(
                                    bounds.left() + x,
                                    y + layout.line_height * row as f32
                                        + if underline {
                                            layout.line_height - px(2.)
                                        } else {
                                            px(0.)
                                        },
                                ),
                                size(
                                    (right - x).max(px(1.)),
                                    if underline {
                                        px(2.)
                                    } else {
                                        layout.line_height
                                    },
                                ),
                            ),
                            theme
                                .primary
                                .base
                                .opacity(if underline { 0.9 } else { 0.26 }),
                        ));
                    }
                }
                row_start = end;
            }
            let _ = line.paint_background(
                point(bounds.left(), y),
                layout.line_height,
                gpui::TextAlign::Left,
                Some(bounds),
                window,
                cx,
            );
            let _ = line.paint(
                point(bounds.left(), y),
                layout.line_height,
                gpui::TextAlign::Left,
                Some(bounds),
                window,
                cx,
            );
            y += line.size(layout.line_height).height;
        }
        if focus {
            if let Some(offset) = caret {
                window.paint_quad(fill(
                    Bounds::new(layout.position(offset), size(px(1.5), layout.line_height)),
                    theme.primary.base,
                ));
            }
        }
    }
}
