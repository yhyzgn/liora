//! Canonical Markdown transactions for the retained native editor.
use super::model::{Block, Kind, MarkdownDocument, Span};
use super::{MarkdownTableEdit, MarkdownTableEditError};
use pulldown_cmark::{Event, Options, Parser, Tag};
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Selection {
    pub anchor: usize,
    pub head: usize,
}

impl Selection {
    pub fn range(self) -> Range<usize> {
        self.anchor.min(self.head)..self.anchor.max(self.head)
    }
    pub fn is_empty(self) -> bool {
        self.anchor == self.head
    }
}

pub(super) enum Command {
    Backspace,
    Delete,
    Enter,
    Indent,
    Outdent,
    Inline(&'static str),
    Block(&'static str),
    ToggleTask(u64),
    MoveBlock { id: u64, direction: isize },
    Undo,
    Redo,
}

struct Snapshot {
    source: String,
    selection: Selection,
    identities: Vec<Identity>,
}

pub(super) struct Engine {
    document: MarkdownDocument,
    selection: Selection,
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
    group_depth: usize,
    group_recorded: bool,
}

impl Engine {
    pub fn new(source: impl Into<String>) -> Self {
        let mut document = MarkdownDocument::new(source);
        editable_whitespace(&mut document);
        Self {
            document,
            selection: Selection { anchor: 0, head: 0 },
            undo: Vec::new(),
            redo: Vec::new(),
            group_depth: 0,
            group_recorded: false,
        }
    }
    pub fn source(&self) -> &str {
        self.document.source()
    }
    pub fn document(&self) -> &MarkdownDocument {
        &self.document
    }
    pub fn selection(&self) -> Selection {
        self.selection
    }
    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = Selection {
            anchor: boundary(self.source(), selection.anchor),
            head: boundary(self.source(), selection.head),
        };
    }
    pub fn begin_undo_group(&mut self) {
        if self.group_depth == 0 {
            self.group_recorded = false;
        }
        self.group_depth += 1;
    }
    pub fn end_undo_group(&mut self) {
        self.group_depth = self.group_depth.saturating_sub(1);
        if self.group_depth == 0 {
            self.group_recorded = false;
        }
    }
    fn snapshot(&self) -> Snapshot {
        Snapshot {
            source: self.source().to_owned(),
            selection: self.selection,
            identities: identities(
                &self.document.blocks,
                &(self.source().len()..self.source().len()),
                0,
            ),
        }
    }
    fn record(&mut self) {
        if self.group_depth == 0 || !self.group_recorded {
            self.undo.push(self.snapshot());
            if self.undo.len() > 128 {
                self.undo.remove(0);
            }
            self.group_recorded = self.group_depth > 0;
        }
        self.redo.clear();
    }
    pub fn set_source(&mut self, source: &str) -> bool {
        if source == self.source() {
            return false;
        }
        self.apply(0..self.source().len(), source, self.selection, false)
    }
    fn apply(
        &mut self,
        range: Range<usize>,
        inserted: &str,
        selection: Selection,
        plain: bool,
    ) -> bool {
        if self.source().get(range.clone()) == Some(inserted) {
            self.set_selection(selection);
            return false;
        }
        // Whole-source callers still describe a local transaction when their unchanged
        // prefix/suffix agree; preserve those blocks and container identities.
        let old = &self.source()[range.clone()];
        let common = old
            .bytes()
            .zip(inserted.bytes())
            .take_while(|(a, b)| a == b)
            .count();
        let common = boundary(old, common);
        let suffix = old[common..]
            .bytes()
            .rev()
            .zip(inserted[common..].bytes().rev())
            .take_while(|(a, b)| a == b)
            .count();
        let mut suffix = suffix;
        while !old.is_char_boundary(old.len() - suffix)
            || !inserted.is_char_boundary(inserted.len() - suffix)
        {
            suffix -= 1;
        }
        let range = range.start + common..range.end - suffix;
        let inserted = &inserted[common..inserted.len() - suffix];
        self.record();
        let delta = inserted.len() as isize - range.len() as isize;
        let identities = if plain {
            Vec::new()
        } else {
            identities(&self.document.blocks, &range, delta)
        };
        let mut source = self.source().to_owned();
        source.replace_range(range.clone(), inserted);
        if plain && patch_plain(&mut self.document.blocks, &range, inserted, delta) {
            self.document.commit_plain_source(source);
        } else {
            self.document.set_source(source);
            self.document.flush();
            editable_whitespace(&mut self.document);
            restore_ids(&mut self.document.blocks, &identities);
        }
        self.set_selection(selection);
        true
    }
    pub fn source_position(&self, block_id: u64, display_offset: usize) -> Option<usize> {
        let block = block_by_id(&self.document.blocks, block_id)?;
        let offset = boundary(&block.text, display_offset);
        let preferred = block.spans.iter().find(|span| span.range.start == offset);
        for span in preferred.into_iter().chain(block.spans.iter()) {
            if span.range.start <= offset && offset <= span.range.end {
                return Some(map_span(self.source(), block, span, offset));
            }
        }
        if matches!(block.kind, Kind::Code(_)) {
            let source = &self.source()[block.source_range.clone()];
            return Some(
                block.source_range.start + source.find('\n').map_or(source.len(), |i| i + 1),
            );
        }
        Some(block.source_range.start)
    }
    pub fn display_position(&self, source_offset: usize) -> Option<(u64, usize)> {
        let offset = boundary(self.source(), source_offset);
        let mut nearest = None;
        let block = nearest_text_block(&self.document.blocks, offset)?;
        for span in &block.spans {
            let original = &self.source()[span.source_range.clone()];
            let visible = &block.text[span.range.clone()];
            if original == visible
                && span.source_range.start <= offset
                && offset <= span.source_range.end
            {
                return Some((
                    block.id,
                    span.range.start + boundary(visible, offset - span.source_range.start),
                ));
            }
            if let Some(start) = original.find(visible).filter(|_| !visible.is_empty()) {
                let content_start = span.source_range.start + start;
                if content_start <= offset && offset <= content_start + visible.len() {
                    return Some((
                        block.id,
                        span.range.start + boundary(visible, offset - content_start),
                    ));
                }
            }
            if original != visible
                && !original.contains(visible)
                && span.source_range.start <= offset
                && offset <= span.source_range.end
            {
                let candidate = visible
                    .char_indices()
                    .map(|(i, _)| span.range.start + i)
                    .chain(std::iter::once(span.range.end))
                    .min_by_key(|display| {
                        map_span(self.source(), block, span, *display).abs_diff(offset)
                    })
                    .unwrap_or(span.range.start);
                return Some((block.id, candidate));
            }
            let candidate = if offset.abs_diff(span.source_range.start)
                <= offset.abs_diff(span.source_range.end)
            {
                span.range.start
            } else {
                span.range.end
            };
            let position = map_span(self.source(), block, span, candidate);
            let distance = position.abs_diff(offset);
            if nearest.as_ref().is_none_or(|(best, _, _)| distance < *best) {
                nearest = Some((distance, block.id, candidate));
            }
        }
        if block.spans.is_empty() {
            return Some((block.id, 0));
        }
        nearest.map(|(_, id, display)| (id, display))
    }
    pub fn selected_text(&self) -> String {
        let range = self.selection.range();
        let mut output = String::new();
        let mut previous = None;
        for block in text_blocks(&self.document.blocks) {
            let mut part = String::new();
            for (start, character) in block.text.char_indices() {
                let token = character_source(self.source(), block, start, character.len_utf8());
                let (a, b) = (token.start, token.end);
                if a < range.end && b > range.start {
                    part.push(character);
                }
            }
            if !part.is_empty() {
                if previous.is_some_and(|id| id != block.id) && !output.ends_with('\n') {
                    output.push('\n');
                }
                output.push_str(&part);
                previous = Some(block.id);
            }
        }
        output
    }
    pub fn type_markdown(&mut self, text: &str) -> bool {
        let range = self.selection.range();
        let caret = range.start + text.len();
        let plain = self.provably_plain(&range, text);
        self.apply(
            range,
            text,
            Selection {
                anchor: caret,
                head: caret,
            },
            plain,
        )
    }

    pub fn replace(&mut self, text: &str) -> Result<bool, String> {
        let range = self.selection.range();
        let raw = self.raw_context(range.start);
        if !range.is_empty()
            && text_blocks(&self.document.blocks).iter().any(|block| {
                matches!(
                    block.kind,
                    Kind::Code(_) | Kind::Math(_) | Kind::Metadata(_)
                ) && block.source_range.start < range.end
                    && range.start < block.source_range.end
                    && !(block.source_range.start <= range.start
                        && range.end <= block.source_range.end)
            })
        {
            return Err("A selection crossing a code, math, or metadata boundary must be edited in source mode".into());
        }
        let inline_raw = text_blocks(&self.document.blocks)
            .iter()
            .flat_map(|b| &b.spans)
            .any(|span| {
                (span.style.code || span.style.math.is_some())
                    && span.source_range.start < range.start
                    && range.end < span.source_range.end
            });
        if inline_raw && text.contains(['`', '$', '\n', '\r']) {
            return Err("Editing a code or math delimiter requires source mode".into());
        }
        let escaped = if raw || inline_raw {
            text.to_owned()
        } else {
            escape(text)
        };
        let literal_length = escaped.len();
        let mut inserted = escaped;
        let same_span = text_blocks(&self.document.blocks).iter().any(|block| {
            block.spans.iter().any(|span| {
                span.source_range.start <= range.start && range.end <= span.source_range.end
            })
        });
        if !range.is_empty() && !raw && !same_span {
            let wrappers = inline_wrappers(self.source());
            // Keep the untouched portions of both endpoint styles syntactically balanced.
            for wrapper in wrappers.iter().rev() {
                if wrapper.content.start <= range.start
                    && range.start < wrapper.content.end
                    && range.end >= wrapper.full.end
                {
                    inserted.push_str(&self.source()[wrapper.content.end..wrapper.full.end]);
                }
            }
            for wrapper in &wrappers {
                if range.start <= wrapper.full.start
                    && wrapper.content.start <= range.end
                    && range.end <= wrapper.content.end
                {
                    inserted.push_str(&self.source()[wrapper.full.start..wrapper.content.start]);
                }
            }
        }
        let caret = range.start + literal_length;
        let plain = self.provably_plain(&range, &inserted);
        Ok(self.apply(
            range,
            &inserted,
            Selection {
                anchor: caret,
                head: caret,
            },
            plain,
        ))
    }
    fn raw_context(&self, position: usize) -> bool {
        text_blocks(&self.document.blocks).into_iter().any(|block| {
            matches!(
                block.kind,
                Kind::Code(_) | Kind::Math(_) | Kind::Metadata(_)
            ) && block.source_range.start <= position
                && position <= block.source_range.end
        })
    }
    fn provably_plain(&self, range: &Range<usize>, inserted: &str) -> bool {
        let safe = |s: &str| s.chars().all(|c| c.is_alphanumeric() || c == ' ');
        if !safe(inserted) || !safe(&self.source()[range.clone()]) {
            return false;
        }
        if !self.source()[..range.start]
            .chars()
            .next_back()
            .is_some_and(char::is_alphanumeric)
            || !self.source()[range.end..]
                .chars()
                .next()
                .is_some_and(char::is_alphanumeric)
        {
            return false;
        }
        self.document.blocks.iter().any(|block| {
            matches!(block.kind, Kind::Paragraph)
                && block.children.is_empty()
                && block.spans.len() == 1
                && block.spans[0].source_range.start < range.start
                && range.end < block.spans[0].source_range.end
                && block
                    .text
                    .chars()
                    .all(|c| c.is_alphanumeric() || c.is_whitespace())
                && self.source().get(block.spans[0].source_range.clone())
                    == Some(block.text.as_ref())
        })
    }
    pub fn command(&mut self, command: Command) -> Result<bool, String> {
        match command {
            Command::Undo | Command::Redo => {
                if self.group_depth > 0 {
                    return Err("Finish the active undo group before undo or redo".into());
                }
                let undo = matches!(command, Command::Undo);
                let snapshot = if undo {
                    self.undo.pop()
                } else {
                    self.redo.pop()
                };
                let Some(snapshot) = snapshot else {
                    return Ok(false);
                };
                let current = self.snapshot();
                if undo {
                    self.redo.push(current);
                } else {
                    self.undo.push(current);
                }
                self.document.set_source(snapshot.source);
                self.document.flush();
                editable_whitespace(&mut self.document);
                restore_ids(&mut self.document.blocks, &snapshot.identities);
                self.set_selection(snapshot.selection);
                Ok(true)
            }
            Command::Backspace => self.delete(true),
            Command::Delete => self.delete(false),
            Command::Enter => self.enter(),
            Command::Indent => self.indent(false),
            Command::Outdent => self.indent(true),
            Command::Inline(marker) => self.inline(marker),
            Command::Block(prefix) => self.block(prefix),
            Command::ToggleTask(id) => {
                let block =
                    block_by_id(&self.document.blocks, id).ok_or("Task block no longer exists")?;
                let range = block
                    .task_marker_range
                    .clone()
                    .ok_or("Block has no task marker")?;
                let marker = if block.task_checked == Some(true) {
                    "[ ]"
                } else {
                    "[x]"
                };
                Ok(self.apply(range, marker, self.selection, false))
            }
            Command::MoveBlock { id, direction } => self.move_block(id, direction),
        }
    }
    fn delete(&mut self, backwards: bool) -> Result<bool, String> {
        if !self.selection.is_empty() {
            return self.replace("");
        }
        let caret = self.selection.head;
        if backwards && caret > 0 && self.is_block_boundary(caret) {
            return self.backspace_boundary(caret);
        }
        if let Some((id, offset)) = self.display_position(caret) {
            let block =
                block_by_id(&self.document.blocks, id).ok_or("Text block no longer exists")?;
            let mapped = self.source_position(id, offset).unwrap_or(caret);
            let opening = backwards
                && (offset > 0 || self.source()[..block.source_range.start].trim().is_empty())
                && block.text[..offset]
                    .char_indices()
                    .next_back()
                    .is_none_or(|(index, ch)| {
                        character_source(self.source(), block, index, ch.len_utf8()).end < caret
                    });
            if mapped != caret || opening {
                let edge = if backwards {
                    self.source()[..caret]
                        .grapheme_indices(true)
                        .next_back()
                        .map(|(index, _)| index)
                } else {
                    self.source()[caret..]
                        .graphemes(true)
                        .next()
                        .map(|text| caret + text.len())
                };
                if let Some(edge) = edge {
                    let range = caret.min(edge)..caret.max(edge);
                    if self.source()[range.clone()]
                        .chars()
                        .all(|ch| "*_~`$[]()!#>=".contains(ch))
                    {
                        return Ok(self.apply(
                            range.clone(),
                            "",
                            Selection {
                                anchor: range.start,
                                head: range.start,
                            },
                            false,
                        ));
                    }
                }
            }
            if backwards && offset == 0 && caret <= mapped && self.is_block_boundary(caret) {
                return self.backspace_boundary(caret);
            }
            let target = if backwards {
                block
                    .text
                    .grapheme_indices(true)
                    .map(|(i, _)| i)
                    .take_while(|i| *i < offset)
                    .last()
            } else {
                block
                    .text
                    .grapheme_indices(true)
                    .map(|(i, _)| i)
                    .find(|i| *i > offset)
                    .or_else(|| (offset < block.text.len()).then_some(block.text.len()))
            };
            if let Some(target) = target {
                let token_start = offset.min(target);
                let token_end = offset.max(target);
                let mut tokens = block.text[token_start..token_end]
                    .char_indices()
                    .map(|(i, c)| {
                        character_source(self.source(), block, token_start + i, c.len_utf8())
                    });
                let Some(first) = tokens.next() else {
                    return Ok(false);
                };
                let source_end = tokens.last().map_or(first.end, |range| range.end);
                let before = self.selection;
                self.set_selection(Selection {
                    anchor: first.start,
                    head: source_end,
                });
                let result = self.replace("");
                if result.is_err() {
                    self.selection = before;
                } else if self.undo.last().is_some_and(|snapshot| {
                    snapshot.selection
                        == Selection {
                            anchor: first.start,
                            head: source_end,
                        }
                }) {
                    if let Some(snapshot) = self.undo.last_mut() {
                        snapshot.selection = before;
                    }
                }
                return result;
            }
            if !backwards {
                return self.join_next(id);
            }
        }
        if backwards {
            self.backspace_boundary(caret)
        } else {
            Ok(false)
        }
    }
    fn is_block_boundary(&self, caret: usize) -> bool {
        let line = line_range(self.source(), caret);
        nearest_text_block(&self.document.blocks, line.end).is_some_and(|block| {
            let block_line = line_range(self.source(), block.source_range.start);
            block_line.start == line.start
                && caret <= block.source_range.start
                && self.source()[caret..block.source_range.start]
                    .chars()
                    .all(char::is_whitespace)
        })
    }

    fn backspace_boundary(&mut self, caret: usize) -> Result<bool, String> {
        let line = line_range(self.source(), caret);
        let prefix = prefix(&self.source()[line.clone()]);
        if caret == line.start + prefix.content && prefix.content > 0 && !prefix.heading {
            let remove = prefix.last..prefix.content;
            let range = line.start + remove.start..line.start + remove.end;
            return Ok(self.apply(
                range.clone(),
                "",
                Selection {
                    anchor: range.start,
                    head: range.start,
                },
                false,
            ));
        }
        let blocks = text_blocks(&self.document.blocks);
        let current = blocks.iter().position(|b| {
            (b.source_range.start <= caret && caret <= b.source_range.end)
                || (line_range(self.source(), b.source_range.start).start == line.start
                    && caret <= b.source_range.start
                    && self.source()[caret..b.source_range.start]
                        .chars()
                        .all(char::is_whitespace))
        });
        if let Some(index) = current.filter(|i| *i > 0) {
            let previous = blocks[index - 1];
            let next = blocks[index];
            return self.join(previous.id, next.id);
        }
        Ok(false)
    }
    fn join_next(&mut self, id: u64) -> Result<bool, String> {
        let blocks = text_blocks(&self.document.blocks);
        let Some(index) = blocks.iter().position(|block| block.id == id) else {
            return Ok(false);
        };
        let Some(next) = blocks.get(index + 1) else {
            return Ok(false);
        };
        self.join(id, next.id)
    }
    fn join(&mut self, previous: u64, next: u64) -> Result<bool, String> {
        let a = block_by_id(&self.document.blocks, previous).ok_or("Previous block missing")?;
        let b = block_by_id(&self.document.blocks, next).ok_or("Next block missing")?;
        if !matches!(a.kind, Kind::Paragraph | Kind::Heading(_))
            || !matches!(b.kind, Kind::Paragraph | Kind::Heading(_))
        {
            return Err("Joining tables, code, math, or metadata requires source mode".into());
        }
        let start = a.source_range.end
            - self.source()[a.source_range.clone()]
                .chars()
                .rev()
                .take_while(|c| matches!(c, '\n' | '\r'))
                .map(char::len_utf8)
                .sum::<usize>();
        let next_line = line_range(self.source(), b.source_range.start);
        let next_prefix = prefix(&self.source()[next_line.clone()]);
        let end = if matches!(b.kind, Kind::Heading(_)) {
            next_line.start + next_prefix.content
        } else {
            let physical = &self.source()[next_line.clone()];
            if physical[..b.source_range.start - next_line.start]
                .chars()
                .all(|c| matches!(c, ' ' | '\t'))
            {
                next_line.start
            } else {
                b.source_range.start
            }
        };
        if start > end {
            return Err("Blocks do not have a contiguous join boundary".into());
        }
        let separator = &self.source()[start..end];
        if separator.chars().any(|character| {
            !character.is_whitespace()
                && !matches!(
                    character,
                    '>' | '-' | '+' | '*' | '#' | '.' | ')' | '[' | ']' | 'x' | 'X'
                )
                && !character.is_ascii_digit()
        }) {
            return Err("Joining across reference definitions or unsupported structural source requires source mode".into());
        }
        // Delimiters on either side remain intact; only structural separators disappear.
        Ok(self.apply(
            start..end,
            "",
            Selection {
                anchor: start,
                head: start,
            },
            false,
        ))
    }
    fn enter(&mut self) -> Result<bool, String> {
        let range = self.selection.range();
        if !range.is_empty() {
            // One transaction: derive the separator at the start, then use balanced replacement.
            let separator = self.enter_separator(range.start)?;
            return self.replace_structural(&separator);
        }
        let line = line_range(self.source(), range.start);
        let text = &self.source()[line.clone()];
        let parsed = prefix(text);
        if !self.raw_context(range.start)
            && parsed.content > 0
            && text[parsed.content..].trim().is_empty()
        {
            let remove = line.start + parsed.last..line.start + parsed.content;
            return Ok(self.apply(
                remove.clone(),
                "",
                Selection {
                    anchor: remove.start,
                    head: remove.start,
                },
                false,
            ));
        }
        let separator = self.enter_separator(range.start)?;
        let wrappers = inline_wrappers(self.source())
            .into_iter()
            .filter(|wrapper| {
                wrapper.content.start < range.start && range.start < wrapper.content.end
            })
            .collect::<Vec<_>>();
        let mut inserted = String::new();
        for wrapper in wrappers.iter().rev() {
            inserted.push_str(&self.source()[wrapper.content.end..wrapper.full.end]);
        }
        inserted.push_str(&separator);
        for wrapper in &wrappers {
            inserted.push_str(&self.source()[wrapper.full.start..wrapper.content.start]);
        }
        let caret = range.start + inserted.len();
        Ok(self.apply(
            range,
            &inserted,
            Selection {
                anchor: caret,
                head: caret,
            },
            false,
        ))
    }
    fn enter_separator(&self, position: usize) -> Result<String, String> {
        let newline = if self.source().contains("\r\n") {
            "\r\n"
        } else {
            "\n"
        };
        if self.raw_context(position) {
            let line = line_range(self.source(), position);
            let text = &self.source()[line];
            let quoted = quote_indent(text);
            let indentation = text[quoted..]
                .bytes()
                .take_while(|c| matches!(c, b' ' | b'\t'))
                .count();
            return Ok(format!("{newline}{}", &text[..quoted + indentation]));
        }
        if self
            .display_position(position)
            .and_then(|(id, _)| block_by_id(&self.document.blocks, id))
            .is_some_and(|b| matches!(b.kind, Kind::Cell))
        {
            return Err(
                "Use table row insertion to add a row; a newline would corrupt the table".into(),
            );
        }
        let line = line_range(self.source(), position);
        let text = &self.source()[line];
        let parsed = prefix(text);
        let mut continuation = text[..parsed.content].to_owned();
        if let Some(task) = continuation
            .find("[x]")
            .or_else(|| continuation.find("[X]"))
        {
            continuation.replace_range(task..task + 3, "[ ]");
        }
        if let Some((range, number)) = parsed.number {
            continuation.replace_range(range, &number.saturating_add(1).to_string());
        }
        if parsed.heading {
            continuation.clear();
        }
        if continuation.is_empty() {
            Ok(format!("{newline}{newline}"))
        } else {
            Ok(format!("{newline}{continuation}"))
        }
    }
    fn replace_structural(&mut self, text: &str) -> Result<bool, String> {
        let before = self.snapshot();
        self.begin_undo_group();
        let result = self.replace("");
        if let Err(error) = result {
            self.end_undo_group();
            return Err(error);
        }
        let range = self.selection.range();
        let caret = range.start + text.len();
        let changed = self.apply(
            range,
            text,
            Selection {
                anchor: caret,
                head: caret,
            },
            false,
        );
        self.end_undo_group();
        Ok(changed || before.source != self.source())
    }
    fn indent(&mut self, outdent: bool) -> Result<bool, String> {
        let selection = self.selection;
        let range = selection.range();
        let start = line_range(self.source(), range.start).start;
        let end = line_range(
            self.source(),
            range.end.saturating_sub(usize::from(!range.is_empty())),
        )
        .end;
        if text_blocks(&self.document.blocks).iter().any(|b| {
            matches!(b.kind, Kind::Cell) && b.source_range.start < end && b.source_range.end > start
        }) {
            return Err("Table indentation must be changed on its containing block".into());
        }
        let mut next = String::new();
        let mut edits = Vec::new();
        let mut offset = start;
        for line in self.source()[start..end].split_inclusive('\n') {
            let quoted = quote_indent(line);
            let count = if outdent {
                if line[quoted..].starts_with('\t') {
                    1
                } else {
                    line[quoted..]
                        .bytes()
                        .take_while(|c| *c == b' ')
                        .count()
                        .min(4)
                }
            } else {
                0
            };
            next.push_str(&line[..quoted]);
            if outdent {
                next.push_str(&line[quoted + count..]);
            } else {
                next.push_str("    ");
                next.push_str(&line[quoted..]);
            }
            edits.push((offset + quoted, if outdent { -(count as isize) } else { 4 }));
            offset += line.len();
        }
        let map = |position: usize| {
            edits
                .iter()
                .fold(position as isize, |value, (at, delta)| {
                    if position >= *at {
                        value + *delta
                    } else {
                        value
                    }
                })
                .max(start as isize) as usize
        };
        Ok(self.apply(
            start..end,
            &next,
            Selection {
                anchor: map(selection.anchor),
                head: map(selection.head),
            },
            false,
        ))
    }
    fn inline(&mut self, marker: &'static str) -> Result<bool, String> {
        if !matches!(marker, "**" | "*" | "~~" | "`" | "==") {
            return Err("Unsupported inline Markdown marker".into());
        }
        let range = self.selection.range();
        if self.raw_context(range.start) {
            return Err("Inline formatting is unavailable inside code, math, or metadata".into());
        }
        if self.source()[range.clone()].contains(['\n', '\r']) {
            return Err(
                "Inline formatting cannot span physical lines; format each paragraph separately"
                    .into(),
            );
        }
        if range.start >= marker.len()
            && self.source().get(range.start - marker.len()..range.start) == Some(marker)
            && self.source().get(range.end..range.end + marker.len()) == Some(marker)
        {
            let selected = self.source()[range.clone()].to_owned();
            return Ok(self.apply(
                range.start - marker.len()..range.end + marker.len(),
                &selected,
                Selection {
                    anchor: range.start - marker.len(),
                    head: range.end - marker.len(),
                },
                false,
            ));
        }
        let selected = &self.source()[range.clone()];
        if marker == "`" && selected.contains('`') {
            return Err(
                "Inline code containing backticks requires a source-mode delimiter run".into(),
            );
        }
        let inserted = format!("{marker}{selected}{marker}");
        Ok(self.apply(
            range.clone(),
            &inserted,
            Selection {
                anchor: range.start + marker.len(),
                head: range.end + marker.len(),
            },
            false,
        ))
    }
    fn block(&mut self, requested: &'static str) -> Result<bool, String> {
        let prefix_text = match requested {
            "paragraph" => "",
            "heading1" => "# ",
            "heading2" => "## ",
            "heading3" => "### ",
            "quote" => "> ",
            "bullet" => "- ",
            "ordered" => "1. ",
            "task" => "- [ ] ",
            "" | "# " | "## " | "### " | "#### " | "##### " | "###### " | "> " | "- " | "1. "
            | "- [ ] " => requested,
            "code" | "```" => "```",
            "rule" | "---" => "---",
            _ => return Err("Unsupported Markdown block conversion".into()),
        };
        let range = self.selection.range();
        if self.raw_context(range.start) {
            return Err("Convert code, math, or metadata delimiters in source mode".into());
        }
        if self
            .display_position(range.start)
            .and_then(|(id, _)| block_by_id(&self.document.blocks, id))
            .is_some_and(|block| matches!(block.kind, Kind::Cell))
        {
            return Err("Convert tables through table operations or source mode".into());
        }
        let line = line_range(self.source(), range.start);
        let parsed = prefix(&self.source()[line.clone()]);
        if prefix_text == "```" || prefix_text == "---" {
            let newline = if self.source().contains("\r\n") {
                "\r\n"
            } else {
                "\n"
            };
            let physical = &self.source()[line.clone()];
            let content = &physical[parsed.content..];
            let opening = &physical[..parsed.content];
            let quoted = quote_indent(physical);
            let mut continuation = physical[..quoted].to_owned();
            continuation.extend(
                physical[quoted..parsed.content]
                    .chars()
                    .map(|c| if c == '\t' { '\t' } else { ' ' }),
            );
            let inserted = if prefix_text == "```" {
                format!("{opening}```{newline}{continuation}{content}{newline}{continuation}```")
            } else {
                format!("{opening}---{newline}{continuation}{newline}{continuation}{content}")
            };
            let caret =
                line.start + opening.len() + prefix_text.len() + newline.len() + continuation.len();
            return Ok(self.apply(
                line,
                &inserted,
                Selection {
                    anchor: caret,
                    head: caret,
                },
                false,
            ));
        }
        let start = line.start + quote_indent(&self.source()[line.clone()]);
        let end = line.start + parsed.content;
        let caret = start + prefix_text.len();
        Ok(self.apply(
            start..end,
            prefix_text,
            Selection {
                anchor: caret,
                head: caret,
            },
            false,
        ))
    }
    fn move_block(&mut self, id: u64, direction: isize) -> Result<bool, String> {
        fn siblings(blocks: &[Block], id: u64) -> Option<(&[Block], usize)> {
            for (index, block) in blocks.iter().enumerate() {
                if block.id == id {
                    return Some((blocks, index));
                }
                if let Some(found) = siblings(&block.children, id) {
                    return Some(found);
                }
            }
            None
        }
        let (blocks, index) =
            siblings(&self.document.blocks, id).ok_or("Block no longer exists")?;
        if direction == 0 {
            return Ok(false);
        }
        let Some(target) = index
            .checked_add_signed(direction.signum())
            .filter(|i| *i < blocks.len())
        else {
            return Ok(false);
        };
        let a = &blocks[index.min(target)];
        let b = &blocks[index.max(target)];
        if matches!(a.kind, Kind::Cell | Kind::Row(_))
            || matches!(b.kind, Kind::Cell | Kind::Row(_))
        {
            return Err("Move table cells with table operations".into());
        }
        let start = line_range(self.source(), a.source_range.start).start;
        let middle = line_range(self.source(), b.source_range.start).start;
        let end = b.source_range.end;
        if middle <= start || end <= middle {
            return Err("Overlapping block ranges cannot be moved safely".into());
        }
        let first = &self.source()[start..middle];
        let second = &self.source()[middle..end];
        let newline = if self.source().contains("\r\n") {
            "\r\n"
        } else {
            "\n"
        };
        let mut inserted = second.to_owned();
        if !inserted.ends_with('\n') {
            inserted.push_str(newline);
        }
        if !matches!(a.kind, Kind::Item) && !inserted.ends_with(&format!("{newline}{newline}")) {
            inserted.push_str(newline);
        }
        let shifted = start + inserted.len();
        inserted.push_str(first);
        let moved_start = if index < target { shifted } else { start };
        let offset = self
            .selection
            .head
            .saturating_sub(blocks[index].source_range.start);
        let caret = moved_start + offset.min(blocks[index].source_range.len());
        Ok(self.apply(
            start..end,
            &inserted,
            Selection {
                anchor: caret,
                head: caret,
            },
            false,
        ))
    }
    fn searchable(&self) -> (String, Vec<Range<usize>>) {
        let mut text = String::new();
        let mut positions = Vec::new();
        for block in text_blocks(&self.document.blocks) {
            if !text.is_empty() {
                text.push('\n');
                positions.push(block.source_range.start..block.source_range.start);
            }
            for (offset, character) in block.text.char_indices() {
                let token = character_source(self.source(), block, offset, character.len_utf8());
                for _ in 0..character.len_utf8() {
                    positions.push(token.clone());
                }
                text.push(character);
            }
        }
        (text, positions)
    }
    pub fn find(&mut self, needle: &str, backwards: bool) -> bool {
        if needle.is_empty() {
            return false;
        }
        let (text, positions) = self.searchable();
        let matches = text
            .match_indices(needle)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let found = if backwards {
            matches
                .iter()
                .copied()
                .rev()
                .find(|i| positions[*i].start < self.selection.range().start)
                .or_else(|| matches.last().copied())
        } else {
            matches
                .iter()
                .copied()
                .find(|i| positions[*i].start >= self.selection.range().end)
                .or_else(|| matches.first().copied())
        };
        let Some(found) = found else {
            return false;
        };
        self.set_selection(Selection {
            anchor: positions[found].start,
            head: positions[found + needle.len() - 1].end,
        });
        true
    }
    pub fn replace_all(&mut self, needle: &str, replacement: &str) -> Result<bool, String> {
        if needle.is_empty() {
            return Ok(false);
        }
        // Stage the complete operation so any unsupported boundary leaves history and source intact.
        let (text, positions) = self.searchable();
        let ranges = text
            .match_indices(needle)
            .map(|(i, _)| positions[i].start..positions[i + needle.len() - 1].end)
            .collect::<Vec<_>>();
        if ranges.is_empty() {
            return Ok(false);
        }
        if ranges.windows(2).any(|pair| pair[0].end > pair[1].start) {
            return Err("A match splits an indivisible entity or synthetic label; edit the complete token in source mode".into());
        }
        let mut staged = Engine::new(self.source());
        for range in ranges.into_iter().rev() {
            staged.set_selection(Selection {
                anchor: range.start,
                head: range.end,
            });
            staged.replace(replacement)?;
        }
        let next = staged.source().to_owned();
        let selection = staged.selection;
        Ok(self.apply(0..self.source().len(), &next, selection, false))
    }
    pub fn edit_table(
        &mut self,
        id: u64,
        edit: MarkdownTableEdit,
    ) -> Result<bool, MarkdownTableEditError> {
        let mut staged = MarkdownDocument::new(self.source());
        // IDs belong to the retained tree, not the staging parser.
        let old = self.document.tables();
        let index = old
            .iter()
            .position(|table| table.id == id)
            .ok_or(MarkdownTableEditError::TableNotFound)?;
        let staged_id = staged
            .tables()
            .get(index)
            .ok_or(MarkdownTableEditError::TableNotFound)?
            .id;
        staged.edit_table(staged_id, edit)?;
        Ok(self.apply(
            0..self.source().len(),
            staged.source(),
            self.selection,
            false,
        ))
    }
}

fn boundary(text: &str, offset: usize) -> usize {
    let mut offset = offset.min(text.len());
    while !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}
// Markdown omits edge spaces and empty paragraphs; the editing model must retain their geometry.
fn editable_whitespace(document: &mut MarkdownDocument) {
    fn empty_leaf(range: Range<usize>, source: &str) -> Block {
        let mut block = Block::new(Kind::Paragraph, range.start);
        block.id = liora_core::next_unique_id();
        block.source_range = range.clone();
        block.text = source[range.clone()].to_owned().into();
        block.spans.push(Span {
            range: 0..block.text.len(),
            source_range: range,
            style: Default::default(),
        });
        block
    }
    fn container_content(line: &str) -> usize {
        let mut offset = 0;
        loop {
            let content = offset;
            offset += line[offset..]
                .bytes()
                .take_while(|c| matches!(c, b' ' | b'\t'))
                .count();
            if line.as_bytes().get(offset) == Some(&b'>') {
                offset += 1;
                if line.as_bytes().get(offset) == Some(&b' ') {
                    offset += 1;
                }
                continue;
            }
            if matches!(line.as_bytes().get(offset), Some(b'-' | b'+' | b'*'))
                && matches!(line.as_bytes().get(offset + 1), Some(b' ' | b'\t'))
            {
                offset += 2;
                if line
                    .get(offset..offset + 4)
                    .is_some_and(|s| matches!(s, "[ ] " | "[x] " | "[X] "))
                {
                    offset += 4;
                }
                continue;
            }
            let digits = line[offset..]
                .bytes()
                .take_while(u8::is_ascii_digit)
                .count();
            if digits > 0
                && matches!(line.as_bytes().get(offset + digits), Some(b'.' | b')'))
                && matches!(line.as_bytes().get(offset + digits + 1), Some(b' ' | b'\t'))
            {
                offset += digits + 2;
                continue;
            }
            return content;
        }
    }

    fn edges(blocks: &mut [Block], source: &str, owner: Range<usize>, in_item: bool) {
        for block in blocks {
            let item = matches!(block.kind, Kind::Item);
            edges(
                &mut block.children,
                source,
                block.source_range.clone(),
                in_item || item,
            );
            if matches!(block.kind, Kind::Item | Kind::Quote | Kind::Callout(_)) {
                let mut start = line_range(source, block.source_range.start).start;
                while start < block.source_range.end {
                    let line = line_range(source, start);
                    let content = start + container_content(&source[line.clone()]);
                    // Markers belong to the container; only its omitted empty content is editable.
                    if content <= line.end
                        && source[content..line.end]
                            .bytes()
                            .all(|c| matches!(c, b' ' | b'\t'))
                        && !block.children.iter().any(|child| {
                            child.source_range.start <= line.end
                                && child.source_range.end >= content
                        })
                    {
                        block.children.push(empty_leaf(content..line.end, source));
                    }
                    start = source[line.end..]
                        .find('\n')
                        .map_or(source.len(), |i| line.end + i + 1);
                }
                block.children.sort_by_key(|child| child.source_range.start);
            }
            if !matches!(
                block.kind,
                Kind::Paragraph
                    | Kind::Heading(_)
                    | Kind::DefinitionTitle
                    | Kind::DefinitionDefinition
            ) {
                continue;
            }
            if matches!(block.kind, Kind::Paragraph) {
                let line = line_range(source, block.source_range.start);
                let start = (line.start
                    + if in_item {
                        prefix(&source[line.clone()]).content
                    } else {
                        quote_indent(&source[line.clone()])
                    })
                .max(owner.start);
                let end = start
                    + source[start..line.end]
                        .bytes()
                        .take_while(|c| matches!(c, b' ' | b'\t'))
                        .count();
                if start < end && end <= block.source_range.start {
                    let mut text = source[start..end].to_owned();
                    text.push_str(&block.text);
                    for span in &mut block.spans {
                        span.range.start += end - start;
                        span.range.end += end - start;
                    }
                    for image in &mut block.images {
                        image.range.start += end - start;
                        image.range.end += end - start;
                    }
                    block.spans.insert(
                        0,
                        Span {
                            range: 0..end - start,
                            source_range: start..end,
                            style: Default::default(),
                        },
                    );
                    block.text = text.into();
                    block.source_range.start = start;
                    block.start = start;
                }
            }
            let raw = source[block.source_range.clone()].trim_end_matches(['\r', '\n']);
            let mut end = block.source_range.start + raw.len();
            // Tight-list paragraphs end at their final inline token, before trailing spaces.
            let line_end = line_range(source, end.saturating_sub(1)).end.min(owner.end);
            if end <= line_end {
                end = line_end;
            }
            let raw = &source[block.source_range.start..end];
            let start = block.source_range.start + raw.trim_end_matches([' ', '\t']).len();
            if start < end
                && block
                    .spans
                    .last()
                    .is_some_and(|span| span.source_range.end <= start)
            {
                let mut text = block.text.to_string();
                let offset = text.len();
                text.push_str(&source[start..end]);
                block.text = text.into();
                block.spans.push(Span {
                    range: offset..block.text.len(),
                    source_range: start..end,
                    style: Default::default(),
                });
                block.source_range.end = block.source_range.end.max(end);
            }
        }
    }
    let mut blocks = std::mem::take(&mut document.blocks);
    let source = document.source();
    edges(&mut blocks, source, 0..source.len(), false);
    let mut previous = 0;
    let mut gaps = Vec::new();
    for block in &blocks {
        let gap = previous..block.source_range.start;
        let separator = if source[..previous].ends_with('\n') {
            1
        } else {
            2
        };
        let mut start = gap.start;
        for _ in 0..if previous > 0 { separator } else { 0 } {
            if source[start..gap.end].starts_with("\r\n") {
                start += 2;
            } else if source[start..gap.end].starts_with('\n') {
                start += 1;
            }
        }
        if start < gap.end && source[start..gap.end].chars().all(char::is_whitespace) {
            gaps.push(start..gap.end);
        }
        previous = block.source_range.end;
    }
    let tail = &source[previous..];
    if tail.chars().all(char::is_whitespace) && (previous == 0 || tail.contains('\n')) {
        let separator = if source[..previous].ends_with('\n') {
            1
        } else {
            2
        };
        let mut start = previous;
        for _ in 0..if previous > 0 { separator } else { 0 } {
            if source[start..].starts_with("\r\n") {
                start += 2;
            } else if source[start..].starts_with('\n') {
                start += 1;
            }
        }
        gaps.push(start..source.len());
    }
    for range in gaps {
        blocks.push(empty_leaf(range, source));
    }
    blocks.sort_by_key(|block| block.source_range.start);
    document.blocks = blocks;
}

fn block_by_id(blocks: &[Block], id: u64) -> Option<&Block> {
    for block in blocks {
        if block.id == id {
            return Some(block);
        }
        if let Some(found) = block_by_id(&block.children, id) {
            return Some(found);
        }
    }
    None
}
fn nearest_text_block(blocks: &[Block], offset: usize) -> Option<&Block> {
    if blocks.is_empty() {
        return None;
    }
    let index = blocks
        .partition_point(|block| block.source_range.start <= offset)
        .saturating_sub(1);
    let block = &blocks[index];
    if !block.children.is_empty() {
        return nearest_text_block(&block.children, offset);
    }
    if !block.text.is_empty()
        || matches!(
            block.kind,
            Kind::Paragraph
                | Kind::Heading(_)
                | Kind::Code(_)
                | Kind::Math(_)
                | Kind::Metadata(_)
                | Kind::Cell
        )
    {
        return Some(block);
    }
    blocks
        .get(index + 1)
        .and_then(|block| nearest_text_block(std::slice::from_ref(block), offset))
        .or_else(|| {
            index
                .checked_sub(1)
                .and_then(|previous| nearest_text_block(&blocks[..=previous], offset))
        })
}
fn text_blocks(blocks: &[Block]) -> Vec<&Block> {
    fn collect<'a>(blocks: &'a [Block], output: &mut Vec<&'a Block>) {
        for block in blocks {
            if !block.text.is_empty()
                || matches!(
                    block.kind,
                    Kind::Paragraph
                        | Kind::Heading(_)
                        | Kind::Code(_)
                        | Kind::Math(_)
                        | Kind::Metadata(_)
                        | Kind::Cell
                )
            {
                output.push(block);
            }
            collect(&block.children, output);
        }
    }
    let mut output = Vec::new();
    collect(blocks, &mut output);
    output
}
fn map_span(source: &str, block: &Block, span: &Span, offset: usize) -> usize {
    let visible = &block.text[span.range.clone()];
    let original = &source[span.source_range.clone()];
    let relative = offset.saturating_sub(span.range.start).min(visible.len());
    if original == visible {
        let start = span.source_range.start;
        if relative == 0 && start > 0 && source.as_bytes()[start - 1] == b'\\' {
            let slashes = source[..start]
                .bytes()
                .rev()
                .take_while(|c| *c == b'\\')
                .count();
            if slashes % 2 == 1 {
                return start - 1;
            }
        }
        return start + relative;
    }
    if let Some(start) = original.find(visible).filter(|_| !visible.is_empty()) {
        return span.source_range.start + start + relative;
    }
    if span.style.code {
        if let Some(position) = normalized_code_position(original, visible, relative) {
            return span.source_range.start + position;
        }
    }
    // Entity expansion, normalized code and synthetic footnote labels are indivisible
    // source tokens. Their display boundaries map to the entire original token.
    if relative == 0 {
        span.source_range.start
    } else if relative == visible.len() {
        span.source_range.end
    } else {
        span.source_range.start
    }
}
fn normalized_code_position(original: &str, visible: &str, relative: usize) -> Option<usize> {
    let fence = original.bytes().take_while(|byte| *byte == b'`').count();
    if fence == 0 || original.len() < fence * 2 {
        return None;
    }
    let content = &original[fence..original.len() - fence];
    let mut normalized = String::new();
    let mut boundaries = vec![fence];
    let mut characters = content.char_indices().peekable();
    while let Some((offset, character)) = characters.next() {
        let mut end = fence + offset + character.len_utf8();
        let character = if character == '\r' {
            if characters.peek().is_some_and(|(_, c)| *c == '\n') {
                let (offset, character) = characters.next()?;
                end = fence + offset + character.len_utf8();
            }
            ' '
        } else if character == '\n' {
            ' '
        } else {
            character
        };
        normalized.push(character);
        for _ in 0..character.len_utf8() {
            boundaries.push(end);
        }
    }
    let trim = normalized.starts_with(' ')
        && normalized.ends_with(' ')
        && normalized.chars().any(|c| c != ' ');
    let start = usize::from(trim);
    let end = normalized.len() - usize::from(trim);
    if normalized.get(start..end)? != visible {
        return None;
    }
    boundaries.get(start + relative).copied()
}
fn character_source(source: &str, block: &Block, offset: usize, length: usize) -> Range<usize> {
    let Some(span) = block
        .spans
        .iter()
        .find(|span| span.range.start <= offset && offset < span.range.end)
    else {
        return block.source_range.start..block.source_range.start;
    };
    let mapped =
        map_span(source, block, span, offset)..map_span(source, block, span, offset + length);
    if mapped.is_empty() && span.source_range.len() > 0 {
        span.source_range.clone()
    } else {
        mapped
    }
}
fn escape(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    for character in text.chars() {
        if "\\*_`~[]&<>#=|$+-^.)".contains(character) {
            output.push('\\');
        }
        output.push(character);
    }
    output
}

struct Wrapper {
    full: Range<usize>,
    content: Range<usize>,
}
fn inline_wrappers(source: &str) -> Vec<Wrapper> {
    let mut wrappers = Vec::new();
    for (event, full) in Parser::new_ext(source, Options::all()).into_offset_iter() {
        let marker = match event {
            Event::Start(Tag::Strong) => Some(2),
            Event::Start(Tag::Emphasis) => Some(1),
            Event::Start(Tag::Strikethrough) => Some(2),
            Event::Start(Tag::Link { .. }) => Some(1),
            Event::Code(_) => Some(
                source[full.clone()]
                    .bytes()
                    .take_while(|c| *c == b'`')
                    .count(),
            ),
            Event::InlineMath(_) => Some(1),
            Event::DisplayMath(_) => Some(2),
            Event::Text(_) => {
                let raw = &source[full.clone()];
                let mut cursor = 0;
                while let Some(open) = raw[cursor..].find("==").map(|i| cursor + i) {
                    let Some(close) = raw[open + 2..].find("==").map(|i| open + 2 + i) else {
                        break;
                    };
                    if close > open + 2 {
                        wrappers.push(Wrapper {
                            full: full.start + open..full.start + close + 2,
                            content: full.start + open + 2..full.start + close,
                        });
                    }
                    cursor = close + 2;
                }
                None
            }
            _ => None,
        };
        if let Some(width) = marker {
            let raw = &source[full.clone()];
            let end_width = if raw.starts_with('[') {
                raw.rfind(']').map_or(width, |i| raw.len() - i)
            } else {
                width
            };
            if full.len() >= width + end_width {
                wrappers.push(Wrapper {
                    content: full.start + width..full.end - end_width,
                    full,
                });
            }
        }
    }
    wrappers.sort_by_key(|wrapper| (wrapper.full.start, std::cmp::Reverse(wrapper.full.end)));
    wrappers
}

fn line_range(source: &str, position: usize) -> Range<usize> {
    let position = boundary(source, position);
    let start = source[..position].rfind('\n').map_or(0, |i| i + 1);
    let end = source[position..]
        .find('\n')
        .map_or(source.len(), |i| position + i);
    start..if end > start && source.as_bytes()[end - 1] == b'\r' {
        end - 1
    } else {
        end
    }
}
struct Prefix {
    content: usize,
    last: usize,
    number: Option<(Range<usize>, u64)>,
    heading: bool,
}
fn quote_indent(line: &str) -> usize {
    let mut offset = 0;
    loop {
        let spaces = line[offset..].bytes().take_while(|c| *c == b' ').count();
        if line.as_bytes().get(offset + spaces) != Some(&b'>') {
            return offset;
        }
        offset += spaces + 1;
        if line.as_bytes().get(offset) == Some(&b' ') {
            offset += 1;
        }
    }
}
fn prefix(line: &str) -> Prefix {
    let mut offset = quote_indent(line);
    let mut last = if offset > 0 {
        line[..offset.saturating_sub(1)].rfind('>').unwrap_or(0)
    } else {
        0
    };
    let indentation = line[offset..]
        .bytes()
        .take_while(|c| matches!(c, b' ' | b'\t'))
        .count();
    offset += indentation;
    let marker_start = offset;
    let mut number = None;
    let mut heading = false;
    if matches!(line.as_bytes().get(offset), Some(b'-' | b'+' | b'*'))
        && matches!(line.as_bytes().get(offset + 1), Some(b' ' | b'\t'))
    {
        offset += 2;
        last = marker_start;
    } else {
        let digits = line[offset..]
            .bytes()
            .take_while(u8::is_ascii_digit)
            .count();
        if digits > 0
            && matches!(line.as_bytes().get(offset + digits), Some(b'.' | b')'))
            && matches!(line.as_bytes().get(offset + digits + 1), Some(b' ' | b'\t'))
        {
            number = line[offset..offset + digits]
                .parse()
                .ok()
                .map(|n| (offset..offset + digits, n));
            offset += digits + 2;
            last = marker_start;
        } else {
            let hashes = line[offset..].bytes().take_while(|c| *c == b'#').count();
            if (1..=6).contains(&hashes) && line.as_bytes().get(offset + hashes) == Some(&b' ') {
                offset += hashes + 1;
                heading = true;
                last = marker_start;
            }
        }
    }
    if line
        .get(offset..offset + 3)
        .is_some_and(|s| matches!(s, "[ ]" | "[x]" | "[X]"))
        && line.as_bytes().get(offset + 3) == Some(&b' ')
    {
        offset += 4;
    }
    Prefix {
        content: offset,
        last,
        number,
        heading,
    }
}

struct Identity {
    id: u64,
    range: Range<usize>,
    kind: Kind,
}
fn identities(blocks: &[Block], edited: &Range<usize>, delta: isize) -> Vec<Identity> {
    fn collect(blocks: &[Block], edited: &Range<usize>, delta: isize, output: &mut Vec<Identity>) {
        for block in blocks {
            let mut range = block.source_range.clone();
            if range.start >= edited.end && !(edited.is_empty() && range.start == edited.start) {
                range = range.start.saturating_add_signed(delta)
                    ..range.end.saturating_add_signed(delta);
            } else if range.start <= edited.start && range.end >= edited.end {
                range.end = range.end.saturating_add_signed(delta);
            } else if range.end > edited.start {
                collect(&block.children, edited, delta, output);
                continue;
            }
            output.push(Identity {
                id: block.id,
                range,
                kind: block.kind.clone(),
            });
            collect(&block.children, edited, delta, output);
        }
    }
    let mut output = Vec::new();
    collect(blocks, edited, delta, &mut output);
    output
}
fn restore_ids(blocks: &mut [Block], identities: &[Identity]) {
    let lookup = identities
        .iter()
        .map(|identity| {
            (
                (
                    identity.range.start,
                    identity.range.end,
                    std::mem::discriminant(&identity.kind),
                ),
                identity.id,
            )
        })
        .collect::<std::collections::HashMap<_, _>>();
    fn restore(
        blocks: &mut [Block],
        lookup: &std::collections::HashMap<(usize, usize, std::mem::Discriminant<Kind>), u64>,
    ) {
        for block in blocks {
            block.id = lookup
                .get(&(
                    block.source_range.start,
                    block.source_range.end,
                    std::mem::discriminant(&block.kind),
                ))
                .copied()
                .unwrap_or_else(liora_core::next_unique_id);
            restore(&mut block.children, lookup);
        }
    }
    restore(blocks, &lookup);
}
fn patch_plain(blocks: &mut [Block], edited: &Range<usize>, inserted: &str, delta: isize) -> bool {
    let mut patched = false;
    for block in blocks {
        if block.source_range.start <= edited.start
            && edited.end <= block.source_range.end
            && matches!(block.kind, Kind::Paragraph)
            && block.spans.len() == 1
        {
            let span = &mut block.spans[0];
            if span.source_range.start <= edited.start && edited.end <= span.source_range.end {
                let mut text = block.text.to_string();
                text.replace_range(
                    edited.start - span.source_range.start..edited.end - span.source_range.start,
                    inserted,
                );
                block.text = text.into();
                span.range.end = span.range.end.saturating_add_signed(delta);
                span.source_range.end = span.source_range.end.saturating_add_signed(delta);
                block.source_range.end = block.source_range.end.saturating_add_signed(delta);
                patched = true;
                continue;
            }
        }
        if block.source_range.start >= edited.end {
            shift_block(block, delta);
        }
    }
    patched
}
fn shift_block(block: &mut Block, delta: isize) {
    block.start = block.start.saturating_add_signed(delta);
    block.source_range = block.source_range.start.saturating_add_signed(delta)
        ..block.source_range.end.saturating_add_signed(delta);
    for span in &mut block.spans {
        span.source_range = span.source_range.start.saturating_add_signed(delta)
            ..span.source_range.end.saturating_add_signed(delta);
    }
    if let Some(range) = &mut block.task_marker_range {
        *range = range.start.saturating_add_signed(delta)..range.end.saturating_add_signed(delta);
    }
    for child in &mut block.children {
        shift_block(child, delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn select(engine: &mut Engine, start: usize, end: usize) {
        engine.set_selection(Selection {
            anchor: start,
            head: end,
        });
    }

    #[test]
    fn literal_typing_preserves_styles_neighbors_and_identity() {
        let mut engine = Engine::new("**hello**\r\n\r\nuntouched &amp; bytes\r\n");
        let ids = engine.document().block_ids().collect::<Vec<_>>();
        select(&mut engine, 4, 4);
        engine.replace("*").unwrap();
        assert_eq!(
            engine.source(),
            "**he\\*llo**\r\n\r\nuntouched &amp; bytes\r\n"
        );
        assert_eq!(engine.document().block_ids().collect::<Vec<_>>(), ids);
    }

    #[test]
    fn grouped_history_restores_selection_in_both_directions() {
        let mut engine = Engine::new("hello");
        select(&mut engine, 1, 4);
        engine.begin_undo_group();
        engine.replace("a").unwrap();
        engine.replace("b").unwrap();
        engine.end_undo_group();
        engine.command(Command::Undo).unwrap();
        assert_eq!(engine.source(), "hello");
        assert_eq!(engine.selection(), Selection { anchor: 1, head: 4 });
        engine.command(Command::Redo).unwrap();
        assert_eq!(engine.source(), "habo");
        assert_eq!(engine.selection().head, 3);
    }

    #[test]
    fn replacement_balances_cross_style_and_cross_block_ranges() {
        let mut engine = Engine::new("**alpha** and *beta*\n\nlast");
        select(&mut engine, 4, 17);
        engine.replace("X").unwrap();
        assert_eq!(engine.document().plain_text(), "alXta\nlast\n");
        assert!(engine.document().blocks[0].spans[0].style.strong);
        let start = engine.source().find('X').unwrap();
        let end = engine.source().find("last").unwrap() + 2;
        select(&mut engine, start, end);
        engine.replace("Z").unwrap();
        assert_eq!(engine.document().plain_text(), "alZst\n");
    }

    #[test]
    fn escaped_entities_map_and_clipboard_omits_delimiters() {
        let mut engine = Engine::new("**a &amp; \\* 文**");
        let id = engine.document().blocks[0].id;
        let source = engine.source_position(id, "a & ".len()).unwrap();
        assert_eq!(&engine.source()[source..source + 2], "\\*");
        assert_eq!(engine.display_position(source + 1), Some((id, 4)));
        let end = engine.source().len();
        select(&mut engine, 0, end);
        assert_eq!(engine.selected_text(), "a & * 文");
    }

    #[test]
    fn backspace_removes_one_displayed_grapheme() {
        let mut engine = Engine::new("a👩‍👩‍👧‍👦e\u{301}");
        let end = engine.source().len();
        select(&mut engine, end, end);
        engine.command(Command::Backspace).unwrap();
        assert_eq!(engine.source(), "a👩‍👩‍👧‍👦");
        engine.command(Command::Backspace).unwrap();
        assert_eq!(engine.source(), "a");
    }

    #[test]
    fn nested_tasks_continue_exit_and_crlf_survives() {
        let mut engine = Engine::new("> - [x] done\r\n");
        let end = engine.source().find('\r').unwrap();
        select(&mut engine, end, end);
        engine.command(Command::Enter).unwrap();
        assert_eq!(engine.source(), "> - [x] done\r\n> - [ ] \r\n");
        engine.command(Command::Enter).unwrap();
        assert_eq!(engine.source(), "> - [x] done\r\n> \r\n");
        engine.command(Command::Enter).unwrap();
        assert_eq!(engine.source(), "> - [x] done\r\n\r\n");
    }

    #[test]
    fn find_replace_sees_entities_and_styles_as_text() {
        let mut engine = Engine::new("**a** &amp; a\n\n[a][ref]\n\n[ref]: https://example.com\n");
        assert!(engine.find("a & a", false));
        assert_eq!(engine.selected_text(), "a & a");
        engine.replace_all("a", "*").unwrap();
        assert!(engine.document().plain_text().starts_with("* & *\n*\n"));
        assert!(engine.source().ends_with("[ref]: https://example.com\n"));
    }

    #[test]
    fn plain_typing_skips_parse_and_keeps_following_ids() {
        let mut engine = Engine::new("alpha\n\nbeta\n");
        let parses = engine.document().parse_count();
        let ids = engine.document().block_ids().collect::<Vec<_>>();
        select(&mut engine, 2, 2);
        engine.replace("z").unwrap();
        assert_eq!(engine.document().parse_count(), parses);
        assert_eq!(engine.document().block_ids().collect::<Vec<_>>(), ids);
        assert_eq!(
            engine.document().plain_text(),
            "alZpha\nbeta\n".replace('Z', "z")
        );
    }

    #[test]
    fn plain_typing_benchmark_and_later_block_identity() {
        let source = (0..1000)
            .map(|i| format!("paragraph {i} content\n\n"))
            .collect::<String>();
        let mut engine = Engine::new(source.clone());
        let ids = engine.document().block_ids().collect::<Vec<_>>();
        let parses = engine.document().parse_count();
        select(&mut engine, 4, 4);
        let started = std::time::Instant::now();
        for _ in 0..40 {
            engine.replace("word").unwrap();
        }
        let incremental = started.elapsed();
        assert_eq!(engine.document().parse_count(), parses);
        assert_eq!(engine.document().block_ids().collect::<Vec<_>>(), ids);
        assert_eq!(&engine.source()[164..], &source[4..]);
        let mut baseline = MarkdownDocument::new(source.clone());
        let mut next = source;
        let started = std::time::Instant::now();
        for i in 0..40 {
            next.insert_str(4 + i * 4, "word");
            baseline.set_source(next.clone());
            baseline.flush();
        }
        eprintln!(
            "markdown 1000 paragraphs / 40 word inserts: incremental={incremental:?}, full_parse={:?}",
            started.elapsed()
        );
        assert_eq!(engine.source(), baseline.source());
    }

    #[test]
    fn reference_definition_change_reparses_global_dependents() {
        let mut engine = Engine::new("[label][ref]\n\n[ref]: https://old.example\n");
        let before = engine.document().parse_count();
        let next = engine.source().replace("old.example", "new.example");
        assert!(engine.set_source(&next));
        assert!(engine.document().parse_count() > before);
        assert_eq!(
            engine.document().blocks[0].spans[0].style.link.as_deref(),
            Some("https://new.example")
        );
    }

    #[test]
    fn composition_caret_stays_inside_bold() {
        let mut engine = Engine::new("- **old**\n\n> quoted\n");
        select(&mut engine, 4, 7);
        engine.begin_undo_group();
        engine.replace("候选").unwrap();
        engine.end_undo_group();
        engine.replace("!").unwrap();
        assert_eq!(engine.source(), "- **候选!**\n\n> quoted\n");
    }

    #[test]
    fn crlf_paragraph_join_keeps_styled_bytes() {
        let mut engine = Engine::new("**one**\r\n\r\n*two*\r\n");
        select(&mut engine, 12, 12);
        engine.command(Command::Backspace).unwrap();
        assert_eq!(engine.source(), "**one***two*\r\n");
    }

    #[test]
    fn indentation_and_task_toggle_are_undoable() {
        let mut engine = Engine::new("- [ ] first\n- second\n");
        let id = engine.document().blocks[0].children[0].id;
        engine.command(Command::ToggleTask(id)).unwrap();
        assert!(engine.source().starts_with("- [x] first"));
        select(&mut engine, 0, 0);
        engine.command(Command::Indent).unwrap();
        assert!(engine.source().starts_with("    - [x] first"));
        engine.command(Command::Outdent).unwrap();
        assert!(engine.source().starts_with("- [x] first"));
        engine.command(Command::Undo).unwrap();
        assert!(engine.source().starts_with("    - [x] first"));
    }

    #[test]
    fn table_edits_commit_once_and_restore_selection() {
        let mut engine = Engine::new("| a | b |\r\n| - | - |\r\n| c | d |\r\n");
        let id = engine.document().tables()[0].id;
        select(&mut engine, 2, 3);
        engine
            .edit_table(
                id,
                MarkdownTableEdit::SetCell {
                    row: 1,
                    column: 0,
                    text: "x|y".into(),
                },
            )
            .unwrap();
        assert!(engine.source().contains("x\\|y"));
        assert!(!engine.source().replace("\r\n", "").contains('\n'));
        engine.command(Command::Undo).unwrap();
        assert!(engine.source().contains("| c | d |"));
        assert_eq!(engine.selection(), Selection { anchor: 2, head: 3 });
    }

    #[test]
    fn whole_styled_span_replacement_uses_content_boundaries() {
        let mut engine = Engine::new("a **b**");
        let id = engine.document().blocks[0].id;
        let start = engine.source_position(id, 2).unwrap();
        let end = engine.source_position(id, 3).unwrap();
        assert_eq!((start, end), (4, 5));
        select(&mut engine, start, end);
        engine.replace("X").unwrap();
        assert_eq!(engine.source(), "a **X**");
    }

    #[test]
    fn replacement_before_styled_content_reopens_suffix_wrapper() {
        let mut engine = Engine::new("a **b**");
        let id = engine.document().blocks[0].id;
        let start = engine.source_position(id, 0).unwrap();
        let end = engine.source_position(id, 2).unwrap();
        select(&mut engine, start, end);
        engine.replace("X").unwrap();
        assert_eq!(engine.source(), "X**b**");
    }

    #[test]
    fn split_does_not_reuse_shifted_sibling_identity() {
        let mut engine = Engine::new("alpha\n\nbeta\n");
        let beta = engine.document().blocks[1].id;
        select(&mut engine, 2, 2);
        engine.replace("abc\n\n").unwrap();
        assert_eq!(engine.source(), "alabc\n\npha\n\nbeta\n");
        assert_eq!(engine.document().blocks[2].id, beta);
        let ids = engine
            .document()
            .block_ids()
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn enter_retains_nested_fence_containers() {
        for (source, expected) in [
            ("> ```\n> ab\n> ```", "> ```\n> a\n> b\n> ```"),
            ("- ```\n  ab\n  ```", "- ```\n  a\n  b\n  ```"),
        ] {
            let mut engine = Engine::new(source);
            let caret = source.find("ab").unwrap() + 1;
            select(&mut engine, caret, caret);
            engine.command(Command::Enter).unwrap();
            assert_eq!(engine.source(), expected);
        }
    }

    #[test]
    fn normalized_and_synthetic_tokens_copy_every_character() {
        for source in [
            "`a\nb`",
            "&NotEqualTilde;",
            "text[^note]\n\n[^note]: definition\n",
        ] {
            let mut engine = Engine::new(source);
            let visible = engine
                .document()
                .plain_text()
                .trim_end_matches('\n')
                .to_owned();
            select(&mut engine, 0, source.len());
            assert_eq!(engine.selected_text(), visible);
        }
        let engine = Engine::new("`a\nb`");
        let id = engine.document().blocks[0].id;
        assert_eq!(engine.source_position(id, 1), Some(2));
        assert_eq!(engine.source_position(id, 2), Some(3));
        assert_eq!(engine.display_position(3), Some((id, 2)));
    }

    #[test]
    fn replacement_balances_highlight_code_and_math() {
        for (source, expected) in [
            ("==alpha== beta", "==alX==ta"),
            ("`alpha` beta", "`alX`ta"),
            ("$alpha$ beta", "$alX$ta"),
        ] {
            let mut engine = Engine::new(source);
            let start = source.find("alpha").unwrap() + 2;
            let end = source.find("beta").unwrap() + 2;
            select(&mut engine, start, end);
            engine.replace("X").unwrap();
            assert_eq!(engine.source(), expected);
        }
    }

    #[test]
    fn joining_heading_removes_structural_marker_both_directions() {
        for (caret, command) in [(3, Command::Delete), (7, Command::Backspace)] {
            let mut engine = Engine::new("one\n\n# two\n");
            select(&mut engine, caret, caret);
            engine.command(command).unwrap();
            assert_eq!(engine.source(), "onetwo\n");
        }
    }

    #[test]
    fn ordered_and_superscript_paste_stays_literal() {
        let mut engine = Engine::new("");
        engine.replace("1. item\n2) next\n^2^").unwrap();
        assert_eq!(engine.document().plain_text(), "1. item\n2) next\n^2^\n");
        assert!(matches!(engine.document().blocks[0].kind, Kind::Paragraph));
    }

    #[test]
    fn code_conversion_retains_quote_and_list_containers() {
        for (source, expected) in [
            ("> hello", "> ```\n> hello\n> ```"),
            ("- hello", "- ```\n  hello\n  ```"),
        ] {
            let mut engine = Engine::new(source);
            select(&mut engine, source.len(), source.len());
            engine.command(Command::Block("code")).unwrap();
            assert_eq!(engine.source(), expected);
        }
    }

    #[test]
    fn paragraph_enter_creates_two_blocks() {
        let mut engine = Engine::new("alpha");
        select(&mut engine, 2, 2);
        engine.command(Command::Enter).unwrap();
        assert_eq!(engine.source(), "al\n\npha");
        assert_eq!(engine.document().blocks.len(), 2);
    }

    #[test]
    fn full_source_callers_preserve_local_and_unaffected_identities() {
        let mut engine = Engine::new("before\n\n| a | b |\n| - | - |\n| c | d |\n\nafter\n");
        let original = engine.document().block_ids().collect::<Vec<_>>();
        let table = engine.document().tables()[0].id;
        engine
            .edit_table(
                table,
                MarkdownTableEdit::SetCell {
                    row: 1,
                    column: 0,
                    text: "longer".into(),
                },
            )
            .unwrap();
        assert_eq!(engine.document().tables()[0].id, table);
        assert_eq!(engine.document().block_ids().collect::<Vec<_>>(), original);
        let changed = engine.source().replace("before", "bef文ore");
        engine.set_source(&changed);
        assert_eq!(engine.document().block_ids().collect::<Vec<_>>(), original);
        assert_eq!(engine.document().tables()[0].id, table);
    }

    #[test]
    fn enter_then_backspace_restores_leading_space_and_source() {
        let mut engine = Engine::new("hello world\n\nlast");
        select(&mut engine, 5, 5);
        engine.command(Command::Enter).unwrap();
        assert_eq!(engine.source(), "hello\n\n world\n\nlast");
        assert_eq!(engine.selection().head, 7);
        engine.command(Command::Backspace).unwrap();
        assert_eq!(engine.source(), "hello world\n\nlast");
        assert_eq!(engine.selection().head, 5);
    }

    #[test]
    fn backspace_inside_multiline_blocks_deletes_only_the_local_break() {
        for (source, expected) in [
            ("second\nthird", "secondthird"),
            ("first\n\nsecond\nthird", "first\n\nsecondthird"),
            ("```\nsecond\nthird\n```", "```\nsecondthird\n```"),
        ] {
            let mut engine = Engine::new(source);
            let caret = source.find("third").unwrap();
            select(&mut engine, caret, caret);
            assert!(engine.command(Command::Backspace).unwrap());
            assert_eq!(engine.source(), expected);
            assert!(engine.command(Command::Undo).unwrap());
            assert_eq!(engine.source(), source);
            assert_eq!(engine.selection().head, caret);
        }
    }

    #[test]
    fn leading_spaces_stay_in_the_owning_styled_paragraph() {
        for source in [" hello", " **hello**", ">  **hello**"] {
            let engine = Engine::new(source);
            let leaves = text_blocks(&engine.document().blocks);
            assert_eq!(leaves.len(), 1, "{source:?}");
            let block = leaves[0];
            assert_eq!(block.text.as_ref(), " hello", "{source:?}");
            let start = source.find(' ').unwrap();
            let start = if source.starts_with('>') {
                start + 1
            } else {
                start
            };
            assert_eq!(engine.source_position(block.id, 0), Some(start));
            assert_eq!(engine.display_position(start), Some((block.id, 0)));
            assert_eq!(
                engine.source_position(block.id, 1),
                Some(source.find("hello").unwrap())
            );
            assert_eq!(engine.source(), source);
            if source.contains("**") {
                assert!(block.spans.iter().any(|span| span.style.strong));
                assert!(!block.spans[0].style.strong);
            }
        }
    }

    #[test]
    fn tight_list_trailing_spaces_keep_mapping_style_and_history() {
        for source in ["- hello ", "- **hello** \r\n- next\r\n", "> - hello \r\n"] {
            let mut engine = Engine::new(source);
            let block = text_blocks(&engine.document().blocks)[0];
            let id = block.id;
            assert_eq!(block.text.as_ref(), "hello ", "{source:?}");
            let space = source.find(" \r").unwrap_or(source.len() - 1);
            assert_eq!(engine.source_position(id, 5), Some(space));
            assert_eq!(engine.source_position(id, 6), Some(space + 1));
            assert_eq!(engine.display_position(space + 1), Some((id, 6)));
            let ids = identities(&engine.document().blocks, &(source.len()..source.len()), 0)
                .into_iter()
                .map(|identity| identity.id)
                .collect::<Vec<_>>();
            select(&mut engine, space + 1, space + 1);
            engine.type_markdown("x");
            assert_eq!(
                engine.source(),
                format!("{}x{}", &source[..space + 1], &source[space + 1..])
            );
            engine.command(Command::Undo).unwrap();
            assert_eq!(engine.source(), source);
            assert_eq!(
                identities(&engine.document().blocks, &(source.len()..source.len()), 0)
                    .into_iter()
                    .map(|identity| identity.id)
                    .collect::<Vec<_>>(),
                ids
            );
            assert_eq!(
                text_blocks(&engine.document().blocks)[0].text.as_ref(),
                "hello "
            );
            if source.contains("**") {
                assert!(
                    text_blocks(&engine.document().blocks)[0].spans[0]
                        .style
                        .strong
                );
            }
        }
    }

    #[test]
    fn enter_creates_editable_empty_leaves_inside_list_and_quote_containers() {
        for ending in ["\n", "\r\n"] {
            for marker in ["- ", "> ", "> - [ ] "] {
                let source = format!(
                    "{marker}hello{}",
                    if ending == "\r\n" { ending } else { "" }
                );
                let mut engine = Engine::new(source.clone());
                let caret = source.find("hello").unwrap() + 5;
                select(&mut engine, caret, caret);
                engine.command(Command::Enter).unwrap();
                let expected = format!("{}{ending}{marker}{}", &source[..caret], &source[caret..]);
                assert_eq!(engine.source(), expected);
                let leaves = text_blocks(&engine.document().blocks);
                assert_eq!(leaves.len(), 2, "{expected:?}");
                assert_eq!(leaves[0].text.as_ref(), "hello");
                let empty = leaves[1];
                assert!(empty.text.is_empty(), "{expected:?}");
                let id = empty.id;
                let position = caret + ending.len() + marker.len();
                assert_eq!(engine.source_position(id, 0), Some(position));
                assert_eq!(engine.display_position(position), Some((id, 0)));
                assert_eq!(engine.document().blocks.len(), 1);
                engine.type_markdown("文");
                assert_eq!(
                    engine.source(),
                    format!("{}文{}", &expected[..position], &expected[position..])
                );
                engine.command(Command::Undo).unwrap();
                assert_eq!(engine.source(), expected);
                assert_eq!(text_blocks(&engine.document().blocks)[1].id, id);
                engine.command(Command::Undo).unwrap();
                assert_eq!(engine.source(), source);
            }
        }
    }

    #[test]
    fn invalid_commands_and_table_edits_are_atomic() {
        let mut engine = Engine::new("hello");
        select(&mut engine, 1, 4);
        assert!(engine.command(Command::Inline("unknown")).is_err());
        assert!(
            engine
                .edit_table(999, super::super::MarkdownTableEdit::DeleteRow { index: 0 })
                .is_err()
        );
        assert_eq!(engine.source(), "hello");
        assert_eq!(engine.selection(), Selection { anchor: 1, head: 4 });
    }
}

#[cfg(test)]
mod nested_whitespace_regressions {
    use super::*;

    #[test]
    fn empty_quotes_inside_list_items_map_to_their_content() {
        for source in ["- > ", "- > hello\n  > ", "- > hello\r\n  > "] {
            let engine = Engine::new(source);
            let (id, display) = engine.display_position(source.len()).unwrap();
            let leaf = block_by_id(&engine.document().blocks, id).unwrap();
            assert!(leaf.text.is_empty(), "{source:?}");
            assert_eq!(display, 0);
            assert_eq!(engine.source_position(id, 0), Some(source.len()));
        }
    }
}
