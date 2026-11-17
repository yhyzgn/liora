use gpui::SharedString;
pub use pulldown_cmark::MetadataBlockKind;
use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, Event, Options, Parser, Tag, TagEnd,
};
use std::{collections::HashMap, ops::Range};

/// Cached, host-independent Markdown source and native document tree.
/// Updates mark the cache dirty; call `flush` once after a batch, or let MarkdownView do so.
#[derive(Default)]
pub struct MarkdownDocument {
    source: String,
    pub(super) blocks: Vec<Block>,
    dirty: bool,
    parse_count: u64,
}

impl MarkdownDocument {
    /// Parses initial source, including unfinished fences and inline syntax.
    pub fn new(source: impl Into<String>) -> Self {
        let mut document = Self {
            source: source.into(),
            dirty: true,
            ..Self::default()
        };
        document.flush();
        document
    }

    /// Current source, including updates not yet parsed.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Replaces source without parsing immediately.
    pub fn set_source(&mut self, source: impl Into<String>) {
        let source = source.into();
        if self.source != source {
            self.source = source;
            self.dirty = true;
        }
    }

    /// Commits a literal edit after the editor has updated every affected block and range.
    /// The caller must rule out syntax, heading-anchor and global-reference changes.
    pub(super) fn commit_plain_source(&mut self, source: String) {
        self.source = source;
        self.dirty = false;
    }

    /// Appends a streaming chunk without parsing or copying the existing source.
    pub fn append(&mut self, chunk: &str) {
        if !chunk.is_empty() {
            self.source.push_str(chunk);
            self.dirty = true;
        }
    }
    /// Rebuilds once per dirty batch. Reference definitions can change earlier blocks,
    /// so reparsing only a suffix would be incorrect.
    pub fn flush(&mut self) -> bool {
        if !self.dirty {
            return false;
        }
        let mut blocks = parse(&self.source);
        assign_heading_ids(&mut blocks);
        let mut old = HashMap::new();
        collect_ids(&self.blocks, &mut old);
        assign_ids(&mut blocks, &old);
        self.blocks = blocks;
        self.dirty = false;
        self.parse_count += 1;
        true
    }
    /// Returns document-local heading anchors in source order.
    pub fn headings(&self) -> Vec<MarkdownHeading> {
        fn collect(blocks: &[Block], headings: &mut Vec<MarkdownHeading>) {
            for block in blocks {
                if let (Kind::Heading(level), Some(id)) = (&block.kind, &block.heading_id) {
                    headings.push(MarkdownHeading {
                        id: id.clone(),
                        level: *level,
                        title: block.text.clone(),
                    });
                }
                collect(&block.children, headings);
            }
        }
        let mut headings = Vec::new();
        collect(&self.blocks, &mut headings);
        headings
    }
    /// Returns footnote definitions and references in deterministic source order.
    ///
    /// Fragment targets are generated from numeric IDs rather than raw labels, so
    /// arbitrary Markdown labels never become native element IDs.
    pub fn footnotes(&self) -> Vec<MarkdownFootnote> {
        let numbers = footnote_numbers(&self.source);
        let mut definitions = HashMap::new();
        fn collect_definitions(
            blocks: &[Block],
            definitions: &mut HashMap<String, (u64, Range<usize>)>,
        ) {
            for block in blocks {
                if let Kind::Footnote(label) = &block.kind {
                    definitions.insert(label.to_string(), (block.id, block.source_range.clone()));
                }
                collect_definitions(&block.children, definitions);
            }
        }
        collect_definitions(&self.blocks, &mut definitions);

        let mut labels = Vec::new();
        let mut references: HashMap<String, Vec<MarkdownFootnoteReference>> = HashMap::new();
        for (event, range) in Parser::new_ext(&self.source, markdown_options()).into_offset_iter() {
            let Event::FootnoteReference(label) = event else {
                continue;
            };
            let label = label.to_string();
            if !labels.iter().any(|known| known == &label) {
                labels.push(label.clone());
            }
            let Some(&number) = numbers.get(&label) else {
                continue;
            };
            let occurrence = references.get(&label).map_or(1, |items| items.len() + 1);
            references
                .entry(label)
                .or_default()
                .push(MarkdownFootnoteReference {
                    number,
                    occurrence,
                    source_range: range,
                    definition_fragment: format!("_footnote-{number}").into(),
                    reference_fragment: format!("_footnote-ref-{number}-{occurrence}").into(),
                });
        }
        let mut remaining = definitions.iter().collect::<Vec<_>>();
        remaining.sort_by_key(|(_, (_, range))| range.start);
        for (label, _) in remaining {
            if !labels.iter().any(|known| known == label) {
                labels.push(label.clone());
            }
        }
        labels
            .into_iter()
            .map(|label| MarkdownFootnote {
                number: numbers.get(&label).copied(),
                definition_block_id: definitions.get(&label).map(|(id, _)| *id),
                definition_range: definitions.get(&label).map(|(_, range)| range.clone()),
                references: references.remove(&label).unwrap_or_default(),
                label: label.into(),
            })
            .collect()
    }

    /// Returns parsed tables in document order.
    pub fn tables(&self) -> Vec<MarkdownTable> {
        fn collect(blocks: &[Block], tables: &mut Vec<MarkdownTable>) {
            for block in blocks {
                if let Kind::Table(alignments) = &block.kind {
                    tables.push(table_snapshot(block, alignments));
                }
                collect(&block.children, tables);
            }
        }
        let mut tables = Vec::new();
        collect(&self.blocks, &mut tables);
        tables
    }

    /// Returns the leading parsed frontmatter as flat, source-backed fields.
    /// Values are trimmed source text, not evaluated YAML/TOML or application settings.
    pub fn metadata(&self) -> Option<MarkdownMetadata<'_>> {
        if self.dirty {
            return None;
        }
        let block = self.blocks.first()?;
        let Kind::Metadata(kind) = &block.kind else {
            return None;
        };
        let kind = *kind;
        let source = self.source.get(block.source_range.clone())?;
        let mut lines = source.split_inclusive('\n');
        let opening = lines.next()?;
        let mut offset = block.source_range.start + opening.len();
        let mut metadata = MarkdownMetadata {
            kind,
            source_range: block.source_range.clone(),
            fields: Vec::new(),
            diagnostics: Vec::new(),
        };
        let delimiter = match kind {
            MetadataBlockKind::YamlStyle => ':',
            MetadataBlockKind::PlusesStyle => '=',
        };
        for line in lines {
            let end = offset + line.len();
            let text = line.trim_end_matches(['\r', '\n']);
            let closing = text.trim_end_matches(' ');
            let is_closing = match kind {
                MetadataBlockKind::YamlStyle => closing == "---" || closing == "...",
                MetadataBlockKind::PlusesStyle => closing == "+++",
            };
            if is_closing {
                break;
            }
            let trimmed = text.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                let field = text
                    .split_once(delimiter)
                    .map(|(key, value)| (key.trim_end(), value))
                    .filter(|(key, value)| {
                        !value.trim().is_empty()
                            && !key.is_empty()
                            && key
                                .chars()
                                .all(|c| c.is_alphanumeric() || matches!(c, '_' | '-' | '.'))
                    });
                if let Some((key, value)) = field {
                    metadata.fields.push(MarkdownMetadataField {
                        key,
                        value: value.trim(),
                        source_range: offset..end,
                    });
                } else {
                    metadata.diagnostics.push(MarkdownMetadataDiagnostic {
                        source_range: offset..end,
                        message: "Expected a flat, nonempty metadata field",
                    });
                }
            }
            offset = end;
        }
        Some(metadata)
    }

    /// Applies a bounded structural edit and reparses the canonical source.
    pub fn edit_table(
        &mut self,
        table_id: u64,
        edit: MarkdownTableEdit,
    ) -> Result<(), MarkdownTableEditError> {
        self.flush();
        let table = self
            .tables()
            .into_iter()
            .find(|table| table.id == table_id)
            .ok_or(MarkdownTableEditError::TableNotFound)?;
        if !table.rows.iter().any(|row| row.header) {
            return Err(MarkdownTableEditError::HeaderRowRequired);
        }
        match edit {
            MarkdownTableEdit::SetCell { row, column, text } => {
                let row = table
                    .rows
                    .get(row)
                    .ok_or(MarkdownTableEditError::RowOutOfBounds)?;
                if column >= row.cells.len() {
                    return Err(MarkdownTableEditError::ColumnOutOfBounds);
                }
                let range = table_row_content(&self.source, row)?;
                let (mut parts, leading, trailing) = split_table_row(&self.source[range.clone()]);
                let present = parts.len() - leading - usize::from(trailing);
                if present > table.alignments.len() {
                    return Err(MarkdownTableEditError::ColumnOutOfBounds);
                }
                for _ in present..table.alignments.len() {
                    parts.insert(parts.len() - usize::from(trailing), " ");
                }
                if parts.len() != table.alignments.len() + leading + usize::from(trailing) {
                    return Err(MarkdownTableEditError::ColumnOutOfBounds);
                }
                let text = format!(" {} ", normalize_table_cell(&text));
                parts[column + leading] = &text;
                self.source.replace_range(range, &parts.join("|"));
            }
            MarkdownTableEdit::InsertRow { index, cells } => {
                let body = table
                    .rows
                    .iter()
                    .filter(|row| !row.header)
                    .collect::<Vec<_>>();
                if index > body.len() {
                    return Err(MarkdownTableEditError::RowOutOfBounds);
                }
                if cells.len() != table.alignments.len() {
                    return Err(MarkdownTableEditError::ColumnOutOfBounds);
                }
                let lines = table_column_lines(&self.source, &table)?;
                let reference = body.get(index).or_else(|| body.last());
                let (insertion, prefix) = if let Some(row) = reference {
                    let physical = table_row_physical(&self.source, row)?;
                    let prefix = &self.source[physical.start..row.source_range.start];
                    let insertion = if index < body.len() {
                        physical.start
                    } else {
                        physical.end
                    };
                    (insertion, prefix)
                } else {
                    let delimiter = &lines[1];
                    let start = self.source[..delimiter.start]
                        .rfind('\n')
                        .map_or(0, |offset| offset + 1);
                    let prefix = &self.source[start..delimiter.start];
                    let end = delimiter.end
                        + if self.source[delimiter.end..].starts_with("\r\n") {
                            2
                        } else if self.source[delimiter.end..].starts_with('\n') {
                            1
                        } else {
                            0
                        };
                    (end, prefix)
                };
                let needs_break = insertion > 0 && self.source.as_bytes()[insertion - 1] != b'\n';
                let newline = if self.source[table.source_range.clone()].contains("\r\n") {
                    "\r\n"
                } else {
                    "\n"
                };
                let mut line = String::new();
                if needs_break {
                    line.push_str(newline);
                }
                line.push_str(prefix);
                line.push_str("| ");
                line.push_str(
                    &cells
                        .iter()
                        .map(|cell| normalize_table_cell(cell))
                        .collect::<Vec<_>>()
                        .join(" | "),
                );
                line.push_str(" |");
                line.push_str(newline);
                self.source.insert_str(insertion, &line);
            }
            MarkdownTableEdit::DeleteRow { index } => {
                let row = table
                    .rows
                    .iter()
                    .filter(|row| !row.header)
                    .nth(index)
                    .ok_or(MarkdownTableEditError::RowOutOfBounds)?;
                let range = table_row_physical(&self.source, row)?;
                self.source.replace_range(range, "");
            }
            MarkdownTableEdit::InsertColumn { index, cells } => {
                if index > table.alignments.len() || cells.len() != table.rows.len() {
                    return Err(MarkdownTableEditError::ColumnOutOfBounds);
                }
                let lines = table_column_lines(&self.source, &table)?;
                let mut replacements = Vec::with_capacity(lines.len());
                for (position, range) in lines.iter().enumerate() {
                    let text = if position == 1 {
                        String::from("---")
                    } else {
                        let row = if position == 0 { 0 } else { position - 1 };
                        cells[row].clone()
                    };
                    let line = edit_table_column_line(
                        &self.source[range.clone()],
                        table.alignments.len(),
                        index,
                        Some(&text),
                        position != 1,
                    )?;
                    replacements.push((range.clone(), line));
                }
                for (range, line) in replacements.into_iter().rev() {
                    self.source.replace_range(range, &line);
                }
            }
            MarkdownTableEdit::DeleteColumn { index } => {
                if index >= table.alignments.len() || table.alignments.len() == 1 {
                    return Err(MarkdownTableEditError::ColumnOutOfBounds);
                }
                let mut replacements = Vec::new();
                for range in table_column_lines(&self.source, &table)? {
                    let line = edit_table_column_line(
                        &self.source[range.clone()],
                        table.alignments.len(),
                        index,
                        None,
                        true,
                    )?;
                    replacements.push((range, line));
                }
                for (range, line) in replacements.into_iter().rev() {
                    self.source.replace_range(range, &line);
                }
            }
            MarkdownTableEdit::MoveColumn { from, to } => {
                let columns = table.alignments.len();
                if from >= columns || to >= columns {
                    return Err(MarkdownTableEditError::ColumnOutOfBounds);
                }
                if from == to {
                    return Ok(());
                }
                let mut replacements = Vec::new();
                for range in table_column_lines(&self.source, &table)? {
                    let line = &self.source[range.clone()];
                    let (mut parts, leading) = table_column_parts(line, columns)?;
                    let moved = parts.remove(from + leading);
                    parts.insert(to + leading, moved);
                    replacements.push((range, parts.join("|")));
                }
                for (range, line) in replacements.into_iter().rev() {
                    self.source.replace_range(range, &line);
                }
            }
            MarkdownTableEdit::MoveRow { from, to } => {
                let body = table
                    .rows
                    .iter()
                    .filter(|row| !row.header)
                    .collect::<Vec<_>>();
                body.get(from)
                    .ok_or(MarkdownTableEditError::RowOutOfBounds)?;
                if to >= body.len() {
                    return Err(MarkdownTableEditError::RowOutOfBounds);
                }
                if from == to {
                    return Ok(());
                }
                let ranges = body
                    .iter()
                    .map(|row| table_row_physical(&self.source, row))
                    .collect::<Result<Vec<_>, _>>()?;
                let start = ranges.first().unwrap().start;
                let end = ranges.last().unwrap().end;
                if ranges.windows(2).any(|pair| pair[0].end != pair[1].start) {
                    return Err(MarkdownTableEditError::TableNotFound);
                }
                let mut order = (0..body.len()).collect::<Vec<_>>();
                order.remove(from);
                order.insert(to, from);
                let mut reordered = String::with_capacity(end - start + 1);
                let newline = if self.source[table.source_range.clone()].contains("\r\n") {
                    "\r\n"
                } else {
                    "\n"
                };
                for (position, index) in order.into_iter().enumerate() {
                    let range = &ranges[index];
                    reordered.push_str(&self.source[range.clone()]);
                    if position + 1 < body.len() && !reordered.ends_with('\n') {
                        reordered.push_str(newline);
                    }
                }
                self.source.replace_range(start..end, &reordered);
            }
        }
        self.dirty = true;
        self.flush();
        Ok(())
    }

    /// Number of completed parses, useful for measuring streaming batching.
    pub fn parse_count(&self) -> u64 {
        self.parse_count
    }

    /// IDs of the last parsed top-level blocks. IDs persist at matching parent,
    /// source start, block kind and sibling position (including growing blocks).
    /// Edits moving these coordinates receive new IDs; synthetic cells remain distinct.
    pub fn block_ids(&self) -> impl Iterator<Item = u64> + '_ {
        self.blocks.iter().map(|b| b.id)
    }

    /// Plain text from the parsed cache, in document order.
    pub fn plain_text(&self) -> String {
        fn collect(block: &Block, output: &mut String) {
            output.push_str(&block.text);
            if !block.text.is_empty() {
                output.push('\n');
            }
            for child in &block.children {
                collect(child, output);
            }
        }
        let mut output = String::new();
        for block in &self.blocks {
            collect(block, &mut output);
        }
        output
    }
}

/// Leading parsed Markdown frontmatter; fields never change application configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownMetadata<'a> {
    /// Syntax used by the detected frontmatter block.
    pub kind: MetadataBlockKind,
    /// Byte range of the frontmatter in the original Markdown source.
    pub source_range: Range<usize>,
    /// Flat fields in source order.
    pub fields: Vec<MarkdownMetadataField<'a>>,
    /// Unsupported or malformed lines in the frontmatter.
    pub diagnostics: Vec<MarkdownMetadataDiagnostic>,
}

/// One flat, source-backed metadata field; the value is not YAML/TOML evaluated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownMetadataField<'a> {
    /// Field key as written in the source.
    pub key: &'a str,
    /// Field value as written in the source.
    pub value: &'a str,
    /// Byte range of this field in the original Markdown source.
    pub source_range: Range<usize>,
}

/// Unsupported or malformed frontmatter line, with its original source range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownMetadataDiagnostic {
    /// Byte range of the problematic line in the original Markdown source.
    pub source_range: Range<usize>,
    /// Diagnostic message describing the syntax problem.
    pub message: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A document-local table-of-contents entry.
pub struct MarkdownHeading {
    /// Slug used by a `#fragment` link.
    pub id: SharedString,
    /// Heading depth from one through six.
    pub level: u8,
    /// Visible heading text.
    pub title: SharedString,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A parsed footnote definition and its source references.
pub struct MarkdownFootnote {
    /// Original Markdown label.
    pub label: SharedString,
    /// First-reference number, or `None` when the definition is unreferenced.
    pub number: Option<usize>,
    /// Stable parsed block ID for the definition.
    pub definition_block_id: Option<u64>,
    /// Source range of the definition block.
    pub definition_range: Option<Range<usize>>,
    /// References in source order.
    pub references: Vec<MarkdownFootnoteReference>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// One source reference to a footnote.
pub struct MarkdownFootnoteReference {
    /// Number shown at the reference site.
    pub number: usize,
    /// One-based occurrence for repeated references to the same label.
    pub occurrence: usize,
    /// Source range of the `[^label]` reference.
    pub source_range: Range<usize>,
    /// Safe native target ID for the definition.
    pub definition_fragment: SharedString,
    /// Safe native target ID for this reference, used by a back-link.
    pub reference_fragment: SharedString,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Alignment of one Markdown table column.
pub enum MarkdownTableAlignment {
    /// No explicit alignment.
    #[default]
    None,
    /// Left alignment.
    Left,
    /// Center alignment.
    Center,
    /// Right alignment.
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A source-backed Markdown table snapshot.
pub struct MarkdownTable {
    /// Stable parsed block ID.
    pub id: u64,
    /// Source range replaced by structural edits.
    pub source_range: Range<usize>,
    /// Column alignments in source order.
    pub alignments: Vec<MarkdownTableAlignment>,
    /// Rows, including the optional header row at its parsed position.
    pub rows: Vec<MarkdownTableRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// One Markdown table row.
pub struct MarkdownTableRow {
    /// Whether this row is the Markdown header row.
    pub header: bool,
    /// Source range of this row, including its trailing line break when present.
    pub source_range: Range<usize>,
    /// Cells in source order.
    pub cells: Vec<MarkdownTableCell>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// One Markdown table cell.
pub struct MarkdownTableCell {
    /// Visible cell text without table delimiters.
    pub text: SharedString,
    /// Source range of the parsed cell.
    pub source_range: Range<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Source-preserving structural table operation.
pub enum MarkdownTableEdit {
    /// Replace one existing cell with literal plain text; line breaks become spaces.
    /// Existing inline Markdown markup in that cell is intentionally discarded.
    SetCell {
        /// Zero-based parsed row index, including the header row when present.
        row: usize,
        /// Zero-based column index.
        column: usize,
        /// Literal plain text to write into the cell.
        text: String,
    },
    /// Insert a body row at the given zero-based body-row index.
    InsertRow {
        /// Zero-based position among body rows.
        index: usize,
        /// One cell value per column.
        cells: Vec<String>,
    },
    /// Delete a body row at the given zero-based body-row index.
    DeleteRow {
        /// Zero-based index among body rows.
        index: usize,
    },
    /// Insert a column at the zero-based index; cells include the header followed by body rows.
    InsertColumn {
        /// Zero-based insertion position among columns.
        index: usize,
        /// Cell values in row order, including the header when present.
        cells: Vec<String>,
    },
    /// Delete a column; the last remaining column cannot be deleted.
    DeleteColumn {
        /// Zero-based column index.
        index: usize,
    },
    /// Move a body row between zero-based body-row positions.
    MoveRow {
        /// Zero-based source index among body rows.
        from: usize,
        /// Zero-based destination index among body rows.
        to: usize,
    },
    /// Move a column and its alignment delimiter between zero-based positions.
    MoveColumn {
        /// Zero-based source column index.
        from: usize,
        /// Zero-based destination column index.
        to: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Rejection reason for a table edit.
pub enum MarkdownTableEditError {
    /// The table ID is absent or its source rows cannot be mapped contiguously.
    TableNotFound,
    /// A requested row index is outside the available rows.
    RowOutOfBounds,
    /// A column index or supplied column values do not fit the table.
    ColumnOutOfBounds,
    /// The table has no header row, which all table edits require.
    HeaderRowRequired,
    /// The public editor cannot commit a table edit during IME composition.
    CompositionActive,
}
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Kind {
    Paragraph,
    Heading(u8),
    Quote,
    Callout(BlockQuoteKind),
    List(Option<u64>),
    Item,
    Code(SharedString),
    Math(bool),
    Metadata(MetadataBlockKind),
    Footnote(SharedString),
    DefinitionList,
    DefinitionTitle,
    DefinitionDefinition,
    Table(Vec<Alignment>),
    Row(bool),
    Cell,
    Rule,
    Container,
}

#[derive(Clone, Debug, Default)]
pub(super) struct Style {
    pub strong: bool,
    pub emphasis: bool,
    pub strike: bool,
    pub code: bool,
    pub link: Option<SharedString>,
    pub superscript: bool,
    pub subscript: bool,
    pub highlight: bool,
    pub math: Option<bool>,
    pub footnote: Option<SharedString>,
    pub footnote_occurrence: usize,
    pub image: Option<SharedString>,
}

pub(super) struct Span {
    pub range: Range<usize>,
    pub source_range: Range<usize>,
    pub style: Style,
}
pub(super) struct ImageOccurrence {
    pub destination: SharedString,
    pub range: Range<usize>,
}
pub(super) struct Block {
    pub id: u64,
    pub start: usize,
    pub source_range: Range<usize>,
    pub kind: Kind,
    pub text: SharedString,
    pub heading_id: Option<SharedString>,
    pub task_checked: Option<bool>,
    pub task_marker_range: Option<Range<usize>>,
    pub footnote_number: Option<usize>,
    buffer: String,
    buffer_start: usize,
    pub spans: Vec<Span>,
    pub images: Vec<ImageOccurrence>,
    pub children: Vec<Block>,
}

impl Block {
    pub(super) fn new(kind: Kind, start: usize) -> Self {
        Self {
            id: 0,
            start,
            source_range: start..start,
            kind,
            text: SharedString::default(),
            heading_id: None,
            task_checked: None,
            task_marker_range: None,
            footnote_number: None,
            buffer: String::new(),
            buffer_start: start,
            spans: Vec::new(),
            images: Vec::new(),
            children: Vec::new(),
        }
    }

    fn flush_item_inline(&mut self) {
        if !matches!(self.kind, Kind::Item) || (self.buffer.is_empty() && self.images.is_empty()) {
            return;
        }
        let mut paragraph = Self::new(Kind::Paragraph, self.buffer_start);
        paragraph.source_range.end = self
            .spans
            .iter()
            .map(|span| span.source_range.end)
            .max()
            .unwrap_or(self.buffer_start);
        paragraph.text = std::mem::take(&mut self.buffer).into();
        paragraph.spans = std::mem::take(&mut self.spans);
        paragraph.images = std::mem::take(&mut self.images);
        self.children.push(paragraph);
    }
}

fn table_snapshot(block: &Block, alignments: &[Alignment]) -> MarkdownTable {
    MarkdownTable {
        id: block.id,
        source_range: block.source_range.clone(),
        alignments: alignments
            .iter()
            .map(|alignment| match alignment {
                Alignment::None => MarkdownTableAlignment::None,
                Alignment::Left => MarkdownTableAlignment::Left,
                Alignment::Center => MarkdownTableAlignment::Center,
                Alignment::Right => MarkdownTableAlignment::Right,
            })
            .collect(),
        rows: block
            .children
            .iter()
            .filter_map(|row| {
                let Kind::Row(header) = row.kind else {
                    return None;
                };
                Some(MarkdownTableRow {
                    header,
                    source_range: row.source_range.clone(),
                    cells: row
                        .children
                        .iter()
                        .map(|cell| MarkdownTableCell {
                            text: cell.text.clone(),
                            source_range: cell.source_range.clone(),
                        })
                        .collect(),
                })
            })
            .collect(),
    }
}

fn normalize_table_cell(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    let padding = |ch| matches!(ch, ' ' | '\t' | '\r' | '\n');
    let start = text.len() - text.trim_start_matches(padding).len();
    let end = text.trim_end_matches(padding).len();
    for (index, ch) in text.char_indices() {
        // Entities distinguish literal edge whitespace from Markdown table padding.
        if padding(ch) && (index < start || index >= end) {
            escaped.push_str(if ch == '\t' { "&#9;" } else { "&#32;" });
            continue;
        }
        if ch.is_ascii_punctuation() {
            escaped.push('\\');
        }
        escaped.push(if matches!(ch, '\n' | '\r') { ' ' } else { ch });
    }
    escaped
}

fn table_row_physical(
    source: &str,
    row: &MarkdownTableRow,
) -> Result<Range<usize>, MarkdownTableEditError> {
    let range = &row.source_range;
    if range.start > range.end || range.end > source.len() || !source.is_char_boundary(range.start)
    {
        return Err(MarkdownTableEditError::TableNotFound);
    }
    let start = source[..range.start]
        .rfind('\n')
        .map_or(0, |offset| offset + 1);
    let end = source[range.start..]
        .find('\n')
        .map_or(source.len(), |offset| range.start + offset + 1);
    Ok(start..end)
}

fn table_row_content(
    source: &str,
    row: &MarkdownTableRow,
) -> Result<Range<usize>, MarkdownTableEditError> {
    let physical = table_row_physical(source, row)?;
    let end = source[..physical.end].trim_end_matches(['\n', '\r']).len();
    if row.source_range.start > end {
        return Err(MarkdownTableEditError::TableNotFound);
    }
    Ok(row.source_range.start..end)
}

fn table_column_lines(
    source: &str,
    table: &MarkdownTable,
) -> Result<Vec<Range<usize>>, MarkdownTableEditError> {
    let header = table
        .rows
        .iter()
        .find(|row| row.header)
        .ok_or(MarkdownTableEditError::HeaderRowRequired)?;
    if table.source_range.end > source.len() || header.source_range.start < table.source_range.start
    {
        return Err(MarkdownTableEditError::TableNotFound);
    }
    let content_end = |physical: &Range<usize>| {
        physical.end
            - if source[physical.clone()].ends_with("\r\n") {
                2
            } else if source[physical.clone()].ends_with('\n') {
                1
            } else {
                0
            }
    };
    let header_line = table_row_physical(source, header)?;
    let delimiter_line = header_line.end;
    let delimiter_start = delimiter_line;
    let delimiter_physical_end = source[delimiter_start..table.source_range.end]
        .find('\n')
        .map_or(table.source_range.end, |offset| {
            delimiter_start + offset + 1
        });
    let delimiter_physical = delimiter_start..delimiter_physical_end;
    let prefix_len = header.source_range.start - header_line.start;
    let delimiter_content_start = delimiter_start + prefix_len;
    let mut lines = Vec::with_capacity(table.rows.len() + 1);
    lines.push(header.source_range.start..content_end(&header_line));
    lines.push(delimiter_content_start..content_end(&delimiter_physical));
    for row in table.rows.iter().filter(|row| !row.header) {
        let physical = table_row_physical(source, row)?;
        if row.source_range.start < physical.start
            || row.source_range.start > content_end(&physical)
        {
            return Err(MarkdownTableEditError::TableNotFound);
        }
        lines.push(row.source_range.start..content_end(&physical));
    }
    Ok(lines)
}

fn table_column_parts(
    line: &str,
    columns: usize,
) -> Result<(Vec<&str>, usize), MarkdownTableEditError> {
    let (parts, leading, trailing) = split_table_row(line);
    if parts.len() != columns + leading + usize::from(trailing) {
        return Err(MarkdownTableEditError::ColumnOutOfBounds);
    }
    Ok((parts, leading))
}

fn split_table_row(line: &str) -> (Vec<&str>, usize, bool) {
    let bytes = line.as_bytes();
    let mut separators = Vec::new();
    let mut cursor = 0;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'\\' => cursor += if cursor + 1 < bytes.len() { 2 } else { 1 },
            b'`' => {
                let start = cursor;
                while cursor < bytes.len() && bytes[cursor] == b'`' {
                    cursor += 1;
                }
                let ticks = cursor - start;
                let mut close = cursor;
                while close < bytes.len() {
                    if bytes[close] != b'`' {
                        close += 1;
                        continue;
                    }
                    let run = close;
                    while close < bytes.len() && bytes[close] == b'`' {
                        close += 1;
                    }
                    if close - run == ticks {
                        cursor = close;
                        break;
                    }
                }
            }
            b'|' => {
                separators.push(cursor);
                cursor += 1;
            }
            _ => cursor += 1,
        }
    }
    let mut parts = Vec::with_capacity(separators.len() + 2);
    let mut start = 0;
    for separator in separators {
        parts.push(&line[start..separator]);
        start = separator + 1;
    }
    parts.push(&line[start..]);
    let leading = parts.first().is_some_and(|part| part.trim().is_empty())
        && line.trim_start().starts_with('|');
    let trailing =
        parts.last().is_some_and(|part| part.trim().is_empty()) && line.trim_end().ends_with('|');
    (parts, usize::from(leading), trailing)
}
fn edit_table_column_line(
    line: &str,
    columns: usize,
    index: usize,
    inserted: Option<&str>,
    escape_inserted: bool,
) -> Result<String, MarkdownTableEditError> {
    let (mut parts, leading) = table_column_parts(line, columns)?;
    let position = index + leading;
    let trailing = parts.len() == columns + leading + 1;
    let escaped = inserted.map(|text| {
        format!(
            " {} ",
            if escape_inserted {
                normalize_table_cell(text)
            } else {
                text.to_owned()
            }
        )
    });
    if let Some(escaped) = &escaped {
        parts.insert(position, escaped);
        Ok(parts.join("|"))
    } else {
        parts.remove(position);
        let line = parts.join("|");
        Ok(if columns == 2 {
            format!(
                "{}{line}{}",
                if leading == 0 { "|" } else { "" },
                if trailing { "" } else { "|" }
            )
        } else {
            line
        })
    }
}

fn key(block: &Block) -> (usize, u8) {
    let kind = match block.kind {
        Kind::Paragraph => 0,
        Kind::Heading(_) => 1,
        Kind::Quote => 2,
        Kind::Callout(_) => 3,
        Kind::List(_) => 4,
        Kind::Item => 5,
        Kind::Code(_) => 6,
        Kind::Math(_) => 7,
        Kind::Metadata(_) => 8,
        Kind::Footnote(_) => 9,
        Kind::DefinitionList => 10,
        Kind::DefinitionTitle => 11,
        Kind::DefinitionDefinition => 12,
        Kind::Table(_) => 13,
        Kind::Row(_) => 14,
        Kind::Cell => 15,
        Kind::Rule => 16,
        Kind::Container => 17,
    };
    (block.start, kind)
}

type Identity = (u64, usize, u8, usize);

fn collect_ids(blocks: &[Block], ids: &mut HashMap<Identity, u64>) {
    fn collect(blocks: &[Block], parent: u64, ids: &mut HashMap<Identity, u64>) {
        for (index, block) in blocks.iter().enumerate() {
            let (start, kind) = key(block);
            ids.insert((parent, start, kind, index), block.id);
            collect(&block.children, block.id, ids);
        }
    }
    collect(blocks, 0, ids);
}
fn assign_ids(blocks: &mut [Block], ids: &HashMap<Identity, u64>) {
    fn assign(blocks: &mut [Block], parent: u64, ids: &HashMap<Identity, u64>) {
        for (index, block) in blocks.iter_mut().enumerate() {
            let (start, kind) = key(block);
            block.id = ids
                .get(&(parent, start, kind, index))
                .copied()
                .unwrap_or_else(liora_core::next_unique_id);
            assign(&mut block.children, block.id, ids);
        }
    }
    assign(blocks, 0, ids);
}
fn assign_heading_ids(blocks: &mut [Block]) {
    fn slugify(text: &str) -> String {
        let mut slug = String::new();
        for ch in text.chars() {
            if ch.is_alphanumeric() {
                slug.extend(ch.to_lowercase().filter(|part| part.is_alphanumeric()));
            } else if !slug.ends_with('-') {
                slug.push('-');
            }
        }
        while slug.ends_with('-') {
            slug.pop();
        }
        if slug.is_empty() {
            "section".to_owned()
        } else {
            slug
        }
    }

    fn visit(
        blocks: &mut [Block],
        counts: &mut HashMap<String, usize>,
        used: &mut std::collections::HashSet<String>,
    ) {
        for block in blocks {
            if matches!(block.kind, Kind::Heading(_)) {
                let base = slugify(block.text.as_ref());
                let count = counts.entry(base.clone()).or_default();
                let id = loop {
                    *count += 1;
                    let candidate = if *count == 1 {
                        base.clone()
                    } else {
                        format!("{base}-{count}")
                    };
                    if used.insert(candidate.clone()) {
                        break candidate;
                    }
                };
                block.heading_id = Some(id.into());
            }
            visit(&mut block.children, counts, used);
        }
    }

    visit(
        blocks,
        &mut HashMap::new(),
        &mut std::collections::HashSet::new(),
    );
}

// Bound every later recursive walk and drop, while flattening deeper text visibly.
const MAX_BLOCK_DEPTH: usize = 64;
fn markdown_options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_DEFINITION_LIST
        | Options::ENABLE_SUPERSCRIPT
        | Options::ENABLE_SUBSCRIPT
        | Options::ENABLE_MATH
        | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
        | Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS
        | Options::ENABLE_GFM
}

fn footnote_numbers(source: &str) -> HashMap<String, usize> {
    let mut numbers = HashMap::new();
    for event in Parser::new_ext(source, markdown_options()) {
        let Event::FootnoteReference(label) = event else {
            continue;
        };
        let next = numbers.len() + 1;
        numbers.entry(label.to_string()).or_insert(next);
    }
    numbers
}
struct InlinePart {
    text: String,
    source_range: Range<usize>,
    highlighted: bool,
}

fn highlight_parts(text: &str, source_start: usize) -> Option<Vec<InlinePart>> {
    let mut parts = Vec::new();
    let mut cursor = 0;
    let mut search = 0;
    let mut found = false;
    while let Some(open_offset) = text[search..].find("==") {
        let open = search + open_offset;
        let Some(close_offset) = text[open + 2..].find("==") else {
            break;
        };
        let close = open + 2 + close_offset;
        if close == open + 2 {
            search = close + 2;
            continue;
        }
        if open > cursor {
            parts.push(InlinePart {
                text: text[cursor..open].to_owned(),
                source_range: source_start + cursor..source_start + open,
                highlighted: false,
            });
        }
        parts.push(InlinePart {
            text: text[open + 2..close].to_owned(),
            source_range: source_start + open..source_start + close + 2,
            highlighted: true,
        });
        cursor = close + 2;
        search = cursor;
        found = true;
    }
    if !found {
        return None;
    }
    if cursor < text.len() {
        parts.push(InlinePart {
            text: text[cursor..].to_owned(),
            source_range: source_start + cursor..source_start + text.len(),
            highlighted: false,
        });
    }
    Some(parts)
}

fn parse(source: &str) -> Vec<Block> {
    let mut stack = vec![Block::new(Kind::Container, 0)];
    let mut styles = vec![Style::default()];
    let mut flattened_depth = 0usize;
    let mut separator_pending = false;
    let mut image_starts = Vec::new();
    let footnote_numbers = footnote_numbers(source);
    let mut footnote_occurrences: HashMap<String, usize> = HashMap::new();
    let options = markdown_options();
    for (event, range) in Parser::new_ext(source, options).into_offset_iter() {
        match event {
            Event::Start(tag) => {
                let mut style = styles.last().cloned().unwrap_or_default();
                let kind = match tag {
                    Tag::Emphasis => {
                        style.emphasis = true;
                        None
                    }
                    Tag::Strong => {
                        style.strong = true;
                        None
                    }
                    Tag::Strikethrough => {
                        style.strike = true;
                        None
                    }
                    Tag::Superscript => {
                        style.superscript = true;
                        None
                    }
                    Tag::Subscript => {
                        style.subscript = true;
                        None
                    }
                    Tag::Link { dest_url, .. } => {
                        style.link = Some(dest_url.to_string().into());
                        None
                    }
                    Tag::Image { dest_url, .. } => {
                        style.image = Some(dest_url.to_string().into());
                        image_starts.push((
                            style.image.clone().unwrap(),
                            stack.last().unwrap().buffer.len(),
                        ));
                        None
                    }
                    Tag::Paragraph => Some(Kind::Paragraph),
                    Tag::Heading { level, .. } => Some(Kind::Heading(level as u8)),
                    Tag::BlockQuote(kind) => Some(kind.map(Kind::Callout).unwrap_or(Kind::Quote)),
                    Tag::List(start) => Some(Kind::List(start)),
                    Tag::Item => Some(Kind::Item),
                    Tag::CodeBlock(kind) => {
                        let language: SharedString = match kind {
                            CodeBlockKind::Indented => SharedString::default(),
                            CodeBlockKind::Fenced(info) => info
                                .split_whitespace()
                                .next()
                                .unwrap_or("")
                                .to_owned()
                                .into(),
                        };
                        if matches!(language.as_ref(), "math" | "latex" | "tex") {
                            Some(Kind::Math(true))
                        } else {
                            Some(Kind::Code(language))
                        }
                    }
                    Tag::FootnoteDefinition(label) => {
                        Some(Kind::Footnote(label.into_string().into()))
                    }
                    Tag::DefinitionList => Some(Kind::DefinitionList),
                    Tag::DefinitionListTitle => Some(Kind::DefinitionTitle),
                    Tag::DefinitionListDefinition => Some(Kind::DefinitionDefinition),
                    Tag::MetadataBlock(kind) => Some(Kind::Metadata(kind)),
                    Tag::Table(alignments) => Some(Kind::Table(alignments)),
                    Tag::TableHead => Some(Kind::Row(true)),
                    Tag::TableRow => Some(Kind::Row(false)),
                    Tag::TableCell => Some(Kind::Cell),
                    _ => Some(Kind::Container),
                };
                if let Some(kind) = kind {
                    if stack.len() <= MAX_BLOCK_DEPTH && flattened_depth == 0 {
                        stack.last_mut().unwrap().flush_item_inline();
                        let mut block = Block::new(kind, range.start);
                        if let Kind::Footnote(label) = &block.kind {
                            block.footnote_number = footnote_numbers.get(label.as_ref()).copied();
                        }
                        stack.push(block);
                    } else {
                        flattened_depth += 1;
                    }
                } else {
                    styles.push(style);
                }
            }
            Event::End(end) => {
                if matches!(
                    end,
                    TagEnd::Emphasis
                        | TagEnd::Strong
                        | TagEnd::Strikethrough
                        | TagEnd::Superscript
                        | TagEnd::Subscript
                        | TagEnd::Link
                        | TagEnd::Image
                ) {
                    if end == TagEnd::Image {
                        if let Some((destination, start)) = image_starts.pop() {
                            let block = stack.last_mut().unwrap();
                            block.images.push(ImageOccurrence {
                                destination,
                                range: start..block.buffer.len(),
                            });
                        }
                    }
                    if styles.len() > 1 {
                        styles.pop();
                    }
                } else if flattened_depth > 0 {
                    flattened_depth -= 1;
                    separator_pending = true;
                } else if stack.len() > 1 {
                    separator_pending = false;
                    let mut block = stack.pop().unwrap();
                    block.source_range.end = range.end;
                    block.flush_item_inline();
                    block.text = std::mem::take(&mut block.buffer).into();
                    stack.last_mut().unwrap().children.push(block);
                }
            }
            Event::Rule => {
                let block = stack.last_mut().unwrap();
                block.flush_item_inline();
                let mut rule = Block::new(Kind::Rule, range.start);
                rule.source_range = range;
                block.children.push(rule);
            }
            event => {
                let range_start = range.start;
                let task_checked = match &event {
                    Event::TaskListMarker(checked) => Some(*checked),
                    _ => None,
                };
                let highlighted_parts = match &event {
                    Event::Text(text)
                        if text.as_ref() == &source[range.clone()]
                            && !source[..range.start].ends_with('\\') =>
                    {
                        highlight_parts(text, range.start)
                    }
                    _ => None,
                };
                let mut style = styles.last().cloned().unwrap_or_default();
                let text = match event {
                    Event::Text(text) | Event::Html(text) | Event::InlineHtml(text) => {
                        text.to_string()
                    }
                    Event::Code(text) => {
                        style.code = true;
                        text.to_string()
                    }
                    Event::SoftBreak | Event::HardBreak => "\n".to_owned(),
                    Event::TaskListMarker(_) => String::new(),
                    Event::FootnoteReference(label) => {
                        let label = label.to_string();
                        let occurrence = footnote_occurrences.entry(label.clone()).or_default();
                        *occurrence += 1;
                        style.footnote_occurrence = *occurrence;
                        style.footnote = Some(label.clone().into());
                        format!(
                            "[{}]",
                            footnote_numbers.get(&label).copied().unwrap_or_default()
                        )
                    }
                    Event::InlineMath(text) => {
                        style.math = Some(false);
                        text.to_string()
                    }
                    Event::DisplayMath(text) => {
                        style.math = Some(true);
                        text.to_string()
                    }
                    _ => source[range.clone()].to_owned(),
                };
                let block = stack.last_mut().unwrap();
                if let Some(checked) = task_checked {
                    block.task_checked = Some(checked);
                    block.task_marker_range = Some(range.clone());
                }
                if text.is_empty() {
                    continue;
                }
                if block.buffer.is_empty() {
                    block.buffer_start = range_start;
                }
                if separator_pending && !text.is_empty() {
                    if !block.buffer.is_empty()
                        && !block.buffer.ends_with('\n')
                        && !text.starts_with('\n')
                    {
                        let start = block.buffer.len();
                        block.buffer.push('\n');
                        block.spans.push(Span {
                            range: start..start + 1,
                            source_range: range_start..range_start,
                            style: Style::default(),
                        });
                    }
                    separator_pending = false;
                }
                if let Some(parts) = highlighted_parts {
                    for part in parts {
                        if part.text.is_empty() {
                            continue;
                        }
                        let start = block.buffer.len();
                        block.buffer.push_str(&part.text);
                        let mut part_style = style.clone();
                        part_style.highlight = part.highlighted;
                        block.spans.push(Span {
                            range: start..start + part.text.len(),
                            source_range: part.source_range,
                            style: part_style,
                        });
                    }
                } else {
                    let start = block.buffer.len();
                    block.buffer.push_str(&text);
                    block.spans.push(Span {
                        range: start..start + text.len(),
                        source_range: range_start..range.end,
                        style,
                    });
                }
            }
        }
    }
    stack.pop().unwrap().children
}

/// Only explicit web and email schemes are allowed. Relative paths, file/data/javascript
/// URLs, controls, whitespace, and backslashes never reach the platform URL opener.
pub fn is_safe_markdown_link(destination: &str) -> bool {
    if destination
        .chars()
        .any(|ch| ch.is_control() || ch.is_whitespace() || ch == '\\')
    {
        return false;
    }
    let Some((scheme, rest)) = destination.split_once(':') else {
        return false;
    };
    if scheme.eq_ignore_ascii_case("mailto") {
        return !rest.is_empty() && !rest.starts_with('/');
    }
    if !scheme.eq_ignore_ascii_case("https") && !scheme.eq_ignore_ascii_case("http") {
        return false;
    }
    rest.strip_prefix("//").is_some_and(|rest| {
        let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
        !authority.is_empty() && !authority.contains('@')
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn streaming_batches_reparse_references_and_keep_ids() {
        let mut document = MarkdownDocument::new("[guide][docs]\n\n```rust\nlet value");
        let ids = document.block_ids().collect::<Vec<_>>();
        for chunk in [" = 1;", "\n```\n", "\n[docs]: https://example.com\n"] {
            document.append(chunk);
        }
        assert_eq!(document.parse_count(), 1);
        assert!(document.flush());
        assert_eq!(document.parse_count(), 2);
        assert!(!document.flush());
        assert_eq!(document.block_ids().collect::<Vec<_>>(), ids);
        assert_eq!(
            document.blocks[0].spans[0].style.link.as_deref(),
            Some("https://example.com")
        );
        assert!(document.plain_text().contains("let value = 1;"));
    }
    #[test]
    fn tight_list_keeps_text_after_fenced_code_in_source_order() {
        let document = MarkdownDocument::new("- ```text\n  first\n  ```\n  second\n");
        let item = &document.blocks[0].children[0];
        assert!(
            item.text.is_empty(),
            "item inline content belongs in the ordered child sequence"
        );
        assert!(matches!(item.children[0].kind, Kind::Code(_)));
        assert!(matches!(item.children[1].kind, Kind::Paragraph));
        assert_eq!(item.children[1].text.as_ref(), "second");
        assert_eq!(
            document
                .plain_text()
                .lines()
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>(),
            ["first", "second"]
        );
    }

    #[test]
    fn tight_list_preserves_multiple_inline_runs_around_structural_children() {
        let document = MarkdownDocument::new(
            "- **first**\n  ```text\n  middle\n  ```\n  second\n  ```text\n  next\n  ```\n  last\n",
        );
        let item = &document.blocks[0].children[0];
        assert!(item.text.is_empty());
        assert_eq!(item.children.len(), 5);
        for index in [0, 2, 4] {
            assert!(matches!(item.children[index].kind, Kind::Paragraph));
        }
        for index in [1, 3] {
            assert!(matches!(item.children[index].kind, Kind::Code(_)));
        }
        assert!(
            item.children[0].spans[0].style.strong,
            "flushing inline runs must preserve styles"
        );
        assert_eq!(
            document
                .plain_text()
                .lines()
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>(),
            ["first", "middle", "second", "next", "last"]
        );
    }
    #[test]
    fn tight_list_inline_offsets_are_captured_before_structural_moves() {
        let source = "- first\n  ```text\n  middle\n  ```\n  last\n";
        let document = MarkdownDocument::new(source);
        let item = &document.blocks[0].children[0];
        assert_eq!(item.children[0].start, source.find("first").unwrap());
        assert_eq!(item.children[1].start, source.find("```text").unwrap());
        assert_eq!(item.children[2].start, source.rfind("last").unwrap());
    }

    #[test]
    fn incomplete_syntax_and_html_remain_visible_without_execution() {
        let mut document = MarkdownDocument::new(
            "# Title\n\nHello **未完成\n\n<script>alert(1)</script>\n\n```rust\nfn main(",
        );
        assert!(document.plain_text().contains("**未完成"));
        assert!(document.plain_text().contains("<script>"));
        assert!(document.plain_text().contains("fn main("));
        document.set_source("replacement");
        document.flush();
        assert_eq!(document.plain_text(), "replacement\n");
    }
    #[test]
    fn native_tree_covers_tables_lists_quotes_styles_and_images() {
        let document = MarkdownDocument::new(
            "1. **bold** and *italic*\n\n> quote\n\n| H | V |\n| :- | -: |\n| `a` | ![alt](file:///secret) |\n",
        );
        assert!(matches!(document.blocks[0].kind, Kind::List(Some(1))));
        assert!(matches!(document.blocks[1].kind, Kind::Quote));
        let table = &document.blocks[2];
        assert!(matches!(table.kind, Kind::Table(_)));
        assert_eq!(table.children.len(), 2);
        assert_eq!(
            table.children[1].children[1].spans[0]
                .style
                .image
                .as_deref(),
            Some("file:///secret")
        );
    }
    #[test]
    fn short_table_cells_keep_unique_ids_after_append() {
        fn ids(blocks: &[Block], output: &mut Vec<u64>) {
            for block in blocks {
                output.push(block.id);
                ids(&block.children, output);
            }
        }
        let mut document =
            MarkdownDocument::new("| L | C | R |\n| :--- | :---: | ---: |\n| short |\n");
        let mut before = Vec::new();
        ids(&document.blocks, &mut before);
        assert_eq!(
            before
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            before.len()
        );
        document.append("| next | middle | last |\n");
        document.flush();
        let mut after = Vec::new();
        ids(&document.blocks, &mut after);
        assert_eq!(&after[..before.len()], before.as_slice());
        assert_eq!(
            after
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            after.len()
        );
        assert!(
            matches!(&document.blocks[0].kind, Kind::Table(columns) if columns == &[Alignment::Left, Alignment::Center, Alignment::Right])
        );
    }

    #[test]
    fn deep_markdown_flattens_without_losing_visible_text() {
        fn depth(blocks: &[Block]) -> usize {
            blocks
                .iter()
                .map(|block| 1 + depth(&block.children))
                .max()
                .unwrap_or(0)
        }
        let document = MarkdownDocument::new(format!("{}visible sentinel\n", "> ".repeat(4096)));
        assert!(depth(&document.blocks) <= MAX_BLOCK_DEPTH + 1);
        assert!(document.plain_text().contains("visible sentinel"));
        assert_eq!(
            document.plain_text().lines().count(),
            1,
            "closing deep containers must not create empty rows"
        );
        let prefix = "> ".repeat(4096);
        let siblings = MarkdownDocument::new(format!("{prefix}first\n{prefix}\n{prefix}second\n"));
        let text = siblings.plain_text();
        assert!(
            text.contains("first\nsecond"),
            "flattened sibling paragraphs need one separator"
        );
        assert!(
            text.lines().count() <= 2,
            "nesting depth must not amplify line count"
        );
    }

    #[test]
    fn repeated_image_destinations_preserve_each_alt_and_empty_occurrence() {
        let document = MarkdownDocument::new("![first](same) ![**second**](same) ![](same)");
        let paragraph = &document.blocks[0];
        assert_eq!(paragraph.images.len(), 3);
        let alts = paragraph
            .images
            .iter()
            .map(|image| &paragraph.text[image.range.clone()])
            .collect::<Vec<_>>();
        assert_eq!(alts, ["first", "second", ""]);
        assert!(
            paragraph
                .images
                .iter()
                .all(|image| image.destination.as_ref() == "same")
        );
    }

    #[test]
    fn links_reject_unsafe_schemes_and_obfuscation() {
        for url in [
            "javascript:alert(1)",
            "file:///etc/passwd",
            "data:text/html,x",
            "//example.com",
            "https://",
            "https://user@example.com",
            "https://example.com\\x",
            "https://example.com\nx",
        ] {
            assert!(!is_safe_markdown_link(url), "{url}");
        }
        for url in [
            "https://example.com/docs",
            "HTTP://example.com",
            "mailto:hello@example.com",
        ] {
            assert!(is_safe_markdown_link(url), "{url}");
        }
    }
    #[test]
    fn common_extensions_keep_visible_text() {
        let document = MarkdownDocument::new(
            "~subscript~ ^superscript^\n\n[^note]: footnote text\n\nterm\n: definition\n\n- [x] done\n",
        );
        let text = document.plain_text();
        for expected in [
            "subscript",
            "superscript",
            "footnote text",
            "term",
            "definition",
            "done",
        ] {
            assert!(text.contains(expected), "missing {expected:?} in {text:?}");
        }
        fn styles(blocks: &[Block]) -> Vec<Style> {
            blocks
                .iter()
                .flat_map(|block| {
                    block
                        .spans
                        .iter()
                        .map(|span| span.style.clone())
                        .chain(styles(&block.children))
                })
                .collect()
        }
        let styles = styles(&document.blocks);
        assert!(styles.iter().any(|style| style.subscript));
        assert!(styles.iter().any(|style| style.superscript));
    }
    #[test]
    fn headings_have_stable_unique_fragment_ids() {
        let document = MarkdownDocument::new("# Hello, World!\n\n## Hello World\n\n# 中文");
        let headings = document.headings();
        assert_eq!(
            headings
                .iter()
                .map(|heading| heading.id.as_ref())
                .collect::<Vec<_>>(),
            ["hello-world", "hello-world-2", "中文"]
        );
    }
    #[test]
    fn heading_and_footnote_targets_do_not_collide_or_contain_unsafe_chars() {
        let document =
            MarkdownDocument::new("# İstanbul\n\n# Footnote 1\n\nnote[^a]\n\n[^a]: note");
        let ids = document.headings();
        assert!(
            ids[0]
                .id
                .chars()
                .all(|ch| ch.is_alphanumeric() || ch == '-')
        );
        let footnote = document.footnotes().remove(0);
        assert_ne!(ids[1].id, footnote.references[0].definition_fragment);
    }
    #[test]
    fn headings_do_not_collide_with_explicitly_suffixed_titles() {
        let document = MarkdownDocument::new("# Topic\n# Topic-2\n# Topic\n# Topic-2\n");
        assert_eq!(
            document
                .headings()
                .iter()
                .map(|heading| heading.id.as_ref())
                .collect::<Vec<_>>(),
            ["topic", "topic-2", "topic-3", "topic-2-2"]
        );
    }

    #[test]
    fn task_markers_are_semantic_and_not_copied_into_plain_text() {
        let document = MarkdownDocument::new("- [x] done\n- [ ] later\n");
        let list = &document.blocks[0];
        assert_eq!(list.children[0].task_checked, Some(true));
        assert_eq!(list.children[1].task_checked, Some(false));
        assert!(!document.plain_text().contains('☑'));
        assert!(!document.plain_text().contains('☐'));
        assert!(document.plain_text().contains("done"));
    }

    #[test]
    fn advanced_blocks_keep_semantic_kinds_and_visible_source() {
        let document = MarkdownDocument::new(
            "---\ntitle: Demo\n---\n\n> [!WARNING]\n> caution\n\n$inline$\n\n```math\nx^2\n```\n\nterm\n: definition\n",
        );
        assert!(matches!(document.blocks[0].kind, Kind::Metadata(_)));
        assert!(
            document
                .blocks
                .iter()
                .any(|block| matches!(block.kind, Kind::Callout(_)))
        );
        assert!(
            document
                .blocks
                .iter()
                .any(|block| matches!(block.kind, Kind::Math(true)))
        );
        assert!(document.plain_text().contains("inline"));
        assert!(document.plain_text().contains("x^2"));
        assert!(document.plain_text().contains("definition"));
    }
    #[test]
    fn footnotes_expose_numbered_references_and_safe_targets() {
        let document = MarkdownDocument::new(
            "First[^long label] and again[^long label], then[^other].\n\n[^long label]: First note\n[^other]: Second note\n",
        );
        let footnotes = document.footnotes();
        assert_eq!(
            footnotes
                .iter()
                .map(|footnote| footnote.label.as_ref())
                .collect::<Vec<_>>(),
            ["long label", "other"]
        );
        assert_eq!(footnotes[0].number, Some(1));
        assert_eq!(footnotes[0].references.len(), 2);
        assert_eq!(footnotes[0].references[1].occurrence, 2);
        assert_eq!(
            footnotes[0].references[0].definition_fragment,
            "_footnote-1"
        );
        assert_eq!(
            footnotes[0].references[1].reference_fragment,
            "_footnote-ref-1-2"
        );
        assert!(footnotes[0].definition_block_id.is_some());
        assert!(footnotes[0].definition_range.is_some());
        assert_eq!(footnotes[1].number, Some(2));
        assert!(document.plain_text().contains("[1]"));
        assert!(document.plain_text().contains("[2]"));
        assert!(
            document.blocks[0]
                .spans
                .iter()
                .any(|span| span.style.footnote.as_deref() == Some("long label"))
        );
    }
    #[test]
    fn highlights_hide_markers_and_preserve_source_mapping() {
        let document = MarkdownDocument::new("before ==marked== after");
        let paragraph = &document.blocks[0];
        assert_eq!(paragraph.text.as_ref(), "before marked after");
        assert!(!paragraph.text.contains("=="));
        let highlighted = paragraph
            .spans
            .iter()
            .find(|span| span.style.highlight)
            .expect("highlight span");
        assert_eq!(&paragraph.text[highlighted.range.clone()], "marked");
        assert_eq!(
            &document.source()[highlighted.source_range.clone()],
            "==marked=="
        );
    }
    #[test]
    fn escaped_highlight_delimiters_remain_literal() {
        let document = MarkdownDocument::new(r"\==literal==");
        assert_eq!(document.blocks[0].text.as_ref(), "==literal==");
        assert!(
            document.blocks[0]
                .spans
                .iter()
                .all(|span| !span.style.highlight)
        );
    }

    #[test]
    fn table_edits_preserve_untouched_markdown_and_reject_invalid_indices() {
        use super::{MarkdownTableEdit, MarkdownTableEditError};
        let mut document = MarkdownDocument::new(
            "before\n\n| Name | Value |\n| --- | ---: |\n| **bold** | `code` |\n\nafter\n",
        );
        let table = document.tables().remove(0);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[1].cells[0].text.as_ref(), "bold");
        assert_eq!(
            document.edit_table(
                table.id,
                MarkdownTableEdit::SetCell {
                    row: 5,
                    column: 0,
                    text: "invalid".into(),
                }
            ),
            Err(MarkdownTableEditError::RowOutOfBounds)
        );
        assert!(document.source().contains("**bold**"));
        document
            .edit_table(
                table.id,
                MarkdownTableEdit::InsertRow {
                    index: 0,
                    cells: vec!["new".into(), "a|b".into()],
                },
            )
            .unwrap();
        assert!(
            document
                .source()
                .contains("| new | a\\|b |\n| **bold** | `code` |")
        );
        let table = document.tables().remove(0);
        document
            .edit_table(table.id, MarkdownTableEdit::DeleteRow { index: 0 })
            .unwrap();
        assert!(document.source().contains("| **bold** | `code` |"));
        assert!(!document.source().contains("| new |"));
        let table = document.tables().remove(0);
        document
            .edit_table(
                table.id,
                MarkdownTableEdit::SetCell {
                    row: 1,
                    column: 1,
                    text: "next|value".into(),
                },
            )
            .unwrap();
        assert!(
            document.source().contains("| **bold** | next\\|value |"),
            "{}",
            document.source()
        );
        assert!(document.source().starts_with("before\n\n"));
        assert!(document.source().ends_with("\nafter\n"));
    }

    #[test]
    fn table_can_insert_first_body_row_without_touching_following_paragraph() {
        use super::MarkdownTableEdit;
        let mut document = MarkdownDocument::new("| A |\n| --- |\n\nnext\n");
        let table = document.tables().remove(0);
        document
            .edit_table(
                table.id,
                MarkdownTableEdit::InsertRow {
                    index: 0,
                    cells: vec!["first".into()],
                },
            )
            .unwrap();
        assert_eq!(document.source(), "| A |\n| --- |\n| first |\n\nnext\n");
    }
    #[test]
    fn set_cell_replaces_inline_markup_with_plain_text_by_contract() {
        let mut document = MarkdownDocument::new("| A |\n| --- |\n| **bold** |\n");
        let table_id = document.tables()[0].id;
        document
            .edit_table(
                table_id,
                MarkdownTableEdit::SetCell {
                    row: 1,
                    column: 0,
                    text: "plain".into(),
                },
            )
            .unwrap();
        assert!(document.source().contains("| plain |"));
        assert!(!document.source().contains("**bold**"));
    }
    #[test]
    fn table_can_fill_empty_cell_without_changing_neighbors() {
        use super::MarkdownTableEdit;
        let mut document = MarkdownDocument::new("| A | B |\n| --- | --- |\n| | value |\n");
        let table = document.tables().remove(0);
        document
            .edit_table(
                table.id,
                MarkdownTableEdit::SetCell {
                    row: 1,
                    column: 0,
                    text: "filled".into(),
                },
            )
            .unwrap();
        assert!(
            document.source().contains("| filled | value |"),
            "{}",
            document.source()
        );
    }

    #[test]
    fn table_append_after_unterminated_last_row_creates_separate_row() {
        use super::MarkdownTableEdit;
        let mut document = MarkdownDocument::new("| A |\n| --- |\n| old |");
        let table = document.tables().remove(0);
        document
            .edit_table(
                table.id,
                MarkdownTableEdit::InsertRow {
                    index: 1,
                    cells: vec!["new".into()],
                },
            )
            .unwrap();
        assert_eq!(document.source(), "| A |\n| --- |\n| old |\n| new |\n");
        assert_eq!(document.tables()[0].rows.len(), 3);
    }
    #[test]
    fn table_columns_keep_existing_markup_and_escape_pipes() {
        let mut document = MarkdownDocument::new(
            "before\n\n| H | V |\n| :--- | ---: |\n| **bold** | `a|b` |\n\nafter\n",
        );
        let id = document.tables()[0].id;
        document
            .edit_table(
                id,
                MarkdownTableEdit::InsertColumn {
                    index: 1,
                    cells: vec!["new|head".into(), "x|y".into()],
                },
            )
            .unwrap();
        assert_eq!(
            document.source(),
            "before\n\n| H | new\\|head | V |\n| :--- | --- | ---: |\n| **bold** | x\\|y | `a|b` |\n\nafter\n"
        );
        assert_eq!(document.tables()[0].alignments.len(), 3);
        let id = document.tables()[0].id;
        document
            .edit_table(id, MarkdownTableEdit::DeleteColumn { index: 1 })
            .unwrap();
        assert_eq!(
            document.source(),
            "before\n\n| H | V |\n| :--- | ---: |\n| **bold** | `a|b` |\n\nafter\n"
        );
    }

    #[test]
    fn table_column_at_eof_and_invalid_indices_do_not_change_source() {
        let mut document = MarkdownDocument::new("| A | B |\n| --- | --- |\n| a\\|b | last |");
        let id = document.tables()[0].id;
        for edit in [
            MarkdownTableEdit::InsertColumn {
                index: 3,
                cells: vec!["x".into(), "y".into()],
            },
            MarkdownTableEdit::InsertColumn {
                index: 1,
                cells: vec!["x".into()],
            },
            MarkdownTableEdit::DeleteColumn { index: 2 },
            MarkdownTableEdit::MoveRow { from: 1, to: 0 },
        ] {
            let expected = if matches!(edit, MarkdownTableEdit::MoveRow { .. }) {
                MarkdownTableEditError::RowOutOfBounds
            } else {
                MarkdownTableEditError::ColumnOutOfBounds
            };
            assert_eq!(document.edit_table(id, edit), Err(expected));
        }
        assert_eq!(
            document.source(),
            "| A | B |\n| --- | --- |\n| a\\|b | last |"
        );
        document
            .edit_table(
                id,
                MarkdownTableEdit::InsertColumn {
                    index: 2,
                    cells: vec!["C".into(), "end|value".into()],
                },
            )
            .unwrap();
        assert_eq!(
            document.source(),
            "| A | B | C |\n| --- | --- | --- |\n| a\\|b | last | end\\|value |"
        );
    }

    #[test]
    fn table_cannot_delete_last_column() {
        let mut document = MarkdownDocument::new("| A |\n| --- |\n| value |");
        let id = document.tables()[0].id;
        assert_eq!(
            document.edit_table(id, MarkdownTableEdit::DeleteColumn { index: 0 }),
            Err(MarkdownTableEditError::ColumnOutOfBounds)
        );
        assert_eq!(document.source(), "| A |\n| --- |\n| value |");
    }

    #[test]
    fn table_body_reorder_preserves_source_and_separates_eof_row() {
        let mut document = MarkdownDocument::new("| A |\n| --- |\n| **first** |\n| `second` |");
        let id = document.tables()[0].id;
        document
            .edit_table(id, MarkdownTableEdit::MoveRow { from: 1, to: 0 })
            .unwrap();
        assert_eq!(
            document.source(),
            "| A |\n| --- |\n| `second` |\n| **first** |\n"
        );
        assert_eq!(document.tables()[0].rows.len(), 3);
    }
    #[test]
    fn table_column_moves_preserve_unicode_alignment_crlf_and_eof() {
        let source = "before\r\n\r\n| 名称 | 値 | 三 |\r\n| :--- | ---: | :---: |\r\n| 猫\\|犬 | **二** | 終 |";
        let mut document = MarkdownDocument::new(source);
        let id = document.tables()[0].id;
        for (from, to) in [(3, 0), (0, 3), (usize::MAX, 0)] {
            assert_eq!(
                document.edit_table(id, MarkdownTableEdit::MoveColumn { from, to }),
                Err(MarkdownTableEditError::ColumnOutOfBounds)
            );
            assert_eq!(document.source(), source);
        }
        document
            .edit_table(id, MarkdownTableEdit::MoveColumn { from: 0, to: 2 })
            .unwrap();
        assert_eq!(
            document.source(),
            "before\r\n\r\n| 値 | 三 | 名称 |\r\n| ---: | :---: | :--- |\r\n| **二** | 終 | 猫\\|犬 |"
        );
        assert_eq!(
            document.tables()[0].alignments,
            vec![
                super::MarkdownTableAlignment::Right,
                super::MarkdownTableAlignment::Center,
                super::MarkdownTableAlignment::Left
            ]
        );
        let id = document.tables()[0].id;
        document
            .edit_table(id, MarkdownTableEdit::MoveColumn { from: 2, to: 0 })
            .unwrap();
        assert_eq!(document.source(), source);
        document
            .edit_table(id, MarkdownTableEdit::MoveColumn { from: 0, to: 0 })
            .unwrap();
        assert_eq!(document.source(), source);
    }

    #[test]
    fn table_column_parts_preserve_code_runs_and_optional_edges() {
        for line in [
            "| 猫\\|犬 | ``a`|b`` | `x|y` |",
            "猫\\|犬 | ``a`|b`` | `x|y`",
            "  | 猫\\|犬 | ``a`|b`` | `x|y` |  ",
        ] {
            let (mut parts, leading) = super::table_column_parts(line, 3).unwrap();
            let original = parts.clone();
            let moved = parts.remove(leading);
            parts.insert(leading + 2, moved);
            assert_eq!(parts[leading], original[leading + 1]);
            assert_eq!(parts[leading + 2], original[leading]);
            let moved = parts.remove(leading + 2);
            parts.insert(leading, moved);
            assert_eq!(parts.join("|"), line);
        }
        assert_eq!(
            super::table_column_parts("| a | b |", 3),
            Err(MarkdownTableEditError::ColumnOutOfBounds)
        );
    }

    #[test]
    fn table_column_move_rejects_ragged_rows_without_partial_mutation() {
        let source = "| A | B |\n| --- | --- |\n| only |\n";
        let mut document = MarkdownDocument::new(source);
        let id = document.tables()[0].id;
        assert_eq!(
            document.edit_table(id, MarkdownTableEdit::MoveColumn { from: 0, to: 1 }),
            Err(MarkdownTableEditError::ColumnOutOfBounds)
        );
        assert_eq!(document.source(), source);
    }
}
#[cfg(test)]
mod source_range_tests {
    use super::{Kind, MarkdownDocument};

    #[test]
    fn plain_paragraph_source_range_matches_rendered_text() {
        let source = "first paragraph\n\nsecond paragraph";
        let document = MarkdownDocument::new(source);
        let paragraphs = document
            .blocks
            .iter()
            .filter(|block| matches!(block.kind, Kind::Paragraph))
            .collect::<Vec<_>>();
        assert_eq!(paragraphs.len(), 2);
        for block in paragraphs {
            let mut end = block.source_range.end;
            while end > block.source_range.start && source.as_bytes()[end - 1] == b'\n' {
                end -= 1;
            }
            assert_eq!(&source[block.source_range.start..end], block.text.as_ref());
        }
    }
}

#[cfg(test)]
mod metadata_tests {
    use super::*;

    #[test]
    fn yaml_fields_preserve_source_ranges() {
        let source = "---\ntitle:  Hello: world  \nname: café\n# ignored\n...\n\nbody: separate\n";
        let document = MarkdownDocument::new(source);
        let metadata = document.metadata().unwrap();
        assert_eq!(metadata.kind, MetadataBlockKind::YamlStyle);
        assert_eq!(
            &source[metadata.source_range.clone()],
            "---\ntitle:  Hello: world  \nname: café\n# ignored\n..."
        );
        assert_eq!(
            metadata
                .fields
                .iter()
                .map(|field| (field.key, field.value))
                .collect::<Vec<_>>(),
            [("title", "Hello: world"), ("name", "café")]
        );
        assert_eq!(
            &source[metadata.fields[0].source_range.clone()],
            "title:  Hello: world  \n"
        );
        assert_eq!(
            &source[metadata.fields[1].source_range.clone()],
            "name: café\n"
        );
        assert!(metadata.diagnostics.is_empty());
        assert_eq!(document.source(), source);
    }

    #[test]
    fn pluses_fields_do_not_evaluate_values() {
        let source = "+++\ntitle = \"Demo\"\r\nflag = true\r\n+++\r\n\r\nbody\n";
        let document = MarkdownDocument::new(source);
        let metadata = document.metadata().unwrap();
        assert_eq!(metadata.kind, MetadataBlockKind::PlusesStyle);
        assert_eq!(
            metadata
                .fields
                .iter()
                .map(|field| (field.key, field.value))
                .collect::<Vec<_>>(),
            [("title", "\"Demo\""), ("flag", "true")]
        );
        assert_eq!(
            &source[metadata.fields[0].source_range.clone()],
            "title = \"Demo\"\r\n"
        );
        assert!(metadata.diagnostics.is_empty());
    }

    #[test]
    fn malformed_lines_and_boundary_do_not_leak_into_fields() {
        let source = "---\ntitle: good\n bad: indented\nmissing delimiter\nempty:\n---\n\nbody: not a field\n";
        let mut document = MarkdownDocument::new(source);
        let metadata = document.metadata().unwrap();
        assert_eq!(metadata.fields.len(), 1);
        assert_eq!(metadata.fields[0].value, "good");
        assert_eq!(
            metadata
                .diagnostics
                .iter()
                .map(|issue| &source[issue.source_range.clone()])
                .collect::<Vec<_>>(),
            [" bad: indented\n", "missing delimiter\n", "empty:\n"]
        );
        document.set_source("---\ntitle: changed\n---\n");
        assert!(
            document.metadata().is_none(),
            "dirty source must not use stale block ranges"
        );
        document.flush();
        assert_eq!(document.metadata().unwrap().fields[0].value, "changed");
        assert!(
            MarkdownDocument::new("---\ntitle: never closed\n")
                .metadata()
                .is_none()
        );
        assert!(
            MarkdownDocument::new("body first\n\n---\ntitle: later\n---\n")
                .metadata()
                .is_none()
        );
    }
}

#[cfg(test)]
mod table_crlf_tests {
    use super::{MarkdownDocument, MarkdownTableEdit};

    #[test]
    fn row_insertion_and_eof_reordering_keep_crlf() {
        let mut document = MarkdownDocument::new("| A |\r\n| --- |\r\n| 一 |\r\n| 二 |");
        let id = document.tables()[0].id;
        document
            .edit_table(id, MarkdownTableEdit::MoveRow { from: 1, to: 0 })
            .unwrap();
        assert_eq!(
            document.source(),
            "| A |\r\n| --- |\r\n| 二 |\r\n| 一 |\r\n"
        );
        let id = document.tables()[0].id;
        document
            .edit_table(
                id,
                MarkdownTableEdit::InsertRow {
                    index: 1,
                    cells: vec!["三".into()],
                },
            )
            .unwrap();
        assert_eq!(
            document.source(),
            "| A |\r\n| --- |\r\n| 二 |\r\n| 三 |\r\n| 一 |\r\n"
        );
    }
}

#[cfg(test)]
mod table_roundtrip_tests {
    use super::{MarkdownDocument, MarkdownTableEdit, MarkdownTableEditError};

    #[test]
    fn nested_rows_insert_delete_and_move_complete_physical_lines() {
        for (header, prefix) in [("> ", "> "), ("- ", "  "), ("> - ", ">   ")] {
            for newline in ["\n", "\r\n"] {
                let head = format!("{header}| A |{newline}{prefix}| --- |{newline}");
                let source = format!("{head}{prefix}| **first** |{newline}{prefix}| `last` |");
                let mut document = MarkdownDocument::new(&source);
                let id = document.tables()[0].id;
                document
                    .edit_table(
                        id,
                        MarkdownTableEdit::InsertRow {
                            index: 1,
                            cells: vec!["middle".into()],
                        },
                    )
                    .unwrap();
                assert_eq!(
                    document.source(),
                    format!(
                        "{head}{prefix}| **first** |{newline}{prefix}| middle |{newline}{prefix}| `last` |"
                    )
                );
                assert_eq!(document.tables()[0].rows.len(), 4);
                let id = document.tables()[0].id;
                document
                    .edit_table(id, MarkdownTableEdit::DeleteRow { index: 1 })
                    .unwrap();
                assert_eq!(document.source(), source);
                let id = document.tables()[0].id;
                document
                    .edit_table(id, MarkdownTableEdit::MoveRow { from: 1, to: 0 })
                    .unwrap();
                assert_eq!(
                    document.source(),
                    format!("{head}{prefix}| `last` |{newline}{prefix}| **first** |{newline}")
                );
                assert_eq!(document.tables()[0].rows.len(), 3);
                let id = document.tables()[0].id;
                document
                    .edit_table(id, MarkdownTableEdit::DeleteRow { index: 0 })
                    .unwrap();
                assert_eq!(
                    document.source(),
                    format!("{head}{prefix}| **first** |{newline}")
                );
            }
        }
    }

    #[test]
    fn nested_row_insertions_preserve_prefixes_at_every_boundary() {
        for (header, prefix) in [("> ", "> "), ("- ", "  "), ("> - ", ">   ")] {
            for newline in ["\n", "\r\n"] {
                for trailing in ["", newline] {
                    let head = format!("{header}| A |{newline}{prefix}| --- |");
                    for body in [false, true] {
                        let source = if body {
                            format!("{head}{newline}{prefix}| old |{trailing}")
                        } else {
                            format!("{head}{trailing}")
                        };
                        for index in 0..=usize::from(body) {
                            let mut document = MarkdownDocument::new(&source);
                            let id = document.tables()[0].id;
                            document
                                .edit_table(
                                    id,
                                    MarkdownTableEdit::InsertRow {
                                        index,
                                        cells: vec!["new".into()],
                                    },
                                )
                                .unwrap();
                            let expected = if !body {
                                format!("{head}{newline}{prefix}| new |{newline}")
                            } else if index == 0 {
                                format!(
                                    "{head}{newline}{prefix}| new |{newline}{prefix}| old |{trailing}"
                                )
                            } else {
                                format!(
                                    "{head}{newline}{prefix}| old |{newline}{prefix}| new |{newline}"
                                )
                            };
                            assert_eq!(document.source(), expected);
                            assert_eq!(document.tables()[0].rows.len(), 2 + usize::from(body));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn deleting_to_one_column_keeps_explicit_table_pipes() {
        for prefix in ["", "> ", "  "] {
            for index in [0, 1] {
                let mut document = MarkdownDocument::new(format!(
                    "{prefix}A | B\r\n{prefix}:--- | ---:\r\n{prefix}a | b"
                ));
                let id = document.tables()[0].id;
                document
                    .edit_table(id, MarkdownTableEdit::DeleteColumn { index })
                    .unwrap();
                let table = &document.tables()[0];
                assert_eq!(table.alignments.len(), 1);
                assert_eq!(table.rows.len(), 2);
                assert_eq!(
                    table.rows[0].cells[0].text.as_ref(),
                    if index == 0 { "B" } else { "A" }
                );
                assert_eq!(
                    table.rows[1].cells[0].text.as_ref(),
                    if index == 0 { "b" } else { "a" }
                );
                for line in document.source().lines() {
                    let content = line.strip_prefix(prefix).unwrap().trim();
                    assert!(
                        content.starts_with('|') && content.ends_with('|'),
                        "{line:?}"
                    );
                }
                assert!(document.source().contains("\r\n"));
                assert!(!document.source().ends_with('\n'));
            }
        }
    }

    #[test]
    fn all_cell_writes_roundtrip_markdown_and_entity_introducers_literally() {
        for literal in [
            "**name**",
            "&copy; &#169;",
            "`code`",
            "==highlight==",
            "~~strike~~",
            "[link](https://example.com)",
            "![alt](image)",
            "<user@example.com>",
            "$math$ H~2~O^+^",
            r"slash\pipe|end",
            "---",
            "猫犬",
        ] {
            let mut document = MarkdownDocument::new("| A |\n| --- |\n| old |\n");
            let id = document.tables()[0].id;
            document
                .edit_table(
                    id,
                    MarkdownTableEdit::SetCell {
                        row: 1,
                        column: 0,
                        text: literal.into(),
                    },
                )
                .unwrap();
            assert_eq!(document.tables()[0].rows[1].cells[0].text.as_ref(), literal);
            let id = document.tables()[0].id;
            document
                .edit_table(
                    id,
                    MarkdownTableEdit::InsertRow {
                        index: 1,
                        cells: vec![literal.into()],
                    },
                )
                .unwrap();
            assert_eq!(document.tables()[0].rows[2].cells[0].text.as_ref(), literal);
            let id = document.tables()[0].id;
            document
                .edit_table(
                    id,
                    MarkdownTableEdit::InsertColumn {
                        index: 1,
                        cells: vec![literal.into(); 3],
                    },
                )
                .unwrap();
            for row in &document.tables()[0].rows {
                assert_eq!(row.cells[1].text.as_ref(), literal);
            }
        }
    }
    #[test]
    fn two_column_removal_preserves_table_with_and_without_outer_pipes() {
        for source in [
            "A | B\r\n:--- | ---:\r\na | b",
            "| A | B |\n| :--- | ---: |\n| a | b |",
            "> A | B\n> :--- | ---:\n> a | b",
        ] {
            for index in [0, 1] {
                let mut document = MarkdownDocument::new(source);
                let id = document.tables()[0].id;
                document
                    .edit_table(id, MarkdownTableEdit::DeleteColumn { index })
                    .unwrap();
                assert_eq!(document.tables()[0].alignments.len(), 1, "{source:?}");
                assert!(document.source().contains('|'), "{source:?}");
            }
        }
    }

    #[test]
    fn set_cell_materializes_missing_ragged_cells_without_losing_source() {
        for (header, prefix) in [("", ""), ("> ", "> "), ("- ", "  "), ("> - ", ">   ")] {
            for (body, expected) in [
                ("| **kept** |", "| **kept** | | new\\*value |"),
                ("**kept**", "**kept**| | new\\*value "),
                ("|**kept**||", "|**kept**|| new\\*value |"),
            ] {
                for trailing in ["", "\r\n"] {
                    let head = format!("{header}| A | B | C |\r\n{prefix}| --- | --- | --- |\r\n");
                    let mut document =
                        MarkdownDocument::new(format!("{head}{prefix}{body}{trailing}"));
                    let id = document.tables()[0].id;
                    document
                        .edit_table(
                            id,
                            MarkdownTableEdit::SetCell {
                                row: 1,
                                column: 2,
                                text: "new*value".into(),
                            },
                        )
                        .unwrap();
                    assert_eq!(
                        document.source(),
                        format!("{head}{prefix}{expected}{trailing}")
                    );
                    let table = &document.tables()[0];
                    assert_eq!(table.rows[1].cells[0].text.as_ref(), "kept");
                    assert_eq!(table.rows[1].cells[1].text.as_ref(), "");
                    assert_eq!(table.rows[1].cells[2].text.as_ref(), "new*value");
                }
            }
        }
    }

    #[test]
    fn nested_table_errors_leave_every_physical_line_unchanged() {
        let source = "> | A | B |\r\n> | --- | --- |\r\n> | only |\r\n";
        let mut document = MarkdownDocument::new(source);
        let id = document.tables()[0].id;
        for edit in [
            MarkdownTableEdit::InsertColumn {
                index: 1,
                cells: vec!["x".into(); 2],
            },
            MarkdownTableEdit::DeleteColumn { index: 0 },
            MarkdownTableEdit::MoveColumn { from: 0, to: 1 },
            MarkdownTableEdit::SetCell {
                row: 1,
                column: 2,
                text: "x".into(),
            },
            MarkdownTableEdit::InsertRow {
                index: 0,
                cells: vec!["x".into()],
            },
        ] {
            assert_eq!(
                document.edit_table(id, edit),
                Err(MarkdownTableEditError::ColumnOutOfBounds)
            );
            assert_eq!(document.source(), source);
        }
        for edit in [
            MarkdownTableEdit::DeleteRow { index: 1 },
            MarkdownTableEdit::MoveRow { from: 0, to: 1 },
        ] {
            assert_eq!(
                document.edit_table(id, edit),
                Err(MarkdownTableEditError::RowOutOfBounds)
            );
            assert_eq!(document.source(), source);
        }
    }
}

#[test]
fn literal_table_edge_whitespace_roundtrips_through_markdown() {
    for text in [" ", "  猫  ", "\tcat\t", "\ncat\r\n", " &#32; | "] {
        let mut document = MarkdownDocument::new("| A |\n| --- |\n| old |\n");
        let id = document.tables()[0].id;
        document
            .edit_table(
                id,
                MarkdownTableEdit::SetCell {
                    row: 1,
                    column: 0,
                    text: text.into(),
                },
            )
            .unwrap();
        assert_eq!(
            document.tables()[0].rows[1].cells[0].text.as_ref(),
            text.replace(['\n', '\r'], " ")
        );
    }
}
