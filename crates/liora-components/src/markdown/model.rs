use gpui::SharedString;
use pulldown_cmark::{Alignment, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
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
        let mut old = HashMap::new();
        collect_ids(&self.blocks, &mut old);
        assign_ids(&mut blocks, &old);
        self.blocks = blocks;
        self.dirty = false;
        self.parse_count += 1;
        true
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

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Kind {
    Paragraph,
    Heading(u8),
    Quote,
    List(Option<u64>),
    Item,
    Code(SharedString),
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
    pub image: Option<SharedString>,
}

pub(super) struct Span {
    pub range: Range<usize>,
    pub style: Style,
}
pub(super) struct ImageOccurrence {
    pub destination: SharedString,
    pub range: Range<usize>,
}
pub(super) struct Block {
    pub id: u64,
    pub start: usize,
    pub kind: Kind,
    pub text: SharedString,
    buffer: String,
    buffer_start: usize,
    pub spans: Vec<Span>,
    pub images: Vec<ImageOccurrence>,
    pub children: Vec<Block>,
}

impl Block {
    fn new(kind: Kind, start: usize) -> Self {
        Self {
            id: 0,
            start,
            kind,
            text: SharedString::default(),
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
        paragraph.text = std::mem::take(&mut self.buffer).into();
        paragraph.spans = std::mem::take(&mut self.spans);
        paragraph.images = std::mem::take(&mut self.images);
        self.children.push(paragraph);
    }
}

fn key(block: &Block) -> (usize, u8) {
    let kind = match block.kind {
        Kind::Paragraph => 0,
        Kind::Heading(_) => 1,
        Kind::Quote => 2,
        Kind::List(_) => 3,
        Kind::Item => 4,
        Kind::Code(_) => 5,
        Kind::Table(_) => 6,
        Kind::Row(_) => 7,
        Kind::Cell => 8,
        Kind::Rule => 9,
        Kind::Container => 10,
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

// Bound every later recursive walk and drop, while flattening deeper text visibly.
const MAX_BLOCK_DEPTH: usize = 64;

fn parse(source: &str) -> Vec<Block> {
    let mut stack = vec![Block::new(Kind::Container, 0)];
    let mut styles = vec![Style::default()];
    let mut flattened_depth = 0usize;
    let mut separator_pending = false;
    let mut image_starts = Vec::new();
    let options =
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
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
                    Tag::BlockQuote(_) => Some(Kind::Quote),
                    Tag::List(start) => Some(Kind::List(start)),
                    Tag::Item => Some(Kind::Item),
                    Tag::CodeBlock(kind) => Some(Kind::Code(match kind {
                        CodeBlockKind::Indented => SharedString::default(),
                        CodeBlockKind::Fenced(info) => info
                            .split_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_owned()
                            .into(),
                    })),
                    Tag::Table(alignments) => Some(Kind::Table(alignments)),
                    Tag::TableHead => Some(Kind::Row(true)),
                    Tag::TableRow => Some(Kind::Row(false)),
                    Tag::TableCell => Some(Kind::Cell),
                    _ => Some(Kind::Container),
                };
                if let Some(kind) = kind {
                    if stack.len() <= MAX_BLOCK_DEPTH && flattened_depth == 0 {
                        stack.last_mut().unwrap().flush_item_inline();
                        stack.push(Block::new(kind, range.start));
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
                    block.flush_item_inline();
                    block.text = std::mem::take(&mut block.buffer).into();
                    stack.last_mut().unwrap().children.push(block);
                }
            }
            Event::Rule => {
                let block = stack.last_mut().unwrap();
                block.flush_item_inline();
                block.children.push(Block::new(Kind::Rule, range.start));
            }
            event => {
                let range_start = range.start;
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
                    Event::TaskListMarker(checked) => if checked { "☑ " } else { "☐ " }.to_owned(),
                    _ => source[range].to_owned(),
                };
                let block = stack.last_mut().unwrap();
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
                            style: Style::default(),
                        });
                    }
                    separator_pending = false;
                }
                let start = block.buffer.len();
                block.buffer.push_str(&text);
                block.spans.push(Span {
                    range: start..start + text.len(),
                    style,
                });
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
}
