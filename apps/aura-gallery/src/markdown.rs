use aura_components::{Button, Card, Container, Menu, MenuMode, Paragraph, Space, Text, Title};
use aura_core::Config;
use gpui::{
    AnyElement, App, Component, Context, Entity, IntoElement, Render, RenderOnce, SharedString,
    WeakEntity, Window, div, prelude::*, px,
};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

const INTRO_DOC: &str = r#"# Aura Gallery

Aura Gallery 是 Aura UI 的原生文档与组件展示大屏。Markdown 只作为输入格式，最终结果全部映射为 GPUI 原生元素树。

## Native bootstrapping

- 使用 `pulldown-cmark` 解析 Markdown 事件。
- 使用 Aura `Paragraph` 和 `Text` 自举富文本渲染。
- 使用原生 `Container` / `Menu` 构建双栏文档窗口。

> 所有内容仍然运行在 GPUI 原生窗口内，不经过 HTML、CSS、DOM 或 WebView。

```rust
Button::new("Primary")
    .primary()
    .on_click(|_, _, _| {
        // Native GPUI interaction
    });
```
"#;

const TYPOGRAPHY_DOC: &str = r#"# Typography

Aura Typography 现在可以把多个不同样式的文本片段合成为同一个 `StyledText` 流。

这意味着 **strong**、*emphasis*、~~strike~~ 和 `inline code` 可以在同一段落内自动折行，而不是拆成多个独立块。

```rust
Paragraph::new()
    .child(Text::new("Normal "))
    .child(Text::new("Bold").bold())
    .child(Text::new(" code ").code_style(theme));
```
"#;

const COMPONENT_DOC: &str = r#"# Component docs

后续 Phase 4 会支持类似 `::AuraDemo{component="Button"}::` 的活体组件注入语法。

当前 Phase 3 先验证文档壳、导航、纵向滚动和代码块横向滚动。

1. 左侧使用 Aura `Menu`。
2. 右侧使用 Markdown renderer。
3. 文档内容保持原生可组合元素。
"#;

pub struct DocPage {
    pub id: &'static str,
    pub title: &'static str,
    pub markdown: &'static str,
}

const DOC_PAGES: &[DocPage] = &[
    DocPage {
        id: "intro",
        title: "Overview",
        markdown: INTRO_DOC,
    },
    DocPage {
        id: "typography",
        title: "Typography",
        markdown: TYPOGRAPHY_DOC,
    },
    DocPage {
        id: "components",
        title: "Components",
        markdown: COMPONENT_DOC,
    },
];

pub fn render_markdown(md_text: &str) -> AnyElement {
    Component::new(MarkdownDocument::parse(md_text)).into_any_element()
}

struct MarkdownDocument {
    blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Block {
    Paragraph(Vec<InlineSegment>),
    Heading {
        level: HeadingLevel,
        content: Vec<InlineSegment>,
    },
    BlockQuote(Vec<Block>),
    List {
        ordered: bool,
        start: u64,
        items: Vec<Vec<Block>>,
    },
    CodeBlock {
        language: Option<SharedString>,
        code: SharedString,
    },
    Rule,
}

#[derive(Debug)]
enum Frame {
    Root(Vec<Block>),
    Paragraph(Vec<InlineSegment>),
    Heading {
        level: HeadingLevel,
        content: Vec<InlineSegment>,
    },
    BlockQuote(Vec<Block>),
    List {
        ordered: bool,
        start: u64,
        items: Vec<Vec<Block>>,
    },
    Item(Vec<Block>),
    CodeBlock {
        language: Option<SharedString>,
        code: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InlineSegment {
    text: SharedString,
    style: InlineStyle,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct InlineStyle {
    strong: bool,
    emphasis: bool,
    strikethrough: bool,
    code: bool,
}

impl MarkdownDocument {
    fn parse(md_text: &str) -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(md_text, options);
        let mut state = ParserState {
            stack: vec![Frame::Root(Vec::new())],
            inline_style: InlineStyle::default(),
        };

        for event in parser {
            state.handle_event(event);
        }

        while state.stack.len() > 1 {
            state.close_top_frame();
        }

        let blocks = match state.stack.pop() {
            Some(Frame::Root(blocks)) => blocks,
            _ => Vec::new(),
        };

        Self { blocks }
    }

    #[cfg(test)]
    fn blocks(&self) -> &[Block] {
        &self.blocks
    }
}

struct ParserState {
    stack: Vec<Frame>,
    inline_style: InlineStyle,
}

impl ParserState {
    fn handle_event(&mut self, event: Event<'_>) {
        match event {
            Event::Start(tag) => self.start_tag(tag),
            Event::End(tag_end) => self.end_tag(tag_end),
            Event::Text(text) => self.push_text(text.as_ref(), self.inline_style),
            Event::Code(text) | Event::InlineMath(text) => {
                let mut style = self.inline_style;
                style.code = true;
                self.push_text(text.as_ref(), style);
            }
            Event::SoftBreak | Event::HardBreak => self.push_text("\n", self.inline_style),
            Event::Rule => self.push_block(Block::Rule),
            Event::TaskListMarker(checked) => {
                self.push_text(if checked { "☑ " } else { "☐ " }, self.inline_style);
            }
            Event::Html(_)
            | Event::InlineHtml(_)
            | Event::DisplayMath(_)
            | Event::FootnoteReference(_) => {}
        }
    }

    fn start_tag(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Paragraph => self.stack.push(Frame::Paragraph(Vec::new())),
            Tag::Heading { level, .. } => self.stack.push(Frame::Heading {
                level,
                content: Vec::new(),
            }),
            Tag::BlockQuote(_) => self.stack.push(Frame::BlockQuote(Vec::new())),
            Tag::List(start) => self.stack.push(Frame::List {
                ordered: start.is_some(),
                start: start.unwrap_or(1),
                items: Vec::new(),
            }),
            Tag::Item => self.stack.push(Frame::Item(Vec::new())),
            Tag::CodeBlock(kind) => {
                let language = match kind {
                    CodeBlockKind::Indented => None,
                    CodeBlockKind::Fenced(info) => info
                        .split_whitespace()
                        .next()
                        .filter(|lang| !lang.is_empty())
                        .map(|lang| SharedString::from(lang.to_string())),
                };
                self.stack.push(Frame::CodeBlock {
                    language,
                    code: String::new(),
                });
            }
            Tag::Emphasis => self.inline_style.emphasis = true,
            Tag::Strong => self.inline_style.strong = true,
            Tag::Strikethrough => self.inline_style.strikethrough = true,
            Tag::Link { .. } | Tag::Image { .. } => {}
            Tag::HtmlBlock
            | Tag::FootnoteDefinition(_)
            | Tag::DefinitionList
            | Tag::DefinitionListTitle
            | Tag::DefinitionListDefinition
            | Tag::Table(_)
            | Tag::TableHead
            | Tag::TableRow
            | Tag::TableCell
            | Tag::Superscript
            | Tag::Subscript
            | Tag::MetadataBlock(_) => {}
        }
    }

    fn end_tag(&mut self, tag_end: TagEnd) {
        match tag_end {
            TagEnd::Paragraph
            | TagEnd::Heading(_)
            | TagEnd::BlockQuote(_)
            | TagEnd::List(_)
            | TagEnd::Item
            | TagEnd::CodeBlock => {
                self.close_top_frame();
            }
            TagEnd::Emphasis => self.inline_style.emphasis = false,
            TagEnd::Strong => self.inline_style.strong = false,
            TagEnd::Strikethrough => self.inline_style.strikethrough = false,
            TagEnd::Link | TagEnd::Image => {}
            TagEnd::HtmlBlock
            | TagEnd::FootnoteDefinition
            | TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition
            | TagEnd::Table
            | TagEnd::TableHead
            | TagEnd::TableRow
            | TagEnd::TableCell
            | TagEnd::Superscript
            | TagEnd::Subscript
            | TagEnd::MetadataBlock(_) => {}
        }
    }

    fn close_top_frame(&mut self) {
        let Some(frame) = self.stack.pop() else {
            return;
        };

        match frame {
            Frame::Root(blocks) => self.stack.push(Frame::Root(blocks)),
            Frame::Paragraph(segments) => {
                if !segments.is_empty() {
                    self.push_block(Block::Paragraph(segments));
                }
            }
            Frame::Heading { level, content } => {
                if !content.is_empty() {
                    self.push_block(Block::Heading { level, content });
                }
            }
            Frame::BlockQuote(blocks) => {
                if !blocks.is_empty() {
                    self.push_block(Block::BlockQuote(blocks));
                }
            }
            Frame::List {
                ordered,
                start,
                items,
            } => {
                if !items.is_empty() {
                    self.push_block(Block::List {
                        ordered,
                        start,
                        items,
                    });
                }
            }
            Frame::Item(blocks) => {
                if let Some(Frame::List { items, .. }) = self.stack.last_mut() {
                    items.push(blocks);
                } else {
                    self.push_block(Block::List {
                        ordered: false,
                        start: 1,
                        items: vec![blocks],
                    });
                }
            }
            Frame::CodeBlock { language, code } => {
                self.push_block(Block::CodeBlock {
                    language,
                    code: code.into(),
                });
            }
        }
    }

    fn push_text(&mut self, text: &str, style: InlineStyle) {
        if text.is_empty() {
            return;
        }

        let segment = InlineSegment {
            text: SharedString::from(text.to_string()),
            style,
        };

        match self.stack.last_mut() {
            Some(Frame::Paragraph(segments)) => segments.push(segment),
            Some(Frame::Heading { content, .. }) => content.push(segment),
            Some(Frame::CodeBlock { code, .. }) => code.push_str(text),
            _ => self.push_block(Block::Paragraph(vec![segment])),
        }
    }

    fn push_block(&mut self, block: Block) {
        match self.stack.last_mut() {
            Some(Frame::Root(blocks))
            | Some(Frame::BlockQuote(blocks))
            | Some(Frame::Item(blocks)) => {
                blocks.push(block);
            }
            Some(Frame::List { items, .. }) => items.push(vec![block]),
            Some(Frame::Paragraph(_))
            | Some(Frame::Heading { .. })
            | Some(Frame::CodeBlock { .. })
            | None => {}
        }
    }
}

impl RenderOnce for MarkdownDocument {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Space::new()
            .vertical()
            .gap_lg()
            .children(self.blocks.into_iter().map(|block| block.render(&theme)))
    }
}

impl Block {
    fn render(self, theme: &aura_theme::Theme) -> AnyElement {
        match self {
            Self::Paragraph(segments) => render_paragraph(segments, theme),
            Self::Heading { level, content } => {
                let heading = Title::new(inline_plain_text(&content));
                match level {
                    HeadingLevel::H1 => heading.h1(),
                    HeadingLevel::H2 => heading.h2(),
                    HeadingLevel::H3 => heading.h3(),
                    HeadingLevel::H4 => heading.h4(),
                    HeadingLevel::H5 => heading.h5(),
                    HeadingLevel::H6 => heading.h6(),
                }
                .into_any_element()
            }
            Self::BlockQuote(blocks) => div()
                .border_l_1()
                .border_color(theme.primary.base)
                .pl_4()
                .text_color(theme.neutral.text_2)
                .child(
                    Space::new()
                        .vertical()
                        .gap_md()
                        .children(blocks.into_iter().map(|block| block.render(theme))),
                )
                .into_any_element(),
            Self::List {
                ordered,
                start,
                items,
            } => render_list(ordered, start, items, theme),
            Self::CodeBlock { language, code } => render_code_block(language, code, theme),
            Self::Rule => div()
                .h(px(1.0))
                .w_full()
                .bg(theme.neutral.divider)
                .into_any_element(),
        }
    }
}

fn render_code_block(
    language: Option<SharedString>,
    code: SharedString,
    theme: &aura_theme::Theme,
) -> AnyElement {
    let mut content = Space::new().vertical().gap_sm();
    if let Some(language) = language {
        content = content.child(
            div()
                .text_xs()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(theme.neutral.text_3)
                .child(language),
        );
    }
    content = content.child(
        div()
            .font_family("Monospace")
            .text_sm()
            .line_height(px(theme.font_size.md * 1.6))
            .text_color(theme.neutral.text_1)
            .whitespace_nowrap()
            .child(code),
    );

    div()
        .id("aura-markdown-code-scroll")
        .overflow_x_scroll()
        .w_full()
        .rounded(px(theme.radius.md))
        .border_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.hover)
        .p_4()
        .child(content)
        .into_any_element()
}

fn render_paragraph(segments: Vec<InlineSegment>, theme: &aura_theme::Theme) -> AnyElement {
    Paragraph::new()
        .children(segments.into_iter().map(|segment| segment.into_text(theme)))
        .into_any_element()
}

pub fn render_docs_shell(cx: &mut App) -> Entity<DocsShell> {
    cx.new(|_| DocsShell {
        selected: 0,
        nav_menu: None,
    })
}

pub struct DocsShell {
    selected: usize,
    nav_menu: Option<Entity<Menu>>,
}

impl Render for DocsShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected.min(DOC_PAGES.len().saturating_sub(1));
        self.selected = selected;

        let nav_menu = self.nav_menu(selected, cx);
        let page = &DOC_PAGES[selected];

        Container::new()
            .header(
                Space::new()
                    .vertical()
                    .gap_xs()
                    .child(Title::new("Aura Gallery Docs").h2())
                    .child(Text::new(
                        "Native Markdown · GPUI elements · Aura components",
                    )),
            )
            .header_height_lg()
            .aside(nav_menu)
            .aside_width_lg()
            .aside_scroll()
            .main_scroll()
            .main_padding_xl()
            .child(
                Card::new(
                    Space::new()
                        .vertical()
                        .gap_lg()
                        .child(render_markdown(page.markdown))
                        .child(
                            Button::new("Native action")
                                .primary()
                                .on_click(|_, _, _| {}),
                        ),
                )
                .no_shadow()
                .no_shrink(),
            )
    }
}

impl DocsShell {
    fn nav_menu(&mut self, selected: usize, cx: &mut Context<Self>) -> Entity<Menu> {
        if let Some(nav_menu) = &self.nav_menu {
            return nav_menu.clone();
        }

        let docs = cx.entity().downgrade();
        let nav_menu = cx.new(move |_| build_docs_menu(selected, docs));
        self.nav_menu = Some(nav_menu.clone());
        nav_menu
    }
}

fn build_docs_menu(selected: usize, docs: WeakEntity<DocsShell>) -> Menu {
    let mut menu = Menu::new()
        .id("aura-docs-menu")
        .mode(MenuMode::Vertical)
        .default_active(selected.to_string())
        .on_select(move |id, _, cx| {
            let Ok(index) = id.parse::<usize>() else {
                return;
            };
            let _ = docs.update(cx, |docs, cx| {
                docs.selected = index;
                cx.notify();
            });
        });

    for (index, page) in DOC_PAGES.iter().enumerate() {
        menu = menu.item(index.to_string(), page.title, None);
    }

    menu
}

fn render_list(
    ordered: bool,
    start: u64,
    items: Vec<Vec<Block>>,
    theme: &aura_theme::Theme,
) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .children(items.into_iter().enumerate().map(|(index, item_blocks)| {
            let marker = if ordered {
                format!("{}.", start + index as u64)
            } else {
                "•".to_string()
            };

            div()
                .flex()
                .flex_row()
                .items_start()
                .gap_2()
                .child(
                    div()
                        .w(px(24.0))
                        .text_color(theme.neutral.text_3)
                        .child(marker),
                )
                .child(
                    div().flex_1().child(
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .children(item_blocks.into_iter().map(|block| block.render(theme))),
                    ),
                )
        }))
        .into_any_element()
}

fn inline_plain_text(segments: &[InlineSegment]) -> SharedString {
    segments
        .iter()
        .map(|segment| segment.text.as_ref())
        .collect::<String>()
        .into()
}

impl InlineSegment {
    fn into_text(self, theme: &aura_theme::Theme) -> Text {
        let mut text = Text::new(self.text);

        if self.style.code {
            text = text.code_style(theme);
        }

        if self.style.strong {
            text = text.bold();
        }

        if self.style.emphasis {
            text = text.italic();
        }

        if self.style.strikethrough {
            text = text.strikethrough();
        }

        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_markdown_entrypoint_returns_native_element() {
        let _ = render_markdown("# Aura\n\nNative docs");
    }

    #[test]
    fn parses_heading_and_mixed_inline_paragraph_segments() {
        let document =
            MarkdownDocument::parse("# Aura\n\nHello **bold** and *italic* with `code`.");
        let blocks = document.blocks();

        assert_eq!(blocks.len(), 2);
        assert!(matches!(
            &blocks[0],
            Block::Heading {
                level: HeadingLevel::H1,
                ..
            }
        ));

        let Block::Paragraph(segments) = &blocks[1] else {
            panic!("expected paragraph");
        };

        assert!(segments.iter().any(|segment| segment.style.strong));
        assert!(segments.iter().any(|segment| segment.style.emphasis));
        assert!(segments.iter().any(|segment| segment.style.code));
    }

    #[test]
    fn parses_unordered_and_ordered_lists_with_nested_blocks() {
        let document = MarkdownDocument::parse("- One\n- Two\n\n3. Three\n4. Four");
        let blocks = document.blocks();

        assert_eq!(blocks.len(), 2);
        assert!(matches!(
            &blocks[0],
            Block::List {
                ordered: false,
                items,
                ..
            } if items.len() == 2
        ));
        assert!(matches!(
            &blocks[1],
            Block::List {
                ordered: true,
                start: 3,
                items,
            } if items.len() == 2
        ));
    }

    #[test]
    fn parses_blockquote_as_nested_block_stack() {
        let document = MarkdownDocument::parse("> Quote **strong**");
        let [Block::BlockQuote(children)] = document.blocks() else {
            panic!("expected one blockquote");
        };

        assert_eq!(children.len(), 1);
        assert!(matches!(&children[0], Block::Paragraph(_)));
    }

    #[test]
    fn parses_fenced_code_block_with_language() {
        let document = MarkdownDocument::parse("```rust\nlet answer = 42;\n```");
        let [Block::CodeBlock { language, code }] = document.blocks() else {
            panic!("expected one code block");
        };

        assert_eq!(language.as_ref().map(SharedString::as_ref), Some("rust"));
        assert_eq!(code.as_ref(), "let answer = 42;\n");
    }

    #[test]
    fn code_blocks_render_with_horizontal_scroll_shell() {
        let source = include_str!("markdown.rs");

        assert!(source.contains(".overflow_x_scroll()"));
        assert!(source.contains(".font_family(\"Monospace\")"));
        assert!(source.contains(".whitespace_nowrap()"));
    }

    #[test]
    fn docs_shell_uses_native_container_and_menu() {
        let source = include_str!("markdown.rs");
        let registry = include_str!("demos/mod.rs");

        assert!(source.contains("Container::new()"));
        assert!(source.contains("Menu::new()"));
        assert!(source.contains(".aside_scroll()"));
        assert!(source.contains(".main_scroll()"));
        assert!(registry.contains("Docs 原生文档"));
    }
}
