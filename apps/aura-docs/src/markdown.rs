use aura_components::{
    Alert, AlertType, Autocomplete, AutocompleteItem, Avatar, Badge, BadgeType, Button, Card,
    Checkbox, CheckboxGroup, CodeBlock as AuraCodeBlock, Container, Input, InputNumber,
    InputNumberControlsPosition, Menu, MenuMode, Paragraph, Radio, RadioGroup, Rate, Select,
    Slider, Space, Switch, Tag as AuraTag, Text, Textarea, Title, VirtualScrollbar, toast_error,
    toast_info, toast_success, toast_warning,
};
use aura_core::{Config, PassivePortal, Portal};
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, AnyView, App, Component, Context, Entity, IntoElement, ListAlignment, ListState,
    Render, RenderOnce, SharedString, WeakEntity, Window, div, list, prelude::*, px,
};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

const INTRO_DOC: &str = include_str!("../content/pages/overview.md");
const QUICK_START_DOC: &str = include_str!("../content/pages/quick_start.md");
const ARCHITECTURE_DOC: &str = include_str!("../content/pages/architecture.md");
const ABOUT_DOC: &str = include_str!("../content/pages/about.md");

const AFFIX_DOC: &str = include_str!("../content/pages/affix.md");
const ALERT_DOC: &str = include_str!("../content/pages/alert.md");
const ANCHOR_DOC: &str = include_str!("../content/pages/anchor.md");
const AUTOCOMPLETE_DOC: &str = include_str!("../content/pages/autocomplete.md");
const AVATAR_DOC: &str = include_str!("../content/pages/avatar.md");
const BACKTOP_DOC: &str = include_str!("../content/pages/backtop.md");
const BADGE_DOC: &str = include_str!("../content/pages/badge.md");
const BREADCRUMB_DOC: &str = include_str!("../content/pages/breadcrumb.md");
const BUTTON_DOC: &str = include_str!("../content/pages/button.md");
const CARD_DOC: &str = include_str!("../content/pages/card.md");
const CASCADER_DOC: &str = include_str!("../content/pages/cascader.md");
const CHECKBOX_DOC: &str = include_str!("../content/pages/checkbox.md");
const CODE_BLOCK_DOC: &str = include_str!("../content/pages/code_block.md");
const COLLAPSE_DOC: &str = include_str!("../content/pages/collapse.md");
const COLOR_PICKER_DOC: &str = include_str!("../content/pages/color_picker.md");
const CONTAINER_DOC: &str = include_str!("../content/pages/container.md");
const DATE_PICKER_DOC: &str = include_str!("../content/pages/date_picker.md");
const DATE_TIME_PICKER_DOC: &str = include_str!("../content/pages/date_time_picker.md");
const DESCRIPTIONS_DOC: &str = include_str!("../content/pages/descriptions.md");
const DIALOG_DOC: &str = include_str!("../content/pages/dialog.md");
const DRAWER_DOC: &str = include_str!("../content/pages/drawer.md");
const DROPDOWN_DOC: &str = include_str!("../content/pages/dropdown.md");
const EMPTY_DOC: &str = include_str!("../content/pages/empty.md");
const FORM_DOC: &str = include_str!("../content/pages/form.md");
const ICON_DOC: &str = include_str!("../content/pages/icon.md");
const IMAGE_DOC: &str = include_str!("../content/pages/image.md");
const INPUT_DOC: &str = include_str!("../content/pages/input.md");
const INPUT_NUMBER_DOC: &str = include_str!("../content/pages/input_number.md");
const LAYOUT_DOC: &str = include_str!("../content/pages/layout.md");
const LINK_DOC: &str = include_str!("../content/pages/link.md");
const LOADING_DOC: &str = include_str!("../content/pages/loading.md");
const MENU_DOC: &str = include_str!("../content/pages/menu.md");
const MESSAGE_DOC: &str = include_str!("../content/pages/message.md");
const MESSAGE_BOX_DOC: &str = include_str!("../content/pages/message_box.md");
const NOTIFICATION_DOC: &str = include_str!("../content/pages/notification.md");
const PAGE_HEADER_DOC: &str = include_str!("../content/pages/page_header.md");
const PAGINATION_DOC: &str = include_str!("../content/pages/pagination.md");
const POPCONFIRM_DOC: &str = include_str!("../content/pages/popconfirm.md");
const POPOVER_DOC: &str = include_str!("../content/pages/popover.md");
const PREVIEW_DOC: &str = include_str!("../content/pages/preview.md");
const PROGRESS_DOC: &str = include_str!("../content/pages/progress.md");
const RADIO_DOC: &str = include_str!("../content/pages/radio.md");
const RATE_DOC: &str = include_str!("../content/pages/rate.md");
const RESULT_DOC: &str = include_str!("../content/pages/result.md");
const SCROLLBAR_DOC: &str = include_str!("../content/pages/scrollbar.md");
const SEGMENTED_DOC: &str = include_str!("../content/pages/segmented.md");
const SELECT_DOC: &str = include_str!("../content/pages/select.md");
const SKELETON_DOC: &str = include_str!("../content/pages/skeleton.md");
const SLIDER_DOC: &str = include_str!("../content/pages/slider.md");
const SPLITTER_DOC: &str = include_str!("../content/pages/splitter.md");
const STATISTIC_DOC: &str = include_str!("../content/pages/statistic.md");
const STEPS_DOC: &str = include_str!("../content/pages/steps.md");
const SWITCH_DOC: &str = include_str!("../content/pages/switch.md");
const TABLE_DOC: &str = include_str!("../content/pages/table.md");
const TABS_DOC: &str = include_str!("../content/pages/tabs.md");
const TAG_DOC: &str = include_str!("../content/pages/tag.md");
const TEXTAREA_DOC: &str = include_str!("../content/pages/textarea.md");
const TIME_PICKER_DOC: &str = include_str!("../content/pages/time_picker.md");
const TIMELINE_DOC: &str = include_str!("../content/pages/timeline.md");
const TOOLTIP_DOC: &str = include_str!("../content/pages/tooltip.md");
const TRANSFER_DOC: &str = include_str!("../content/pages/transfer.md");
const TREE_DOC: &str = include_str!("../content/pages/tree.md");
const TYPOGRAPHY_DOC: &str = include_str!("../content/pages/typography.md");
const UPLOAD_DOC: &str = include_str!("../content/pages/upload.md");

const MARKDOWN_DOC: &str = include_str!("../content/pages/markdown.md");
const LIVE_DEMO_DOC: &str = include_str!("../content/pages/live_demo.md");
const COMPONENT_DOC: &str = include_str!("../content/pages/authoring.md");

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
        title: "About",
        markdown: ABOUT_DOC,
    },
    DocPage {
        title: "Affix",
        markdown: AFFIX_DOC,
    },
    DocPage {
        title: "Alert",
        markdown: ALERT_DOC,
    },
    DocPage {
        title: "Anchor",
        markdown: ANCHOR_DOC,
    },
    DocPage {
        title: "Autocomplete",
        markdown: AUTOCOMPLETE_DOC,
    },
    DocPage {
        title: "Avatar",
        markdown: AVATAR_DOC,
    },
    DocPage {
        title: "Backtop",
        markdown: BACKTOP_DOC,
    },
    DocPage {
        title: "Badge",
        markdown: BADGE_DOC,
    },
    DocPage {
        title: "Breadcrumb",
        markdown: BREADCRUMB_DOC,
    },
    DocPage {
        title: "Button",
        markdown: BUTTON_DOC,
    },
    DocPage {
        title: "Card",
        markdown: CARD_DOC,
    },
    DocPage {
        title: "Cascader",
        markdown: CASCADER_DOC,
    },
    DocPage {
        title: "Checkbox",
        markdown: CHECKBOX_DOC,
    },
    DocPage {
        title: "CodeBlock",
        markdown: CODE_BLOCK_DOC,
    },
    DocPage {
        title: "Collapse",
        markdown: COLLAPSE_DOC,
    },
    DocPage {
        title: "ColorPicker",
        markdown: COLOR_PICKER_DOC,
    },
    DocPage {
        title: "Container",
        markdown: CONTAINER_DOC,
    },
    DocPage {
        title: "DatePicker",
        markdown: DATE_PICKER_DOC,
    },
    DocPage {
        title: "DateTimePicker",
        markdown: DATE_TIME_PICKER_DOC,
    },
    DocPage {
        title: "Descriptions",
        markdown: DESCRIPTIONS_DOC,
    },
    DocPage {
        title: "Dialog",
        markdown: DIALOG_DOC,
    },
    DocPage {
        title: "Drawer",
        markdown: DRAWER_DOC,
    },
    DocPage {
        title: "Dropdown",
        markdown: DROPDOWN_DOC,
    },
    DocPage {
        title: "Empty",
        markdown: EMPTY_DOC,
    },
    DocPage {
        title: "Form",
        markdown: FORM_DOC,
    },
    DocPage {
        title: "Icon",
        markdown: ICON_DOC,
    },
    DocPage {
        title: "Image",
        markdown: IMAGE_DOC,
    },
    DocPage {
        title: "Input",
        markdown: INPUT_DOC,
    },
    DocPage {
        title: "InputNumber",
        markdown: INPUT_NUMBER_DOC,
    },
    DocPage {
        title: "Layout",
        markdown: LAYOUT_DOC,
    },
    DocPage {
        title: "Link",
        markdown: LINK_DOC,
    },
    DocPage {
        title: "Loading",
        markdown: LOADING_DOC,
    },
    DocPage {
        title: "Menu",
        markdown: MENU_DOC,
    },
    DocPage {
        title: "Message",
        markdown: MESSAGE_DOC,
    },
    DocPage {
        title: "MessageBox",
        markdown: MESSAGE_BOX_DOC,
    },
    DocPage {
        title: "Notification",
        markdown: NOTIFICATION_DOC,
    },
    DocPage {
        title: "PageHeader",
        markdown: PAGE_HEADER_DOC,
    },
    DocPage {
        title: "Pagination",
        markdown: PAGINATION_DOC,
    },
    DocPage {
        title: "Popconfirm",
        markdown: POPCONFIRM_DOC,
    },
    DocPage {
        title: "Popover",
        markdown: POPOVER_DOC,
    },
    DocPage {
        title: "Preview",
        markdown: PREVIEW_DOC,
    },
    DocPage {
        title: "Progress",
        markdown: PROGRESS_DOC,
    },
    DocPage {
        title: "Radio",
        markdown: RADIO_DOC,
    },
    DocPage {
        title: "Rate",
        markdown: RATE_DOC,
    },
    DocPage {
        title: "Result",
        markdown: RESULT_DOC,
    },
    DocPage {
        title: "Scrollbar",
        markdown: SCROLLBAR_DOC,
    },
    DocPage {
        title: "Segmented",
        markdown: SEGMENTED_DOC,
    },
    DocPage {
        title: "Select",
        markdown: SELECT_DOC,
    },
    DocPage {
        title: "Skeleton",
        markdown: SKELETON_DOC,
    },
    DocPage {
        title: "Slider",
        markdown: SLIDER_DOC,
    },
    DocPage {
        title: "Splitter",
        markdown: SPLITTER_DOC,
    },
    DocPage {
        title: "Statistic",
        markdown: STATISTIC_DOC,
    },
    DocPage {
        title: "Steps",
        markdown: STEPS_DOC,
    },
    DocPage {
        title: "Switch",
        markdown: SWITCH_DOC,
    },
    DocPage {
        title: "Table",
        markdown: TABLE_DOC,
    },
    DocPage {
        title: "Tabs",
        markdown: TABS_DOC,
    },
    DocPage {
        title: "Tag",
        markdown: TAG_DOC,
    },
    DocPage {
        title: "Textarea",
        markdown: TEXTAREA_DOC,
    },
    DocPage {
        title: "TimePicker",
        markdown: TIME_PICKER_DOC,
    },
    DocPage {
        title: "Timeline",
        markdown: TIMELINE_DOC,
    },
    DocPage {
        title: "Tooltip",
        markdown: TOOLTIP_DOC,
    },
    DocPage {
        title: "Transfer",
        markdown: TRANSFER_DOC,
    },
    DocPage {
        title: "Tree",
        markdown: TREE_DOC,
    },
    DocPage {
        title: "Typography",
        markdown: TYPOGRAPHY_DOC,
    },
    DocPage {
        title: "Upload",
        markdown: UPLOAD_DOC,
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

pub struct DocPage {
    pub title: &'static str,
    pub markdown: &'static str,
}

#[allow(dead_code)]
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
        source: Option<SharedString>,
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
        source: Option<SharedString>,
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
                let (language, source) = match kind {
                    CodeBlockKind::Indented => (None, None),
                    CodeBlockKind::Fenced(info) => parse_code_block_info(info.as_ref()),
                };
                self.stack.push(Frame::CodeBlock {
                    language,
                    source,
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
            Frame::CodeBlock {
                language,
                source,
                code,
            } => {
                self.push_block(Block::CodeBlock {
                    language,
                    source,
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
            Some(Frame::Item(blocks)) => push_inline_segment_into_blocks(blocks, segment),
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

fn push_inline_segment_into_blocks(blocks: &mut Vec<Block>, segment: InlineSegment) {
    if let Some(Block::Paragraph(segments)) = blocks.last_mut() {
        segments.push(segment);
    } else {
        blocks.push(Block::Paragraph(vec![segment]));
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

fn parse_code_block_info(info: &str) -> (Option<SharedString>, Option<SharedString>) {
    let info = info.trim();
    if info.is_empty() {
        return (None, None);
    }

    let mut language = None;
    let mut attrs = "";

    if let Some(space_index) = info.find(char::is_whitespace) {
        let first = info[..space_index].trim();
        attrs = info[space_index..].trim();
        if !first.is_empty() && !first.contains('=') {
            language = Some(first.to_string().into());
        }
    } else if !info.contains('=') {
        language = Some(info.to_string().into());
    }

    let source = parse_block_attr(attrs, "src");
    (language, source)
}

fn parse_block_attr(attrs: &str, key: &str) -> Option<SharedString> {
    let key = format!("{key}=");
    let start = attrs.find(&key)? + key.len();
    let rest = &attrs[start..];
    let value = if let Some(stripped) = rest.strip_prefix('"') {
        let end = stripped.find('"')?;
        &stripped[..end]
    } else {
        let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        &rest[..end]
    };

    (!value.is_empty()).then(|| value.to_string().into())
}

fn load_code_snippet(path: &str) -> Option<&'static str> {
    match path {
        "quick_start/run.sh" => Some(include_str!("../content/snippets/quick_start/run.sh")),
        "quick_start/deps_fedora.sh" => Some(include_str!(
            "../content/snippets/quick_start/deps_fedora.sh"
        )),
        "quick_start/deps_ubuntu.sh" => Some(include_str!(
            "../content/snippets/quick_start/deps_ubuntu.sh"
        )),
        "quick_start/deps_macos.sh" => Some(include_str!(
            "../content/snippets/quick_start/deps_macos.sh"
        )),
        "quick_start/deps_windows.ps1" => Some(include_str!(
            "../content/snippets/quick_start/deps_windows.ps1"
        )),
        "quick_start/create_project.sh" => Some(include_str!(
            "../content/snippets/quick_start/create_project.sh"
        )),
        "quick_start/app_cargo.toml" => Some(include_str!(
            "../content/snippets/quick_start/app_cargo.toml"
        )),
        "quick_start/main_window.rs" => Some(include_str!(
            "../content/snippets/quick_start/main_window.rs"
        )),
        "quick_start/component_view.rs" => Some(include_str!(
            "../content/snippets/quick_start/component_view.rs"
        )),
        "quick_start/verify.sh" => Some(include_str!("../content/snippets/quick_start/verify.sh")),
        "quick_start/init.rs" => Some(include_str!("../content/snippets/quick_start/init.rs")),
        "quick_start/components.rs" => Some(include_str!(
            "../content/snippets/quick_start/components.rs"
        )),
        "architecture/render_pipeline.rs" => Some(include_str!(
            "../content/snippets/architecture/render_pipeline.rs"
        )),
        "typography/paragraph.rs" => {
            Some(include_str!("../content/snippets/typography/paragraph.rs"))
        }
        "button/types.rs" => Some(include_str!("../content/snippets/button/types.rs")),
        "button/secondary.rs" => Some(include_str!("../content/snippets/button/secondary.rs")),
        "button/text.rs" => Some(include_str!("../content/snippets/button/text.rs")),
        "button/sizes.rs" => Some(include_str!("../content/snippets/button/sizes.rs")),
        "button/states.rs" => Some(include_str!("../content/snippets/button/states.rs")),
        "button/rounded.rs" => Some(include_str!("../content/snippets/button/rounded.rs")),
        "alert/types.rs" => Some(include_str!("../content/snippets/alert/types.rs")),
        "alert/description.rs" => Some(include_str!("../content/snippets/alert/description.rs")),
        "tag/types.rs" => Some(include_str!("../content/snippets/tag/types.rs")),
        "tag/closable.rs" => Some(include_str!("../content/snippets/tag/closable.rs")),
        "tag/themes.rs" => Some(include_str!("../content/snippets/tag/themes.rs")),
        "tag/sizes.rs" => Some(include_str!("../content/snippets/tag/sizes.rs")),
        "tag/round.rs" => Some(include_str!("../content/snippets/tag/round.rs")),
        "autocomplete/basic.rs" => Some(include_str!("../content/snippets/autocomplete/basic.rs")),
        "autocomplete/custom.rs" => {
            Some(include_str!("../content/snippets/autocomplete/custom.rs"))
        }
        "autocomplete/no_suffix.rs" => Some(include_str!(
            "../content/snippets/autocomplete/no_suffix.rs"
        )),
        "autocomplete/disabled.rs" => {
            Some(include_str!("../content/snippets/autocomplete/disabled.rs"))
        }
        "avatar/shapes.rs" => Some(include_str!("../content/snippets/avatar/shapes.rs")),
        "avatar/sizes.rs" => Some(include_str!("../content/snippets/avatar/sizes.rs")),
        "avatar/content.rs" => Some(include_str!("../content/snippets/avatar/content.rs")),
        "badge/basic.rs" => Some(include_str!("../content/snippets/badge/basic.rs")),
        "badge/max.rs" => Some(include_str!("../content/snippets/badge/max.rs")),
        "badge/dot.rs" => Some(include_str!("../content/snippets/badge/dot.rs")),
        "input_number/basic.rs" => Some(include_str!("../content/snippets/input_number/basic.rs")),
        "input_number/vertical.rs" => {
            Some(include_str!("../content/snippets/input_number/vertical.rs"))
        }
        "input_number/precision.rs" => Some(include_str!(
            "../content/snippets/input_number/precision.rs"
        )),
        "textarea/basic.rs" => Some(include_str!("../content/snippets/textarea/basic.rs")),
        "textarea/limit.rs" => Some(include_str!("../content/snippets/textarea/limit.rs")),
        "checkbox/basic.rs" => Some(include_str!("../content/snippets/checkbox/basic.rs")),
        "checkbox/group.rs" => Some(include_str!("../content/snippets/checkbox/group.rs")),
        "checkbox/buttons.rs" => Some(include_str!("../content/snippets/checkbox/buttons.rs")),
        "radio/basic.rs" => Some(include_str!("../content/snippets/radio/basic.rs")),
        "radio/group.rs" => Some(include_str!("../content/snippets/radio/group.rs")),
        "radio/buttons.rs" => Some(include_str!("../content/snippets/radio/buttons.rs")),
        "select/basic.rs" => Some(include_str!("../content/snippets/select/basic.rs")),
        "slider/basic.rs" => Some(include_str!("../content/snippets/slider/basic.rs")),
        "slider/step.rs" => Some(include_str!("../content/snippets/slider/step.rs")),
        "rate/basic.rs" => Some(include_str!("../content/snippets/rate/basic.rs")),
        "rate/custom.rs" => Some(include_str!("../content/snippets/rate/custom.rs")),
        "code_block/basic.rs" => Some(include_str!("../content/snippets/code_block/basic.rs")),
        "code_block/language.rs" => {
            Some(include_str!("../content/snippets/code_block/language.rs"))
        }
        "code_block/theme.rs" => Some(include_str!("../content/snippets/code_block/theme.rs")),
        "code_block/inline.rs" => Some(include_str!("../content/snippets/code_block/inline.rs")),
        "input/basic.rs" => Some(include_str!("../content/snippets/input/basic.rs")),
        "input/password.rs" => Some(include_str!("../content/snippets/input/password.rs")),
        "input/affix.rs" => Some(include_str!("../content/snippets/input/affix.rs")),
        "input/states.rs" => Some(include_str!("../content/snippets/input/states.rs")),
        "switch/basic.rs" => Some(include_str!("../content/snippets/switch/basic.rs")),
        "switch/disabled.rs" => Some(include_str!("../content/snippets/switch/disabled.rs")),
        "switch/callback.rs" => Some(include_str!("../content/snippets/switch/callback.rs")),
        "message/types.rs" => Some(include_str!("../content/snippets/message/types.rs")),
        "message/formatting.rs" => Some(include_str!("../content/snippets/message/formatting.rs")),
        "markdown/state_machine.rs" => Some(include_str!(
            "../content/snippets/markdown/state_machine.rs"
        )),
        "live_demo/button.rs" => Some(include_str!("../content/snippets/live_demo/button.rs")),
        "authoring/code_block.rs" => {
            Some(include_str!("../content/snippets/authoring/code_block.rs"))
        }
        "about/doc_rule.rs" => Some(include_str!("../content/snippets/about/doc_rule.rs")),
        "gallery/affix_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/affix_demo.rs")),
        "gallery/alert_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/alert_demo.rs")),
        "gallery/anchor_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/anchor_demo.rs"))
        }
        "gallery/autocomplete_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/autocomplete_demo.rs"
        )),
        "gallery/avatar_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/avatar_demo.rs"))
        }
        "gallery/backtop_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/backtop_demo.rs"))
        }
        "gallery/badge_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/badge_demo.rs")),
        "gallery/breadcrumb_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/breadcrumb_demo.rs"
        )),
        "gallery/button_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/button_demo.rs"))
        }
        "gallery/card_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/card_demo.rs")),
        "gallery/cascader_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/cascader_demo.rs"
        )),
        "gallery/code_block_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/code_block_demo.rs"
        )),
        "gallery/collapse_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/collapse_demo.rs"
        )),
        "gallery/color_picker_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/color_picker_demo.rs"
        )),
        "gallery/container_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/container_demo.rs"
        )),
        "gallery/date_picker_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/date_picker_demo.rs"
        )),
        "gallery/date_time_picker_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/date_time_picker_demo.rs"
        )),
        "gallery/descriptions_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/descriptions_demo.rs"
        )),
        "gallery/dialog_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/dialog_demo.rs"))
        }
        "gallery/drawer_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/drawer_demo.rs"))
        }
        "gallery/dropdown_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/dropdown_demo.rs"
        )),
        "gallery/empty_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/empty_demo.rs")),
        "gallery/form_controls_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/form_controls_demo.rs"
        )),
        "gallery/form_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/form_demo.rs")),
        "gallery/icon_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/icon_demo.rs")),
        "gallery/image_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/image_demo.rs")),
        "gallery/layout_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/layout_demo.rs"))
        }
        "gallery/link_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/link_demo.rs")),
        "gallery/loading_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/loading_demo.rs"))
        }
        "gallery/menu_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/menu_demo.rs")),
        "gallery/message_box_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/message_box_demo.rs"
        )),
        "gallery/message_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/message_demo.rs"))
        }
        "gallery/notification_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/notification_demo.rs"
        )),
        "gallery/page_header_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/page_header_demo.rs"
        )),
        "gallery/pagination_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/pagination_demo.rs"
        )),
        "gallery/popconfirm_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/popconfirm_demo.rs"
        )),
        "gallery/popover_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/popover_demo.rs"))
        }
        "gallery/preview_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/preview_demo.rs"))
        }
        "gallery/progress_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/progress_demo.rs"
        )),
        "gallery/result_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/result_demo.rs"))
        }
        "gallery/scrollbar_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/scrollbar_demo.rs"
        )),
        "gallery/segmented_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/segmented_demo.rs"
        )),
        "gallery/skeleton_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/skeleton_demo.rs"
        )),
        "gallery/splitter_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/splitter_demo.rs"
        )),
        "gallery/statistic_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/statistic_demo.rs"
        )),
        "gallery/steps_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/steps_demo.rs")),
        "gallery/table_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/table_demo.rs")),
        "gallery/tabs_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/tabs_demo.rs")),
        "gallery/tag_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/tag_demo.rs")),
        "gallery/time_picker_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/time_picker_demo.rs"
        )),
        "gallery/timeline_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/timeline_demo.rs"
        )),
        "gallery/tooltip_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/tooltip_demo.rs"))
        }
        "gallery/transfer_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/transfer_demo.rs"
        )),
        "gallery/tree_demo.rs" => Some(include_str!("../../aura-gallery/src/demos/tree_demo.rs")),
        "gallery/typography_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/typography_demo.rs"
        )),
        "gallery/upload_demo.rs" => {
            Some(include_str!("../../aura-gallery/src/demos/upload_demo.rs"))
        }
        _ => None,
    }
}

impl RenderOnce for MarkdownDocument {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        let children = self
            .blocks
            .into_iter()
            .map(|block| block.render(&theme, cx))
            .collect::<Vec<_>>();

        Space::new().vertical().gap_lg().children(children)
    }
}

impl Block {
    fn render(self, theme: &aura_theme::Theme, cx: &mut App) -> AnyElement {
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
                    Space::new().vertical().gap_md().children(
                        blocks
                            .into_iter()
                            .map(|block| block.render(theme, cx))
                            .collect::<Vec<_>>(),
                    ),
                )
                .into_any_element(),
            Self::List {
                ordered,
                start,
                items,
            } => render_list(ordered, start, items, theme, cx),
            Self::CodeBlock {
                language,
                source,
                code,
            } => render_code_block(language, source, code, theme),
            Self::LiveDemo { component } => Paragraph::with_text(format!(
                "Live demo `{}` requires DocsPageView for persistent state.",
                component.as_ref()
            ))
            .into_any_element(),
            Self::Rule => div()
                .h(px(1.0))
                .w_full()
                .bg(theme.neutral.divider)
                .into_any_element(),
        }
    }
}

struct LiveDemoHost {
    component: SharedString,
    demo: Entity<LiveDemoContent>,
}

impl LiveDemoHost {
    fn new(component: SharedString, cx: &mut Context<Self>) -> Self {
        let demo_component = component.clone();
        let demo = cx.new(|cx| LiveDemoContent::new(demo_component, cx));
        Self { component, demo }
    }
}

impl Render for LiveDemoHost {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        Space::new()
            .vertical()
            .gap_sm()
            .child(Text::new(self.component.clone()).size(px(theme.font_size.sm)))
            .child(self.demo.clone())
    }
}

struct LiveDemoContent {
    component: SharedString,
    gallery_demo: Option<AnyView>,
    autocompletes: Vec<Entity<Autocomplete>>,
    input_numbers: Vec<Entity<InputNumber>>,
    textareas: Vec<Entity<Textarea>>,
    checkboxes: Vec<Entity<Checkbox>>,
    checkbox_groups: Vec<Entity<CheckboxGroup>>,
    inputs: Vec<Entity<Input>>,
    radios: Vec<Entity<Radio>>,
    radio_groups: Vec<Entity<RadioGroup>>,
    rates: Vec<Entity<Rate>>,
    selects: Vec<Entity<Select>>,
    sliders: Vec<Entity<Slider>>,
    switches: Vec<Entity<Switch>>,
}

impl LiveDemoContent {
    fn new(component: SharedString, cx: &mut Context<Self>) -> Self {
        let gallery_demo = aura_gallery::demos::render_doc_demo(component.as_ref(), cx);
        let mut autocompletes = Vec::new();
        let mut input_numbers = Vec::new();
        let mut textareas = Vec::new();
        let mut checkboxes = Vec::new();
        let mut checkbox_groups = Vec::new();
        let mut inputs = Vec::new();
        let mut radios = Vec::new();
        let mut radio_groups = Vec::new();
        let mut rates = Vec::new();
        let mut selects = Vec::new();
        let mut sliders = Vec::new();
        let mut switches = Vec::new();

        match component.as_ref() {
            "AutocompleteBasic" => {
                let suggestions = basic_autocomplete_items();
                autocompletes.push(cx.new(move |cx| {
                    Autocomplete::new(suggestions, cx).placeholder("Search component")
                }));
            }
            "AutocompleteCustom" => {
                let routes = vec![
                    AutocompleteItem::labeled("/dashboard", "Dashboard"),
                    AutocompleteItem::labeled("/settings", "Settings"),
                    AutocompleteItem::labeled("/profile", "Profile"),
                    AutocompleteItem::labeled("/billing", "Billing"),
                ];
                autocompletes.push(cx.new(move |cx| {
                    Autocomplete::new(routes, cx)
                        .placeholder("Jump to route")
                        .width_lg()
                        .max_suggestions(4)
                        .suffix_icon(IconName::Command)
                }));
            }
            "AutocompleteNoSuffix" => {
                let suggestions = basic_autocomplete_items();
                autocompletes.push(cx.new(move |cx| {
                    Autocomplete::new(suggestions, cx)
                        .placeholder("No suffix icon")
                        .no_suffix_icon()
                }));
            }
            "AutocompleteDisabled" => {
                let suggestions = basic_autocomplete_items();
                autocompletes.push(cx.new(move |cx| {
                    Autocomplete::new(suggestions, cx)
                        .placeholder("Disabled")
                        .disabled(true)
                }));
            }
            "InputBasic" => {
                inputs.push(cx.new(|cx| Input::new("", cx)));
                inputs.push(cx.new(|cx| Input::new("", cx).placeholder("Type something...")));
            }
            "InputNumberBasic" => {
                input_numbers.push(cx.new(|cx| InputNumber::new(10.0, cx).min(0.0).max(10.0)));
            }
            "InputNumberVertical" => {
                input_numbers.push(cx.new(|cx| {
                    InputNumber::new(5.0, cx)
                        .min(0.0)
                        .max(10.0)
                        .controls_position(InputNumberControlsPosition::Right)
                }));
            }
            "InputNumberPrecision" => {
                input_numbers.push(cx.new(|cx| InputNumber::new(1.23, cx).precision(2).step(0.01)));
            }
            "InputPassword" => {
                inputs.push(cx.new(|cx| Input::new("", cx).password().placeholder("Password")));
                inputs.push(cx.new(|cx| Input::new("secret", cx).password().mask_char('*')));
            }
            "InputAffix" => {
                inputs.push(cx.new(|cx| Input::new("", cx).prepend_text("http://")));
                inputs.push(cx.new(|cx| Input::new("", cx).append_text(".com")));
                inputs.push(cx.new(|cx| {
                    Input::new("", cx)
                        .prepend_icon(IconName::User)
                        .append_text("Admin")
                }));
            }
            "InputStates" => {
                inputs.push(cx.new(|cx| Input::new("Clear me", cx).clearable(true)));
                inputs.push(cx.new(|cx| Input::new("Disabled", cx).disabled(true)));
            }
            "TextareaBasic" => {
                textareas.push(cx.new(|cx| Textarea::new("Line 1\nLine 2", cx).rows(3)));
            }
            "TextareaLimit" => {
                textareas
                    .push(cx.new(|cx| Textarea::new("Limited content", cx).max_length(50).rows(2)));
            }
            "CheckboxBasic" => {
                checkboxes.push(cx.new(|cx| Checkbox::new(true, cx)));
                checkboxes.push(cx.new(|cx| Checkbox::new(false, cx)));
                checkboxes.push(cx.new(|cx| Checkbox::new(false, cx).label("Label")));
                checkboxes.push(cx.new(|cx| Checkbox::new(false, cx).disabled(true)));
                checkboxes.push(cx.new(|cx| Checkbox::new(true, cx).disabled(true)));
            }
            "CheckboxGroup" => {
                checkbox_groups.push(cx.new(|cx| {
                    CheckboxGroup::new(vec!["Option 1", "Option 2", "Option 3"], vec![0, 2], cx)
                }));
            }
            "CheckboxButtons" => {
                checkbox_groups.push(cx.new(|cx| city_checkbox_group(cx).large()));
                checkbox_groups.push(cx.new(city_checkbox_group));
                checkbox_groups.push(cx.new(|cx| city_checkbox_group(cx).small()));
                checkbox_groups.push(cx.new(|cx| city_checkbox_group(cx).stretch(true)));
            }
            "RadioBasic" => {
                radios.push(cx.new(|cx| Radio::new(true, cx)));
                radios.push(cx.new(|cx| Radio::new(false, cx)));
                radios.push(cx.new(|cx| Radio::new(false, cx).label("Label")));
                radios.push(cx.new(|cx| Radio::new(false, cx).disabled(true)));
                radios.push(cx.new(|cx| Radio::new(true, cx).disabled(true)));
            }
            "RadioGroup" => {
                radio_groups.push(
                    cx.new(|cx| RadioGroup::new(vec!["Option A", "Option B", "Option C"], 1, cx)),
                );
                radio_groups.push(cx.new(|cx| {
                    RadioGroup::new(vec!["Disabled A", "Disabled B"], 0, cx).disabled(true)
                }));
            }
            "RadioButtons" => {
                radio_groups.push(cx.new(|cx| city_radio_group(cx).large()));
                radio_groups.push(cx.new(city_radio_group));
                radio_groups.push(cx.new(|cx| city_radio_group(cx).small()));
                radio_groups.push(cx.new(|cx| city_radio_group(cx).stretch(true)));
            }
            "SelectBasic" => {
                selects.push(cx.new(|cx| {
                    Select::new(
                        vec!["Apple", "Banana", "Orange", "Grape", "Watermelon"],
                        Some(1),
                        cx,
                    )
                }));
            }
            "SliderBasic" => {
                sliders.push(cx.new(|cx| Slider::new(50.0, cx)));
            }
            "SliderStep" => {
                sliders.push(cx.new(|cx| Slider::new(20.0, cx).step(10.0)));
            }
            "RateBasic" => {
                rates.push(cx.new(|cx| Rate::new(3.0, cx)));
            }
            "RateCustom" => {
                rates.push(cx.new(|cx| Rate::new(4.0, cx).max(10)));
            }
            "SwitchBasic" => {
                switches.push(cx.new(|cx| Switch::new(true, cx)));
                switches.push(cx.new(|cx| Switch::new(false, cx)));
            }
            "SwitchDisabled" => {
                switches.push(cx.new(|cx| Switch::new(false, cx).disabled(true)));
                switches.push(cx.new(|cx| Switch::new(true, cx).disabled(true)));
            }
            "SwitchCallback" => {
                switches.push(cx.new(|cx| {
                    Switch::new(false, cx).on_change(|checked, _window, _cx| {
                        if checked {
                            toast_success!("Switch is on");
                        } else {
                            toast_info!("Switch is off");
                        }
                    })
                }));
            }
            _ => {}
        }

        Self {
            component,
            gallery_demo,
            autocompletes,
            input_numbers,
            textareas,
            checkboxes,
            checkbox_groups,
            inputs,
            radios,
            radio_groups,
            rates,
            selects,
            sliders,
            switches,
        }
    }
}

impl Render for LiveDemoContent {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match self.component.as_ref() {
            "Button" => Space::new()
                .vertical()
                .gap_sm()
                .child(Text::new("Live Button demo").bold())
                .child(Button::new("Native Button").primary().on_click(|_, _, _| {
                    toast_success!("Live demo clicked: {}", "Button");
                }))
                .into_any_element(),
            "ButtonTypes" => demo_row(vec![
                Button::new("Default").into_any_element(),
                Button::new("Tertiary").tertiary().into_any_element(),
                Button::new("Primary").primary().into_any_element(),
                Button::new("Info").info().into_any_element(),
                Button::new("Success").success().into_any_element(),
                Button::new("Warning").warning().into_any_element(),
                Button::new("Danger").danger().into_any_element(),
            ]),
            "ButtonSecondary" => demo_row(vec![
                Button::new("Default").secondary().into_any_element(),
                Button::new("Primary")
                    .primary()
                    .secondary()
                    .into_any_element(),
                Button::new("Info").info().secondary().into_any_element(),
                Button::new("Success")
                    .success()
                    .secondary()
                    .into_any_element(),
                Button::new("Warning")
                    .warning()
                    .secondary()
                    .into_any_element(),
                Button::new("Danger")
                    .danger()
                    .secondary()
                    .into_any_element(),
            ]),
            "ButtonText" => demo_row(vec![
                Button::new("Default").text().into_any_element(),
                Button::new("Primary").primary().text().into_any_element(),
                Button::new("Info").info().text().into_any_element(),
                Button::new("Success").success().text().into_any_element(),
                Button::new("Warning").warning().text().into_any_element(),
                Button::new("Danger").danger().text().into_any_element(),
            ]),
            "ButtonSizes" => demo_row(vec![
                Button::new("Small").primary().small().into_any_element(),
                Button::new("Default").primary().into_any_element(),
                Button::new("Large").primary().large().into_any_element(),
            ]),
            "ButtonStates" => demo_row(vec![
                Button::new("Disabled")
                    .primary()
                    .disabled(true)
                    .into_any_element(),
                Button::new("Loading")
                    .primary()
                    .loading(true)
                    .into_any_element(),
                Button::new("Saving")
                    .success()
                    .loading(true)
                    .into_any_element(),
            ]),
            "ButtonRounded" => demo_row(vec![
                Button::new("4px").primary().rounded_sm().into_any_element(),
                Button::new("12px")
                    .primary()
                    .rounded_md()
                    .into_any_element(),
                Button::new("20px")
                    .primary()
                    .rounded_lg()
                    .into_any_element(),
                Button::new("Pill").primary().pill().into_any_element(),
            ]),
            "AlertTypes" => demo_stack(vec![
                Alert::new("Info Alert")
                    .alert_type(AlertType::Info)
                    .into_any_element(),
                Alert::new("Success Alert")
                    .alert_type(AlertType::Success)
                    .into_any_element(),
                Alert::new("Warning Alert")
                    .alert_type(AlertType::Warning)
                    .into_any_element(),
                Alert::new("Error Alert")
                    .alert_type(AlertType::Error)
                    .into_any_element(),
            ]),
            "AlertDescription" => Alert::new("Warning")
                .alert_type(AlertType::Warning)
                .description("More detailed description of the warning.")
                .into_any_element(),
            "TagTypes" => demo_row(vec![
                AuraTag::new("Tag 1").into_any_element(),
                AuraTag::new("Tag 2").success().into_any_element(),
                AuraTag::new("Tag 3").warning().into_any_element(),
                AuraTag::new("Tag 4").danger().into_any_element(),
            ]),
            "TagClosable" => demo_row(vec![
                AuraTag::new("Tag 1").closable(true).into_any_element(),
                AuraTag::new("Tag 2")
                    .success()
                    .closable(true)
                    .into_any_element(),
                AuraTag::new("Tag 3")
                    .warning()
                    .closable(true)
                    .into_any_element(),
                AuraTag::new("Tag 4")
                    .danger()
                    .closable(true)
                    .into_any_element(),
            ]),
            "TagThemes" => demo_stack(vec![
                demo_row(vec![
                    AuraTag::new("Dark").dark().into_any_element(),
                    AuraTag::new("Success").success().dark().into_any_element(),
                    AuraTag::new("Warning").warning().dark().into_any_element(),
                    AuraTag::new("Danger").danger().dark().into_any_element(),
                ]),
                demo_row(vec![
                    AuraTag::new("Plain").plain().into_any_element(),
                    AuraTag::new("Success").success().plain().into_any_element(),
                    AuraTag::new("Warning").warning().plain().into_any_element(),
                    AuraTag::new("Danger").danger().plain().into_any_element(),
                ]),
            ]),
            "TagSizes" => demo_row(vec![
                AuraTag::new("Default").into_any_element(),
                AuraTag::new("Large").large().into_any_element(),
                AuraTag::new("Small").small().into_any_element(),
            ]),
            "TagRound" => demo_row(vec![
                AuraTag::new("Tag 1").round(true).into_any_element(),
                AuraTag::new("Tag 2")
                    .success()
                    .round(true)
                    .into_any_element(),
                AuraTag::new("Tag 3")
                    .warning()
                    .round(true)
                    .into_any_element(),
                AuraTag::new("Tag 4")
                    .danger()
                    .round(true)
                    .into_any_element(),
            ]),
            "AutocompleteBasic"
            | "AutocompleteCustom"
            | "AutocompleteNoSuffix"
            | "AutocompleteDisabled" => demo_stack(
                self.autocompletes
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "InputNumberBasic" | "InputNumberVertical" | "InputNumberPrecision" => demo_stack(
                self.input_numbers
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "TextareaBasic" | "TextareaLimit" => demo_stack(
                self.textareas
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "CheckboxBasic" => demo_row(
                self.checkboxes
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "CheckboxGroup" | "CheckboxButtons" => demo_stack(
                self.checkbox_groups
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "RadioBasic" => demo_row(
                self.radios
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "RadioGroup" | "RadioButtons" => demo_stack(
                self.radio_groups
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "SelectBasic" => demo_stack(
                self.selects
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "SliderBasic" | "SliderStep" => demo_stack(
                self.sliders
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "RateBasic" | "RateCustom" => demo_stack(
                self.rates
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "AvatarShapes" => demo_row(vec![
                Avatar::new().into_any_element(),
                Avatar::new().square().into_any_element(),
            ]),
            "AvatarSizes" => demo_row(vec![
                Avatar::new().small().into_any_element(),
                Avatar::new().into_any_element(),
                Avatar::new().large().into_any_element(),
            ]),
            "AvatarContent" => demo_row(vec![
                Avatar::new().icon(IconName::User).into_any_element(),
                Avatar::new().icon(IconName::Star).into_any_element(),
                Avatar::new()
                    .src("https://github.com/zed-industries.png")
                    .into_any_element(),
            ]),
            "BadgeBasic" => demo_row(vec![
                Badge::new(Button::new("Messages"))
                    .value("5")
                    .into_any_element(),
                Badge::new(Button::new("Updates"))
                    .value("10")
                    .badge_type(BadgeType::Primary)
                    .into_any_element(),
                Badge::new(Button::new("Alerts"))
                    .value("2")
                    .badge_type(BadgeType::Warning)
                    .into_any_element(),
            ]),
            "BadgeMax" => demo_row(vec![
                Badge::new(Button::new("Messages"))
                    .value("200")
                    .max(99)
                    .into_any_element(),
                Badge::new(Button::new("Updates"))
                    .value("50")
                    .max(10)
                    .into_any_element(),
            ]),
            "BadgeDot" => demo_row(vec![
                Badge::new(Text::new("Query"))
                    .is_dot(true)
                    .into_any_element(),
                Badge::new(Avatar::new()).is_dot(true).into_any_element(),
            ]),
            "InputBasic" | "InputPassword" | "InputAffix" | "InputStates" => demo_stack(
                self.inputs
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "SwitchBasic" | "SwitchDisabled" | "SwitchCallback" => demo_row(
                self.switches
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "MessageTypes" => demo_row(vec![
                Button::new("toast_info!")
                    .on_click(|_, _, _| toast_info!("This is an info toast"))
                    .into_any_element(),
                Button::new("toast_success!")
                    .primary()
                    .on_click(|_, _, _| toast_success!("Operation completed"))
                    .into_any_element(),
                Button::new("toast_warning!")
                    .warning()
                    .on_click(|_, _, _| toast_warning!("Please check the input"))
                    .into_any_element(),
                Button::new("toast_error!")
                    .danger()
                    .on_click(|_, _, _| toast_error!("Operation failed"))
                    .into_any_element(),
            ]),
            "MessageFormatting" => demo_row(vec![
                Button::new("位置参数")
                    .on_click(|_, _, _| {
                        let name = "Aura";
                        let count = 4;
                        toast_info!("{}, you have {} toast variants.", name, count);
                    })
                    .into_any_element(),
                Button::new("命名参数")
                    .primary()
                    .on_click(|_, _, _| {
                        let component = "Message";
                        let api = "toast_success!";
                        toast_success!("{component} macro {api} works.");
                    })
                    .into_any_element(),
            ]),
            _ => self.gallery_demo.clone().map_or_else(
                || {
                    Paragraph::with_text(format!(
                        "Unsupported Aura demo component: {}",
                        self.component.as_ref()
                    ))
                    .into_any_element()
                },
                |demo| demo.into_any_element(),
            ),
        }
    }
}

fn demo_row(children: Vec<AnyElement>) -> AnyElement {
    Space::new()
        .wrap()
        .gap_sm()
        .children(children)
        .into_any_element()
}

fn basic_autocomplete_items() -> Vec<AutocompleteItem> {
    vec![
        AutocompleteItem::labeled("rust", "Rust"),
        AutocompleteItem::labeled("gpui", "GPUI"),
        AutocompleteItem::labeled("aura", "Aura UI"),
        AutocompleteItem::labeled("element-plus", "Element Plus"),
        AutocompleteItem::labeled("autocomplete", "Autocomplete"),
    ]
}

fn city_checkbox_group(cx: &mut Context<CheckboxGroup>) -> CheckboxGroup {
    CheckboxGroup::new(
        vec!["New York", "Washington", "Los Angeles", "Chicago"],
        vec![1],
        cx,
    )
    .button()
}

fn city_radio_group(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(
        vec!["New York", "Washington", "Los Angeles", "Chicago"],
        1,
        cx,
    )
    .button()
}

fn demo_stack(children: Vec<AnyElement>) -> AnyElement {
    Space::new()
        .vertical()
        .gap_md()
        .children(children)
        .into_any_element()
}

fn render_code_block(
    language: Option<SharedString>,
    source: Option<SharedString>,
    code: SharedString,
    _theme: &aura_theme::Theme,
) -> AnyElement {
    let rendered_code = if let Some(source) = source {
        load_code_snippet(source.as_ref()).map_or_else(
            || SharedString::from(format!("// Missing external snippet: {}", source.as_ref())),
            |snippet| SharedString::from(snippet.to_string()),
        )
    } else {
        code
    };

    let mut code_block = AuraCodeBlock::new(rendered_code);
    code_block = code_block.selectable(false);
    if let Some(language) = language {
        code_block = code_block.language(language.as_ref());
    }
    code_block.into_any_element()
}

fn collect_live_demo_components(blocks: &[Block], components: &mut Vec<SharedString>) {
    for block in blocks {
        match block {
            Block::LiveDemo { component } => components.push(component.clone()),
            Block::BlockQuote(children) => collect_live_demo_components(children, components),
            Block::List { items, .. } => {
                for item in items {
                    collect_live_demo_components(item, components);
                }
            }
            Block::Paragraph(_) | Block::Heading { .. } | Block::CodeBlock { .. } | Block::Rule => {
            }
        }
    }
}

fn render_persistent_block(
    block: &Block,
    theme: &aura_theme::Theme,
    live_demos: &[Entity<LiveDemoHost>],
    demo_index: &mut usize,
) -> AnyElement {
    match block {
        Block::Paragraph(segments) => render_paragraph(segments.clone(), theme),
        Block::Heading { level, content } => {
            let heading = Title::new(inline_plain_text(content));
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
        Block::BlockQuote(blocks) => div()
            .border_l_1()
            .border_color(theme.primary.base)
            .pl_4()
            .text_color(theme.neutral.text_2)
            .child(
                Space::new().vertical().gap_md().children(
                    blocks
                        .iter()
                        .map(|block| render_persistent_block(block, theme, live_demos, demo_index))
                        .collect::<Vec<_>>(),
                ),
            )
            .into_any_element(),
        Block::List {
            ordered,
            start,
            items,
        } => render_persistent_list(*ordered, *start, items, theme, live_demos, demo_index),
        Block::CodeBlock {
            language,
            source,
            code,
        } => render_code_block(language.clone(), source.clone(), code.clone(), theme),
        Block::LiveDemo { .. } => {
            let demo = live_demos.get(*demo_index).cloned();
            *demo_index += 1;
            demo.map_or_else(
                || Paragraph::with_text("Missing Aura demo host").into_any_element(),
                |demo| demo.into_any_element(),
            )
        }
        Block::Rule => div()
            .h(px(1.0))
            .w_full()
            .bg(theme.neutral.divider)
            .into_any_element(),
    }
}

fn render_persistent_list(
    ordered: bool,
    start: u64,
    items: &[Vec<Block>],
    theme: &aura_theme::Theme,
    live_demos: &[Entity<LiveDemoHost>],
    demo_index: &mut usize,
) -> AnyElement {
    let mut rows = Vec::new();

    for (index, item_blocks) in items.iter().enumerate() {
        let marker = if ordered {
            format!("{}.", start + index as u64)
        } else {
            "•".to_string()
        };
        let item_children = item_blocks
            .iter()
            .map(|block| render_persistent_block(block, theme, live_demos, demo_index))
            .collect::<Vec<_>>();

        rows.push(
            div()
                .relative()
                .pl_8()
                .child(
                    div()
                        .absolute()
                        .left_0()
                        .top_0()
                        .w(px(24.0))
                        .text_color(theme.neutral.text_3)
                        .child(marker),
                )
                .child(
                    div()
                        .w_full()
                        .child(Space::new().vertical().gap_sm().children(item_children)),
                ),
        );
    }

    div()
        .flex()
        .flex_col()
        .gap_2()
        .children(rows)
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
        page_views: vec![None; DOC_PAGES.len()],
    })
}

pub struct DocsShell {
    selected: usize,
    nav_menu: Option<Entity<Menu>>,
    page_views: Vec<Option<Entity<DocsPageView>>>,
}

impl Render for DocsShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected.min(DOC_PAGES.len().saturating_sub(1));
        self.selected = selected;

        let nav_menu = self.nav_menu(selected, cx);
        let page = &DOC_PAGES[selected];
        let page_view = self.page_view(selected, cx);

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
            .main_padding_xl()
            .child(
                div().size_full().child(
                    Card::new(
                        div()
                            .flex()
                            .flex_col()
                            .size_full()
                            .gap_3()
                            .child(Title::new(page.title).h3())
                            .child(div().flex_1().min_h_0().child(page_view)),
                    )
                    .no_shadow()
                    .no_shrink(),
                ),
            )
            .overlay(DocsPortalLayer)
    }
}

struct DocsPageView {
    document: MarkdownDocument,
    live_demos: Vec<Entity<LiveDemoHost>>,
    list_state: ListState,
}

impl DocsPageView {
    fn new(markdown: &'static str, cx: &mut Context<Self>) -> Self {
        let document = MarkdownDocument::parse(markdown);
        let mut demo_components = Vec::new();
        collect_live_demo_components(&document.blocks, &mut demo_components);
        let live_demos = demo_components
            .into_iter()
            .map(|component| cx.new(|cx| LiveDemoHost::new(component, cx)))
            .collect();

        let list_state = ListState::new(document.blocks.len(), ListAlignment::Top, px(640.0));

        Self {
            document,
            live_demos,
            list_state,
        }
    }
}

impl Render for DocsPageView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let blocks = self.document.blocks.clone();
        let live_demos = self.live_demos.clone();
        let theme = cx.global::<Config>().theme.clone();

        div()
            .relative()
            .size_full()
            .child(
                list(self.list_state.clone(), move |index, _window, _cx| {
                    let Some(block) = blocks.get(index) else {
                        return div().into_any_element();
                    };
                    let mut demo_index = live_demo_index_before(&blocks, index);
                    render_persistent_block(block, &theme, &live_demos, &mut demo_index)
                })
                .size_full(),
            )
            .child(VirtualScrollbar::new(self.list_state.clone()))
    }
}

fn live_demo_index_before(blocks: &[Block], end: usize) -> usize {
    blocks.iter().take(end).map(count_live_demos_in_block).sum()
}

fn count_live_demos_in_block(block: &Block) -> usize {
    match block {
        Block::LiveDemo { .. } => 1,
        Block::BlockQuote(children) => children.iter().map(count_live_demos_in_block).sum(),
        Block::List { items, .. } => items
            .iter()
            .flat_map(|item| item.iter())
            .map(count_live_demos_in_block)
            .sum(),
        Block::Paragraph(_) | Block::Heading { .. } | Block::CodeBlock { .. } | Block::Rule => 0,
    }
}

struct DocsPortalLayer;

impl IntoElement for DocsPortalLayer {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for DocsPortalLayer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        aura_components::message::render_messages(cx);
        aura_components::notification::render_notifications(cx);
        aura_components::image::render_image_preview(window, cx);
        aura_core::render_active_tooltip_in_window(window, cx);
        aura_core::render_active_popover_in_window(window, cx);
        aura_core::render_active_modal_in_window(window, cx);
        aura_core::render_active_drawer_in_window(window, cx);

        let passive_portals = if cx.has_global::<PassivePortal>() {
            std::mem::take(&mut cx.global_mut::<PassivePortal>().entries)
        } else {
            Vec::new()
        };
        let portals = if cx.has_global::<Portal>() {
            std::mem::take(&mut cx.global_mut::<Portal>().entries)
        } else {
            Vec::new()
        };

        let mut container = div().absolute().top_0().left_0().size_full();

        if !passive_portals.is_empty() {
            let mut passive_container = div()
                .id("aura-docs-passive-portal-layer")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .bg(gpui::transparent_black());

            for entry in passive_portals {
                passive_container = passive_container.child((entry.render)(window, cx));
            }

            container = container.child(passive_container);
        }

        if !portals.is_empty() {
            let mut active_container = div()
                .id("aura-docs-portal-layer")
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .cursor_default()
                .occlude()
                .bg(gpui::transparent_black())
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_move(|_, _, cx| {
                    cx.stop_propagation();
                });

            for entry in portals {
                active_container = active_container.child((entry.render)(window, cx));
            }

            container = container.child(active_container);
        }

        container.into_any_element()
    }
}

impl DocsShell {
    fn page_view(&mut self, selected: usize, cx: &mut Context<Self>) -> Entity<DocsPageView> {
        if let Some(page_view) = self.page_views.get(selected).and_then(Clone::clone) {
            return page_view;
        }

        let page_view = cx.new(|cx| DocsPageView::new(DOC_PAGES[selected].markdown, cx));
        if let Some(slot) = self.page_views.get_mut(selected) {
            *slot = Some(page_view.clone());
        }
        page_view
    }

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
    cx: &mut App,
) -> AnyElement {
    let mut rows = Vec::new();

    for (index, item_blocks) in items.into_iter().enumerate() {
        let marker = if ordered {
            format!("{}.", start + index as u64)
        } else {
            "•".to_string()
        };
        let item_children = item_blocks
            .into_iter()
            .map(|block| block.render(theme, cx))
            .collect::<Vec<_>>();

        rows.push(
            div()
                .relative()
                .pl_8()
                .child(
                    div()
                        .absolute()
                        .left_0()
                        .top_0()
                        .w(px(24.0))
                        .text_color(theme.neutral.text_3)
                        .child(marker),
                )
                .child(
                    div()
                        .w_full()
                        .child(Space::new().vertical().gap_sm().children(item_children)),
                ),
        );
    }

    div()
        .flex()
        .flex_col()
        .gap_2()
        .children(rows)
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

        assert!(titles.contains(&"Overview"));
        assert!(titles.contains(&"Quick Start"));
        assert!(titles.contains(&"Architecture"));
        assert!(titles.contains(&"About"));
        assert!(titles.contains(&"Button"));
        assert!(titles.contains(&"CodeBlock"));
        assert!(titles.contains(&"Input"));
        assert!(titles.contains(&"Switch"));
        assert!(titles.contains(&"Message"));
        assert!(titles.contains(&"Live Demo"));
        assert!(titles.contains(&"Authoring"));
        assert!(DOC_PAGES.len() >= 8);
    }

    #[test]
    fn core_docs_include_operational_gpui_and_aura_guidance() {
        assert!(INTRO_DOC.contains("纯原生 GPUI"));
        assert!(INTRO_DOC.contains("crates/aura-components"));

        for snippet in [
            "quick_start/deps_fedora.sh",
            "quick_start/deps_ubuntu.sh",
            "quick_start/deps_macos.sh",
            "quick_start/deps_windows.ps1",
            "quick_start/create_project.sh",
            "quick_start/app_cargo.toml",
            "quick_start/main_window.rs",
            "quick_start/component_view.rs",
            "quick_start/verify.sh",
        ] {
            assert!(QUICK_START_DOC.contains(&format!("src=\"{snippet}\"")));
            assert!(load_code_snippet(snippet).is_some());
        }

        assert!(QUICK_START_DOC.contains("Application"));
        assert!(QUICK_START_DOC.contains("Entity<View>"));
        assert!(QUICK_START_DOC.contains("cx.open_window"));
        assert!(ARCHITECTURE_DOC.contains("Workspace 分层"));
        assert!(ARCHITECTURE_DOC.contains("Live Demo 与代码片段"));
        assert!(ABOUT_DOC.contains("贡献文档的规则"));
    }

    #[test]
    fn component_docs_cover_gallery_registry_order() {
        let docs_titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        let gallery_keys = aura_gallery::demos::registry()
            .into_iter()
            .map(|entry| entry.name.split_whitespace().next().unwrap())
            .collect::<Vec<_>>();

        let gallery_source = include_str!("../../aura-gallery/src/demos/mod.rs");
        for key in &gallery_keys {
            assert!(docs_titles.contains(key), "missing docs page for {key}");
            assert!(
                gallery_source.contains(&format!("\"{key}\" => Some(")),
                "missing reusable gallery demo for {key}"
            );
        }

        let docs_component_order = docs_titles
            .iter()
            .filter(|title| gallery_keys.contains(title))
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(docs_component_order, gallery_keys);
    }

    #[test]
    fn component_effect_sections_keep_code_next_to_effect() {
        let gallery_keys = aura_gallery::demos::registry()
            .into_iter()
            .map(|entry| entry.name.split_whitespace().next().unwrap())
            .collect::<Vec<_>>();

        for page in DOC_PAGES {
            if !gallery_keys.contains(&page.title) || !page.markdown.contains("::AuraDemo") {
                continue;
            }

            assert!(
                !page.markdown.contains("\n## 效果\n"),
                "{} should place effects under a named example section",
                page.title
            );
            assert!(
                !page.markdown.contains("\n## 代码\n"),
                "{} should place code under the same named example section",
                page.title
            );

            let mut remaining = page.markdown;
            while let Some(effect_start) = remaining.find("::AuraDemo") {
                let after_effect = &remaining[effect_start..];
                let next_section = after_effect.find("\n## ").unwrap_or(after_effect.len());
                let current_example = &after_effect[..next_section];
                assert!(
                    current_example.contains("\n### 代码\n"),
                    "{} has an effect without adjacent code",
                    page.title
                );
                assert!(
                    current_example.contains("```rust src="),
                    "{} adjacent code should be sourced from an authored file",
                    page.title
                );
                remaining = &after_effect[next_section..];
                if remaining.is_empty() {
                    break;
                }
            }
        }
    }

    #[test]
    fn render_markdown_entrypoint_returns_native_element() {
        let _ = render_markdown("# Aura\n\nNative docs");
    }

    #[test]
    fn docs_shell_does_not_append_extra_smoke_button() {
        let source = include_str!("markdown.rs");

        assert_eq!(source.matches(r#"Button::new("Native action")"#).count(), 1);
        assert!(source.contains("DocsPageView"));
    }

    #[test]
    fn interactive_live_demos_are_persistent_entities() {
        let source = include_str!("markdown.rs");

        assert!(source.contains("struct LiveDemoHost"));
        assert!(source.contains("Entity<LiveDemoContent>"));
        assert!(source.contains("inputs: Vec<Entity<Input>>"));
        assert!(source.contains("switches: Vec<Entity<Switch>>"));
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
        let document = MarkdownDocument::parse("```rust src=\"button/types.rs\"\n```");
        let [
            Block::CodeBlock {
                language,
                source,
                code,
            },
        ] = document.blocks()
        else {
            panic!("expected one code block");
        };

        assert_eq!(language.as_ref().map(SharedString::as_ref), Some("rust"));
        assert_eq!(
            source.as_ref().map(SharedString::as_ref),
            Some("button/types.rs")
        );
        assert_eq!(code.as_ref(), "");
    }

    #[test]
    fn loads_external_code_snippets_by_path() {
        assert!(load_code_snippet("button/types.rs").is_some());
        assert!(load_code_snippet("code_block/theme.rs").is_some());
        assert!(load_code_snippet("quick_start/run.sh").is_some());
        assert!(load_code_snippet("missing.rs").is_none());
    }

    #[test]
    fn rust_snippets_are_imported_by_compile_check_harness() {
        let harness = include_str!("bin/check_snippets.rs");

        for snippet in [
            "button/types.rs",
            "input/basic.rs",
            "switch/callback.rs",
            "message/types.rs",
            "typography/paragraph.rs",
        ] {
            assert!(harness.contains(&format!("../../content/snippets/{snippet}")));
            assert!(load_code_snippet(snippet).is_some());
        }
        assert!(!harness.contains("quick_start/run.sh"));
    }

    #[test]
    fn code_blocks_render_with_horizontal_scroll_shell() {
        let source = include_str!("../content/pages/code_block.md");

        assert!(source.contains("src=\"code_block/basic.rs\""));
        assert!(source.contains("src=\"code_block/theme.rs\""));
        assert!(load_code_snippet("code_block/basic.rs").is_some());
    }

    #[test]
    fn docs_markdown_code_blocks_disable_selectable_text_for_scroll_performance() {
        let source = include_str!("markdown.rs");
        let render_code_block = &source[source
            .find("fn render_code_block(")
            .expect("render_code_block should exist")
            ..source
                .find("fn collect_live_demo_components(")
                .expect("collect_live_demo_components should follow")];

        assert!(render_code_block.contains("selectable(false)"));
    }

    #[test]
    fn docs_page_uses_gpui_virtual_list_for_visible_area_rendering() {
        let source = include_str!("markdown.rs");

        assert!(source.contains("ListState::new(document.blocks.len()"));
        assert!(source.contains("list(self.list_state.clone()"));
        assert!(source.contains("live_demo_index_before"));
    }

    #[test]
    fn docs_shell_uses_native_container_and_menu() {
        let source = include_str!("markdown.rs");

        assert!(source.contains("Container::new()"));
        assert!(source.contains("Menu::new()"));
        assert!(source.contains(".aside_scroll()"));
        assert!(source.contains("list(self.list_state.clone()"));
        assert!(source.contains("VirtualScrollbar::new"));
        let docs_shell_render = &source[source
            .find("impl Render for DocsShell")
            .expect("DocsShell render should exist")
            ..source
                .find("struct DocsPageView")
                .expect("DocsPageView should follow DocsShell")];
        assert!(!docs_shell_render.contains(".main_scroll()"));
        assert!(source.contains("DocsPortalLayer"));
    }

    #[test]
    fn markdown_lists_keep_inline_code_paragraphs_on_full_content_width() {
        let source = include_str!("markdown.rs");
        for (start, end) in [
            ("fn render_persistent_list(", "fn render_list("),
            ("fn render_list(", "fn inline_plain_text("),
        ] {
            let list_renderer = &source[source.find(start).expect("list renderer should exist")
                ..source.find(end).expect("next function should exist")];

            assert!(list_renderer.contains(".relative()"));
            assert!(list_renderer.contains(".pl_8()"));
            assert!(list_renderer.contains(".absolute()"));
            assert!(
                !list_renderer.contains(".flex_row()"),
                "list rows should not use flex-row layout because it narrows StyledText measurement and forces inline punctuation onto new lines"
            );
        }
    }

    #[test]
    fn markdown_inline_code_keeps_following_punctuation_in_same_paragraph() {
        let document = MarkdownDocument::parse(
            "- `crates/aura-components`：所有可复用组件，例如 `Button`、`Input`。",
        );

        let [Block::List { items, .. }] = document.blocks() else {
            panic!("expected list");
        };
        let [Block::Paragraph(segments)] = &items[0][..] else {
            panic!("expected list paragraph");
        };

        assert_eq!(segments[0].text.as_ref(), "crates/aura-components");
        assert!(segments[0].style.code);
        assert_eq!(segments[1].text.as_ref(), "：所有可复用组件，例如 ");
        assert_eq!(segments[2].text.as_ref(), "Button");
        assert!(segments[2].style.code);
        assert_eq!(segments[3].text.as_ref(), "、");
        assert_eq!(segments[4].text.as_ref(), "Input");
        assert!(segments[4].style.code);
        assert_eq!(segments[5].text.as_ref(), "。");
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
    fn component_docs_pair_live_effects_with_code_blocks() {
        let source = include_str!("../content/pages/button.md");

        for snippet in [
            "button/types.rs",
            "button/secondary.rs",
            "button/text.rs",
            "button/sizes.rs",
            "button/states.rs",
            "button/rounded.rs",
        ] {
            assert!(source.contains(&format!("src=\"{snippet}\"")));
            assert!(load_code_snippet(snippet).is_some());
        }
    }

    #[test]
    fn corrected_component_pages_split_each_effect_before_its_code() {
        for (page, first_demo, snippets) in [
            (
                include_str!("../content/pages/alert.md"),
                "AlertTypes",
                &["alert/types.rs", "alert/description.rs"][..],
            ),
            (
                include_str!("../content/pages/tag.md"),
                "TagTypes",
                &[
                    "tag/types.rs",
                    "tag/closable.rs",
                    "tag/themes.rs",
                    "tag/sizes.rs",
                    "tag/round.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/autocomplete.md"),
                "AutocompleteBasic",
                &[
                    "autocomplete/basic.rs",
                    "autocomplete/custom.rs",
                    "autocomplete/no_suffix.rs",
                    "autocomplete/disabled.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/avatar.md"),
                "AvatarShapes",
                &["avatar/shapes.rs", "avatar/sizes.rs", "avatar/content.rs"][..],
            ),
            (
                include_str!("../content/pages/badge.md"),
                "BadgeBasic",
                &["badge/basic.rs", "badge/max.rs", "badge/dot.rs"][..],
            ),
            (
                include_str!("../content/pages/input_number.md"),
                "InputNumberBasic",
                &[
                    "input_number/basic.rs",
                    "input_number/vertical.rs",
                    "input_number/precision.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/textarea.md"),
                "TextareaBasic",
                &["textarea/basic.rs", "textarea/limit.rs"][..],
            ),
            (
                include_str!("../content/pages/checkbox.md"),
                "CheckboxBasic",
                &[
                    "checkbox/basic.rs",
                    "checkbox/group.rs",
                    "checkbox/buttons.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/radio.md"),
                "RadioBasic",
                &["radio/basic.rs", "radio/group.rs", "radio/buttons.rs"][..],
            ),
            (
                include_str!("../content/pages/select.md"),
                "SelectBasic",
                &["select/basic.rs"][..],
            ),
            (
                include_str!("../content/pages/slider.md"),
                "SliderBasic",
                &["slider/basic.rs", "slider/step.rs"][..],
            ),
            (
                include_str!("../content/pages/rate.md"),
                "RateBasic",
                &["rate/basic.rs", "rate/custom.rs"][..],
            ),
        ] {
            assert!(!page.contains("## 完整示例"));
            assert!(!page.contains("src=\"gallery/"));
            assert!(page.contains(&format!("::AuraDemo{{component=\"{first_demo}\"}}::")));

            for snippet in snippets {
                let marker = format!("src=\"{snippet}\"");
                let marker_index = page.find(&marker).expect("snippet should be referenced");
                let preceding_effect = page[..marker_index]
                    .rfind("### 效果")
                    .expect("code should follow an effect section");
                let preceding_code = page[..marker_index]
                    .rfind("### 代码")
                    .expect("snippet should be under a code section");

                assert!(
                    preceding_effect < preceding_code,
                    "each code snippet must be paired after its effect"
                );
                assert!(load_code_snippet(snippet).is_some());
            }
        }
    }

    #[test]
    fn live_demo_renderer_maps_button_to_native_aura_component() {
        let source = include_str!("../content/snippets/live_demo/button.rs");

        assert!(source.contains("Button::new(\"Native Button\")"));
        assert!(source.contains("toast_success!"));
        assert!(source.contains(".on_click(|_, _, _|"));
    }
}
