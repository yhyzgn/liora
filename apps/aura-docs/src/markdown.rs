use aura_components::{
    Button, Card, CodeBlock as AuraCodeBlock, Container, Menu, MenuMode, Paragraph, Space, Text,
    Title,
};
use aura_core::Config;
use gpui::{
    AnyElement, App, Component, Context, Entity, IntoElement, Render, RenderOnce, SharedString,
    WeakEntity, Window, div, prelude::*, px,
};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

const INTRO_DOC: &str = r###"# Aura Docs

Aura Docs 是 Aura UI 的官方原生文档主程序。它不是网页文档站，也不是 WebView，而是一个运行在 GPUI 原生窗口里的 Rust 应用。

## 目标

- 在原生窗口内展示 Aura UI 的设计理念、组件 API 和使用示例。
- 使用 `pulldown-cmark` 只解析 Markdown AST/Event。
- 把所有内容渲染为 Aura/GPUI 原生元素树。
- 通过 Live Demo 把真实组件直接插入文档流。

> 绝对边界：不引入 HTML、CSS、DOM、WebAssembly、WebView 或跨端转译路径。

## 当前文档能力

- 标题、段落、列表、引用块、分割线。
- 粗体、斜体、删除线、行内代码。
- 代码块语言识别、语法高亮和复制。
- `::AuraDemo{component="Button"}::` 活体组件注入。
"###;

const QUICK_START_DOC: &str = r###"# Quick Start

Aura 是一个 Cargo workspace，组件库和官方应用都在同一个仓库中。

## 常用命令

```shell
# 组件看板
cargo run -p aura-gallery

# 官方原生文档主程序
cargo run -p aura-docs

# 检查两个应用
cargo check -p aura-gallery -p aura-docs
```

## 在应用中初始化 Aura

```rust
use aura_core::init_aura;
use aura_theme::Theme;

fn main() {
    gpui_platform::application().run(|cx| {
        init_aura(cx, Theme::light());
        // open_window(...)
    });
}
```

## 使用组件

```rust
use aura_components::{Button, CodeBlock, Space, Title};

Space::new()
    .vertical()
    .gap_lg()
    .child(Title::new("Aura UI").h2())
    .child(Button::new("Primary").primary())
    .child(CodeBlock::new("cargo run -p aura-docs").shell());
```
"###;

const ARCHITECTURE_DOC: &str = r###"# Native Architecture

Aura Docs 的核心原则是“文档也是原生应用”。Markdown 只是输入格式，最终输出必须是 Aura/GPUI 节点。

## Workspace 边界

- `crates/aura-components`：所有可复用 UI 组件。
- `apps/aura-gallery`：组件看板，用于展示组件交互效果。
- `apps/aura-docs`：官方文档主程序，负责 Markdown 文档渲染和活体组件注入。

## 文档渲染流水线

1. `pulldown-cmark` 读取 Markdown 文本并产生事件。
2. Renderer 使用 `Vec` 栈管理块级结构。
3. Inline 样式通过上下文状态记录。
4. 文本片段交给 `Paragraph` / `Text` 渲染为 `StyledText`。
5. 代码块交给 `CodeBlock` 组件。
6. Live Demo 标记转换为真实 Aura 组件。

```rust
pub fn render_markdown(md_text: &str) -> gpui::AnyElement {
    Component::new(MarkdownDocument::parse(md_text)).into_any_element()
}
```

## 为什么不使用 Web 文档站

- Aura 的目标运行时是 GPUI 原生窗口。
- 文档系统必须反向验证组件库自己的排版、滚动、文本和交互能力。
- Live Demo 必须是真实组件，而不是截图、iframe 或转译产物。
"###;

const TYPOGRAPHY_DOC: &str = r###"# Typography

Aura Typography 可以把多个不同样式的文本片段合成为同一个 `StyledText` 流。

这意味着 **strong**、*emphasis*、~~strike~~ 和 `inline code` 可以在同一段落内自动折行，而不是拆成多个独立块。

## Text

`Text` 用于描述一段文字及其样式：颜色、背景、字号、字重、斜体、下划线、删除线和等宽字体。

## Paragraph

`Paragraph` 接收一个或多个 `Text` 片段，并把它们拼接为单个 GPUI `StyledText`。

```rust
Paragraph::new()
    .child(Text::new("Normal "))
    .child(Text::new("Bold").bold())
    .child(Text::new(" code ").code_style(theme));
```

## 自举意义

文档渲染不实现独立排版引擎，而是依赖 Aura 自己的 Typography 组件。这样文档能力和组件库能力会同步成长。
"###;

const CODE_BLOCK_DOC: &str = r###"# CodeBlock

`CodeBlock` 是 Aura 的原生代码显示控件，用于展示代码片段、语言标签和复制按钮。

## 能力

- 块级代码显示。
- 行内代码显示。
- 语言标识：Rust、TOML、JSON、Markdown、Shell、TypeScript、JavaScript。
- `syntect` 语法高亮。
- 复制按钮：使用 GPUI clipboard API。
- 横向滚动：长代码不会撑破布局。

## 基础用法

```rust
CodeBlock::new("cargo run -p aura-docs")
    .shell()
    .copyable(true);
```

## 指定语言

```rust
CodeBlock::new(r#"fn main() { println!(\"Aura\"); }"#)
    .language("rust");
```

## 行内格式

```rust
CodeBlock::new("cargo check")
    .shell()
    .inline();
```

## 设计说明

CodeBlock 使用 Rust 原生 `syntect` 解析 Sublime 语法定义和主题，再转换为 GPUI `StyledText` / `TextRun`。高亮能力更完整，但渲染结果仍然是原生 Aura/GPUI 节点。
"###;

const MARKDOWN_DOC: &str = r###"# Markdown Renderer

Aura Docs 的 Markdown renderer 是一个栈式状态机。

## 块级元素

- Heading
- Paragraph
- BlockQuote
- List / Item
- CodeBlock
- Rule

## 内联元素

- Strong
- Emphasis
- Strikethrough
- Inline code

## 状态机核心

```rust
match event {
    Event::Start(tag) => state.start_tag(tag),
    Event::End(tag) => state.end_tag(tag),
    Event::Text(text) => state.push_text_with_live_demos(text.as_ref(), style),
    Event::Code(text) => state.push_inline_code(text.as_ref()),
    Event::Rule => state.push_block(Block::Rule),
    _ => {}
}
```

## 当前边界

Markdown 表格、图片、链接跳转等能力还未作为交互控件完整实现。它们应该继续以 Aura 原生组件方式补齐，而不是引入浏览器能力。
"###;

const LIVE_DEMO_DOC: &str = r###"# Live Demo

Live Demo 是 Aura Docs 区别于静态 Markdown 文档的核心能力。

当 renderer 识别到特殊语法时，不渲染为普通文字，而是创建真实 Aura 组件节点。

```text
::AuraDemo{component="Button"}::
```

下面的按钮不是截图或文本，而是真实的 Aura `Button` 节点：

::AuraDemo{component="Button"}::

## 为什么这样设计

- 文档示例和组件实现不会分叉。
- Hover、click、focus 等交互保留真实行为。
- 文档本身成为组件库的集成测试面。

## 后续扩展方向

- 支持更多组件：`CodeBlock`、`Input`、`Switch`、`Table`。
- 支持 demo 参数：variant、size、disabled、loading。
- 支持 demo 容器：示例区、源码区、说明区。
"###;

const COMPONENT_DOC: &str = r###"# Component Authoring

新增组件时，应先把可复用能力放进 `crates/aura-components`，再在 Gallery 和 Docs 中使用。

## 推荐流程

1. 在 `crates/aura-components/src/<name>.rs` 实现组件。
2. 在 `crates/aura-components/src/lib.rs` 中 `pub mod` 和 `pub use`。
3. 在 `apps/aura-gallery/src/demos/` 添加交互 demo。
4. 如该组件服务文档系统，在 `apps/aura-docs` 中复用它。
5. 添加最小回归测试。

## 示例：CodeBlock

`CodeBlock` 先进入组件库，然后 Aura Docs 的 fenced code block 渲染改为复用该组件。

```rust
fn render_code_block(language: Option<SharedString>, code: SharedString) -> AnyElement {
    let mut code_block = CodeBlock::new(code);
    if let Some(language) = language {
        code_block = code_block.language(language.as_ref());
    }
    code_block.into_any_element()
}
```

## 约束

- Demo 不能绕过组件库重新实现同类 UI。
- Docs 不能维护一套 app-local 组件替代库。
- 公共组件命名遵循当前 ADR：不加 `Aura` 前缀。
"###;

pub struct DocPage {
    pub title: &'static str,
    pub markdown: &'static str,
}

const DOC_PAGES: &[DocPage] = &[
    DocPage {
        title: "Overview",
        markdown: INTRO_DOC,
    },
    DocPage {
        title: "Quick Start",
        markdown: QUICK_START_DOC,
    },
    DocPage {
        title: "Architecture",
        markdown: ARCHITECTURE_DOC,
    },
    DocPage {
        title: "Typography",
        markdown: TYPOGRAPHY_DOC,
    },
    DocPage {
        title: "CodeBlock",
        markdown: CODE_BLOCK_DOC,
    },
    DocPage {
        title: "Markdown",
        markdown: MARKDOWN_DOC,
    },
    DocPage {
        title: "Live Demo",
        markdown: LIVE_DEMO_DOC,
    },
    DocPage {
        title: "Authoring",
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
    LiveDemo {
        component: SharedString,
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
            Event::Text(text) => {
                if matches!(self.stack.last(), Some(Frame::CodeBlock { .. })) {
                    self.push_text(text.as_ref(), self.inline_style);
                } else {
                    self.push_text_with_live_demos(text.as_ref(), self.inline_style);
                }
            }
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

    fn push_text_with_live_demos(&mut self, text: &str, style: InlineStyle) {
        for part in split_live_demo_parts(text) {
            match part {
                TextPart::Text(text) => self.push_text(text, style),
                TextPart::LiveDemo(component) => self.push_live_demo(component),
            }
        }
    }

    fn push_live_demo(&mut self, component: SharedString) {
        let block = Block::LiveDemo { component };

        if let Some(Frame::Paragraph(segments)) = self.stack.last_mut() {
            if !segments.is_empty() {
                let before_demo = std::mem::take(segments);
                self.push_block_to_parent(Block::Paragraph(before_demo));
            }
            self.push_block_to_parent(block);
        } else {
            self.push_block(block);
        }
    }

    fn push_block_to_parent(&mut self, block: Block) {
        if self.stack.len() >= 2 {
            let parent_index = self.stack.len() - 2;
            push_block_into_frame(&mut self.stack[parent_index], block);
        } else {
            self.push_block(block);
        }
    }

    fn push_block(&mut self, block: Block) {
        match self.stack.last_mut() {
            Some(frame) => push_block_into_frame(frame, block),
            None => {}
        }
    }
}

fn push_block_into_frame(frame: &mut Frame, block: Block) {
    match frame {
        Frame::Root(blocks) | Frame::BlockQuote(blocks) | Frame::Item(blocks) => {
            blocks.push(block);
        }
        Frame::List { items, .. } => items.push(vec![block]),
        Frame::Paragraph(_) | Frame::Heading { .. } | Frame::CodeBlock { .. } => {}
    }
}

enum TextPart<'a> {
    Text(&'a str),
    LiveDemo(SharedString),
}

fn split_live_demo_parts(text: &str) -> Vec<TextPart<'_>> {
    const START: &str = "::AuraDemo{";
    const END: &str = "}::";

    let mut parts = Vec::new();
    let mut cursor = 0;

    while let Some(relative_start) = text[cursor..].find(START) {
        let marker_start = cursor + relative_start;
        if marker_start > cursor {
            parts.push(TextPart::Text(&text[cursor..marker_start]));
        }

        let attr_start = marker_start + START.len();
        let Some(relative_end) = text[attr_start..].find(END) else {
            parts.push(TextPart::Text(&text[marker_start..]));
            cursor = text.len();
            break;
        };
        let marker_end = attr_start + relative_end + END.len();
        let attrs = &text[attr_start..attr_start + relative_end];

        if let Some(component) = parse_demo_component(attrs) {
            parts.push(TextPart::LiveDemo(component));
        } else {
            parts.push(TextPart::Text(&text[marker_start..marker_end]));
        }

        cursor = marker_end;
    }

    if cursor < text.len() {
        parts.push(TextPart::Text(&text[cursor..]));
    }

    parts
}

fn parse_demo_component(attrs: &str) -> Option<SharedString> {
    let component_key = "component=\"";
    let start = attrs.find(component_key)? + component_key.len();
    let rest = &attrs[start..];
    let end = rest.find('"')?;
    let component = &rest[..end];

    (!component.is_empty()).then(|| component.to_string().into())
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
            Self::LiveDemo { component } => render_live_demo(component, theme),
            Self::Rule => div()
                .h(px(1.0))
                .w_full()
                .bg(theme.neutral.divider)
                .into_any_element(),
        }
    }
}

fn render_live_demo(component: SharedString, theme: &aura_theme::Theme) -> AnyElement {
    let demo = match component.as_ref() {
        "Button" => Space::new()
            .vertical()
            .gap_sm()
            .child(Text::new("Live Button demo").bold())
            .child(
                Button::new("Native Button")
                    .primary()
                    .on_click(|_, _, _| {}),
            )
            .into_any_element(),
        _ => Paragraph::with_text(format!(
            "Unsupported Aura demo component: {}",
            component.as_ref()
        ))
        .into_any_element(),
    };

    div()
        .rounded(px(theme.radius.md))
        .border_1()
        .border_color(theme.primary.base.opacity(0.35))
        .bg(theme.primary.light_9)
        .p_3()
        .child(Card::new(demo).no_shadow().width_lg())
        .into_any_element()
}

fn render_code_block(
    language: Option<SharedString>,
    code: SharedString,
    _theme: &aura_theme::Theme,
) -> AnyElement {
    let mut code_block = AuraCodeBlock::new(code);
    if let Some(language) = language {
        code_block = code_block.language(language.as_ref());
    }
    code_block.into_any_element()
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
                    .child(Title::new("Aura Docs").h2())
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
    fn docs_shell_registers_core_documentation_pages() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();

        assert!(titles.contains(&"Quick Start"));
        assert!(titles.contains(&"Architecture"));
        assert!(titles.contains(&"CodeBlock"));
        assert!(titles.contains(&"Live Demo"));
        assert!(titles.contains(&"Authoring"));
        assert!(DOC_PAGES.len() >= 8);
    }

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

        assert!(source.contains("AuraCodeBlock::new"));
        assert!(source.contains(".language(language.as_ref())"));
    }

    #[test]
    fn docs_shell_uses_native_container_and_menu() {
        let source = include_str!("markdown.rs");

        assert!(source.contains("Container::new()"));
        assert!(source.contains("Menu::new()"));
        assert!(source.contains(".aside_scroll()"));
        assert!(source.contains(".main_scroll()"));
    }

    #[test]
    fn parses_live_demo_marker_as_real_block() {
        let document =
            MarkdownDocument::parse("Before\n\n::AuraDemo{component=\"Button\"}::\n\nAfter");
        let blocks = document.blocks();

        assert_eq!(blocks.len(), 3);
        assert!(matches!(
            &blocks[1],
            Block::LiveDemo { component } if component.as_ref() == "Button"
        ));
        assert!(
            !blocks.iter().any(|block| {
                matches!(block, Block::Paragraph(segments) if segments.iter().any(|segment| segment.text.as_ref().contains("::AuraDemo")))
            }),
            "live demo marker should not remain as literal paragraph text"
        );
    }

    #[test]
    fn splits_live_demo_markers_from_surrounding_text() {
        let parts = split_live_demo_parts("A ::AuraDemo{component=\"Button\"}:: B");

        assert_eq!(parts.len(), 3);
        assert!(matches!(parts[0], TextPart::Text("A ")));
        assert!(
            matches!(&parts[1], TextPart::LiveDemo(component) if component.as_ref() == "Button")
        );
        assert!(matches!(parts[2], TextPart::Text(" B")));
    }

    #[test]
    fn live_demo_renderer_maps_button_to_native_aura_component() {
        let source = include_str!("markdown.rs");

        assert!(source.contains("Block::LiveDemo"));
        assert!(source.contains("Button::new(\"Native Button\")"));
        assert!(source.contains(".on_click(|_, _, _| {})"));
    }
}
