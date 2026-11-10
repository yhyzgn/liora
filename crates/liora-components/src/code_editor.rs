//! Code Editor module.
//!
//! This public module implements the Liora native code editor surface for editable snippets and diagnostics. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
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
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use crate::{CodeLanguage, CodeTheme, VirtualScrollbar, gpui_compat::element_id};
use gpui::{
    App, Bounds, ClipboardItem, Context, Element, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, FocusHandle, Focusable, GlobalElementId, HighlightStyle, Hsla,
    InspectorElementId, IntoElement, KeyBinding, LayoutId, ListAlignment, ListState, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point, Render, Rgba,
    ShapedLine, SharedString, Style, TextAlign, TextRun, UTF16Selection, Window, actions, div,
    fill, font, list, point, prelude::*, px, relative, size,
};
use liora_core::{Config, code_font_family, code_font_weight};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashSet},
    fs,
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tree_sitter::{Language as TreeSitterLanguage, Parser, Query, QueryCursor, StreamingIterator};

/// Type alias for code editor change callback values used by the code editor API.
pub type CodeEditorChangeCallback = dyn Fn(&str, &mut Context<CodeEditor>) + 'static;
/// Type alias for code diagnostics provider values used by the code editor API.
pub type CodeDiagnosticsProvider = dyn Fn(&str) -> Vec<CodeDiagnostic> + 'static;
/// Type alias for code completion provider values used by the code editor API.
pub type CodeCompletionProvider = dyn Fn(&str) -> Vec<CodeCompletionItem> + 'static;
/// Type alias for hover/help provider values used by the code editor API.
pub type CodeHoverProvider = dyn Fn(&str) -> Option<CodeHover> + 'static;

actions!(
    code_editor,
    [
        #[doc = "Keyboard action that deletes the character before the caret."]
        CodeEditorBackspace,
        #[doc = "Keyboard action that deletes the character after the caret."]
        CodeEditorDelete,
        #[doc = "Keyboard action that deletes to the previous code word boundary."]
        CodeEditorDeleteWordLeft,
        #[doc = "Keyboard action that deletes to the next code word boundary."]
        CodeEditorDeleteWordRight,
        #[doc = "Keyboard action that moves the caret one character left."]
        CodeEditorLeft,
        #[doc = "Keyboard action that moves the caret one character right."]
        CodeEditorRight,
        #[doc = "Keyboard action that moves the caret to the previous code word boundary."]
        CodeEditorWordLeft,
        #[doc = "Keyboard action that moves the caret to the next code word boundary."]
        CodeEditorWordRight,
        #[doc = "Keyboard action that moves the caret one page up."]
        CodeEditorPageUp,
        #[doc = "Keyboard action that moves the caret one page down."]
        CodeEditorPageDown,
        #[doc = "Keyboard action that moves the caret to the beginning of the buffer."]
        CodeEditorDocumentStart,
        #[doc = "Keyboard action that moves the caret to the end of the buffer."]
        CodeEditorDocumentEnd,
        #[doc = "Keyboard action that moves the caret one visual row up."]
        CodeEditorUp,
        #[doc = "Keyboard action that moves the caret one visual row down."]
        CodeEditorDown,
        #[doc = "Keyboard action that moves the caret to the current line start."]
        CodeEditorHome,
        #[doc = "Keyboard action that moves the caret to the current line end."]
        CodeEditorEnd,
        #[doc = "Keyboard action that selects the full editor buffer."]
        CodeEditorSelectAll,
        #[doc = "Keyboard action that copies the selected editor text."]
        CodeEditorCopy,
        #[doc = "Keyboard action that pastes clipboard text into the editor."]
        CodeEditorPaste,
        #[doc = "Keyboard action that cuts the selected editor text."]
        CodeEditorCut,
        #[doc = "Keyboard action that inserts a newline into the editor."]
        CodeEditorEnter,
        #[doc = "Keyboard action that extends selection one character left."]
        CodeEditorSelectLeft,
        #[doc = "Keyboard action that extends selection one character right."]
        CodeEditorSelectRight,
        #[doc = "Keyboard action that extends selection to the previous code word boundary."]
        CodeEditorSelectWordLeft,
        #[doc = "Keyboard action that extends selection to the next code word boundary."]
        CodeEditorSelectWordRight,
        #[doc = "Keyboard action that extends selection one page up."]
        CodeEditorSelectPageUp,
        #[doc = "Keyboard action that extends selection one page down."]
        CodeEditorSelectPageDown,
        #[doc = "Keyboard action that extends selection to the beginning of the buffer."]
        CodeEditorSelectDocumentStart,
        #[doc = "Keyboard action that extends selection to the end of the buffer."]
        CodeEditorSelectDocumentEnd,
        #[doc = "Keyboard action that extends selection one visual row up."]
        CodeEditorSelectUp,
        #[doc = "Keyboard action that extends selection one visual row down."]
        CodeEditorSelectDown,
        #[doc = "Keyboard action that extends selection to the current line start."]
        CodeEditorSelectHome,
        #[doc = "Keyboard action that extends selection to the current line end."]
        CodeEditorSelectEnd,
        #[doc = "Keyboard action that indents the selected code editor lines."]
        CodeEditorIndent,
        #[doc = "Keyboard action that outdents the selected code editor lines."]
        CodeEditorOutdent,
        #[doc = "Keyboard action that restores the previous edit transaction."]
        CodeEditorUndo,
        #[doc = "Keyboard action that reapplies the next edit transaction."]
        CodeEditorRedo
    ]
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Options that control code diagnostic severity behavior.
pub enum CodeDiagnosticSeverity {
    /// Uses informational semantic color tokens.
    Info,
    /// Uses warning semantic color tokens.
    Warning,
    /// Reports a error failure.
    Error,
}

impl CodeDiagnosticSeverity {
    fn label(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Fluent native GPUI component for rendering Liora code diagnostic.
pub struct CodeDiagnostic {
    /// One-based source line for diagnostics.
    pub line: usize,
    /// One-based source column for diagnostics.
    pub column: usize,
    /// Diagnostic severity used to choose color and icon treatment.
    pub severity: CodeDiagnosticSeverity,
    /// User-facing message associated with this item.
    pub message: SharedString,
}

impl CodeDiagnostic {
    /// Creates `CodeDiagnostic` with default theme-driven styling and no optional callbacks attached.
    pub fn new(
        line: usize,
        column: usize,
        severity: CodeDiagnosticSeverity,
        message: impl Into<SharedString>,
    ) -> Self {
        Self {
            line: line.max(1),
            column: column.max(1),
            severity,
            message: message.into(),
        }
    }

    /// Applies the informational semantic visual variant.
    pub fn info(line: usize, column: usize, message: impl Into<SharedString>) -> Self {
        Self::new(line, column, CodeDiagnosticSeverity::Info, message)
    }

    /// Applies the warning semantic visual variant.
    pub fn warning(line: usize, column: usize, message: impl Into<SharedString>) -> Self {
        Self::new(line, column, CodeDiagnosticSeverity::Warning, message)
    }

    /// Sets the error value used by the component.
    pub fn error(line: usize, column: usize, message: impl Into<SharedString>) -> Self {
        Self::new(line, column, CodeDiagnosticSeverity::Error, message)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Completion candidate rendered by CodeEditor suggestion panels.
pub struct CodeCompletionItem {
    /// Text inserted or referenced by the host application.
    pub label: SharedString,
    /// Optional category such as keyword, function, snippet, or variable.
    pub kind: Option<SharedString>,
    /// Optional explanatory detail shown after the label.
    pub detail: Option<SharedString>,
}

impl CodeCompletionItem {
    /// Creates a completion candidate with a required label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            kind: None,
            detail: None,
        }
    }

    /// Adds a short kind label used for grouping or visual explanation.
    pub fn kind(mut self, kind: impl Into<SharedString>) -> Self {
        self.kind = Some(kind.into());
        self
    }

    /// Adds human-readable detail text.
    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Hover/help content rendered below the editor without binding to a concrete LSP backend.
pub struct CodeHover {
    /// Title for the hover/help panel.
    pub title: SharedString,
    /// Body text shown under the title.
    pub description: SharedString,
}

impl CodeHover {
    /// Creates hover/help content from title and description text.
    pub fn new(title: impl Into<SharedString>, description: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Zero-based point inside the editor buffer.
pub struct CodePoint {
    /// Zero-based line index.
    pub row: usize,
    /// UTF-8 byte column inside the line.
    pub column: usize,
}

impl CodePoint {
    /// Creates a new point and lets the buffer clamp it when used.
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CodeLine {
    text: String,
    start_offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CodeBuffer {
    text: String,
    lines: Vec<CodeLine>,
}

impl CodeBuffer {
    fn new(value: impl Into<String>) -> Self {
        let mut this = Self {
            text: value.into(),
            lines: Vec::new(),
        };
        this.rebuild_lines();
        this
    }

    fn as_str(&self) -> &str {
        &self.text
    }

    fn len(&self) -> usize {
        self.text.len()
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn line_count(&self) -> usize {
        self.lines.len().max(1)
    }

    fn line(&self, row: usize) -> &str {
        self.lines
            .get(row)
            .map(|line| line.text.as_str())
            .unwrap_or("")
    }

    fn line_start(&self, row: usize) -> usize {
        self.lines
            .get(row)
            .map(|line| line.start_offset)
            .unwrap_or(self.text.len())
    }

    fn line_end(&self, row: usize) -> usize {
        let Some(line) = self.lines.get(row) else {
            return self.text.len();
        };
        line.start_offset + line.text.len()
    }

    fn set_text(&mut self, value: impl Into<String>) {
        self.text = value.into();
        self.rebuild_lines();
    }

    fn replace_range(&mut self, range: Range<usize>, replacement: &str) -> usize {
        let range = normalize_replace_range(&self.text, range);
        self.text.replace_range(range.clone(), replacement);
        let cursor = range.start + replacement.len();
        self.rebuild_lines();
        self.clamp_offset(cursor)
    }

    fn point_to_offset(&self, point: CodePoint) -> usize {
        let row = point.row.min(self.line_count().saturating_sub(1));
        let line = self.line(row);
        self.line_start(row) + clamp_to_char_boundary(line, point.column)
    }

    fn offset_to_point(&self, offset: usize) -> CodePoint {
        let offset = self.clamp_offset(offset);
        let row = match self
            .lines
            .binary_search_by(|line| line.start_offset.cmp(&offset))
        {
            Ok(row) => row,
            Err(row) => row.saturating_sub(1),
        };
        CodePoint::new(row, offset.saturating_sub(self.line_start(row)))
    }

    fn clamp_offset(&self, offset: usize) -> usize {
        clamp_to_char_boundary(&self.text, offset)
    }

    fn prev_char(&self, offset: usize) -> usize {
        let offset = self.clamp_offset(offset);
        if offset == 0 {
            return 0;
        }
        let mut previous = offset - 1;
        while previous > 0 && !self.text.is_char_boundary(previous) {
            previous -= 1;
        }
        previous
    }

    fn next_char(&self, offset: usize) -> usize {
        let offset = self.clamp_offset(offset);
        if offset >= self.text.len() {
            return self.text.len();
        }
        let mut next = offset + 1;
        while next < self.text.len() && !self.text.is_char_boundary(next) {
            next += 1;
        }
        next
    }

    fn line_start_at_offset(&self, offset: usize) -> usize {
        let point = self.offset_to_point(offset);
        self.line_start(point.row)
    }

    fn line_end_at_offset(&self, offset: usize) -> usize {
        let point = self.offset_to_point(offset);
        self.line_end(point.row)
    }

    fn first_non_whitespace_at_offset(&self, offset: usize) -> usize {
        let point = self.offset_to_point(offset);
        let line_start = self.line_start(point.row);
        let line = self.line(point.row);
        let relative = line
            .char_indices()
            .find_map(|(index, ch)| (!ch.is_whitespace()).then_some(index))
            .unwrap_or(line.len());
        self.clamp_offset(line_start + relative)
    }

    fn selected_line_bounds(&self, selection: Range<usize>) -> Range<usize> {
        if self.is_empty() {
            return 0..0;
        }
        let start = self.line_start_at_offset(selection.start);
        let mut end_offset = selection.end.min(self.len());
        if end_offset > selection.start
            && end_offset > 0
            && self.text.as_bytes().get(end_offset - 1) == Some(&b'\n')
        {
            end_offset -= 1;
        }
        let end_point = self.offset_to_point(end_offset);
        start..self.line_end(end_point.row)
    }

    fn rebuild_lines(&mut self) {
        self.lines.clear();
        let mut start = 0;
        for segment in self.text.split_inclusive('\n') {
            let line_text = segment.strip_suffix('\n').unwrap_or(segment).to_string();
            self.lines.push(CodeLine {
                text: line_text,
                start_offset: start,
            });
            start += segment.len();
        }
        if self.text.is_empty() || self.text.ends_with('\n') {
            self.lines.push(CodeLine {
                text: String::new(),
                start_offset: self.text.len(),
            });
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CodeSelection {
    range: Range<usize>,
    reversed: bool,
    preferred_column: Option<usize>,
}

impl CodeSelection {
    fn new(cursor: usize) -> Self {
        Self {
            range: cursor..cursor,
            reversed: false,
            preferred_column: None,
        }
    }

    fn cursor(&self) -> usize {
        if self.reversed {
            self.range.start
        } else {
            self.range.end
        }
    }

    fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    fn set_cursor(&mut self, offset: usize) {
        self.range = offset..offset;
        self.reversed = false;
        self.preferred_column = None;
    }

    fn select_to(&mut self, offset: usize) {
        if self.reversed {
            self.range.start = offset;
        } else {
            self.range.end = offset;
        }
        if self.range.end < self.range.start {
            self.reversed = !self.reversed;
            self.range = self.range.end..self.range.start;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CodeDisplayMap {
    row_height: Pixels,
    text_line_height: Pixels,
    gutter_width: Pixels,
    content_left_padding: Pixels,
    row_padding_y: Pixels,
    average_char_width: Pixels,
    viewport_padding: Pixels,
}

impl CodeDisplayMap {
    fn default_for(
        line_numbers: bool,
        text_line_height: Option<Pixels>,
        font_size: Pixels,
        gutter_width: Option<Pixels>,
        content_padding_x: Option<Pixels>,
        row_padding_y: Option<Pixels>,
        viewport_padding_y: Option<Pixels>,
    ) -> Self {
        let text_line_height = text_line_height.unwrap_or_else(|| px(24.0));
        let row_padding_y = row_padding_y.unwrap_or_else(|| px(0.0));
        Self {
            row_height: text_line_height + row_padding_y * 2.0,
            text_line_height,
            gutter_width: if line_numbers {
                gutter_width.unwrap_or_else(|| px(64.0))
            } else {
                px(0.0)
            },
            content_left_padding: content_padding_x.unwrap_or_else(|| px(14.0)),
            row_padding_y,
            average_char_width: px((font_size.as_f32() / 14.0).max(0.5) * 8.0),
            viewport_padding: viewport_padding_y.unwrap_or_else(|| px(24.0)),
        }
    }

    fn viewport_height(self, rows: usize) -> Pixels {
        px(self.row_height.as_f32() * rows.max(1) as f32 + self.viewport_padding.as_f32())
    }

    fn row_height(self) -> Pixels {
        self.row_height
    }

    fn text_line_height(self) -> Pixels {
        self.text_line_height
    }

    fn gutter_width(self, line_numbers: bool) -> Pixels {
        if line_numbers {
            self.gutter_width
        } else {
            px(0.0)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Declarative folded source range for CodeEditor block-folding demos and host state.
pub struct CodeFold {
    /// Zero-based first line that remains visible as the fold header.
    pub start_line: usize,
    /// Zero-based inclusive last line hidden when the fold is collapsed.
    pub end_line: usize,
    /// Label rendered in accessibility-oriented metadata and tests.
    pub label: SharedString,
    /// Whether child rows are currently hidden.
    pub collapsed: bool,
    /// Whether this fold was detected from source structure instead of added by host code.
    pub automatic: bool,
}

impl CodeFold {
    /// Creates a host-declared folded source range that starts collapsed.
    ///
    /// `end_line` is clamped above `start_line` by render helpers. Host
    /// applications can set `collapsed` to `false` when they want the range to
    /// be expandable but initially open.
    pub fn new(start_line: usize, end_line: usize, label: impl Into<SharedString>) -> Self {
        Self {
            start_line,
            end_line,
            label: label.into(),
            collapsed: true,
            automatic: false,
        }
    }

    /// Creates a host-declared fold range that starts expanded while still rendering a toggle affordance.
    pub fn expanded(start_line: usize, end_line: usize, label: impl Into<SharedString>) -> Self {
        Self {
            collapsed: false,
            ..Self::new(start_line, end_line, label)
        }
    }

    fn automatic(start_line: usize, end_line: usize, label: impl Into<SharedString>) -> Self {
        Self {
            start_line,
            end_line,
            label: label.into(),
            collapsed: false,
            automatic: true,
        }
    }

    fn same_range(&self, other: &Self) -> bool {
        self.start_line == other.start_line && self.end_line == other.end_line
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CodeTransaction {
    before_text: String,
    after_text: String,
    before_selection: CodeSelection,
    after_selection: CodeSelection,
}

impl CodeTransaction {
    fn new(before_text: String, before_selection: CodeSelection) -> Self {
        Self {
            before_text,
            after_text: String::new(),
            before_selection,
            after_selection: CodeSelection::new(0),
        }
    }

    fn finish(mut self, after_text: String, after_selection: CodeSelection) -> Self {
        self.after_text = after_text;
        self.after_selection = after_selection;
        self
    }

    fn changed(&self) -> bool {
        self.before_text != self.after_text || self.before_selection != self.after_selection
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CodeViewport {
    rows: usize,
}

impl CodeViewport {
    fn new(rows: usize) -> Self {
        Self { rows: rows.max(1) }
    }

    fn height(self, display: CodeDisplayMap) -> Pixels {
        display.viewport_height(self.rows)
    }
}

fn normalize_folds(folds: impl IntoIterator<Item = CodeFold>, line_count: usize) -> Vec<CodeFold> {
    let mut folds = folds
        .into_iter()
        .filter_map(|mut fold| {
            if line_count == 0 {
                return None;
            }
            fold.start_line = fold.start_line.min(line_count.saturating_sub(1));
            fold.end_line = fold.end_line.min(line_count.saturating_sub(1));
            (fold.end_line > fold.start_line).then_some(fold)
        })
        .collect::<Vec<_>>();
    folds.sort_by_key(|fold| (fold.start_line, fold.end_line, fold.automatic));
    folds.dedup_by(|left, right| {
        left.start_line == right.start_line && left.end_line == right.end_line
    });
    folds
}

fn detect_brace_folds(source: &str) -> Vec<CodeFold> {
    let lines = source.lines().collect::<Vec<_>>();
    let mut stack: Vec<usize> = Vec::new();
    let mut folds = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {
        for ch in line.chars() {
            match ch {
                '{' => stack.push(line_index),
                '}' => {
                    if let Some(start_line) = stack.pop() {
                        if line_index > start_line {
                            let label =
                                fold_label_for_line(lines.get(start_line).copied().unwrap_or(""));
                            folds.push(CodeFold::automatic(start_line, line_index, label));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    normalize_folds(folds, lines.len().max(1))
}

fn fold_label_for_line(line: &str) -> SharedString {
    let trimmed = line.trim();
    let label = trimmed
        .split('{')
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("block");
    SharedString::from(label.to_string())
}

fn visible_rows_for(line_count: usize, folds: &[CodeFold], enabled: bool) -> Vec<usize> {
    if !enabled || folds.is_empty() {
        return (0..line_count).collect();
    }
    let mut rows = Vec::with_capacity(line_count);
    let mut row = 0;
    while row < line_count {
        rows.push(row);
        if let Some(fold) = folds
            .iter()
            .find(|fold| fold.start_line == row && fold.collapsed)
        {
            row = fold.end_line.saturating_add(1).min(line_count);
        } else {
            row += 1;
        }
    }
    rows
}

fn fold_at_row<'a>(folds: &'a [CodeFold], row: usize, enabled: bool) -> Option<&'a CodeFold> {
    enabled
        .then(|| folds.iter().find(|fold| fold.start_line == row))
        .flatten()
}

fn leading_indent_columns(line: &str, tab_size: usize) -> usize {
    line.chars()
        .take_while(|ch| *ch == ' ' || *ch == '\t')
        .map(|ch| if ch == '\t' { tab_size } else { 1 })
        .sum()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CodeLineDisplay {
    text: String,
    source_to_display: Vec<usize>,
    display_to_source: Vec<usize>,
}

impl CodeLineDisplay {
    fn expand_tabs(line: &str, tab_size: usize) -> Self {
        let tab_size = tab_size.max(1);
        let mut text = String::with_capacity(line.len());
        let mut source_to_display = vec![0; line.len() + 1];
        let mut display_to_source = vec![0];
        let mut visual_column = 0usize;

        for (source_index, ch) in line.char_indices() {
            source_to_display[source_index] = text.len();
            if ch == '\t' {
                let spaces = tab_size - (visual_column % tab_size);
                for display_step in 1..=spaces {
                    text.push(' ');
                    let mapped_source = if display_step * 2 < spaces {
                        source_index
                    } else {
                        source_index + ch.len_utf8()
                    };
                    display_to_source.push(mapped_source);
                }
                visual_column += spaces;
                source_to_display[source_index + ch.len_utf8()] = text.len();
            } else {
                text.push(ch);
                let source_end = source_index + ch.len_utf8();
                for _ in 0..ch.len_utf8() {
                    display_to_source.push(source_end);
                }
                visual_column += 1;
                source_to_display[source_end] = text.len();
            }
        }
        source_to_display[line.len()] = text.len();
        Self {
            text,
            source_to_display,
            display_to_source,
        }
    }

    fn display_offset_for_source(&self, source_offset: usize) -> usize {
        let source_offset = source_offset.min(self.source_to_display.len().saturating_sub(1));
        if source_offset == 0 || self.source_to_display[source_offset] != 0 {
            return self.source_to_display[source_offset];
        }
        (0..source_offset)
            .rev()
            .find_map(|index| {
                (index == 0 || self.source_to_display[index] != 0)
                    .then_some(self.source_to_display[index])
            })
            .unwrap_or(0)
    }

    fn display_range_for_source(&self, range: Range<usize>) -> Range<usize> {
        self.display_offset_for_source(range.start)..self.display_offset_for_source(range.end)
    }
}

fn parse_code_theme(value: &str) -> Option<CodeTheme> {
    match value.trim().to_ascii_lowercase().replace('_', "-").as_str() {
        "auto" => Some(CodeTheme::Auto),
        "light" => Some(CodeTheme::Light),
        "dark" => Some(CodeTheme::Dark),
        "liora-light" | "brand-light" => Some(CodeTheme::BrandLight),
        "liora-dark" | "brand-dark" => Some(CodeTheme::BrandDark),
        "github-light" => Some(CodeTheme::GitHubLight),
        "github-dark" => Some(CodeTheme::GitHubDark),
        "one-dark" | "onedark" => Some(CodeTheme::OneDark),
        "nord" => Some(CodeTheme::Nord),
        "dracula" => Some(CodeTheme::Dracula),
        _ => None,
    }
}

fn parse_inline_diagnostics(value: &str) -> CodeEditorInlineDiagnostics {
    match value.trim().to_ascii_lowercase().replace('_', "-").as_str() {
        "none" | "off" | "hidden" => CodeEditorInlineDiagnostics::None,
        "warnings-and-errors" | "warning" | "warnings" => {
            CodeEditorInlineDiagnostics::WarningsAndErrors
        }
        _ => CodeEditorInlineDiagnostics::All,
    }
}

fn parse_whitespace_mode(value: &str) -> CodeEditorWhitespaceMode {
    match value.trim().to_ascii_lowercase().as_str() {
        "all" => CodeEditorWhitespaceMode::All,
        "boundary" | "indent" | "indentation" => CodeEditorWhitespaceMode::Boundary,
        _ => CodeEditorWhitespaceMode::Hidden,
    }
}

fn parse_hex_color(value: &str) -> Option<Hsla> {
    let value = value.trim().trim_start_matches('#');
    let parse = |range: std::ops::Range<usize>| u8::from_str_radix(&value[range], 16).ok();
    let (r, g, b, a) = match value.len() {
        6 => (parse(0..2)?, parse(2..4)?, parse(4..6)?, 255),
        8 => (parse(0..2)?, parse(2..4)?, parse(4..6)?, parse(6..8)?),
        _ => return None,
    };
    Some(
        Rgba {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
        .into(),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Controls how whitespace is displayed inside CodeEditor rows.
pub enum CodeEditorWhitespaceMode {
    /// Do not draw whitespace symbols.
    Hidden,
    /// Draw leading indentation whitespace only.
    Boundary,
    /// Draw all spaces and tab characters.
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Controls which line diagnostics are rendered inline in the code area.
pub enum CodeEditorInlineDiagnostics {
    /// Do not render inline diagnostic rows.
    None,
    /// Render only warning and error diagnostics inline.
    WarningsAndErrors,
    /// Render informational, warning, and error diagnostics inline.
    All,
}

impl CodeEditorInlineDiagnostics {
    fn includes(self, severity: CodeDiagnosticSeverity) -> bool {
        match self {
            Self::None => false,
            Self::WarningsAndErrors => !matches!(severity, CodeDiagnosticSeverity::Info),
            Self::All => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Display and interaction options for CodeEditor chrome and extension panels.
pub struct CodeEditorOptions {
    /// Whether text editing commands may mutate the buffer.
    pub read_only: bool,
    /// Whether the top metadata toolbar is visible.
    pub header: bool,
    /// Whether the bottom status row is visible.
    pub status_bar: bool,
    /// Whether line numbers are visible in the gutter.
    pub line_numbers: bool,
    /// Whether provider/static diagnostics are rendered below the editor.
    pub diagnostics_panel: bool,
    /// Whether completion candidates are rendered below the editor.
    pub completions_panel: bool,
    /// Whether hover/help content is rendered below the editor.
    pub hover_panel: bool,
    /// Whether the current row receives a subtle background highlight.
    pub current_line_highlight: bool,
    /// Whether vertical indentation guide lines are painted for leading whitespace.
    pub indent_guides: bool,
    /// Whether configured fold ranges hide child rows and render fold placeholders.
    pub code_folding: bool,
    /// Whether the editor should render a thin right-edge guide column.
    pub rulers: bool,
    /// Target column for the primary ruler guide.
    pub ruler_column: usize,
    /// Whether line rows should mark diagnostics inline near the affected source line.
    pub inline_diagnostics: CodeEditorInlineDiagnostics,
    /// Whether spaces/tabs should be annotated with visible symbols.
    pub whitespace: CodeEditorWhitespaceMode,
    /// Whether text may be selected via mouse and keyboard.
    pub selection: bool,
    /// Whether copy actions are enabled when text is selected.
    pub copy: bool,
    /// Whether cut/paste actions are enabled when the editor is editable.
    pub clipboard_editing: bool,
    /// Whether the caret blink task is enabled while focused.
    pub cursor_blink: bool,
    /// Whether clicking and dragging can extend the selection.
    pub drag_selection: bool,
    /// Whether double click selects a word.
    pub word_selection: bool,
    /// Whether triple click selects the clicked line.
    pub line_selection: bool,
    /// Whether Tab and Shift+Tab indent or outdent the selected text.
    pub indentation: bool,
    /// Whether undo/redo transactions are recorded and replayed.
    pub history: bool,
    /// Whether cursor movement should reveal the active row in the viewport.
    pub reveal_cursor: bool,
    /// Whether the built-in vertical scrollbar is rendered.
    pub scrollbar: bool,
    /// Maximum completion rows shown by the built-in completion panel.
    pub completion_limit: usize,
    /// Maximum diagnostic rows shown by the built-in diagnostics panel.
    pub diagnostics_limit: usize,
}

impl Default for CodeEditorOptions {
    fn default() -> Self {
        Self {
            read_only: false,
            header: true,
            status_bar: true,
            line_numbers: true,
            diagnostics_panel: true,
            completions_panel: true,
            hover_panel: true,
            current_line_highlight: false,
            indent_guides: false,
            code_folding: true,
            rulers: false,
            ruler_column: 80,
            inline_diagnostics: CodeEditorInlineDiagnostics::All,
            whitespace: CodeEditorWhitespaceMode::Hidden,
            selection: true,
            copy: true,
            clipboard_editing: true,
            cursor_blink: true,
            drag_selection: true,
            word_selection: true,
            line_selection: true,
            indentation: true,
            history: true,
            reveal_cursor: true,
            scrollbar: true,
            completion_limit: 6,
            diagnostics_limit: 8,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
/// File-driven CodeEditor configuration loaded from TOML or JSON.
///
/// The config format intentionally mirrors Zed's editor model: behavior and
/// chrome live in this file, while syntax colors are loaded from a Zed theme JSON
/// (`themes[].style.syntax`). It does not accept TextMate `.tmTheme` files; use a
/// Zed-compatible JSON theme when product teams want portable code colors.
///
/// Apply a config once with [`CodeEditor::config`] or reload explicitly with
/// [`CodeEditor::set_config`] when the host application saves or reloads settings.
pub struct CodeEditorConfig {
    /// Optional language label, such as `rust`, `typescript`, or `json`.
    pub language: Option<String>,
    /// Optional built-in code theme label used as fallback for editor chrome.
    pub theme: Option<String>,
    /// Optional fixed height in pixels.
    pub height_px: Option<f32>,
    /// Optional code font family for editor rows.
    pub font_family: Option<String>,
    /// Optional code font size in pixels.
    pub font_size_px: Option<f32>,
    /// Optional numeric code font weight, such as 400 or 600.
    pub font_weight: Option<f32>,
    /// Optional editor line height in pixels.
    pub line_height_px: Option<f32>,
    /// Optional editor spacing and layout metrics.
    pub layout: Option<CodeEditorLayoutConfig>,
    /// Optional indentation width.
    pub tab_size: Option<usize>,
    /// Optional soft-tabs toggle.
    pub soft_tabs: Option<bool>,
    /// Optional editor behavior and chrome options.
    pub options: Option<CodeEditorOptionsConfig>,
    /// Optional editor visual theme section.
    pub appearance: Option<CodeEditorAppearanceConfig>,
    /// Optional Zed JSON theme file path, resolved relative to this config file.
    pub theme_file: Option<String>,
    /// Optional exact theme name inside a Zed theme family JSON file.
    pub theme_name: Option<String>,
}

impl CodeEditorConfig {
    /// Loads CodeEditor configuration from a TOML or JSON file path.
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, CodeEditorConfigError> {
        let path = path.as_ref();
        let source = fs::read_to_string(path)?;
        let base_dir = path.parent().map(Path::to_path_buf);
        Self::load_from_str_with_base(
            &source,
            path.extension().and_then(|ext| ext.to_str()),
            base_dir,
        )
    }

    /// Loads CodeEditor configuration from TOML text.
    pub fn load_toml(source: &str) -> Result<Self, CodeEditorConfigError> {
        Self::load_from_str_with_base(source, Some("toml"), None)
    }

    /// Loads CodeEditor configuration from JSON text.
    pub fn load_json(source: &str) -> Result<Self, CodeEditorConfigError> {
        Self::load_from_str_with_base(source, Some("json"), None)
    }

    fn load_from_str_with_base(
        source: &str,
        extension: Option<&str>,
        base_dir: Option<PathBuf>,
    ) -> Result<Self, CodeEditorConfigError> {
        let mut config: CodeEditorConfig =
            match extension.unwrap_or("toml").to_ascii_lowercase().as_str() {
                "json" => serde_json::from_str(source)?,
                _ => toml::from_str(source)?,
            };
        config.resolve_theme_file(base_dir)?;
        Ok(config)
    }

    fn resolve_theme_file(
        &mut self,
        base_dir: Option<PathBuf>,
    ) -> Result<(), CodeEditorConfigError> {
        let Some(theme_file) = self.theme_file.clone() else {
            return Ok(());
        };
        let mut path = PathBuf::from(theme_file);
        if path.is_relative() {
            if let Some(base_dir) = base_dir {
                path = base_dir.join(path);
            }
        }
        let source = fs::read_to_string(path)?;
        let external = CodeEditorZedTheme::load(&source, self.theme_name.as_deref())?;
        self.theme = external.base_theme.or_else(|| self.theme.take());
        self.appearance = Some(match self.appearance.take() {
            Some(appearance) => appearance.merge_missing(external.appearance),
            None => external.appearance,
        });
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
/// Serializable CodeEditor layout metrics loaded from config files.
pub struct CodeEditorLayoutConfig {
    /// Width of the line-number gutter in pixels.
    pub gutter_width_px: Option<f32>,
    /// Horizontal padding before code content in pixels.
    pub content_padding_x_px: Option<f32>,
    /// Vertical padding around each visible code row in pixels.
    pub row_padding_y_px: Option<f32>,
    /// Extra vertical padding reserved by the viewport in pixels.
    pub viewport_padding_y_px: Option<f32>,
    /// Header horizontal padding in pixels.
    pub header_padding_x_px: Option<f32>,
    /// Header vertical padding in pixels.
    pub header_padding_y_px: Option<f32>,
    /// Header item gap in pixels.
    pub header_gap_px: Option<f32>,
    /// Diagnostics/completions/hover panel horizontal padding in pixels.
    pub panel_padding_x_px: Option<f32>,
    /// Diagnostics/completions/hover panel vertical padding in pixels.
    pub panel_padding_y_px: Option<f32>,
    /// Diagnostics/completions/hover panel row gap in pixels.
    pub panel_gap_px: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize)]
/// Serializable subset of CodeEditorOptions used by config files.
pub struct CodeEditorOptionsConfig {
    /// Enables read-only mode when `true`.
    pub read_only: Option<bool>,
    /// Shows the editor header/chrome row when `true`.
    pub header: Option<bool>,
    /// Shows the status bar when `true`.
    pub status_bar: Option<bool>,
    /// Shows the line-number gutter when `true`.
    pub line_numbers: Option<bool>,
    /// Shows the diagnostics panel when `true`.
    pub diagnostics_panel: Option<bool>,
    /// Shows the completions panel when `true`.
    pub completions_panel: Option<bool>,
    /// Shows the hover/help panel when `true`.
    pub hover_panel: Option<bool>,
    /// Highlights the active cursor line when `true`.
    pub current_line_highlight: Option<bool>,
    /// Draws indentation guide lines when `true`.
    pub indent_guides: Option<bool>,
    /// Enables fold detection and gutter fold toggles when `true`.
    pub code_folding: Option<bool>,
    /// Draws vertical ruler guides when `true`.
    pub rulers: Option<bool>,
    /// One-based column used for the primary ruler.
    pub ruler_column: Option<usize>,
    /// Inline diagnostic style, for example `off`, `gutter`, or `inline`.
    pub inline_diagnostics: Option<String>,
    /// Whitespace rendering mode, for example `none`, `boundary`, or `all`.
    pub whitespace: Option<String>,
    /// Enables text selection when `true`.
    pub selection: Option<bool>,
    /// Enables copy operations when `true`.
    pub copy: Option<bool>,
    /// Enables clipboard editing operations when `true`.
    pub clipboard_editing: Option<bool>,
    /// Enables caret blinking when `true`.
    pub cursor_blink: Option<bool>,
    /// Enables drag-to-select when `true`.
    pub drag_selection: Option<bool>,
    /// Enables word selection gestures when `true`.
    pub word_selection: Option<bool>,
    /// Enables whole-line selection gestures when `true`.
    pub line_selection: Option<bool>,
    /// Enables indentation helpers such as tab handling when `true`.
    pub indentation: Option<bool>,
    /// Enables undo/redo history when `true`.
    pub history: Option<bool>,
    /// Keeps the caret visible after edits and navigation when `true`.
    pub reveal_cursor: Option<bool>,
    /// Shows the editor scrollbar when `true`.
    pub scrollbar: Option<bool>,
    /// Maximum number of completion items rendered by the sample panel.
    pub completion_limit: Option<usize>,
    /// Maximum number of diagnostics rendered by the diagnostics panel.
    pub diagnostics_limit: Option<usize>,
}

impl CodeEditorOptionsConfig {
    fn apply_to(&self, options: &mut CodeEditorOptions) {
        macro_rules! set_bool {
            ($field:ident) => {
                if let Some(value) = self.$field {
                    options.$field = value;
                }
            };
        }
        set_bool!(read_only);
        set_bool!(header);
        set_bool!(status_bar);
        set_bool!(line_numbers);
        set_bool!(diagnostics_panel);
        set_bool!(completions_panel);
        set_bool!(hover_panel);
        set_bool!(current_line_highlight);
        set_bool!(indent_guides);
        set_bool!(code_folding);
        set_bool!(rulers);
        set_bool!(selection);
        set_bool!(copy);
        set_bool!(clipboard_editing);
        set_bool!(cursor_blink);
        set_bool!(drag_selection);
        set_bool!(word_selection);
        set_bool!(line_selection);
        set_bool!(indentation);
        set_bool!(history);
        set_bool!(reveal_cursor);
        set_bool!(scrollbar);
        if let Some(value) = self.ruler_column {
            options.ruler_column = value.max(1);
        }
        if let Some(value) = self.completion_limit {
            options.completion_limit = value.max(1);
        }
        if let Some(value) = self.diagnostics_limit {
            options.diagnostics_limit = value.max(1);
        }
        if let Some(value) = self.inline_diagnostics.as_deref() {
            options.inline_diagnostics = parse_inline_diagnostics(value);
        }
        if let Some(value) = self.whitespace.as_deref() {
            options.whitespace = parse_whitespace_mode(value);
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
/// Serializable editor appearance overrides. Color values use `#rrggbb` or `#rrggbbaa`.
pub struct CodeEditorAppearanceConfig {
    /// Built-in fallback theme name used before applying explicit color overrides.
    pub base: Option<String>,
    /// Main editor surface color.
    pub surface: Option<String>,
    /// Header, footer, and auxiliary chrome surface color.
    pub chrome_surface: Option<String>,
    /// Line-number and folding gutter surface color.
    pub gutter_surface: Option<String>,
    /// Border color for the editor shell and panels.
    pub border: Option<String>,
    /// Default source text color.
    pub text: Option<String>,
    /// Secondary text color used for gutter labels and metadata.
    pub muted_text: Option<String>,
    /// Caret color.
    pub caret: Option<String>,
    /// Selection highlight color.
    pub selection: Option<String>,
    /// Current-line highlight color.
    pub current_line: Option<String>,
    /// Ruler guide color.
    pub ruler: Option<String>,
    /// Whitespace marker color.
    pub whitespace: Option<String>,
    /// Informational diagnostic accent color.
    pub info: Option<String>,
    /// Warning diagnostic accent color.
    pub warning: Option<String>,
    /// Error diagnostic accent color.
    pub error: Option<String>,
    /// Syntax capture style map keyed by tree-sitter/Zed-style capture names.
    pub syntax: Option<BTreeMap<String, CodeEditorSyntaxStyleConfig>>,
}

impl CodeEditorAppearanceConfig {
    fn merge_missing(mut self, fallback: Self) -> Self {
        macro_rules! fill {
            ($field:ident) => {
                if self.$field.is_none() {
                    self.$field = fallback.$field;
                }
            };
        }
        fill!(base);
        fill!(surface);
        fill!(chrome_surface);
        fill!(gutter_surface);
        fill!(border);
        fill!(text);
        fill!(muted_text);
        fill!(caret);
        fill!(selection);
        fill!(current_line);
        fill!(ruler);
        fill!(whitespace);
        fill!(info);
        fill!(warning);
        fill!(error);
        if self.syntax.is_none() {
            self.syntax = fallback.syntax;
        }
        self
    }

    fn to_theme(&self, fallback: CodeTheme) -> CodeEditorHighlightTheme {
        let mut theme = CodeEditorHighlightTheme::new(
            self.base
                .as_deref()
                .and_then(parse_code_theme)
                .unwrap_or(fallback),
        );
        theme.surface = self.surface.as_deref().and_then(parse_hex_color);
        theme.chrome_surface = self.chrome_surface.as_deref().and_then(parse_hex_color);
        theme.gutter_surface = self.gutter_surface.as_deref().and_then(parse_hex_color);
        theme.border = self.border.as_deref().and_then(parse_hex_color);
        theme.text = self.text.as_deref().and_then(parse_hex_color);
        theme.muted_text = self.muted_text.as_deref().and_then(parse_hex_color);
        theme.caret = self.caret.as_deref().and_then(parse_hex_color);
        theme.selection = self.selection.as_deref().and_then(parse_hex_color);
        theme.current_line = self.current_line.as_deref().and_then(parse_hex_color);
        theme.ruler = self.ruler.as_deref().and_then(parse_hex_color);
        theme.whitespace = self.whitespace.as_deref().and_then(parse_hex_color);
        theme.info = self.info.as_deref().and_then(parse_hex_color);
        theme.warning = self.warning.as_deref().and_then(parse_hex_color);
        theme.error = self.error.as_deref().and_then(parse_hex_color);
        if let Some(syntax) = self.syntax.as_ref() {
            theme.syntax = CodeEditorSyntaxTheme::from_config(syntax);
        }
        theme
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
/// One Zed-style syntax style entry loaded from a config or theme JSON.
pub struct CodeEditorSyntaxStyleConfig {
    /// Foreground color for matching syntax captures.
    pub color: Option<String>,
    /// Optional background color for matching syntax captures.
    pub background_color: Option<String>,
    /// Numeric font weight forwarded to GPUI when present.
    pub font_weight: Option<f32>,
    /// Font style name, usually `normal`, `italic`, or `oblique`.
    pub font_style: Option<String>,
}

impl CodeEditorSyntaxStyleConfig {
    fn to_highlight_style(&self) -> HighlightStyle {
        HighlightStyle {
            color: self.color.as_deref().and_then(parse_hex_color),
            background_color: self.background_color.as_deref().and_then(parse_hex_color),
            font_weight: self.font_weight.map(gpui::FontWeight),
            font_style: self.font_style.as_deref().and_then(|style| {
                match style.trim().to_ascii_lowercase().as_str() {
                    "italic" => Some(gpui::FontStyle::Italic),
                    "oblique" => Some(gpui::FontStyle::Oblique),
                    "normal" => Some(gpui::FontStyle::Normal),
                    _ => None,
                }
            }),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
/// Errors produced while loading CodeEditor configuration files.
pub enum CodeEditorConfigError {
    /// The file could not be read.
    Io(std::io::Error),
    /// TOML parsing failed.
    Toml(toml::de::Error),
    /// JSON parsing failed.
    Json(serde_json::Error),
}

impl std::fmt::Display for CodeEditorConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "failed to read CodeEditor config: {error}"),
            Self::Toml(error) => write!(f, "failed to parse CodeEditor TOML config: {error}"),
            Self::Json(error) => write!(f, "failed to parse CodeEditor JSON config: {error}"),
        }
    }
}

impl std::error::Error for CodeEditorConfigError {}

impl From<std::io::Error> for CodeEditorConfigError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<toml::de::Error> for CodeEditorConfigError {
    fn from(error: toml::de::Error) -> Self {
        Self::Toml(error)
    }
}

impl From<serde_json::Error> for CodeEditorConfigError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

struct CodeEditorZedTheme {
    base_theme: Option<String>,
    appearance: CodeEditorAppearanceConfig,
}

impl CodeEditorZedTheme {
    fn load(source: &str, theme_name: Option<&str>) -> Result<Self, CodeEditorConfigError> {
        let family = serde_json::from_str::<ZedThemeFamilyConfig>(source)?;
        let theme = theme_name
            .and_then(|name| family.themes.iter().find(|theme| theme.name == name))
            .or_else(|| family.themes.first());
        let Some(theme) = theme else {
            return Ok(Self {
                base_theme: None,
                appearance: CodeEditorAppearanceConfig::default(),
            });
        };
        let get = |key: &str| {
            theme
                .style
                .get(key)
                .and_then(|value| value.as_str())
                .map(str::to_string)
        };
        let syntax = theme.style.get("syntax").and_then(|value| {
            serde_json::from_value::<BTreeMap<String, CodeEditorSyntaxStyleConfig>>(value.clone())
                .ok()
        });
        Ok(Self {
            base_theme: theme.appearance.as_deref().map(|appearance| {
                if appearance.eq_ignore_ascii_case("dark") {
                    "one-dark".to_string()
                } else {
                    "github-light".to_string()
                }
            }),
            appearance: CodeEditorAppearanceConfig {
                surface: get("editor.background").or_else(|| get("background")),
                chrome_surface: get("editor.subheader.background")
                    .or_else(|| get("panel.background")),
                gutter_surface: get("editor.gutter.background"),
                border: get("border"),
                text: get("editor.foreground").or_else(|| get("text")),
                muted_text: get("editor.line_number").or_else(|| get("text.muted")),
                caret: get("editor.active_line_number").or_else(|| get("text.accent")),
                selection: get("editor.document_highlight.read_background")
                    .or_else(|| get("search.match_background")),
                current_line: get("editor.active_line.background"),
                ruler: get("editor.wrap_guide"),
                whitespace: get("editor.invisible"),
                info: get("info"),
                warning: get("warning"),
                error: get("error"),
                syntax,
                ..CodeEditorAppearanceConfig::default()
            },
        })
    }
}

#[derive(Deserialize)]
struct ZedThemeFamilyConfig {
    themes: Vec<ZedThemeConfig>,
}

#[derive(Deserialize)]
struct ZedThemeConfig {
    name: String,
    appearance: Option<String>,
    style: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq)]
/// Zed-style syntax palette keyed by tree-sitter capture names.
///
/// This mirrors Zed's `SyntaxTheme` lookup semantics at the component level: a
/// capture such as `function.method.call` can fall back to `function.method` and
/// then `function` when only broader styles are present.
pub struct CodeEditorSyntaxTheme {
    styles: BTreeMap<String, HighlightStyle>,
}

impl Default for CodeEditorSyntaxTheme {
    fn default() -> Self {
        Self::new(default_zed_syntax_styles())
    }
}

impl CodeEditorSyntaxTheme {
    /// Creates a syntax theme from capture-name/style pairs.
    pub fn new(styles: impl IntoIterator<Item = (String, HighlightStyle)>) -> Self {
        Self {
            styles: styles.into_iter().collect(),
        }
    }

    /// Creates a syntax theme from serializable Zed-style config entries.
    pub fn from_config(styles: &BTreeMap<String, CodeEditorSyntaxStyleConfig>) -> Self {
        Self::new(
            styles
                .iter()
                .map(|(name, style)| (name.clone(), style.to_highlight_style())),
        )
    }

    /// Creates a Liora fallback syntax theme for a built-in code theme.
    pub fn from_code_theme(code_theme: CodeTheme, app_theme: &liora_theme::Theme) -> Self {
        let dark = match code_theme {
            CodeTheme::BrandLight | CodeTheme::GitHubLight | CodeTheme::Light => false,
            CodeTheme::BrandDark
            | CodeTheme::GitHubDark
            | CodeTheme::OneDark
            | CodeTheme::Nord
            | CodeTheme::Dracula
            | CodeTheme::Dark => true,
            CodeTheme::Auto => app_theme.name.eq_ignore_ascii_case("dark"),
        };
        Self::new(default_zed_syntax_styles_for_mode(dark))
    }

    /// Adds or replaces one capture style and returns the updated theme.
    pub fn with_style(mut self, capture: impl Into<String>, style: HighlightStyle) -> Self {
        self.styles.insert(capture.into(), style);
        self
    }

    fn style_for_capture(&self, capture: &str) -> Option<HighlightStyle> {
        let mut candidate = Some(capture);
        while let Some(name) = candidate {
            if let Some(style) = self.styles.get(name) {
                return Some(*style);
            }
            candidate = name.rsplit_once('.').map(|(parent, _)| parent);
        }
        None
    }
}

fn default_zed_syntax_styles() -> Vec<(String, HighlightStyle)> {
    default_zed_syntax_styles_for_mode(true)
}

fn default_zed_syntax_styles_for_mode(dark: bool) -> Vec<(String, HighlightStyle)> {
    let color = |hex: u32| HighlightStyle {
        color: Some(gpui::rgb(hex).into()),
        ..Default::default()
    };
    let italic = |hex: u32| HighlightStyle {
        color: Some(gpui::rgb(hex).into()),
        font_style: Some(gpui::FontStyle::Italic),
        ..Default::default()
    };
    let styles: &[(&str, HighlightStyle)] = if dark {
        &[
            ("attribute", color(0x74ade8)),
            ("boolean", color(0xbf956a)),
            ("comment", italic(0x5d636f)),
            ("comment.doc", italic(0x878e98)),
            ("constant", color(0xdfc184)),
            ("constructor", color(0x73ade9)),
            ("embedded", color(0xabb2bf)),
            ("emphasis", italic(0xabb2bf)),
            ("function", color(0x61afef)),
            ("function.method", color(0x61afef)),
            ("field", color(0xe06c75)),
            ("parameter", color(0xd19a66)),
            ("storageclass", color(0xc678dd)),
            ("conditional", color(0xc678dd)),
            ("float", color(0xd19a66)),
            ("keyword", color(0xc678dd)),
            ("label", color(0xe5c07b)),
            ("link_uri", color(0x56b6c2)),
            ("number", color(0xd19a66)),
            ("operator", color(0x56b6c2)),
            ("property", color(0xe06c75)),
            ("punctuation", color(0xabb2bf)),
            ("string", color(0x98c379)),
            ("string.escape", color(0x56b6c2)),
            ("tag", color(0xe06c75)),
            ("text", color(0xabb2bf)),
            ("type", color(0xe5c07b)),
            ("variable", color(0xabb2bf)),
            ("variable.special", color(0xe06c75)),
        ]
    } else {
        &[
            ("attribute", color(0x0969da)),
            ("boolean", color(0x953800)),
            ("comment", italic(0x6e7781)),
            ("comment.doc", italic(0x57606a)),
            ("constant", color(0x0550ae)),
            ("constructor", color(0x8250df)),
            ("embedded", color(0x24292f)),
            ("emphasis", italic(0x24292f)),
            ("function", color(0x8250df)),
            ("function.method", color(0x8250df)),
            ("field", color(0x116329)),
            ("parameter", color(0x953800)),
            ("storageclass", color(0xcf222e)),
            ("conditional", color(0xcf222e)),
            ("float", color(0x0550ae)),
            ("keyword", color(0xcf222e)),
            ("label", color(0x953800)),
            ("link_uri", color(0x0969da)),
            ("number", color(0x0550ae)),
            ("operator", color(0x0550ae)),
            ("property", color(0x116329)),
            ("punctuation", color(0x24292f)),
            ("string", color(0x0a3069)),
            ("string.escape", color(0x0550ae)),
            ("tag", color(0x116329)),
            ("text", color(0x24292f)),
            ("type", color(0x953800)),
            ("variable", color(0x24292f)),
            ("variable.special", color(0x953800)),
        ]
    };
    styles
        .iter()
        .map(|(name, style)| ((*name).to_string(), *style))
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
/// Theme overrides used by CodeEditor without replacing the global Liora theme.
pub struct CodeEditorHighlightTheme {
    /// Optional base syntax palette used before custom token overrides are applied.
    pub base: CodeTheme,
    /// Editor surface background.
    pub surface: Option<Hsla>,
    /// Header/status/panel background.
    pub chrome_surface: Option<Hsla>,
    /// Gutter background.
    pub gutter_surface: Option<Hsla>,
    /// Border color around the editor.
    pub border: Option<Hsla>,
    /// Default code text color.
    pub text: Option<Hsla>,
    /// Muted text color for gutters and metadata.
    pub muted_text: Option<Hsla>,
    /// Caret color.
    pub caret: Option<Hsla>,
    /// Selection background color.
    pub selection: Option<Hsla>,
    /// Current-line highlight background color.
    pub current_line: Option<Hsla>,
    /// Ruler guide color.
    pub ruler: Option<Hsla>,
    /// Visible whitespace symbol color.
    pub whitespace: Option<Hsla>,
    /// Diagnostic info color.
    pub info: Option<Hsla>,
    /// Diagnostic warning color.
    pub warning: Option<Hsla>,
    /// Diagnostic error color.
    pub error: Option<Hsla>,
    /// Zed-style syntax theme used to color tree-sitter capture names.
    pub syntax: CodeEditorSyntaxTheme,
}

impl Default for CodeEditorHighlightTheme {
    fn default() -> Self {
        Self {
            base: CodeTheme::Auto,
            surface: None,
            chrome_surface: None,
            gutter_surface: None,
            border: None,
            text: None,
            muted_text: None,
            caret: None,
            selection: None,
            current_line: None,
            ruler: None,
            whitespace: None,
            info: None,
            warning: None,
            error: None,
            syntax: CodeEditorSyntaxTheme::default(),
        }
    }
}

impl CodeEditorHighlightTheme {
    /// Creates a custom editor highlight theme from an existing syntax palette.
    pub fn new(base: CodeTheme) -> Self {
        Self {
            base,
            ..Self::default()
        }
    }

    /// Sets the editor surface background.
    pub fn surface(mut self, color: Hsla) -> Self {
        self.surface = Some(color);
        self
    }

    /// Sets the chrome surface background for header, status, and panels.
    pub fn chrome_surface(mut self, color: Hsla) -> Self {
        self.chrome_surface = Some(color);
        self
    }

    /// Sets the gutter background.
    pub fn gutter_surface(mut self, color: Hsla) -> Self {
        self.gutter_surface = Some(color);
        self
    }

    /// Sets the editor border color.
    pub fn border(mut self, color: Hsla) -> Self {
        self.border = Some(color);
        self
    }

    /// Sets the default code text color.
    pub fn text(mut self, color: Hsla) -> Self {
        self.text = Some(color);
        self
    }

    /// Sets the muted metadata color.
    pub fn muted_text(mut self, color: Hsla) -> Self {
        self.muted_text = Some(color);
        self
    }

    /// Sets caret, selection, and current-line colors in one call.
    pub fn interaction(mut self, caret: Hsla, selection: Hsla, current_line: Hsla) -> Self {
        self.caret = Some(caret);
        self.selection = Some(selection);
        self.current_line = Some(current_line);
        self
    }

    /// Sets the ruler guide color.
    pub fn ruler(mut self, color: Hsla) -> Self {
        self.ruler = Some(color);
        self
    }

    /// Sets the visible whitespace symbol color.
    pub fn whitespace(mut self, color: Hsla) -> Self {
        self.whitespace = Some(color);
        self
    }

    /// Sets diagnostic semantic colors.
    pub fn diagnostics(mut self, info: Hsla, warning: Hsla, error: Hsla) -> Self {
        self.info = Some(info);
        self.warning = Some(warning);
        self.error = Some(error);
        self
    }

    /// Replaces the syntax palette used for tree-sitter capture names.
    pub fn syntax_theme(mut self, syntax: CodeEditorSyntaxTheme) -> Self {
        self.syntax = syntax;
        self
    }
}

#[derive(Clone, Debug)]
struct ResolvedCodeEditorTheme {
    surface: Hsla,
    chrome_surface: Hsla,
    gutter_surface: Hsla,
    border: Hsla,
    text: Hsla,
    muted_text: Hsla,
    caret: Hsla,
    selection: Hsla,
    current_line: Hsla,
    ruler: Hsla,
    whitespace: Hsla,
    info: Hsla,
    warning: Hsla,
    error: Hsla,
    syntax: CodeEditorSyntaxTheme,
}

impl ResolvedCodeEditorTheme {
    fn resolve(
        overrides: Option<&CodeEditorHighlightTheme>,
        code_theme: CodeTheme,
        app_theme: &liora_theme::Theme,
    ) -> Self {
        Self {
            surface: overrides
                .and_then(|theme| theme.surface)
                .unwrap_or(app_theme.neutral.hover.opacity(0.18)),
            chrome_surface: overrides
                .and_then(|theme| theme.chrome_surface)
                .unwrap_or(app_theme.neutral.hover.opacity(0.52)),
            gutter_surface: overrides
                .and_then(|theme| theme.gutter_surface)
                .unwrap_or(app_theme.neutral.card.opacity(0.0)),
            border: overrides
                .and_then(|theme| theme.border)
                .unwrap_or(app_theme.neutral.border),
            text: overrides
                .and_then(|theme| theme.text)
                .unwrap_or(app_theme.neutral.text_1),
            muted_text: overrides
                .and_then(|theme| theme.muted_text)
                .unwrap_or(app_theme.neutral.text_3),
            caret: overrides
                .and_then(|theme| theme.caret)
                .unwrap_or(app_theme.primary.base),
            selection: overrides
                .and_then(|theme| theme.selection)
                .unwrap_or(app_theme.primary.base.opacity(0.30)),
            current_line: overrides
                .and_then(|theme| theme.current_line)
                .unwrap_or(app_theme.primary.base.opacity(0.055)),
            ruler: overrides
                .and_then(|theme| theme.ruler)
                .unwrap_or(app_theme.neutral.border.opacity(0.76)),
            whitespace: overrides
                .and_then(|theme| theme.whitespace)
                .unwrap_or(app_theme.neutral.text_3.opacity(0.42)),
            info: overrides
                .and_then(|theme| theme.info)
                .unwrap_or(app_theme.info.base),
            warning: overrides
                .and_then(|theme| theme.warning)
                .unwrap_or(app_theme.warning.base),
            error: overrides
                .and_then(|theme| theme.error)
                .unwrap_or(app_theme.danger.base),
            syntax: overrides
                .map(|theme| theme.syntax.clone())
                .unwrap_or_else(|| CodeEditorSyntaxTheme::from_code_theme(code_theme, app_theme)),
        }
    }

    fn diagnostic_color(&self, severity: CodeDiagnosticSeverity) -> Hsla {
        match severity {
            CodeDiagnosticSeverity::Info => self.info,
            CodeDiagnosticSeverity::Warning => self.warning,
            CodeDiagnosticSeverity::Error => self.error,
        }
    }
}

#[derive(Clone, Debug)]
struct CodeEditorSyntaxRun {
    range: Range<usize>,
    capture: String,
}

#[derive(Clone, Debug)]
struct CodeEditorSyntaxCache {
    language: CodeLanguage,
    text_revision: u64,
    runs: Arc<Vec<CodeEditorSyntaxRun>>,
}

fn tree_sitter_language(language: CodeLanguage) -> Option<TreeSitterLanguage> {
    match language {
        CodeLanguage::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
        CodeLanguage::Sql => Some(tree_sitter_sequel::LANGUAGE.into()),
        CodeLanguage::Shell => Some(tree_sitter_bash::LANGUAGE.into()),
        CodeLanguage::Xml => Some(tree_sitter_xml::LANGUAGE_XML.into()),
        CodeLanguage::Toml => Some(tree_sitter_toml_ng::LANGUAGE.into()),
        CodeLanguage::Yaml => Some(tree_sitter_yaml::LANGUAGE.into()),
        CodeLanguage::Conf => Some(tree_sitter_ini::LANGUAGE.into()),
        CodeLanguage::Json => Some(tree_sitter_json::LANGUAGE.into()),
        CodeLanguage::Markdown => Some(tree_sitter_md::LANGUAGE.into()),
        CodeLanguage::TypeScript | CodeLanguage::JavaScript => {
            Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        }
        CodeLanguage::PlainText => None,
    }
}

fn tree_sitter_highlight_query(language: CodeLanguage) -> Option<&'static str> {
    match language {
        CodeLanguage::Rust => Some(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/liora/queries/rust/highlights.scm"
        ))),
        CodeLanguage::Sql => Some(tree_sitter_sequel::HIGHLIGHTS_QUERY),
        CodeLanguage::Shell => Some(tree_sitter_bash::HIGHLIGHT_QUERY),
        CodeLanguage::Xml => Some(tree_sitter_xml::XML_HIGHLIGHT_QUERY),
        CodeLanguage::Toml => Some(tree_sitter_toml_ng::HIGHLIGHTS_QUERY),
        CodeLanguage::Yaml => Some(tree_sitter_yaml::HIGHLIGHTS_QUERY),
        CodeLanguage::Conf => Some(tree_sitter_ini::HIGHLIGHTS_QUERY),
        CodeLanguage::Json => Some(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/liora/queries/json/highlights.scm"
        ))),
        CodeLanguage::Markdown => Some(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/liora/queries/markdown/highlights.scm"
        ))),
        CodeLanguage::TypeScript => Some(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/liora/queries/typescript/highlights.scm"
        ))),
        CodeLanguage::JavaScript => Some(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/liora/queries/javascript/highlights.scm"
        ))),
        CodeLanguage::PlainText => None,
    }
}

fn zed_tree_sitter_syntax_runs(source: &str, language: CodeLanguage) -> Vec<CodeEditorSyntaxRun> {
    let Some(ts_language) = tree_sitter_language(language) else {
        return Vec::new();
    };
    let Some(query_source) = tree_sitter_highlight_query(language) else {
        return Vec::new();
    };
    let Ok(query) = Query::new(&ts_language, query_source) else {
        return Vec::new();
    };
    let mut parser = Parser::new();
    if parser.set_language(&ts_language).is_err() {
        return Vec::new();
    }
    let Some(tree) = parser.parse(source, None) else {
        return Vec::new();
    };
    let capture_names = query.capture_names();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    let mut runs = Vec::new();
    while let Some(mat) = matches.next() {
        for capture in mat.captures {
            let range = capture.node.byte_range();
            if range.start >= range.end || range.end > source.len() {
                continue;
            }
            let Some(capture_name) = capture_names.get(capture.index as usize) else {
                continue;
            };
            runs.push(CodeEditorSyntaxRun {
                range,
                capture: (*capture_name).to_string(),
            });
        }
    }
    runs.sort_by(|left, right| {
        left.range
            .start
            .cmp(&right.range.start)
            .then_with(|| right.range.end.cmp(&left.range.end))
    });
    runs
}

fn zed_tree_sitter_display_line_runs(
    display_line: &CodeLineDisplay,
    line_start: usize,
    source_runs: &[CodeEditorSyntaxRun],
    syntax_theme: &CodeEditorSyntaxTheme,
    default_color: Hsla,
    code_family: &SharedString,
    code_weight: Option<gpui::FontWeight>,
) -> Vec<TextRun> {
    let source_line_len = display_line.source_to_display.len().saturating_sub(1);
    let line_end = line_start + source_line_len;
    if display_line.text.is_empty() {
        return vec![base_code_text_run(
            0,
            default_color,
            None,
            code_family,
            code_weight,
            None,
        )];
    }

    let mut segments: Vec<(usize, usize, HighlightStyle)> = source_runs
        .iter()
        .filter_map(|run| {
            let start = run.range.start.max(line_start);
            let end = run.range.end.min(line_end);
            if start >= end {
                return None;
            }
            let display_start = display_line.display_offset_for_source(start - line_start);
            let display_end = display_line.display_offset_for_source(end - line_start);
            if display_start >= display_end {
                return None;
            }
            syntax_theme
                .style_for_capture(&run.capture)
                .map(|style| (display_start, display_end, style))
        })
        .collect();
    segments.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| (right.1 - right.0).cmp(&(left.1 - left.0)))
    });

    let mut runs = Vec::new();
    let mut cursor = 0;
    for (start, end, style) in segments {
        let start = clamp_to_char_boundary(&display_line.text, start);
        let end = clamp_to_char_boundary(&display_line.text, end);
        if end <= cursor || start >= end {
            continue;
        }
        if start > cursor {
            runs.push(base_code_text_run(
                start - cursor,
                default_color,
                None,
                code_family,
                code_weight,
                None,
            ));
        }
        runs.push(base_code_text_run(
            end - start,
            style.color.unwrap_or(default_color),
            style.background_color,
            code_family,
            style.font_weight.or(code_weight),
            style.font_style,
        ));
        cursor = end;
    }
    if cursor < display_line.text.len() {
        runs.push(base_code_text_run(
            display_line.text.len() - cursor,
            default_color,
            None,
            code_family,
            code_weight,
            None,
        ));
    }
    if runs.is_empty() {
        runs.push(base_code_text_run(
            display_line.text.len(),
            default_color,
            None,
            code_family,
            code_weight,
            None,
        ));
    }
    runs
}

fn base_code_text_run(
    len: usize,
    color: Hsla,
    background_color: Option<Hsla>,
    code_family: &SharedString,
    code_weight: Option<gpui::FontWeight>,
    font_style: Option<gpui::FontStyle>,
) -> TextRun {
    let mut run_font = font(code_family.clone());
    if let Some(weight) = code_weight {
        run_font.weight = weight;
    }
    if let Some(style) = font_style {
        run_font.style = style;
    }
    TextRun {
        len,
        font: run_font,
        color,
        background_color,
        underline: None,
        strikethrough: None,
    }
}

/// Native code editing surface with virtualized rows, line numbers, indentation metadata,
/// live syntax highlighting and pluggable diagnostics.
///
/// The v2 foundation follows Zed's public architecture lessons without copying or
/// depending on GPL editor crates: buffer state, selection state, viewport state,
/// and rendered rows are separated so the component can grow toward tree-sitter,
/// LSP, folding, and diff features while remaining pure Rust + native GPUI.
pub struct CodeEditor {
    buffer: CodeBuffer,
    selection: CodeSelection,
    marked_range: Option<Range<usize>>,
    focus_handle: FocusHandle,
    list_state: ListState,
    language: CodeLanguage,
    theme: CodeTheme,
    highlight_theme: Option<CodeEditorHighlightTheme>,
    options: CodeEditorOptions,
    tab_size: usize,
    soft_tabs: bool,
    viewport: CodeViewport,
    line_height: Option<Pixels>,
    folds: Vec<CodeFold>,
    height: Option<Pixels>,
    width: Option<Pixels>,
    full_width: bool,
    font_family: Option<SharedString>,
    font_size: Option<Pixels>,
    font_weight: Option<gpui::FontWeight>,
    gutter_width: Option<Pixels>,
    content_padding_x: Option<Pixels>,
    row_padding_y: Option<Pixels>,
    viewport_padding_y: Option<Pixels>,
    header_padding_x: Option<Pixels>,
    header_padding_y: Option<Pixels>,
    header_gap: Option<Pixels>,
    panel_padding_x: Option<Pixels>,
    panel_padding_y: Option<Pixels>,
    panel_gap: Option<Pixels>,
    editor_bounds: Option<Bounds<Pixels>>,
    cursor_visible: bool,
    blink_task: Option<gpui::Task<()>>,
    drag_selecting: bool,
    undo_stack: Vec<CodeTransaction>,
    redo_stack: Vec<CodeTransaction>,
    diagnostics: Vec<CodeDiagnostic>,
    diagnostics_provider: Option<Arc<CodeDiagnosticsProvider>>,
    completion_items: Vec<CodeCompletionItem>,
    completion_provider: Option<Arc<CodeCompletionProvider>>,
    hover: Option<CodeHover>,
    hover_provider: Option<Arc<CodeHoverProvider>>,
    search_query: Option<SharedString>,
    on_change: Option<Arc<CodeEditorChangeCallback>>,
    row_layouts: Vec<Option<CodeEditorRowLayout>>,
    text_revision: u64,
    syntax_cache: Option<CodeEditorSyntaxCache>,
}

impl CodeEditor {
    /// Creates `CodeEditor` initialized from the supplied value.
    pub fn new(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let value = value.into();
        let buffer = CodeBuffer::new(value.to_string());
        let row_count = buffer.line_count();
        let folds = detect_brace_folds(buffer.as_str());
        Self {
            selection: CodeSelection::new(buffer.len()),
            marked_range: None,
            buffer,
            focus_handle: cx.focus_handle(),
            list_state: ListState::new(row_count, ListAlignment::Top, px(160.0)),
            language: CodeLanguage::PlainText,
            theme: CodeTheme::Auto,
            highlight_theme: None,
            options: CodeEditorOptions::default(),
            tab_size: 4,
            soft_tabs: true,
            viewport: CodeViewport::new(row_count.max(8).min(24)),
            line_height: None,
            folds,
            height: None,
            width: None,
            full_width: true,
            font_family: None,
            font_size: None,
            font_weight: None,
            gutter_width: None,
            content_padding_x: None,
            row_padding_y: None,
            viewport_padding_y: None,
            header_padding_x: None,
            header_padding_y: None,
            header_gap: None,
            panel_padding_x: None,
            panel_padding_y: None,
            panel_gap: None,
            editor_bounds: None,
            cursor_visible: true,
            blink_task: None,
            drag_selecting: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            diagnostics: Vec::new(),
            diagnostics_provider: None,
            completion_items: Vec::new(),
            completion_provider: None,
            hover: None,
            hover_provider: None,
            search_query: None,
            on_change: None,
            row_layouts: vec![None; row_count],
            text_revision: 0,
            syntax_cache: None,
        }
    }

    /// Creates a GPUI entity that owns this component state across render passes.
    pub fn entity(value: impl Into<SharedString>, cx: &mut App) -> Entity<Self> {
        let value = value.into();
        cx.new(|cx| Self::new(value, cx))
    }

    /// Returns the serialized value used by forms, configuration, or persistence.
    pub fn value(&self, _cx: &App) -> SharedString {
        SharedString::from(self.buffer.as_str().to_string())
    }

    /// Updates the stored value value and keeps the existing component identity.
    pub fn set_value(&mut self, value: impl Into<SharedString>, cx: &mut Context<Self>) {
        let value = value.into();
        if self.buffer.as_str() == value.as_ref() {
            return;
        }
        self.buffer.set_text(value.to_string());
        self.bump_text_revision();
        self.selection.set_cursor(self.buffer.len());
        self.marked_range = None;
        self.invalidate_row_layouts();
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.refresh_automatic_folds();
        self.sync_list_state();
        self.handle_buffer_change(cx);
    }

    /// Returns the current zero-based caret point inside the editor buffer.
    pub fn cursor_point(&self) -> CodePoint {
        self.buffer.offset_to_point(self.selection.cursor())
    }

    /// Returns the current selected byte range.
    pub fn selected_range(&self) -> Range<usize> {
        self.selection.range.clone()
    }

    /// Updates the selected byte range and clamps it to valid UTF-8 boundaries.
    pub fn set_selection(&mut self, range: Range<usize>, cx: &mut Context<Self>) {
        let start = self.buffer.clamp_offset(range.start);
        let end = self.buffer.clamp_offset(range.end);
        self.selection.range = start.min(end)..start.max(end);
        self.selection.reversed = false;
        self.selection.preferred_column = None;
        self.marked_range = None;
        if self.options.reveal_cursor {
            self.reveal_cursor();
        }
        self.reset_blink(cx);
    }

    /// Sets the language identifier used for code display.
    pub fn language(mut self, language: impl Into<CodeLanguage>) -> Self {
        self.language = language.into();
        self
    }

    /// Updates the stored language value and keeps the existing component identity.
    pub fn set_language(&mut self, language: impl Into<CodeLanguage>, cx: &mut Context<Self>) {
        let language = language.into();
        if self.language != language {
            self.language = language;
            self.syntax_cache = None;
            self.invalidate_row_layouts();
            cx.notify();
        }
    }

    /// Applies an explicit built-in syntax theme or theme mode.
    pub fn theme(mut self, theme: CodeTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Applies a custom editor highlight theme on top of a built-in syntax palette.
    pub fn highlight_theme(mut self, theme: CodeEditorHighlightTheme) -> Self {
        self.theme = theme.base;
        self.highlight_theme = Some(theme);
        self
    }

    /// Clears custom editor highlight theme overrides and returns to the built-in code theme.
    pub fn clear_highlight_theme(mut self) -> Self {
        self.highlight_theme = None;
        self
    }

    /// Applies a loaded config file to the editor builder.
    pub fn config(mut self, config: CodeEditorConfig) -> Self {
        if let Some(language) = config.language {
            self.language = CodeLanguage::from_label(&language);
        }
        if let Some(theme) = config.theme.as_deref().and_then(parse_code_theme) {
            self.theme = theme;
        }
        if let Some(height) = config.height_px {
            self.height = Some(px(height));
        }
        if let Some(font_family) = config.font_family {
            self.font_family = Some(SharedString::from(font_family));
        }
        if let Some(font_size) = config.font_size_px {
            self.font_size = Some(px(font_size.max(8.0)));
        }
        if let Some(font_weight) = config.font_weight {
            self.font_weight = Some(gpui::FontWeight(font_weight));
        }
        if let Some(line_height) = config.line_height_px {
            self.line_height = Some(px(line_height.max(12.0)));
        }
        if let Some(layout) = config.layout {
            self.apply_layout_config(layout);
        }
        if let Some(tab_size) = config.tab_size {
            self.tab_size = tab_size.max(1);
        }
        if let Some(soft_tabs) = config.soft_tabs {
            self.soft_tabs = soft_tabs;
        }
        if let Some(options) = config.options {
            options.apply_to(&mut self.options);
        }
        if let Some(appearance) = config.appearance {
            let theme = appearance.to_theme(self.theme);
            self.theme = theme.base;
            self.highlight_theme = Some(theme);
        }
        self
    }

    /// Updates an existing editor entity from a loaded config file.
    pub fn set_config(&mut self, config: CodeEditorConfig, cx: &mut Context<Self>) {
        if let Some(language) = config.language {
            self.language = CodeLanguage::from_label(&language);
        }
        if let Some(theme) = config.theme.as_deref().and_then(parse_code_theme) {
            self.theme = theme;
        }
        if let Some(height) = config.height_px {
            self.height = Some(px(height));
        }
        if let Some(font_family) = config.font_family {
            self.font_family = Some(SharedString::from(font_family));
        }
        if let Some(font_size) = config.font_size_px {
            self.font_size = Some(px(font_size.max(8.0)));
        }
        if let Some(font_weight) = config.font_weight {
            self.font_weight = Some(gpui::FontWeight(font_weight));
        }
        if let Some(line_height) = config.line_height_px {
            self.line_height = Some(px(line_height.max(12.0)));
        }
        if let Some(layout) = config.layout {
            self.apply_layout_config(layout);
        }
        if let Some(tab_size) = config.tab_size {
            self.tab_size = tab_size.max(1);
        }
        if let Some(soft_tabs) = config.soft_tabs {
            self.soft_tabs = soft_tabs;
        }
        if let Some(options) = config.options {
            options.apply_to(&mut self.options);
        }
        if let Some(appearance) = config.appearance {
            let theme = appearance.to_theme(self.theme);
            self.theme = theme.base;
            self.highlight_theme = Some(theme);
        }
        self.invalidate_row_layouts();
        cx.notify();
    }

    /// Loads a config file once and applies it to this builder.
    pub fn config_file(mut self, path: impl AsRef<Path>) -> Result<Self, CodeEditorConfigError> {
        let config = CodeEditorConfig::load_from_path(path)?;
        self = self.config(config);
        Ok(self)
    }

    fn apply_layout_config(&mut self, layout: CodeEditorLayoutConfig) {
        if let Some(value) = layout.gutter_width_px {
            self.gutter_width = Some(px(value.max(0.0)));
        }
        if let Some(value) = layout.content_padding_x_px {
            self.content_padding_x = Some(px(value.max(0.0)));
        }
        if let Some(value) = layout.row_padding_y_px {
            self.row_padding_y = Some(px(value.max(0.0)));
        }
        if let Some(value) = layout.viewport_padding_y_px {
            self.viewport_padding_y = Some(px(value.max(0.0)));
        }
        if let Some(value) = layout.header_padding_x_px {
            self.header_padding_x = Some(px(value.max(0.0)));
        }
        if let Some(value) = layout.header_padding_y_px {
            self.header_padding_y = Some(px(value.max(0.0)));
        }
        if let Some(value) = layout.header_gap_px {
            self.header_gap = Some(px(value.max(0.0)));
        }
        if let Some(value) = layout.panel_padding_x_px {
            self.panel_padding_x = Some(px(value.max(0.0)));
        }
        if let Some(value) = layout.panel_padding_y_px {
            self.panel_padding_y = Some(px(value.max(0.0)));
        }
        if let Some(value) = layout.panel_gap_px {
            self.panel_gap = Some(px(value.max(0.0)));
        }
    }

    /// Sets the code font family used by visible editor rows.
    pub fn font_family(mut self, family: impl Into<SharedString>) -> Self {
        self.font_family = Some(family.into());
        self
    }

    /// Sets the code font size using plain pixel units.
    pub fn font_size_units(mut self, size: f32) -> Self {
        self.font_size = Some(px(size.max(8.0)));
        self
    }

    /// Sets the line-number gutter width using plain pixel units.
    pub fn gutter_width_units(mut self, width: f32) -> Self {
        self.gutter_width = Some(px(width.max(0.0)));
        self
    }

    /// Sets horizontal code content padding using plain pixel units.
    pub fn content_padding_x_units(mut self, padding: f32) -> Self {
        self.content_padding_x = Some(px(padding.max(0.0)));
        self
    }

    /// Sets vertical padding around each visible code row using plain pixel units.
    pub fn row_padding_y_units(mut self, padding: f32) -> Self {
        self.row_padding_y = Some(px(padding.max(0.0)));
        self
    }

    /// Sets the line numbers value used by the component.
    pub fn line_numbers(mut self, enabled: bool) -> Self {
        self.options.line_numbers = enabled;
        self
    }

    /// Sets all CodeEditor display and interaction options at once.
    pub fn options(mut self, options: CodeEditorOptions) -> Self {
        self.options = options;
        self
    }

    /// Controls whether editing commands may mutate the buffer.
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.options.read_only = read_only;
        self
    }

    /// Controls whether the top CodeEditor metadata toolbar is rendered.
    pub fn header(mut self, visible: bool) -> Self {
        self.options.header = visible;
        self
    }

    /// Controls whether the bottom CodeEditor status bar is rendered.
    pub fn status_bar(mut self, visible: bool) -> Self {
        self.options.status_bar = visible;
        self
    }

    /// Controls whether the diagnostics panel is rendered.
    pub fn diagnostics_panel(mut self, visible: bool) -> Self {
        self.options.diagnostics_panel = visible;
        self
    }

    /// Controls whether the completions panel is rendered.
    pub fn completions_panel(mut self, visible: bool) -> Self {
        self.options.completions_panel = visible;
        self
    }

    /// Controls whether the hover/help panel is rendered.
    pub fn hover_panel(mut self, visible: bool) -> Self {
        self.options.hover_panel = visible;
        self
    }

    /// Controls whether the active line receives a subtle highlight.
    pub fn current_line_highlight(mut self, enabled: bool) -> Self {
        self.options.current_line_highlight = enabled;
        self
    }

    /// Controls whether a right-edge ruler guide is rendered.
    pub fn rulers(mut self, enabled: bool) -> Self {
        self.options.rulers = enabled;
        self
    }

    /// Sets the primary ruler guide column.
    pub fn ruler_column(mut self, column: usize) -> Self {
        self.options.ruler_column = column.max(1);
        self
    }

    /// Controls visible whitespace rendering.
    pub fn whitespace(mut self, mode: CodeEditorWhitespaceMode) -> Self {
        self.options.whitespace = mode;
        self
    }

    /// Controls which diagnostics are shown inline near source rows.
    pub fn inline_diagnostics(mut self, mode: CodeEditorInlineDiagnostics) -> Self {
        self.options.inline_diagnostics = mode;
        self
    }

    /// Controls whether mouse and keyboard selection is enabled.
    pub fn selection(mut self, enabled: bool) -> Self {
        self.options.selection = enabled;
        self
    }

    /// Controls whether copy is enabled for selected text.
    pub fn copy_enabled(mut self, enabled: bool) -> Self {
        self.options.copy = enabled;
        self
    }

    /// Controls whether cut and paste can mutate editable text.
    pub fn clipboard_editing(mut self, enabled: bool) -> Self {
        self.options.clipboard_editing = enabled;
        self
    }

    /// Controls whether the caret blinks while focused.
    pub fn cursor_blink(mut self, enabled: bool) -> Self {
        self.options.cursor_blink = enabled;
        self
    }

    /// Controls whether drag selection is enabled.
    pub fn drag_selection(mut self, enabled: bool) -> Self {
        self.options.drag_selection = enabled;
        self
    }

    /// Controls whether indentation key bindings mutate selected lines.
    pub fn indentation(mut self, enabled: bool) -> Self {
        self.options.indentation = enabled;
        self
    }

    /// Controls whether undo/redo history is recorded.
    pub fn history(mut self, enabled: bool) -> Self {
        self.options.history = enabled;
        self
    }

    /// Controls whether cursor movement reveals the active row.
    pub fn reveal_cursor_on_move(mut self, enabled: bool) -> Self {
        self.options.reveal_cursor = enabled;
        self
    }

    /// Controls whether the built-in vertical scrollbar is rendered.
    pub fn scrollbar(mut self, enabled: bool) -> Self {
        self.options.scrollbar = enabled;
        self
    }

    /// Sets the maximum number of completion candidates rendered by the built-in panel.
    pub fn completion_limit(mut self, limit: usize) -> Self {
        self.options.completion_limit = limit.max(1);
        self
    }

    /// Sets the maximum number of diagnostics rendered by the built-in diagnostics panel.
    pub fn diagnostics_limit(mut self, limit: usize) -> Self {
        self.options.diagnostics_limit = limit.max(1);
        self
    }

    /// Sets the tab size value used by the component.
    pub fn tab_size(mut self, size: usize) -> Self {
        self.tab_size = size.max(1);
        self
    }

    /// Sets the soft tabs value used by the component.
    pub fn soft_tabs(mut self, enabled: bool) -> Self {
        self.soft_tabs = enabled;
        self
    }

    /// Sets the row line height used by visible editor rows.
    pub fn line_height(mut self, height: impl Into<Pixels>) -> Self {
        self.line_height = Some(height.into());
        self
    }

    /// Sets the row line height using plain pixel units.
    pub fn line_height_units(self, height: f32) -> Self {
        self.line_height(px(height.max(12.0)))
    }

    /// Controls whether leading indentation guide lines are painted.
    pub fn indent_guides(mut self, enabled: bool) -> Self {
        self.options.indent_guides = enabled;
        self
    }

    /// Controls whether configured fold ranges hide child rows.
    pub fn code_folding(mut self, enabled: bool) -> Self {
        self.options.code_folding = enabled;
        self.sync_list_state();
        self
    }

    /// Sets the collapsed state for a configured fold header line.
    pub fn set_fold_collapsed(
        &mut self,
        start_line: usize,
        collapsed: bool,
        cx: &mut Context<Self>,
    ) {
        if let Some(fold) = self
            .folds
            .iter_mut()
            .find(|fold| fold.start_line == start_line)
        {
            if fold.collapsed != collapsed {
                fold.collapsed = collapsed;
                self.sync_list_state();
                if self.options.reveal_cursor {
                    self.reveal_cursor();
                }
                cx.notify();
            }
        }
    }

    /// Toggles the collapsed state for a configured fold header line.
    pub fn toggle_fold(&mut self, start_line: usize, cx: &mut Context<Self>) {
        if let Some(collapsed) = self
            .folds
            .iter()
            .find(|fold| fold.start_line == start_line)
            .map(|fold| fold.collapsed)
        {
            self.set_fold_collapsed(start_line, !collapsed, cx);
        }
    }

    /// Replaces configured fold ranges.
    pub fn folds(mut self, folds: impl IntoIterator<Item = CodeFold>) -> Self {
        self.folds = normalize_folds(folds, self.buffer.line_count());
        self.sync_list_state();
        self
    }

    /// Adds one folded source range.
    pub fn fold_range(
        mut self,
        start_line: usize,
        end_line: usize,
        label: impl Into<SharedString>,
    ) -> Self {
        self.folds.push(CodeFold::new(start_line, end_line, label));
        self.folds = normalize_folds(self.folds, self.buffer.line_count());
        self.sync_list_state();
        self
    }

    /// Sets the visible row count for editor-like controls.
    pub fn rows(mut self, rows: usize) -> Self {
        self.viewport = CodeViewport::new(rows);
        self
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets the component width token used during GPUI layout.
    ///
    /// Calling this switches the editor from its default parent-filling behavior
    /// to an explicit fixed-width surface.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self.full_width = false;
        self
    }

    /// Sets the component width using plain pixel units for demo and config code.
    pub fn width_units(self, width: f32) -> Self {
        self.width(px(width))
    }

    /// Restores the default behavior where the editor fills the parent width.
    pub fn full_width(mut self) -> Self {
        self.width = None;
        self.full_width = true;
        self
    }

    /// Uses the built-in wide editor width suitable for constrained showcase cards.
    pub fn width_wide(self) -> Self {
        self.width(px(720.0))
    }

    /// Updates the stored width value and keeps the existing component identity.
    pub fn set_width(&mut self, width: impl Into<Pixels>, cx: &mut Context<Self>) {
        self.width = Some(width.into());
        self.full_width = false;
        cx.notify();
    }

    /// Updates the editor to fill the parent width and keeps the existing component identity.
    pub fn set_full_width(&mut self, cx: &mut Context<Self>) {
        self.width = None;
        self.full_width = true;
        cx.notify();
    }

    /// Preserves source compatibility with earlier CodeEditor versions.
    ///
    /// CodeEditor v2 renders syntax highlighting inside the editable virtual rows,
    /// so there is no separate preview panel to toggle. This builder intentionally
    /// remains a no-op to avoid breaking existing applications while keeping the
    /// editing surface and the live visual result unified.
    pub fn preview(self, _preview: bool) -> Self {
        self
    }

    /// Sets the diagnostics value used by the component.
    pub fn diagnostics(mut self, diagnostics: impl IntoIterator<Item = CodeDiagnostic>) -> Self {
        self.diagnostics = diagnostics.into_iter().collect();
        self
    }

    /// Updates the stored diagnostics value and keeps the existing component identity.
    pub fn set_diagnostics(
        &mut self,
        diagnostics: impl IntoIterator<Item = CodeDiagnostic>,
        cx: &mut Context<Self>,
    ) {
        self.diagnostics = diagnostics.into_iter().collect();
        cx.notify();
    }

    /// Performs the diagnostics provider operation used by this component.
    pub fn diagnostics_provider(
        mut self,
        provider: impl Fn(&str) -> Vec<CodeDiagnostic> + 'static,
    ) -> Self {
        self.diagnostics_provider = Some(Arc::new(provider));
        self
    }

    /// Updates the stored diagnostics provider value and keeps the existing component identity.
    pub fn set_diagnostics_provider(
        &mut self,
        provider: impl Fn(&str) -> Vec<CodeDiagnostic> + 'static,
        cx: &mut Context<Self>,
    ) {
        self.diagnostics_provider = Some(Arc::new(provider));
        self.refresh_providers();
        cx.notify();
    }

    /// Clears the current diagnostics provider state.
    pub fn clear_diagnostics_provider(&mut self, cx: &mut Context<Self>) {
        self.diagnostics_provider = None;
        cx.notify();
    }

    /// Sets static completion candidates rendered by the editor.
    pub fn completions(mut self, items: impl IntoIterator<Item = CodeCompletionItem>) -> Self {
        self.completion_items = items.into_iter().collect();
        self
    }

    /// Installs a provider that derives completion candidates from the current source.
    pub fn completion_provider(
        mut self,
        provider: impl Fn(&str) -> Vec<CodeCompletionItem> + 'static,
    ) -> Self {
        self.completion_provider = Some(Arc::new(provider));
        self
    }

    /// Sets static hover/help content rendered by the editor.
    pub fn hover(mut self, hover: CodeHover) -> Self {
        self.hover = Some(hover);
        self
    }

    /// Installs a provider that derives hover/help content from the current source.
    pub fn hover_provider(
        mut self,
        provider: impl Fn(&str) -> Option<CodeHover> + 'static,
    ) -> Self {
        self.hover_provider = Some(Arc::new(provider));
        self
    }

    /// Sets a plain-text search query and renders match count metadata.
    pub fn search_query(mut self, query: impl Into<SharedString>) -> Self {
        let query = query.into();
        self.search_query = (!query.is_empty()).then_some(query);
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(
        mut self,
        callback: impl Fn(&str, &mut Context<CodeEditor>) + 'static,
    ) -> Self {
        self.on_change = Some(Arc::new(callback));
        self
    }

    /// Updates the stored on change value and keeps the existing component identity.
    pub fn set_on_change(
        &mut self,
        callback: impl Fn(&str, &mut Context<CodeEditor>) + 'static,
        cx: &mut Context<Self>,
    ) {
        self.on_change = Some(Arc::new(callback));
        cx.notify();
    }

    /// Updates all display and interaction options for an existing editor entity.
    pub fn set_options(&mut self, options: CodeEditorOptions, cx: &mut Context<Self>) {
        self.options = options;
        cx.notify();
    }

    /// Updates the custom highlight theme for an existing editor entity.
    pub fn set_highlight_theme(
        &mut self,
        theme: Option<CodeEditorHighlightTheme>,
        cx: &mut Context<Self>,
    ) {
        if let Some(theme) = theme.as_ref() {
            self.theme = theme.base;
        }
        self.highlight_theme = theme;
        self.invalidate_row_layouts();
        cx.notify();
    }

    /// Updates the read-only state for an existing editor entity.
    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        self.options.read_only = read_only;
        cx.notify();
    }

    /// Updates whether line numbers are visible for an existing editor entity.
    pub fn set_line_numbers(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.options.line_numbers = enabled;
        cx.notify();
    }

    /// Updates the completion panel limit for an existing editor entity.
    pub fn set_completion_limit(&mut self, limit: usize, cx: &mut Context<Self>) {
        self.options.completion_limit = limit.max(1);
        cx.notify();
    }

    /// Updates the diagnostics panel limit for an existing editor entity.
    pub fn set_diagnostics_limit(&mut self, limit: usize, cx: &mut Context<Self>) {
        self.options.diagnostics_limit = limit.max(1);
        cx.notify();
    }

    /// Performs the indent unit operation used by this component.
    pub fn indent_unit(&self) -> String {
        if self.soft_tabs {
            " ".repeat(self.tab_size)
        } else {
            "\t".to_string()
        }
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("backspace", CodeEditorBackspace, None),
            KeyBinding::new("delete", CodeEditorDelete, None),
            KeyBinding::new("ctrl-backspace", CodeEditorDeleteWordLeft, None),
            KeyBinding::new("alt-backspace", CodeEditorDeleteWordLeft, None),
            KeyBinding::new("ctrl-delete", CodeEditorDeleteWordRight, None),
            KeyBinding::new("alt-delete", CodeEditorDeleteWordRight, None),
            KeyBinding::new("left", CodeEditorLeft, None),
            KeyBinding::new("shift-left", CodeEditorSelectLeft, None),
            KeyBinding::new("right", CodeEditorRight, None),
            KeyBinding::new("shift-right", CodeEditorSelectRight, None),
            KeyBinding::new("ctrl-left", CodeEditorWordLeft, None),
            KeyBinding::new("alt-left", CodeEditorWordLeft, None),
            KeyBinding::new("ctrl-shift-left", CodeEditorSelectWordLeft, None),
            KeyBinding::new("alt-shift-left", CodeEditorSelectWordLeft, None),
            KeyBinding::new("ctrl-right", CodeEditorWordRight, None),
            KeyBinding::new("alt-right", CodeEditorWordRight, None),
            KeyBinding::new("ctrl-shift-right", CodeEditorSelectWordRight, None),
            KeyBinding::new("alt-shift-right", CodeEditorSelectWordRight, None),
            KeyBinding::new("pageup", CodeEditorPageUp, None),
            KeyBinding::new("shift-pageup", CodeEditorSelectPageUp, None),
            KeyBinding::new("pagedown", CodeEditorPageDown, None),
            KeyBinding::new("shift-pagedown", CodeEditorSelectPageDown, None),
            KeyBinding::new("ctrl-home", CodeEditorDocumentStart, None),
            KeyBinding::new("cmd-up", CodeEditorDocumentStart, None),
            KeyBinding::new("ctrl-shift-home", CodeEditorSelectDocumentStart, None),
            KeyBinding::new("cmd-shift-up", CodeEditorSelectDocumentStart, None),
            KeyBinding::new("ctrl-end", CodeEditorDocumentEnd, None),
            KeyBinding::new("cmd-down", CodeEditorDocumentEnd, None),
            KeyBinding::new("ctrl-shift-end", CodeEditorSelectDocumentEnd, None),
            KeyBinding::new("cmd-shift-down", CodeEditorSelectDocumentEnd, None),
            KeyBinding::new("up", CodeEditorUp, None),
            KeyBinding::new("shift-up", CodeEditorSelectUp, None),
            KeyBinding::new("down", CodeEditorDown, None),
            KeyBinding::new("shift-down", CodeEditorSelectDown, None),
            KeyBinding::new("home", CodeEditorHome, None),
            KeyBinding::new("shift-home", CodeEditorSelectHome, None),
            KeyBinding::new("end", CodeEditorEnd, None),
            KeyBinding::new("shift-end", CodeEditorSelectEnd, None),
            KeyBinding::new("cmd-a", CodeEditorSelectAll, None),
            KeyBinding::new("ctrl-a", CodeEditorSelectAll, None),
            KeyBinding::new("cmd-c", CodeEditorCopy, None),
            KeyBinding::new("ctrl-c", CodeEditorCopy, None),
            KeyBinding::new("cmd-v", CodeEditorPaste, None),
            KeyBinding::new("ctrl-v", CodeEditorPaste, None),
            KeyBinding::new("cmd-x", CodeEditorCut, None),
            KeyBinding::new("ctrl-x", CodeEditorCut, None),
            KeyBinding::new("enter", CodeEditorEnter, None),
            KeyBinding::new("tab", CodeEditorIndent, None),
            KeyBinding::new("shift-tab", CodeEditorOutdent, None),
            KeyBinding::new("cmd-z", CodeEditorUndo, None),
            KeyBinding::new("ctrl-z", CodeEditorUndo, None),
            KeyBinding::new("cmd-shift-z", CodeEditorRedo, None),
            KeyBinding::new("ctrl-shift-z", CodeEditorRedo, None),
        ]);
    }

    fn display_map_for_font_size(&self, font_size: Pixels) -> CodeDisplayMap {
        CodeDisplayMap::default_for(
            self.options.line_numbers,
            self.line_height,
            font_size,
            self.gutter_width,
            self.content_padding_x,
            self.row_padding_y,
            self.viewport_padding_y,
        )
    }

    fn visible_rows(&self) -> Vec<usize> {
        visible_rows_for(
            self.buffer.line_count(),
            &self.folds,
            self.options.code_folding,
        )
    }

    fn refresh_automatic_folds(&mut self) {
        let existing = self.folds.clone();
        let mut folds = existing
            .iter()
            .filter(|fold| !fold.automatic)
            .cloned()
            .collect::<Vec<_>>();
        for mut detected in detect_brace_folds(self.buffer.as_str()) {
            if let Some(previous) = existing.iter().find(|fold| fold.same_range(&detected)) {
                detected.collapsed = previous.collapsed;
            }
            if !folds.iter().any(|fold| fold.same_range(&detected)) {
                folds.push(detected);
            }
        }
        self.folds = normalize_folds(folds, self.buffer.line_count());
    }

    fn display_row_for_buffer_row(&self, row: usize) -> usize {
        let visible_rows = self.visible_rows();
        visible_rows
            .iter()
            .position(|visible| *visible >= row)
            .unwrap_or_else(|| visible_rows.len().saturating_sub(1))
    }

    fn point_for_editor_position(&self, position: Point<Pixels>) -> usize {
        if let Some(offset) = self.offset_for_shaped_position(position) {
            return offset;
        }

        // The shaped layout cache is normally populated for visible rows during
        // prepaint. If a click arrives before a row has reported layout, use the
        // current live row layouts rather than stale virtual-list history.
        let mut fallback_row = 0;
        for (row, layout) in self.row_layouts.iter().enumerate() {
            let Some(layout) = layout else {
                continue;
            };
            if position.y >= layout.bounds.top() {
                fallback_row = row;
            }
        }
        let line = self.buffer.line(fallback_row);
        self.buffer
            .point_to_offset(CodePoint::new(fallback_row, line.len()))
    }

    fn offset_for_shaped_position(&self, position: Point<Pixels>) -> Option<usize> {
        for (row, layout) in self.row_layouts.iter().enumerate() {
            let Some(layout) = layout else {
                continue;
            };
            if position.y >= layout.bounds.top() && position.y <= layout.bounds.bottom() {
                let local_x = (position.x - layout.bounds.left()).max(px(0.0));
                let display_column = layout.shaped.closest_index_for_x(local_x);
                let source_column = layout.source_offset_for_display(display_column);
                return Some(
                    self.buffer
                        .point_to_offset(CodePoint::new(row, source_column)),
                );
            }
        }

        None
    }

    fn retain_visible_row_layouts(&mut self, visible_rows: &[usize]) {
        let visible_rows = visible_rows.iter().copied().collect::<HashSet<_>>();
        for (row, layout) in self.row_layouts.iter_mut().enumerate() {
            if layout.is_some() && !visible_rows.contains(&row) {
                *layout = None;
            }
        }
    }

    fn bounds_for_offset_range(&self, range: Range<usize>) -> Option<Bounds<Pixels>> {
        let range = normalize_replace_range(self.buffer.as_str(), range);
        let code_point = self.buffer.offset_to_point(range.start);
        let layout = self.row_layouts.get(code_point.row)?.as_ref()?;
        let row_start = self.buffer.line_start(code_point.row);
        let row_end = self.buffer.line_end(code_point.row);
        let start = range.start.clamp(row_start, row_end) - row_start;
        let end = range.end.clamp(row_start, row_end) - row_start;
        let display_start = layout.display_offset_for_source(start);
        let display_end = layout.display_offset_for_source(end);
        let x_start = layout.shaped.x_for_index(display_start);
        let x_end = layout
            .shaped
            .x_for_index(display_end)
            .max(x_start + px(1.0));
        Some(Bounds::new(
            point(layout.bounds.left() + x_start, layout.bounds.top()),
            size(x_end - x_start, layout.bounds.size.height),
        ))
    }

    fn word_range_at_offset(&self, offset: usize) -> Range<usize> {
        code_word_range_at_offset(&self.buffer, offset)
    }

    fn line_content_range_at_offset(&self, offset: usize) -> Range<usize> {
        code_line_content_range_at_offset(&self.buffer, offset)
    }

    fn position_hits_scrollbar(&self, position: Point<Pixels>) -> bool {
        self.editor_bounds.is_some_and(|bounds| {
            let hit_width = crate::virtual_scrollbar_hit_width();
            position.x >= bounds.right() - hit_width && position.x <= bounds.right()
        })
    }

    fn smart_line_start_at_offset(&self, offset: usize) -> usize {
        let line_start = self.buffer.line_start_at_offset(offset);
        let first_text = self.buffer.first_non_whitespace_at_offset(offset);
        if offset == first_text {
            line_start
        } else {
            first_text
        }
    }

    fn previous_word_boundary(&self, offset: usize) -> usize {
        code_previous_word_boundary(&self.buffer, offset)
    }

    fn next_word_boundary(&self, offset: usize) -> usize {
        code_next_word_boundary(&self.buffer, offset)
    }

    fn page_row_delta(&self) -> isize {
        self.viewport.rows.saturating_sub(1).max(1) as isize
    }

    fn invalidate_row_layouts(&mut self) {
        self.row_layouts.clear();
        self.row_layouts.resize(self.buffer.line_count(), None);
    }

    fn bump_text_revision(&mut self) {
        self.text_revision = self.text_revision.wrapping_add(1);
        self.syntax_cache = None;
    }

    fn syntax_runs(&mut self) -> Arc<Vec<CodeEditorSyntaxRun>> {
        if let Some(cache) = &self.syntax_cache {
            if cache.language == self.language && cache.text_revision == self.text_revision {
                return cache.runs.clone();
            }
        }

        let runs = Arc::new(zed_tree_sitter_syntax_runs(
            self.buffer.as_str(),
            self.language,
        ));
        self.syntax_cache = Some(CodeEditorSyntaxCache {
            language: self.language,
            text_revision: self.text_revision,
            runs: runs.clone(),
        });
        runs
    }

    fn update_row_layout(
        &mut self,
        row: usize,
        bounds: Bounds<Pixels>,
        shaped: ShapedLine,
        source_to_display: Vec<usize>,
        display_to_source: Vec<usize>,
    ) {
        if self.row_layouts.len() < self.buffer.line_count() {
            self.row_layouts.resize(self.buffer.line_count(), None);
        }
        if row < self.row_layouts.len() {
            self.row_layouts[row] = Some(CodeEditorRowLayout {
                bounds,
                shaped,
                source_to_display,
                display_to_source,
            });
        }
    }

    fn start_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        if !self.options.cursor_blink {
            self.blink_task = None;
            return;
        }
        let executor = cx.background_executor().clone();
        self.blink_task = Some(cx.spawn(async move |this, cx| {
            loop {
                executor.timer(Duration::from_millis(500)).await;
                let result = this.update(cx, |this, cx| {
                    this.cursor_visible = !this.cursor_visible;
                    cx.notify();
                });
                if result.is_err() {
                    break;
                }
            }
        }));
    }

    fn reset_blink(&mut self, cx: &mut Context<Self>) {
        self.cursor_visible = true;
        if !self.options.cursor_blink {
            self.blink_task = None;
            cx.notify();
        } else if self.blink_task.is_none() {
            self.start_blink(cx);
        } else {
            cx.notify();
        }
    }

    fn clear_selection_on_blur(&mut self, cx: &mut Context<Self>) {
        if !self.selection.is_empty() || self.drag_selecting {
            self.selection.set_cursor(self.selection.cursor());
            self.drag_selecting = false;
            cx.notify();
        }
    }

    fn mouse_down_in_editor(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.position_hits_scrollbar(event.position) {
            self.drag_selecting = false;
            return;
        }
        window.focus(&self.focus_handle, cx);
        let offset = self.point_for_editor_position(event.position);
        if event.modifiers.shift && self.options.selection {
            self.selection.select_to(offset);
        } else if event.click_count >= 3 && self.options.selection && self.options.line_selection {
            self.selection.range = self.line_content_range_at_offset(offset);
            self.selection.reversed = false;
            self.selection.preferred_column = None;
        } else if event.click_count == 2 && self.options.selection && self.options.word_selection {
            self.selection.range = self.word_range_at_offset(offset);
            self.selection.reversed = false;
            self.selection.preferred_column = None;
        } else {
            self.selection.set_cursor(offset);
        }
        self.drag_selecting = self.options.selection && self.options.drag_selection;
        if self.options.reveal_cursor {
            self.reveal_cursor();
        }
        self.reset_blink(cx);
    }

    fn mouse_move_in_editor(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.drag_selecting
            || !self.options.selection
            || !self.options.drag_selection
            || self.position_hits_scrollbar(event.position)
        {
            return;
        }
        if event.pressed_button != Some(MouseButton::Left) {
            self.drag_selecting = false;
            cx.notify();
            return;
        }
        let offset = self.point_for_editor_position(event.position);
        self.selection.select_to(offset);
        if self.options.reveal_cursor {
            self.reveal_cursor();
        }
        self.reset_blink(cx);
    }

    fn mouse_up_in_editor(
        &mut self,
        event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if event.button == MouseButton::Left && self.drag_selecting {
            self.drag_selecting = false;
            self.reset_blink(cx);
        }
    }

    fn sync_list_state(&mut self) {
        let line_count = self.buffer.line_count();
        let visible_count = self.visible_rows().len();
        let current_count = self.list_state.item_count();
        if visible_count > current_count {
            self.list_state
                .splice(current_count..current_count, visible_count - current_count);
        } else if visible_count < current_count {
            self.list_state.splice(visible_count..current_count, 0);
        }
        if self.row_layouts.len() != line_count {
            self.invalidate_row_layouts();
        }
    }

    fn refresh_providers(&mut self) {
        let value = self.buffer.as_str();
        if let Some(provider) = self.diagnostics_provider.clone() {
            self.diagnostics = provider(value);
        }
        if let Some(provider) = self.completion_provider.clone() {
            self.completion_items = provider(value);
        }
        if let Some(provider) = self.hover_provider.clone() {
            self.hover = provider(value);
        }
    }

    fn handle_buffer_change(&mut self, cx: &mut Context<Self>) {
        self.refresh_automatic_folds();
        self.sync_list_state();
        self.refresh_providers();
        if let Some(callback) = self.on_change.clone() {
            let value = self.buffer.as_str().to_string();
            callback(&value, cx);
        }
        if self.options.reveal_cursor {
            self.reveal_cursor();
        }
        self.reset_blink(cx);
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        self.buffer.as_str()[..offset.min(self.buffer.len())]
            .chars()
            .map(char::len_utf16)
            .sum()
    }

    fn offset_from_utf16(&self, target: usize) -> usize {
        let mut utf8 = 0;
        let mut utf16 = 0;
        for ch in self.buffer.as_str().chars() {
            if utf16 >= target {
                break;
            }
            utf16 += ch.len_utf16();
            utf8 += ch.len_utf8();
        }
        self.buffer.clamp_offset(utf8)
    }

    fn replace_selection(&mut self, replacement: &str, cx: &mut Context<Self>) {
        if self.options.read_only {
            return;
        }
        let transaction =
            CodeTransaction::new(self.buffer.as_str().to_string(), self.selection.clone());
        let range = self.selection.range.clone();
        let cursor = self.buffer.replace_range(range, replacement);
        self.selection.set_cursor(cursor);
        self.commit_transaction(transaction, cx);
    }

    fn commit_transaction(&mut self, transaction: CodeTransaction, cx: &mut Context<Self>) {
        let transaction =
            transaction.finish(self.buffer.as_str().to_string(), self.selection.clone());
        if !transaction.changed() {
            return;
        }
        if self.options.history {
            self.undo_stack.push(transaction);
            self.redo_stack.clear();
        }
        self.bump_text_revision();
        self.invalidate_row_layouts();
        self.sync_list_state();
        self.handle_buffer_change(cx);
    }

    fn restore_transaction_snapshot(
        &mut self,
        text: String,
        selection: CodeSelection,
        cx: &mut Context<Self>,
    ) {
        self.buffer.set_text(text);
        self.bump_text_revision();
        self.invalidate_row_layouts();
        self.selection = selection;
        self.selection.range =
            normalize_replace_range(self.buffer.as_str(), self.selection.range.clone());
        self.selection.preferred_column = None;
        self.sync_list_state();
        self.handle_buffer_change(cx);
    }

    fn move_to(&mut self, offset: usize, select: bool, cx: &mut Context<Self>) {
        let offset = self.buffer.clamp_offset(offset);
        if select && self.options.selection {
            self.selection.select_to(offset);
        } else {
            self.selection.set_cursor(offset);
        }
        self.marked_range = None;
        if self.options.reveal_cursor {
            self.reveal_cursor();
        }
        self.reset_blink(cx);
    }

    fn move_vertical(&mut self, delta: isize, select: bool, cx: &mut Context<Self>) {
        let cursor = self.selection.cursor();
        let point = self.buffer.offset_to_point(cursor);
        let preferred = self.selection.preferred_column.unwrap_or(point.column);
        let max_row = self.buffer.line_count().saturating_sub(1) as isize;
        let target_row = (point.row as isize + delta).clamp(0, max_row) as usize;
        let target = self
            .buffer
            .point_to_offset(CodePoint::new(target_row, preferred));
        self.selection.preferred_column = Some(preferred);
        if select {
            self.selection.select_to(target);
        } else {
            self.selection.set_cursor(target);
            self.selection.preferred_column = Some(preferred);
        }
        self.marked_range = None;
        self.reveal_cursor();
        self.reset_blink(cx);
    }

    fn reveal_cursor(&self) {
        let point = self.buffer.offset_to_point(self.selection.cursor());
        self.list_state
            .scroll_to_reveal_item(self.display_row_for_buffer_row(point.row));
    }

    fn indent(&mut self, _: &CodeEditorIndent, _: &mut Window, cx: &mut Context<Self>) {
        if self.options.read_only || !self.options.indentation {
            return;
        }
        let indent = self.indent_unit();
        if indent.is_empty() {
            return;
        }
        if self.selection.is_empty() {
            self.replace_selection(&indent, cx);
            return;
        }
        self.reindent_selected_lines(&indent, true, cx);
    }

    fn outdent(&mut self, _: &CodeEditorOutdent, _: &mut Window, cx: &mut Context<Self>) {
        if self.options.read_only || !self.options.indentation {
            return;
        }
        let indent = self.indent_unit();
        self.reindent_selected_lines(&indent, false, cx);
    }

    fn reindent_selected_lines(&mut self, indent: &str, indenting: bool, cx: &mut Context<Self>) {
        if self.options.read_only || !self.options.indentation {
            return;
        }
        let selection = self.selection.range.clone();
        let line_bounds = self.buffer.selected_line_bounds(selection.clone());
        let source = self.buffer.as_str().to_string();
        let mut next = String::with_capacity(source.len() + indent.len() * 4);
        next.push_str(&source[..line_bounds.start]);

        let mut selection_start_delta = 0isize;
        let mut selection_end_delta = 0isize;
        let mut cursor = line_bounds.start;
        let mut changed = false;

        for line in source[line_bounds.clone()].split_inclusive('\n') {
            let line_abs_start = cursor;
            let (line_body, line_ending) = line
                .strip_suffix('\n')
                .map_or((line, ""), |body| (body, "\n"));
            if indenting {
                next.push_str(indent);
                next.push_str(line_body);
                next.push_str(line_ending);
                changed = true;
                if line_abs_start <= selection.start {
                    selection_start_delta += indent.len() as isize;
                }
                if line_abs_start < selection.end || selection.is_empty() {
                    selection_end_delta += indent.len() as isize;
                }
            } else if let Some(remove_len) = removable_indent_len(line_body, indent) {
                next.push_str(&line_body[remove_len..]);
                next.push_str(line_ending);
                changed = true;
                if line_abs_start < selection.start {
                    selection_start_delta -= remove_len as isize;
                }
                if line_abs_start < selection.end {
                    selection_end_delta -= remove_len as isize;
                }
            } else {
                next.push_str(line_body);
                next.push_str(line_ending);
            }
            cursor += line.len();
        }

        if !changed {
            return;
        }

        next.push_str(&source[line_bounds.end..]);
        let transaction = CodeTransaction::new(source.clone(), self.selection.clone());
        self.buffer.set_text(next);
        self.invalidate_row_layouts();
        let start = apply_signed_delta(selection.start, selection_start_delta);
        let end = apply_signed_delta(selection.end, selection_end_delta).max(start);
        self.selection.range = self.buffer.clamp_offset(start)..self.buffer.clamp_offset(end);
        self.selection.reversed = false;
        self.selection.preferred_column = None;
        self.commit_transaction(transaction, cx);
    }

    fn backspace(&mut self, _: &CodeEditorBackspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.options.read_only {
            return;
        }
        if self.selection.is_empty() {
            let cursor = self.selection.cursor();
            let previous = self.buffer.prev_char(cursor);
            if previous == cursor {
                return;
            }
            self.selection.range = previous..cursor;
        }
        self.replace_selection("", cx);
    }

    fn delete(&mut self, _: &CodeEditorDelete, _: &mut Window, cx: &mut Context<Self>) {
        if self.options.read_only {
            return;
        }
        if self.selection.is_empty() {
            let cursor = self.selection.cursor();
            let next = self.buffer.next_char(cursor);
            if next == cursor {
                return;
            }
            self.selection.range = cursor..next;
        }
        self.replace_selection("", cx);
    }

    fn delete_word_left(
        &mut self,
        _: &CodeEditorDeleteWordLeft,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.options.read_only {
            return;
        }
        if self.selection.is_empty() {
            let cursor = self.selection.cursor();
            let previous = self.previous_word_boundary(cursor);
            if previous == cursor {
                return;
            }
            self.selection.range = previous..cursor;
        }
        self.replace_selection("", cx);
    }

    fn delete_word_right(
        &mut self,
        _: &CodeEditorDeleteWordRight,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.options.read_only {
            return;
        }
        if self.selection.is_empty() {
            let cursor = self.selection.cursor();
            let next = self.next_word_boundary(cursor);
            if next == cursor {
                return;
            }
            self.selection.range = cursor..next;
        }
        self.replace_selection("", cx);
    }

    fn left(&mut self, _: &CodeEditorLeft, _: &mut Window, cx: &mut Context<Self>) {
        if self.selection.is_empty() {
            self.move_to(self.buffer.prev_char(self.selection.cursor()), false, cx);
        } else {
            self.move_to(self.selection.range.start, false, cx);
        }
    }

    fn select_left(&mut self, _: &CodeEditorSelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.buffer.prev_char(self.selection.cursor()), true, cx);
    }

    fn right(&mut self, _: &CodeEditorRight, _: &mut Window, cx: &mut Context<Self>) {
        if self.selection.is_empty() {
            self.move_to(self.buffer.next_char(self.selection.cursor()), false, cx);
        } else {
            self.move_to(self.selection.range.end, false, cx);
        }
    }

    fn select_right(&mut self, _: &CodeEditorSelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.buffer.next_char(self.selection.cursor()), true, cx);
    }

    fn word_left(&mut self, _: &CodeEditorWordLeft, _: &mut Window, cx: &mut Context<Self>) {
        let target = if self.selection.is_empty() {
            self.previous_word_boundary(self.selection.cursor())
        } else {
            self.selection.range.start
        };
        self.move_to(target, false, cx);
    }

    fn select_word_left(
        &mut self,
        _: &CodeEditorSelectWordLeft,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to(
            self.previous_word_boundary(self.selection.cursor()),
            true,
            cx,
        );
    }

    fn word_right(&mut self, _: &CodeEditorWordRight, _: &mut Window, cx: &mut Context<Self>) {
        let target = if self.selection.is_empty() {
            self.next_word_boundary(self.selection.cursor())
        } else {
            self.selection.range.end
        };
        self.move_to(target, false, cx);
    }

    fn select_word_right(
        &mut self,
        _: &CodeEditorSelectWordRight,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to(self.next_word_boundary(self.selection.cursor()), true, cx);
    }

    fn page_up(&mut self, _: &CodeEditorPageUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(-self.page_row_delta(), false, cx);
    }

    fn select_page_up(
        &mut self,
        _: &CodeEditorSelectPageUp,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_vertical(-self.page_row_delta(), true, cx);
    }

    fn page_down(&mut self, _: &CodeEditorPageDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(self.page_row_delta(), false, cx);
    }

    fn select_page_down(
        &mut self,
        _: &CodeEditorSelectPageDown,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_vertical(self.page_row_delta(), true, cx);
    }

    fn document_start(
        &mut self,
        _: &CodeEditorDocumentStart,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to(0, false, cx);
    }

    fn select_document_start(
        &mut self,
        _: &CodeEditorSelectDocumentStart,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to(0, true, cx);
    }

    fn document_end(&mut self, _: &CodeEditorDocumentEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.buffer.len(), false, cx);
    }

    fn select_document_end(
        &mut self,
        _: &CodeEditorSelectDocumentEnd,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to(self.buffer.len(), true, cx);
    }

    fn up(&mut self, _: &CodeEditorUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(-1, false, cx);
    }

    fn select_up(&mut self, _: &CodeEditorSelectUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(-1, true, cx);
    }

    fn down(&mut self, _: &CodeEditorDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(1, false, cx);
    }

    fn select_down(&mut self, _: &CodeEditorSelectDown, _: &mut Window, cx: &mut Context<Self>) {
        self.move_vertical(1, true, cx);
    }

    fn home(&mut self, _: &CodeEditorHome, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(
            self.smart_line_start_at_offset(self.selection.cursor()),
            false,
            cx,
        );
    }

    fn select_home(&mut self, _: &CodeEditorSelectHome, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(
            self.smart_line_start_at_offset(self.selection.cursor()),
            true,
            cx,
        );
    }

    fn end(&mut self, _: &CodeEditorEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(
            self.buffer.line_end_at_offset(self.selection.cursor()),
            false,
            cx,
        );
    }

    fn select_end(&mut self, _: &CodeEditorSelectEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(
            self.buffer.line_end_at_offset(self.selection.cursor()),
            true,
            cx,
        );
    }

    fn select_all(&mut self, _: &CodeEditorSelectAll, _: &mut Window, cx: &mut Context<Self>) {
        if !self.options.selection {
            return;
        }
        self.selection.range = 0..self.buffer.len();
        self.selection.reversed = false;
        self.selection.preferred_column = None;
        self.reset_blink(cx);
    }

    fn copy(&mut self, _: &CodeEditorCopy, _: &mut Window, cx: &mut Context<Self>) {
        if self.options.copy && !self.selection.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.buffer.as_str()[self.selection.range.clone()].to_string(),
            ));
        }
    }

    fn paste(&mut self, _: &CodeEditorPaste, _: &mut Window, cx: &mut Context<Self>) {
        if self.options.read_only || !self.options.clipboard_editing {
            return;
        }
        if let Some(clipboard) = cx.read_from_clipboard() {
            if let Some(text) = clipboard.text() {
                self.replace_selection(&text, cx);
            }
        }
    }

    fn cut(&mut self, _: &CodeEditorCut, window: &mut Window, cx: &mut Context<Self>) {
        if self.options.read_only || !self.options.clipboard_editing {
            return;
        }
        if !self.selection.is_empty() {
            self.copy(&CodeEditorCopy, window, cx);
            self.replace_selection("", cx);
        }
    }

    fn enter(&mut self, _: &CodeEditorEnter, _: &mut Window, cx: &mut Context<Self>) {
        if self.options.read_only {
            return;
        }
        self.replace_selection("\n", cx);
    }

    fn undo(&mut self, _: &CodeEditorUndo, _: &mut Window, cx: &mut Context<Self>) {
        if self.options.read_only || !self.options.history {
            return;
        }
        if let Some(transaction) = self.undo_stack.pop() {
            self.restore_transaction_snapshot(
                transaction.before_text.clone(),
                transaction.before_selection.clone(),
                cx,
            );
            self.redo_stack.push(transaction);
        }
    }

    fn redo(&mut self, _: &CodeEditorRedo, _: &mut Window, cx: &mut Context<Self>) {
        if self.options.read_only || !self.options.history {
            return;
        }
        if let Some(transaction) = self.redo_stack.pop() {
            self.restore_transaction_snapshot(
                transaction.after_text.clone(),
                transaction.after_selection.clone(),
                cx,
            );
            self.undo_stack.push(transaction);
        }
    }
}

impl EntityInputHandler for CodeEditor {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let start = self.offset_from_utf16(range_utf16.start);
        let end = self.offset_from_utf16(range_utf16.end);
        (start <= end && end <= self.buffer.len())
            .then(|| self.buffer.as_str()[start..end].to_string())
    }

    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        self.options.selection.then(|| UTF16Selection {
            range: self.offset_to_utf16(self.selection.range.start)
                ..self.offset_to_utf16(self.selection.range.end),
            reversed: self.selection.reversed,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end))
    }

    fn unmark_text(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.marked_range = None;
        self.reset_blink(cx);
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.options.read_only {
            return;
        }
        let range = range_utf16
            .map(|range| self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end))
            .or_else(|| self.marked_range.clone())
            .unwrap_or_else(|| self.selection.range.clone());
        self.selection.range = normalize_replace_range(self.buffer.as_str(), range);
        self.selection.reversed = false;
        self.marked_range = None;
        self.replace_selection(new_text, cx);
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected: Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.options.read_only {
            return;
        }
        let replacement_start = range_utf16
            .as_ref()
            .map(|range| self.offset_from_utf16(range.start))
            .or_else(|| self.marked_range.as_ref().map(|range| range.start))
            .unwrap_or_else(|| self.selection.range.start);
        let range = range_utf16
            .map(|range| self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end))
            .or_else(|| self.marked_range.clone())
            .unwrap_or_else(|| self.selection.range.clone());
        self.selection.range = normalize_replace_range(self.buffer.as_str(), range);
        self.selection.reversed = false;
        self.marked_range = None;
        self.replace_selection(new_text, cx);
        self.marked_range = (!new_text.is_empty()).then_some(
            replacement_start..self.buffer.clamp_offset(replacement_start + new_text.len()),
        );
        if let Some(selected) = new_selected {
            let selected_start = utf16_offset_in_text(new_text, selected.start);
            let selected_end = utf16_offset_in_text(new_text, selected.end);
            let start = replacement_start + selected_start;
            let end = replacement_start + selected_end;
            self.selection.range = normalize_replace_range(self.buffer.as_str(), start..end);
            self.selection.reversed = false;
            self.reset_blink(cx);
        }
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _bounds: Bounds<Pixels>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let start = self.offset_from_utf16(range_utf16.start);
        let end = self.offset_from_utf16(range_utf16.end);
        self.bounds_for_offset_range(start..end)
    }

    fn character_index_for_point(
        &mut self,
        pt: Point<Pixels>,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        Some(self.offset_to_utf16(self.point_for_editor_position(pt)))
    }
}

struct CodeEditorInputLayer {
    editor: Entity<CodeEditor>,
}

impl Element for CodeEditorInputLayer {
    type RequestLayoutState = ();
    type PrepaintState = ();

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
        let mut style = Style::default();
        style.position = gpui::Position::Absolute;
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.editor.read(cx).focus_handle.clone();
        let input_bounds = editor_text_input_bounds(bounds);
        if self.editor.read(cx).editor_bounds != Some(bounds) {
            self.editor.update(cx, |editor, _| {
                editor.editor_bounds = Some(bounds);
            });
        }
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(input_bounds, self.editor.clone()),
            cx,
        );
    }
}

fn editor_text_input_bounds(bounds: Bounds<Pixels>) -> Bounds<Pixels> {
    let hit_width = crate::virtual_scrollbar_hit_width();
    Bounds {
        origin: bounds.origin,
        size: size(
            (bounds.size.width - hit_width).max(px(1.0)),
            bounds.size.height,
        ),
    }
}

impl Focusable for CodeEditor {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CodeEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        cx.on_blur(&self.focus_handle, window, |this, _, cx| {
            this.clear_selection_on_blur(cx);
        })
        .detach();

        self.refresh_providers();
        let focused = self.focus_handle(cx).is_focused(window);
        if focused && self.blink_task.is_none() {
            self.start_blink(cx);
        } else if !focused && self.blink_task.is_some() {
            self.blink_task = None;
            self.cursor_visible = true;
        }

        let theme = cx.global::<Config>().theme.clone();
        let code_family = self
            .font_family
            .clone()
            .unwrap_or_else(|| code_font_family(cx));
        let code_weight = self.font_weight.or_else(|| code_font_weight(cx));
        let code_font_size = self.font_size.unwrap_or_else(|| px(theme.font_size.sm));
        let line_count = self.buffer.line_count();
        let cursor = self.cursor_point();
        let indent_label = if self.soft_tabs {
            format!("spaces:{}", self.tab_size)
        } else {
            "tabs".to_string()
        };
        let options = self.options;
        let editor_theme =
            ResolvedCodeEditorTheme::resolve(self.highlight_theme.as_ref(), self.theme, &theme);
        let display_map = self.display_map_for_font_size(code_font_size);
        let editor_height = self
            .height
            .unwrap_or_else(|| self.viewport.height(display_map));
        let focus_handle = self.focus_handle(cx);
        let list_state = self.list_state.clone();
        let buffer = self.buffer.clone();
        let visible_rows_for_layout = self.visible_rows();
        self.retain_visible_row_layouts(&visible_rows_for_layout);
        let visible_rows = Arc::new(visible_rows_for_layout);
        let selection = self.selection.clone();
        let marked_range = self.marked_range.clone();
        let diagnostics = self.diagnostics.clone();
        let self_folds_for_rows = self.folds.clone();
        let show_line_numbers = options.line_numbers;
        let row_display_map = display_map;
        let row_tab_size = self.tab_size;
        let row_language = self.language;
        let row_code_theme = self.theme;
        let code_family_for_rows = code_family.clone();
        let theme_for_rows = theme.clone();
        let editor_theme_for_rows = editor_theme.clone();
        let code_weight_for_rows = code_weight;
        let syntax_runs_for_rows = self.syntax_runs();
        let editor_entity = cx.entity();
        let editor_entity_for_rows = editor_entity.clone();
        let cursor_active = focused;
        let cursor_visible = focused && self.cursor_visible;
        let header_padding_x = self.header_padding_x.unwrap_or_else(|| px(16.0));
        let header_padding_y = self.header_padding_y.unwrap_or_else(|| px(8.0));
        let header_gap = self.header_gap.unwrap_or_else(|| px(12.0));
        let panel_padding_x = self.panel_padding_x.unwrap_or_else(|| px(16.0));
        let panel_padding_y = self.panel_padding_y.unwrap_or_else(|| px(12.0));
        let panel_gap = self.panel_gap.unwrap_or_else(|| px(4.0));

        div()
            .flex()
            .flex_col()
            .when(self.full_width, |s| {
                s.w_full().min_w(px(0.0)).flex_shrink(1.0)
            })
            .when_some(self.width, |s, width| s.w(width))
            .rounded(px(theme.radius.lg))
            .border_1()
            .border_color(if focused {
                editor_theme.caret
            } else {
                editor_theme.border
            })
            .bg(editor_theme.surface)
            .overflow_hidden()
            .track_focus(&focus_handle)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::delete_word_left))
            .on_action(cx.listener(Self::delete_word_right))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::word_left))
            .on_action(cx.listener(Self::select_word_left))
            .on_action(cx.listener(Self::word_right))
            .on_action(cx.listener(Self::select_word_right))
            .on_action(cx.listener(Self::page_up))
            .on_action(cx.listener(Self::select_page_up))
            .on_action(cx.listener(Self::page_down))
            .on_action(cx.listener(Self::select_page_down))
            .on_action(cx.listener(Self::document_start))
            .on_action(cx.listener(Self::select_document_start))
            .on_action(cx.listener(Self::document_end))
            .on_action(cx.listener(Self::select_document_end))
            .on_action(cx.listener(Self::up))
            .on_action(cx.listener(Self::select_up))
            .on_action(cx.listener(Self::down))
            .on_action(cx.listener(Self::select_down))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::select_home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::select_end))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::enter))
            .on_action(cx.listener(Self::indent))
            .on_action(cx.listener(Self::outdent))
            .on_action(cx.listener(Self::undo))
            .on_action(cx.listener(Self::redo))
            .when(options.header, |s| {
                s.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(header_gap)
                        .px(header_padding_x)
                        .py(header_padding_y)
                        .border_b_1()
                        .border_color(editor_theme.border)
                        .bg(editor_theme.chrome_surface)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .text_sm()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(editor_theme.text)
                                .child(
                                    Icon::new(IconName::FileCode)
                                        .size(px(14.0))
                                        .color(editor_theme.caret),
                                )
                                .child("CodeEditor")
                                .when(options.read_only, |s| {
                                    s.child(
                                        div()
                                            .rounded_md()
                                            .border_1()
                                            .border_color(editor_theme.border)
                                            .px_1()
                                            .text_xs()
                                            .text_color(editor_theme.muted_text)
                                            .child("read-only"),
                                    )
                                }),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(header_gap)
                                .text_xs()
                                .text_color(editor_theme.muted_text)
                                .child(self.language.label())
                                .child(indent_label)
                                .child(format!("{} lines", line_count))
                                .child(format!("{}:{}", cursor.row + 1, cursor.column + 1))
                                .when_some(self.search_query.clone(), |s, query| {
                                    s.child(format!(
                                        "matches:{}",
                                        search_match_count(self.buffer.as_str(), query.as_ref())
                                    ))
                                }),
                        ),
                )
            })
            .child(
                div()
                    .relative()
                    .h(editor_height)
                    .bg(editor_theme.surface)
                    .when(!options.read_only, |s| s.cursor_text())
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::mouse_down_in_editor))
                    .on_mouse_move(cx.listener(Self::mouse_move_in_editor))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::mouse_up_in_editor))
                    .on_mouse_up_out(MouseButton::Left, cx.listener(Self::mouse_up_in_editor))
                    .child(
                        list(list_state.clone(), move |row, _window, _cx| {
                            let editor_for_row = editor_entity_for_rows.clone();
                            let buffer_row = visible_rows.get(row).copied().unwrap_or(row);
                            render_editor_row(
                                buffer_row,
                                &buffer,
                                &selection,
                                marked_range.as_ref(),
                                &diagnostics,
                                &self_folds_for_rows,
                                show_line_numbers,
                                row_language,
                                row_code_theme,
                                &theme_for_rows,
                                &editor_theme_for_rows,
                                syntax_runs_for_rows.as_ref(),
                                code_family_for_rows.clone(),
                                code_weight_for_rows,
                                code_font_size,
                                row_display_map,
                                row_tab_size,
                                options,
                                cursor_active,
                                cursor_visible,
                                editor_for_row,
                            )
                            .into_any_element()
                        })
                        .size_full()
                        .into_any_element(),
                    )
                    .child(CodeEditorInputLayer {
                        editor: editor_entity,
                    })
                    .when(options.rulers, |s| {
                        s.child(render_ruler(
                            display_map,
                            options.ruler_column,
                            &editor_theme,
                        ))
                    })
                    .when(options.scrollbar, |s| {
                        s.child(VirtualScrollbar::new(list_state))
                    }),
            )
            .when(options.status_bar, |s| {
                s.child(render_status_bar(
                    cursor,
                    line_count,
                    self.buffer.len(),
                    options,
                    &theme,
                    &editor_theme,
                    panel_padding_x,
                    panel_padding_y,
                ))
            })
            .when(
                options.diagnostics_panel && !self.diagnostics.is_empty(),
                |s| {
                    s.child(render_diagnostics(
                        &self.diagnostics,
                        options.diagnostics_limit,
                        &theme,
                        &editor_theme,
                        panel_padding_x,
                        panel_padding_y,
                        panel_gap,
                    ))
                },
            )
            .when(
                options.completions_panel && !self.completion_items.is_empty(),
                |s| {
                    s.child(render_completions(
                        &self.completion_items,
                        options.completion_limit,
                        &theme,
                        &editor_theme,
                        panel_padding_x,
                        panel_padding_y,
                        panel_gap,
                    ))
                },
            )
            .when_some(
                options.hover_panel.then(|| self.hover.clone()).flatten(),
                |s, hover| {
                    s.child(render_hover(
                        hover,
                        &theme,
                        &editor_theme,
                        panel_padding_x,
                        panel_padding_y,
                    ))
                },
            )
    }
}

#[derive(Clone)]
struct CodeEditorRowLayout {
    bounds: Bounds<Pixels>,
    shaped: ShapedLine,
    source_to_display: Vec<usize>,
    display_to_source: Vec<usize>,
}

impl CodeEditorRowLayout {
    fn display_offset_for_source(&self, source_offset: usize) -> usize {
        self.source_to_display
            .get(source_offset.min(self.source_to_display.len().saturating_sub(1)))
            .copied()
            .unwrap_or(0)
    }

    fn source_offset_for_display(&self, display_offset: usize) -> usize {
        self.display_to_source
            .get(display_offset.min(self.display_to_source.len().saturating_sub(1)))
            .copied()
            .unwrap_or(0)
    }
}

fn render_editor_row(
    row: usize,
    buffer: &CodeBuffer,
    selection: &CodeSelection,
    marked_range: Option<&Range<usize>>,
    diagnostics: &[CodeDiagnostic],
    folds: &[CodeFold],
    line_numbers: bool,
    language: CodeLanguage,
    code_theme: CodeTheme,
    theme: &liora_theme::Theme,
    editor_theme: &ResolvedCodeEditorTheme,
    syntax_runs: &[CodeEditorSyntaxRun],
    code_family: SharedString,
    code_weight: Option<gpui::FontWeight>,
    code_font_size: Pixels,
    display_map: CodeDisplayMap,
    tab_size: usize,
    options: CodeEditorOptions,
    cursor_active: bool,
    cursor_visible: bool,
    editor: Entity<CodeEditor>,
) -> gpui::Div {
    let line = buffer.line(row);
    let display_line = CodeLineDisplay::expand_tabs(line, tab_size);
    let cursor_point = buffer.offset_to_point(selection.cursor());
    let cursor_row = cursor_point.row == row;
    let cursor_column =
        cursor_row.then(|| display_line.display_offset_for_source(cursor_point.column));
    let selection_range = options
        .selection
        .then(|| line_selection_range(buffer, selection, row))
        .flatten()
        .map(|range| display_line.display_range_for_source(range));
    let marked_range = marked_range
        .and_then(|range| line_offset_range(buffer, range, row))
        .map(|range| display_line.display_range_for_source(range));
    let line_diagnostics = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.line.saturating_sub(1) == row)
        .filter(|diagnostic| options.inline_diagnostics.includes(diagnostic.severity))
        .collect::<Vec<_>>();
    let fold = fold_at_row(folds, row, options.code_folding);
    let highlight_current_line = options.current_line_highlight && cursor_row;
    let line_start = buffer.line_start(row);
    let runs = zed_tree_sitter_display_line_runs(
        &display_line,
        line_start,
        syntax_runs,
        &editor_theme.syntax,
        editor_theme.text,
        &code_family,
        code_weight,
    );
    let _ = (language, code_theme, theme);

    div()
        .flex()
        .items_start()
        .min_h(display_map.row_height())
        .child(if line_numbers {
            let fold_for_gutter = fold.cloned();
            let row_number_color = if cursor_row {
                editor_theme.caret
            } else {
                editor_theme.muted_text
            };
            let toggle_editor = editor.clone();
            div()
                .flex_none()
                .w(display_map.gutter_width(line_numbers))
                .min_h(display_map.row_height())
                .px(px(8.0))
                .py(display_map.row_padding_y)
                .border_r_1()
                .border_color(editor_theme.border)
                .bg(editor_theme.gutter_surface)
                .font_family(code_family.clone())
                .when_some(code_weight, |s, weight| s.font_weight(weight))
                .text_xs()
                .text_color(row_number_color)
                .flex()
                .items_center()
                .gap_1()
                .child(match fold_for_gutter {
                    Some(fold) => div()
                        .id(element_id(format!("code-editor-fold-toggle-{row}")))
                        .w(px(14.0))
                        .h(px(16.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_sm()
                        .cursor_pointer()
                        .text_color(editor_theme.muted_text)
                        .hover(|s| {
                            s.bg(editor_theme.chrome_surface)
                                .text_color(editor_theme.caret)
                        })
                        .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                            toggle_editor.update(cx, |editor, cx| editor.toggle_fold(row, cx));
                            cx.stop_propagation();
                        })
                        .child(if fold.collapsed { "▶" } else { "▼" })
                        .into_any_element(),
                    None => div().w(px(14.0)).h(px(16.0)).into_any_element(),
                })
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.0))
                        .text_right()
                        .child(format!("{}", row + 1)),
                )
                .into_any_element()
        } else {
            div().into_any_element()
        })
        .child(
            div()
                .flex_1()
                .min_w_0()
                .px(display_map.content_left_padding)
                .py(display_map.row_padding_y)
                .font_family(code_family.clone())
                .when_some(code_weight, |s, weight| s.font_weight(weight))
                .text_size(code_font_size)
                .text_color(editor_theme.text)
                .child(CodeEditorLineElement {
                    row,
                    text: SharedString::from(display_line.text.clone()),
                    runs,
                    source_to_display: display_line.source_to_display.clone(),
                    display_to_source: display_line.display_to_source.clone(),
                    selection_range,
                    marked_range,
                    cursor_column,
                    cursor_visible: cursor_active && cursor_visible && cursor_column.is_some(),
                    font_size: code_font_size,
                    line_height: display_map.text_line_height(),
                    current_line_highlight: highlight_current_line,
                    indent_guides: options.indent_guides,
                    indent_columns: leading_indent_columns(line, tab_size),
                    editor_theme: editor_theme.clone(),
                    whitespace: options.whitespace,
                    editor: editor.clone(),
                })
                .when(!line_diagnostics.is_empty(), |s| {
                    let mut diagnostics_row = div().flex().flex_col().gap_1().mt_1();
                    for diagnostic in line_diagnostics {
                        let color = editor_theme.diagnostic_color(diagnostic.severity);
                        diagnostics_row = diagnostics_row.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .text_xs()
                                .text_color(color)
                                .child(div().size(px(5.0)).rounded_full().bg(color))
                                .child(diagnostic.message.clone()),
                        );
                    }
                    s.child(diagnostics_row)
                }),
        )
}

struct CodeEditorLineElement {
    row: usize,
    text: SharedString,
    runs: Vec<TextRun>,
    source_to_display: Vec<usize>,
    display_to_source: Vec<usize>,
    selection_range: Option<Range<usize>>,
    marked_range: Option<Range<usize>>,
    cursor_column: Option<usize>,
    cursor_visible: bool,
    font_size: Pixels,
    line_height: Pixels,
    current_line_highlight: bool,
    indent_guides: bool,
    indent_columns: usize,
    editor_theme: ResolvedCodeEditorTheme,
    whitespace: CodeEditorWhitespaceMode,
    editor: Entity<CodeEditor>,
}

struct CodeEditorLinePrepaint {
    shaped: ShapedLine,
    current_line: Option<PaintQuad>,
    selection: Vec<PaintQuad>,
    whitespace: Vec<PaintQuad>,
    indent_guides: Vec<PaintQuad>,
    marked: Option<PaintQuad>,
    caret: Option<PaintQuad>,
}

impl Element for CodeEditorLineElement {
    type RequestLayoutState = ShapedLine;
    type PrepaintState = CodeEditorLinePrepaint;

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
        let shaped =
            window
                .text_system()
                .shape_line(self.text.clone(), self.font_size, &self.runs, None);
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = self.line_height.into();
        (window.request_layout(style, [], cx), shaped)
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        shaped: &mut Self::RequestLayoutState,
        _window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let should_update_layout = self
            .editor
            .read(cx)
            .row_layouts
            .get(self.row)
            .is_none_or(|layout| layout.as_ref().is_none_or(|layout| layout.bounds != bounds));
        if should_update_layout {
            self.editor.update(cx, |editor, _| {
                editor.update_row_layout(
                    self.row,
                    bounds,
                    shaped.clone(),
                    self.source_to_display.clone(),
                    self.display_to_source.clone(),
                );
            });
        }

        let current_line = self.current_line_highlight.then(|| {
            fill(
                Bounds::new(
                    point(bounds.left(), bounds.top()),
                    size(bounds.size.width.max(px(1.0)), self.line_height),
                ),
                self.editor_theme.current_line,
            )
        });

        let mut selection = Vec::new();
        if let Some(range) = self.selection_range.clone() {
            let range = normalize_replace_range(self.text.as_ref(), range);
            if range.start < range.end {
                let x_start = shaped.x_for_index(range.start);
                let x_end = shaped.x_for_index(range.end);
                selection.push(fill(
                    Bounds::new(
                        point(bounds.left() + x_start, bounds.top()),
                        size((x_end - x_start).max(px(1.0)), self.line_height),
                    ),
                    self.editor_theme.selection,
                ));
            }
        }

        let whitespace = whitespace_quads(
            self.text.as_ref(),
            self.whitespace,
            shaped,
            bounds,
            self.line_height,
            self.editor_theme.whitespace,
        );

        let indent_guides = indent_guide_quads(
            self.indent_guides,
            self.indent_columns,
            shaped,
            bounds,
            self.line_height,
            self.editor_theme.ruler.opacity(0.45),
        );

        let marked = self.marked_range.clone().and_then(|range| {
            let range = normalize_replace_range(self.text.as_ref(), range);
            (range.start < range.end).then(|| {
                let x_start = shaped.x_for_index(range.start);
                let x_end = shaped.x_for_index(range.end);
                fill(
                    Bounds::new(
                        point(bounds.left() + x_start, bounds.bottom() - px(2.0)),
                        size((x_end - x_start).max(px(1.0)), px(2.0)),
                    ),
                    self.editor_theme.caret.opacity(0.78),
                )
            })
        });

        let caret = self.cursor_column.map(|column| {
            let column = clamp_to_char_boundary(self.text.as_ref(), column);
            let x = shaped.x_for_index(column);
            fill(
                Bounds::new(
                    point(bounds.left() + x, bounds.top() + px(3.0)),
                    size(px(2.0), (self.line_height - px(6.0)).max(px(1.0))),
                ),
                if self.cursor_visible {
                    self.editor_theme.caret
                } else {
                    self.editor_theme.caret.opacity(0.0)
                },
            )
        });

        CodeEditorLinePrepaint {
            shaped: shaped.clone(),
            current_line,
            selection,
            whitespace,
            indent_guides,
            marked,
            caret,
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(current_line) = prepaint.current_line.take() {
            window.paint_quad(current_line);
        }
        for quad in prepaint.indent_guides.drain(..) {
            window.paint_quad(quad);
        }
        for quad in prepaint.selection.drain(..) {
            window.paint_quad(quad);
        }
        for quad in prepaint.whitespace.drain(..) {
            window.paint_quad(quad);
        }
        let _ = prepaint.shaped.paint(
            point(bounds.left(), bounds.top()),
            self.line_height,
            TextAlign::Left,
            None,
            window,
            cx,
        );
        if let Some(marked) = prepaint.marked.take() {
            window.paint_quad(marked);
        }
        if let Some(caret) = prepaint.caret.take() {
            window.paint_quad(caret);
        }
    }
}

fn whitespace_quads(
    text: &str,
    mode: CodeEditorWhitespaceMode,
    shaped: &ShapedLine,
    bounds: Bounds<Pixels>,
    line_height: Pixels,
    color: Hsla,
) -> Vec<PaintQuad> {
    if matches!(mode, CodeEditorWhitespaceMode::Hidden) || text.is_empty() {
        return Vec::new();
    }

    let mut quads = Vec::new();
    let mut leading = true;
    for (index, ch) in text.char_indices() {
        let is_space = ch == ' ' || ch == '\t';
        if !is_space {
            leading = false;
            continue;
        }
        if matches!(mode, CodeEditorWhitespaceMode::Boundary) && !leading {
            continue;
        }
        let x = shaped.x_for_index(index);
        let y = bounds.top() + line_height * 0.52;
        let width = if ch == '\t' { px(10.0) } else { px(3.0) };
        quads.push(fill(
            Bounds::new(point(bounds.left() + x + px(2.0), y), size(width, px(1.4))),
            color,
        ));
    }
    quads
}

fn line_selection_range(
    buffer: &CodeBuffer,
    selection: &CodeSelection,
    row: usize,
) -> Option<Range<usize>> {
    if selection.is_empty() {
        return None;
    }
    line_offset_range(buffer, &selection.range, row)
}

fn line_offset_range(
    buffer: &CodeBuffer,
    range: &Range<usize>,
    row: usize,
) -> Option<Range<usize>> {
    let row_start = buffer.line_start(row);
    let row_end = buffer.line_end(row);
    let start = range.start.max(row_start);
    let end = range.end.min(row_end);
    if start < end {
        Some(start - row_start..end - row_start)
    } else {
        None
    }
}

fn search_match_count(value: &str, query: &str) -> usize {
    if query.is_empty() {
        return 0;
    }
    value.matches(query).count()
}

fn clamp_to_char_boundary(text: &str, mut offset: usize) -> usize {
    offset = offset.min(text.len());
    while offset > 0 && !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

fn normalize_replace_range(text: &str, range: Range<usize>) -> Range<usize> {
    let start = clamp_to_char_boundary(text, range.start);
    let end = clamp_to_char_boundary(text, range.end);
    start.min(end)..start.max(end)
}

fn code_word_range_at_offset(buffer: &CodeBuffer, offset: usize) -> Range<usize> {
    let text = buffer.as_str();
    if text.is_empty() {
        return 0..0;
    }
    let offset = buffer.clamp_offset(offset);
    let candidate = if offset < text.len() {
        offset
    } else {
        buffer.prev_char(offset)
    };
    let next = buffer.next_char(candidate);
    let Some(ch) = text
        .get(candidate..next)
        .and_then(|value| value.chars().next())
    else {
        return offset..offset;
    };
    if !is_code_word_char(ch) {
        return candidate..next;
    }

    let mut start = candidate;
    while start > 0 {
        let previous = buffer.prev_char(start);
        let ch = text[previous..start].chars().next().unwrap_or(' ');
        if !is_code_word_char(ch) {
            break;
        }
        start = previous;
    }

    let mut end = next;
    while end < text.len() {
        let following = buffer.next_char(end);
        let ch = text[end..following].chars().next().unwrap_or(' ');
        if !is_code_word_char(ch) {
            break;
        }
        end = following;
    }
    start..end
}

fn code_line_content_range_at_offset(buffer: &CodeBuffer, offset: usize) -> Range<usize> {
    let point = buffer.offset_to_point(offset);
    buffer.line_start(point.row)..buffer.line_end(point.row)
}

fn utf16_offset_in_text(text: &str, target: usize) -> usize {
    let mut utf8 = 0;
    let mut utf16 = 0;
    for ch in text.chars() {
        if utf16 >= target {
            break;
        }
        utf16 += ch.len_utf16();
        utf8 += ch.len_utf8();
    }
    clamp_to_char_boundary(text, utf8)
}

fn code_previous_word_boundary(buffer: &CodeBuffer, offset: usize) -> usize {
    let text = buffer.as_str();
    let mut cursor = buffer.clamp_offset(offset);
    while cursor > 0 {
        let previous = buffer.prev_char(cursor);
        let ch = text[previous..cursor].chars().next().unwrap_or(' ');
        if is_code_word_char(ch) {
            break;
        }
        cursor = previous;
    }
    while cursor > 0 {
        let previous = buffer.prev_char(cursor);
        let ch = text[previous..cursor].chars().next().unwrap_or(' ');
        if !is_code_word_char(ch) {
            break;
        }
        cursor = previous;
    }
    cursor
}

fn code_next_word_boundary(buffer: &CodeBuffer, offset: usize) -> usize {
    let text = buffer.as_str();
    let mut cursor = buffer.clamp_offset(offset);
    while cursor < text.len() {
        let next = buffer.next_char(cursor);
        let ch = text[cursor..next].chars().next().unwrap_or(' ');
        if is_code_word_char(ch) {
            break;
        }
        cursor = next;
    }
    while cursor < text.len() {
        let next = buffer.next_char(cursor);
        let ch = text[cursor..next].chars().next().unwrap_or(' ');
        if !is_code_word_char(ch) {
            break;
        }
        cursor = next;
    }
    cursor
}

fn is_code_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

fn removable_indent_len(line: &str, indent: &str) -> Option<usize> {
    if line.starts_with(indent) {
        return Some(indent.len());
    }
    if indent.chars().all(|ch| ch == ' ') {
        let max_spaces = indent.len();
        let spaces = line
            .as_bytes()
            .iter()
            .take_while(|byte| **byte == b' ')
            .take(max_spaces)
            .count();
        if spaces > 0 {
            return Some(spaces);
        }
    }
    if indent == "\t" && line.starts_with('\t') {
        return Some(1);
    }
    None
}

fn apply_signed_delta(value: usize, delta: isize) -> usize {
    if delta.is_negative() {
        value.saturating_sub(delta.unsigned_abs())
    } else {
        value.saturating_add(delta as usize)
    }
}

fn indent_guide_quads(
    enabled: bool,
    indent_columns: usize,
    shaped: &ShapedLine,
    bounds: Bounds<Pixels>,
    line_height: Pixels,
    color: Hsla,
) -> Vec<PaintQuad> {
    if !enabled || indent_columns < 2 {
        return Vec::new();
    }
    (4..=indent_columns)
        .step_by(4)
        .map(|column| {
            let x = shaped.x_for_index(column.min(shaped.len()));
            fill(
                Bounds::new(
                    point(bounds.left() + x, bounds.top() + px(4.0)),
                    size(px(1.0), (line_height - px(8.0)).max(px(1.0))),
                ),
                color,
            )
        })
        .collect()
}

fn render_ruler(
    display_map: CodeDisplayMap,
    column: usize,
    editor_theme: &ResolvedCodeEditorTheme,
) -> gpui::Div {
    div()
        .absolute()
        .top(px(0.0))
        .bottom(px(0.0))
        .left(
            display_map.gutter_width(true)
                + display_map.content_left_padding
                + display_map.average_char_width * column.max(1) as f32,
        )
        .w(px(1.0))
        .bg(editor_theme.ruler)
}

fn render_status_bar(
    cursor: CodePoint,
    line_count: usize,
    byte_count: usize,
    options: CodeEditorOptions,
    _theme: &liora_theme::Theme,
    editor_theme: &ResolvedCodeEditorTheme,
    padding_x: Pixels,
    padding_y: Pixels,
) -> gpui::Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap_3()
        .border_t_1()
        .border_color(editor_theme.border)
        .bg(editor_theme.chrome_surface)
        .px(padding_x)
        .py(padding_y)
        .text_xs()
        .text_color(editor_theme.muted_text)
        .child(format!(
            "Ln {}, Col {} · {} lines · {} bytes",
            cursor.row + 1,
            cursor.column + 1,
            line_count,
            byte_count
        ))
        .child(if options.read_only {
            "read-only"
        } else {
            "editable"
        })
}

fn render_diagnostics(
    diagnostics: &[CodeDiagnostic],
    limit: usize,
    _theme: &liora_theme::Theme,
    editor_theme: &ResolvedCodeEditorTheme,
    padding_x: Pixels,
    padding_y: Pixels,
    gap: Pixels,
) -> gpui::Div {
    let mut panel = div()
        .flex()
        .flex_col()
        .gap(gap)
        .border_t_1()
        .border_color(editor_theme.border)
        .bg(editor_theme.chrome_surface)
        .px(padding_x)
        .py(padding_y);

    for diagnostic in diagnostics.iter().take(limit) {
        let color = editor_theme.diagnostic_color(diagnostic.severity);
        panel = panel.child(
            div()
                .flex()
                .items_start()
                .gap_2()
                .text_sm()
                .child(div().mt(px(7.0)).size(px(6.0)).rounded_full().bg(color))
                .child(
                    div()
                        .flex_1()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(gpui::FontWeight::BOLD)
                                .text_color(color)
                                .child(format!(
                                    "{} at {}:{}",
                                    diagnostic.severity.label(),
                                    diagnostic.line,
                                    diagnostic.column
                                )),
                        )
                        .child(
                            div()
                                .text_color(editor_theme.text)
                                .child(diagnostic.message.clone()),
                        ),
                ),
        );
    }

    panel
}

fn render_completions(
    items: &[CodeCompletionItem],
    limit: usize,
    _theme: &liora_theme::Theme,
    editor_theme: &ResolvedCodeEditorTheme,
    padding_x: Pixels,
    padding_y: Pixels,
    gap: Pixels,
) -> gpui::Div {
    let mut panel = div()
        .flex()
        .flex_col()
        .gap(gap)
        .border_t_1()
        .border_color(editor_theme.border)
        .bg(editor_theme.surface)
        .px(padding_x)
        .py(padding_y)
        .child(
            div()
                .text_xs()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(editor_theme.muted_text)
                .child("Completions"),
        );
    for item in items.iter().take(limit) {
        panel = panel.child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .text_sm()
                .child(
                    div()
                        .text_color(editor_theme.caret)
                        .child(item.label.clone()),
                )
                .when_some(item.kind.clone(), |s, kind| {
                    s.child(
                        div()
                            .text_xs()
                            .text_color(editor_theme.muted_text)
                            .child(kind),
                    )
                })
                .when_some(item.detail.clone(), |s, detail| {
                    s.child(div().text_color(editor_theme.text).child(detail))
                }),
        );
    }
    panel
}

fn render_hover(
    hover: CodeHover,
    _theme: &liora_theme::Theme,
    editor_theme: &ResolvedCodeEditorTheme,
    padding_x: Pixels,
    padding_y: Pixels,
) -> gpui::Div {
    div()
        .border_t_1()
        .border_color(editor_theme.border)
        .bg(editor_theme.chrome_surface)
        .px(padding_x)
        .py(padding_y)
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(editor_theme.info)
                .child(hover.title),
        )
        .child(
            div()
                .text_sm()
                .text_color(editor_theme.text)
                .child(hover.description),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_constructors_clamp_to_one_based_locations() {
        let diagnostic = CodeDiagnostic::warning(0, 0, "missing semicolon");
        assert_eq!(diagnostic.line, 1);
        assert_eq!(diagnostic.column, 1);
        assert_eq!(diagnostic.severity, CodeDiagnosticSeverity::Warning);
    }

    #[test]
    fn code_buffer_maps_offsets_and_points() {
        let buffer = CodeBuffer::new("alpha\nbeta\nγamma");
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.point_to_offset(CodePoint::new(1, 2)), 8);
        assert_eq!(buffer.offset_to_point(8), CodePoint::new(1, 2));
        assert_eq!(buffer.line(2), "γamma");
    }

    #[test]
    fn code_buffer_replaces_ranges_on_char_boundaries() {
        let mut buffer = CodeBuffer::new("aγc");
        let cursor = buffer.replace_range(1..3, "b");
        assert_eq!(buffer.as_str(), "abc");
        assert_eq!(cursor, 2);
    }

    #[test]
    fn code_selection_tracks_directionless_range() {
        let mut selection = CodeSelection::new(5);
        selection.select_to(2);
        assert_eq!(selection.range, 2..5);
        assert!(selection.reversed);
        assert_eq!(selection.cursor(), 2);
    }

    #[test]
    fn code_editor_exposes_v2_foundation_api() {
        let source = include_str!("code_editor.rs");
        assert!(source.contains("struct CodeBuffer"));
        assert!(source.contains("struct CodeSelection"));
        assert!(source.contains("struct CodeViewport"));
        assert!(source.contains("ListState::new"));
        assert!(source.contains("VirtualScrollbar::new"));
        assert!(source.contains("mouse_down_in_editor"));
        assert!(source.contains("mouse_up_in_editor"));
        assert!(source.contains("clear_selection_on_blur"));
        assert!(source.contains("cx.on_blur(&self.focus_handle"));
        assert!(source.contains("point_for_editor_position"));
        assert!(source.contains("render_editor_row"));
        assert!(source.contains("CodeEditorLineElement"));
        assert!(source.contains("zed_tree_sitter_syntax_runs"));
        assert!(source.contains("tree_sitter_highlight_query"));
        assert!(
            !source
                .lines()
                .any(|line| line.trim_start().starts_with("input:"))
        );
    }

    #[test]
    fn code_editor_advanced_models_track_content() {
        let item = CodeCompletionItem::new("println!")
            .kind("macro")
            .detail("debug output");
        assert_eq!(item.label, SharedString::from("println!"));
        assert_eq!(search_match_count("let value = value + 1", "value"), 2);
        assert_eq!(
            CodeHover::new("fn main", "entry point").title,
            SharedString::from("fn main")
        );
    }

    #[test]
    fn selected_line_bounds_cover_complete_lines() {
        let buffer = CodeBuffer::new("one\ntwo\nthree");
        assert_eq!(buffer.selected_line_bounds(5..6), 4..7);
        assert_eq!(buffer.selected_line_bounds(1..6), 0..7);
    }

    #[test]
    fn code_editor_selection_units_cover_words_lines_and_symbols() {
        let buffer = CodeBuffer::new(
            r#"let alpha_beta = 42;
println!("ok");"#,
        );

        assert_eq!(code_word_range_at_offset(&buffer, 5), 4..14);
        assert_eq!(
            &buffer.as_str()[code_word_range_at_offset(&buffer, 5)],
            "alpha_beta"
        );
        assert_eq!(code_word_range_at_offset(&buffer, 15), 15..16);
        assert_eq!(code_line_content_range_at_offset(&buffer, 6), 0..20);
        assert_eq!(
            &buffer.as_str()[code_line_content_range_at_offset(&buffer, 24)],
            r#"println!("ok");"#
        );
    }

    #[test]
    fn code_editor_triple_click_line_selection_keeps_cursor_at_clicked_line_end() {
        let buffer = CodeBuffer::new(
            "alpha
beta
gamma",
        );
        let mut selection = CodeSelection::new(0);
        selection.range = code_line_content_range_at_offset(&buffer, 2);
        selection.reversed = false;

        assert_eq!(selection.range, 0..5);
        assert_eq!(selection.cursor(), 5);
        assert_eq!(
            buffer.offset_to_point(selection.cursor()),
            CodePoint::new(0, 5)
        );
    }

    #[test]
    fn code_editor_word_boundaries_skip_symbols_and_whitespace() {
        let buffer = CodeBuffer::new("let alpha_beta = call_next();");
        let alpha_start = buffer.as_str().find("alpha_beta").unwrap();
        let call_start = buffer.as_str().find("call_next").unwrap();
        let after_alpha = alpha_start + "alpha_beta".len();

        assert_eq!(code_next_word_boundary(&buffer, alpha_start), after_alpha);
        assert_eq!(
            code_next_word_boundary(&buffer, after_alpha),
            call_start + "call_next".len()
        );
        assert_eq!(
            code_previous_word_boundary(&buffer, call_start),
            alpha_start
        );
        assert_eq!(utf16_offset_in_text("a😀b", 2), "a😀".len());
    }

    #[test]
    fn code_line_selection_range_is_column_scoped() {
        let buffer = CodeBuffer::new("alpha\nbeta\ngamma");
        let selection = CodeSelection {
            range: 2..8,
            reversed: false,
            preferred_column: None,
        };

        assert_eq!(line_selection_range(&buffer, &selection, 0), Some(2..5));
        assert_eq!(line_selection_range(&buffer, &selection, 1), Some(0..2));
        assert_eq!(line_selection_range(&buffer, &selection, 2), None);
    }

    #[test]
    fn code_editor_renders_column_level_caret_and_selection() {
        let source = include_str!("code_editor.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("code editor source should have a production section");
        assert!(production_source.contains("cursor_visible: bool"));
        assert!(production_source.contains("blink_task: Option<gpui::Task<()>>"));
        assert!(production_source.contains("fn start_blink"));
        assert!(production_source.contains("fn line_selection_range"));
        assert!(production_source.contains("cursor_column"));
        assert!(production_source.contains("CodeEditorLineElement"));
        assert!(
            production_source.contains("text_system") && production_source.contains("shape_line")
        );
        assert!(production_source.contains("shaped.x_for_index"));
        assert!(production_source.contains("shaped.closest_index_for_x"));
        assert!(production_source.contains("window.paint_quad"));
        assert!(production_source.contains("prepaint.shaped.paint"));
        assert!(production_source.contains("on_mouse_up_out(MouseButton::Left"));
        assert!(production_source.contains("highlight_current_line"));
        assert!(production_source.contains("current_line_highlight: highlight_current_line"));
        assert!(production_source.contains("cursor_active"));
        assert!(production_source.contains("cursor_visible"));
        assert!(!production_source.contains(
            "let selected = selection.range.start < row_end && selection.range.end > row_start;"
        ));
        assert!(!production_source.contains("fn render_line_segments"));
        assert!(!production_source.contains("fn cursor_element"));
        assert!(
            !production_source
                .contains(".when(selected, |s| s.bg(theme.primary.base.opacity(0.22)))")
        );
    }

    #[test]
    fn code_transactions_restore_text_and_selection_snapshots() {
        let before = CodeSelection::new(0);
        let mut after = CodeSelection::new(5);
        after.select_to(9);
        let transaction = CodeTransaction::new("alpha".to_string(), before.clone());
        let transaction = transaction.finish("alpha beta".to_string(), after.clone());

        assert_eq!(transaction.before_text, "alpha");
        assert_eq!(transaction.after_text, "alpha beta");
        assert_eq!(transaction.before_selection, before);
        assert_eq!(transaction.after_selection, after);
    }

    #[test]
    fn code_editor_exposes_undo_redo_actions_and_bindings() {
        let source = include_str!("code_editor.rs");
        assert!(source.contains("CodeEditorUndo"));
        assert!(source.contains("CodeEditorRedo"));
        assert!(source.contains("KeyBinding::new(\"cmd-z\""));
        assert!(source.contains("KeyBinding::new(\"ctrl-z\""));
        assert!(source.contains("undo_stack"));
        assert!(source.contains("redo_stack"));
    }

    #[test]
    fn code_editor_options_default_to_full_editing_chrome() {
        let options = CodeEditorOptions::default();

        assert!(!options.read_only);
        assert!(options.header);
        assert!(options.status_bar);
        assert!(options.line_numbers);
        assert!(options.diagnostics_panel);
        assert!(options.completions_panel);
        assert!(options.hover_panel);
        assert!(!options.current_line_highlight);
        assert!(!options.indent_guides);
        assert!(options.code_folding);
        assert!(!options.rulers);
        assert_eq!(options.ruler_column, 80);
        assert_eq!(options.inline_diagnostics, CodeEditorInlineDiagnostics::All);
        assert_eq!(options.whitespace, CodeEditorWhitespaceMode::Hidden);
        assert!(options.selection);
        assert!(options.copy);
        assert!(options.clipboard_editing);
        assert!(options.cursor_blink);
        assert!(options.drag_selection);
        assert!(options.word_selection);
        assert!(options.line_selection);
        assert!(options.indentation);
        assert!(options.history);
        assert!(options.reveal_cursor);
        assert!(options.scrollbar);
        assert_eq!(options.completion_limit, 6);
        assert_eq!(options.diagnostics_limit, 8);
    }

    #[test]
    fn code_editor_highlight_theme_tracks_editor_specific_overrides() {
        let theme = CodeEditorHighlightTheme::new(CodeTheme::Nord)
            .surface(gpui::rgb(0x0f172a).into())
            .syntax_theme(CodeEditorSyntaxTheme::default().with_style(
                "keyword",
                HighlightStyle {
                    color: Some(gpui::rgb(0xffffff).into()),
                    ..Default::default()
                },
            ));

        assert_eq!(theme.base, CodeTheme::Nord);
        assert!(theme.surface.is_some());
        assert!(theme.syntax.style_for_capture("keyword.control").is_some());
        assert!(theme.chrome_surface.is_none());
    }

    #[test]
    fn code_editor_defaults_to_parent_width() {
        let source = include_str!("code_editor.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("code editor source should have a production section");

        assert!(production_source.contains("width: Option<Pixels>"));
        assert!(production_source.contains("full_width: bool"));
        assert!(production_source.contains("width: None"));
        assert!(production_source.contains("full_width: true"));
        assert!(production_source.contains("pub fn width(mut self"));
        assert!(production_source.contains("pub fn width_units(self"));
        assert!(production_source.contains("pub fn full_width(mut self)"));
        assert!(production_source.contains("pub fn set_full_width"));
        assert!(production_source.contains(".when(self.full_width, |s|"));
        assert!(production_source.contains("s.w_full().min_w(px(0.0)).flex_shrink(1.0)"));
        assert!(production_source.contains(".when_some(self.width, |s, width| s.w(width))"));
    }

    #[test]
    fn code_editor_caches_tree_sitter_runs_between_renders() {
        let source = include_str!("code_editor.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("code editor source should have a production section");

        assert!(production_source.contains("syntax_cache: Option<CodeEditorSyntaxCache>"));
        assert!(production_source.contains("text_revision: u64"));
        assert!(production_source.contains("fn syntax_runs(&mut self)"));
        assert!(production_source.contains("cache.text_revision == self.text_revision"));
        assert!(production_source.contains("self.bump_text_revision()"));
        assert!(!production_source.contains(
            "let syntax_runs = zed_tree_sitter_syntax_runs(self.buffer.as_str(), self.language);"
        ));
    }

    #[test]
    fn code_editor_advanced_layout_options_are_real_render_inputs() {
        let source = include_str!("code_editor.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("code editor source should have a production section");

        assert!(production_source.contains("pub fn line_height_units"));
        assert!(production_source.contains("pub fn font_size_units"));
        assert!(production_source.contains("pub fn content_padding_x_units"));
        assert!(production_source.contains("pub fn row_padding_y_units"));
        assert!(production_source.contains("pub fn indent_guides"));
        assert!(production_source.contains("pub fn fold_range"));
        assert!(production_source.contains("pub fn toggle_fold"));
        assert!(production_source.contains("pub fn set_fold_collapsed"));
        assert!(production_source.contains("fn visible_rows_for"));
        assert!(production_source.contains("fn indent_guide_quads"));
        assert!(production_source.contains("code-editor-fold-toggle-{row}"));
        assert!(production_source.contains("let fold_for_gutter = fold.cloned()"));
        assert!(production_source.contains(".cursor_pointer()"));
        assert!(production_source.contains("editor.toggle_fold(row, cx)"));
        assert!(!production_source.contains("folded lines"));
        assert_eq!(
            visible_rows_for(8, &[CodeFold::new(2, 5, "body")], true),
            vec![0, 1, 2, 6, 7]
        );
        assert_eq!(
            visible_rows_for(8, &[CodeFold::expanded(2, 5, "body")], true),
            vec![0, 1, 2, 3, 4, 5, 6, 7]
        );
        assert_eq!(
            visible_rows_for(8, &[CodeFold::new(2, 5, "body")], false),
            vec![0, 1, 2, 3, 4, 5, 6, 7]
        );
        let detected =
            detect_brace_folds("fn main() {\n    if ready {\n        run();\n    }\n}\n");
        assert_eq!(detected.len(), 2);
        assert!(detected.iter().all(|fold| fold.automatic));
        assert!(detected.iter().all(|fold| !fold.collapsed));
        assert!(
            detected
                .iter()
                .any(|fold| fold.start_line == 0 && fold.end_line == 4)
        );
        assert!(
            detected
                .iter()
                .any(|fold| fold.start_line == 1 && fold.end_line == 3)
        );
        assert_eq!(leading_indent_columns("        let value = 1;", 4), 8);
    }

    #[test]
    fn zed_tree_sitter_highlighter_generates_capture_runs_for_rust() {
        let source = "fn main() { println!(\"hi\"); }";
        let runs = zed_tree_sitter_syntax_runs(source, CodeLanguage::Rust);

        assert!(!runs.is_empty());
        assert!(
            runs.iter().any(
                |run| run.capture.starts_with("keyword") || run.capture.starts_with("function")
            )
        );
    }

    #[test]
    fn zed_tree_sitter_highlighter_supports_common_config_and_script_languages() {
        let samples = [
            (
                CodeLanguage::Sql,
                "select * from users where active = true;",
            ),
            (
                CodeLanguage::Shell,
                "#!/usr/bin/env bash
echo \"ok\"",
            ),
            (
                CodeLanguage::Xml,
                "<window title=\"Liora\"><content /></window>",
            ),
            (
                CodeLanguage::Toml,
                "[package]
name = \"liora\"",
            ),
            (
                CodeLanguage::Yaml,
                "name: liora
features:
  - gpui",
            ),
            (
                CodeLanguage::Conf,
                "[window]
theme=system",
            ),
        ];

        for (language, source) in samples {
            let runs = zed_tree_sitter_syntax_runs(source, language);
            assert!(
                !runs.is_empty(),
                "{} should produce syntax runs",
                language.label()
            );
        }
    }

    #[test]
    fn code_line_display_expands_tabs_by_configured_tab_size() {
        let narrow = CodeLineDisplay::expand_tabs("a	b", 2);
        let wide = CodeLineDisplay::expand_tabs("a	b", 4);

        assert_eq!(narrow.text, "a b");
        assert_eq!(wide.text, "a   b");
        assert_eq!(wide.display_offset_for_source(2), 4);
        assert_eq!(wide.display_to_source[1], 1);
        assert_eq!(wide.display_to_source[3], 2);
        assert_eq!(wide.display_range_for_source(0..2), 0..4);
    }

    #[test]
    fn code_editor_config_loads_font_and_layout_metrics() {
        let config = CodeEditorConfig::load_toml(
            r##"language = "rust"
font_family = "Monospace"
font_size_px = 16
font_weight = 500
line_height_px = 26
height_px = 320
tab_size = 2

[layout]
gutter_width_px = 72
content_padding_x_px = 18
row_padding_y_px = 2
viewport_padding_y_px = 18
header_padding_x_px = 18
header_padding_y_px = 10
header_gap_px = 16
panel_padding_x_px = 18
panel_padding_y_px = 10
panel_gap_px = 6
"##,
        )
        .unwrap();

        assert_eq!(config.font_family.as_deref(), Some("Monospace"));
        assert_eq!(config.font_size_px, Some(16.0));
        assert_eq!(config.font_weight, Some(500.0));
        assert_eq!(config.line_height_px, Some(26.0));
        assert_eq!(config.height_px, Some(320.0));
        assert_eq!(config.tab_size, Some(2));
        let layout = config.layout.expect("layout section should parse");
        assert_eq!(layout.gutter_width_px, Some(72.0));
        assert_eq!(layout.content_padding_x_px, Some(18.0));
        assert_eq!(layout.row_padding_y_px, Some(2.0));
        assert_eq!(layout.panel_gap_px, Some(6.0));
    }

    #[test]
    fn code_editor_runtime_layout_avoids_unconditional_paint_updates() {
        let source = include_str!("code_editor.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("code editor source should have a production section");

        assert!(
            production_source.contains("row_layouts")
                && production_source.contains("get(self.row)")
                && production_source.contains("is_none_or")
        );
        assert!(production_source.contains("retain_visible_row_layouts"));
        assert!(
            production_source.contains("editor_bounds")
                && production_source.contains("!= Some(bounds)")
        );
    }

    #[test]
    fn code_editor_config_loads_zed_syntax_theme_json() {
        let theme_json = r##"{
          "themes": [{
            "name": "Example Dark",
            "appearance": "dark",
            "style": {
              "editor.background": "#101010ff",
              "syntax": {
                "keyword": { "color": "#ff0000ff", "font_style": "italic" }
              }
            }
          }]
        }"##;
        let zed_theme = CodeEditorZedTheme::load(theme_json, Some("Example Dark")).unwrap();
        let theme = zed_theme.appearance.to_theme(CodeTheme::Auto);

        assert!(theme.surface.is_some());
        assert!(theme.syntax.style_for_capture("keyword.control").is_some());
    }

    #[test]
    fn code_editor_exposes_navigation_and_ime_foundation() {
        let source = include_str!("code_editor.rs");
        assert!(source.contains("CodeEditorWordLeft"));
        assert!(source.contains("CodeEditorDeleteWordLeft"));
        assert!(source.contains("CodeEditorPageDown"));
        assert!(source.contains("CodeEditorDocumentStart"));
        assert!(source.contains(r#"KeyBinding::new("ctrl-left""#));
        assert!(source.contains(r#"KeyBinding::new("alt-left""#));
        assert!(source.contains(r#"KeyBinding::new("ctrl-backspace""#));
        assert!(source.contains(r#"KeyBinding::new("pagedown""#));
        assert!(source.contains(r#"KeyBinding::new("cmd-down""#));
        assert!(source.contains("marked_range: Option<Range<usize>>"));
        assert!(source.contains("fn marked_text_range"));
        assert!(source.contains("replace_and_mark_text_in_range"));
        assert!(source.contains("utf16_offset_in_text"));
        assert!(source.contains("CodeEditorOptions"));
        assert!(source.contains("fn read_only"));
        assert!(source.contains("fn completion_limit"));
        assert!(source.contains("fn render_status_bar"));
    }

    #[test]
    fn code_editor_input_layer_leaves_virtual_scrollbar_hit_area_interactive() {
        let bounds = Bounds::new(point(px(0.0), px(0.0)), size(px(200.0), px(100.0)));
        let input_bounds = editor_text_input_bounds(bounds);

        assert!(input_bounds.right() < bounds.right());
        assert_eq!(
            bounds.right() - input_bounds.right(),
            crate::virtual_scrollbar_hit_width()
        );
    }

    #[test]
    fn code_editor_exposes_explicit_config_file_api_without_background_watcher() {
        let source = include_str!("code_editor.rs");
        let production_source = source
            .split("#[cfg(test)]")
            .next()
            .expect("code editor source should have a production section");

        assert!(production_source.contains("pub fn config_file(mut self"));
        assert!(production_source.contains("pub fn set_config"));
        assert!(!production_source.contains("pub rows: Option<usize>"));
        assert!(!production_source.contains("watch_config_file"));
        assert!(!production_source.contains("CodeEditorConfigFileWatcher"));
        assert!(!production_source.contains("config_watch_task"));
    }
}
