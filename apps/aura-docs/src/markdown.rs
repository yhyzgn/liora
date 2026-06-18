use aura_components::{
    Affix, AffixPosition, Alert, AlertType, Anchor, AnchorLink, AnchorTarget, Autocomplete,
    AutocompleteItem, Avatar, Backtop, Badge, BadgeType, Button, ButtonColors, Card, Checkbox,
    CheckboxGroup, CheckboxOptionStyle, CodeBlock as AuraCodeBlock, CodeDiagnostic, CodeEditor,
    CodeLanguage, CodeTheme, Container, Dropdown, Flex, Form, FormItem, HorizontalList, Image,
    Input, InputNumber, InputNumberControlsPosition, Link, Loading, Menu, MenuMode,
    NotificationType, Paragraph, Popconfirm, Popover, Preview, Progress, ProgressStatus, QrCode,
    QrEcLevel, QrFinderStyle, QrGradientDirection, QrModuleStyle, Radio, RadioGroup,
    RadioOptionStyle, Rate, Result as AuraResult, ResultStatus, Select, Skeleton, SkeletonItem,
    SkeletonVariant, Slider, Space, Statistic, Switch, Tag as AuraTag, Text, Textarea, Timer,
    TimerFormat, TimerUnit, Title, Transfer, TransferItem, Tree, TreeNode, Upload, UploadFile,
    UploadStatus, VirtualizedList, VirtualizedTable, VirtualizedTree, show_notification,
    toast_error, toast_info, toast_success, toast_warning,
};
use aura_core::{Config, PassivePortal, Placement, Portal, clear_popover};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, AnyView, App, Component, Context, Entity, FontWeight, IntoElement, Render,
    RenderOnce, ScrollHandle, SharedString, WeakEntity, Window, div, prelude::*, px, rgb,
};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

const INTRO_DOC: &str = include_str!("../content/pages/overview.md");
const QUICK_START_DOC: &str = include_str!("../content/pages/quick_start.md");
const ARCHITECTURE_DOC: &str = include_str!("../content/pages/architecture.md");
const PACKAGING_WORKFLOW_DOC: &str = include_str!("../content/pages/packaging_workflow.md");
const ADOPTION_DOC: &str = include_str!("../content/pages/adoption.md");
const DASHBOARD_APP_DOC: &str = include_str!("../content/pages/dashboard_app.md");
const DASHBOARD_PATTERNS_DOC: &str = include_str!("../content/pages/dashboard_patterns.md");
const DASHBOARD_STATE_DOC: &str = include_str!("../content/pages/dashboard_state.md");
const ABOUT_DOC: &str = include_str!("../content/pages/about.md");

const AFFIX_DOC: &str = include_str!("../content/pages/affix.md");
const ALERT_DOC: &str = include_str!("../content/pages/alert.md");
const AREA_CHART_DOC: &str = include_str!("../content/pages/area_chart.md");
const ANCHOR_DOC: &str = include_str!("../content/pages/anchor.md");
const AUTOCOMPLETE_DOC: &str = include_str!("../content/pages/autocomplete.md");
const AVATAR_DOC: &str = include_str!("../content/pages/avatar.md");
const BACKTOP_DOC: &str = include_str!("../content/pages/backtop.md");
const BADGE_DOC: &str = include_str!("../content/pages/badge.md");
const BAR_CHART_DOC: &str = include_str!("../content/pages/bar_chart.md");
const BREADCRUMB_DOC: &str = include_str!("../content/pages/breadcrumb.md");
const CALENDAR_DOC: &str = include_str!("../content/pages/calendar.md");
const BUTTON_DOC: &str = include_str!("../content/pages/button.md");
const CAROUSEL_DOC: &str = include_str!("../content/pages/carousel.md");
const CARD_DOC: &str = include_str!("../content/pages/card.md");
const CASCADER_DOC: &str = include_str!("../content/pages/cascader.md");
const CHECKBOX_DOC: &str = include_str!("../content/pages/checkbox.md");
const CODE_BLOCK_DOC: &str = include_str!("../content/pages/code_block.md");
const CODE_EDITOR_DOC: &str = include_str!("../content/pages/code_editor.md");
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
const HEAT_BAR_DOC: &str = include_str!("../content/pages/heat_bar.md");
const LABEL_DOC: &str = include_str!("../content/pages/label.md");
const OPERATION_DOC: &str = include_str!("../content/pages/operation.md");
const SEGMENT_RATIO_BAR_DOC: &str = include_str!("../content/pages/segment_ratio_bar.md");
const SIGNAL_METER_DOC: &str = include_str!("../content/pages/signal_meter.md");
const ICON_DOC: &str = include_str!("../content/pages/icon.md");
const IMAGE_DOC: &str = include_str!("../content/pages/image.md");
const HORIZONTAL_LIST_DOC: &str = include_str!("../content/pages/horizontal_list.md");
const INPUT_DOC: &str = include_str!("../content/pages/input.md");
const INPUT_TAG_DOC: &str = include_str!("../content/pages/input_tag.md");
const INPUT_NUMBER_DOC: &str = include_str!("../content/pages/input_number.md");
const LAYOUT_DOC: &str = include_str!("../content/pages/layout.md");
const LINK_DOC: &str = include_str!("../content/pages/link.md");
const LINE_CHART_DOC: &str = include_str!("../content/pages/line_chart.md");
const LOADING_DOC: &str = include_str!("../content/pages/loading.md");
const MENTION_DOC: &str = include_str!("../content/pages/mention.md");
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
const QR_CODE_DOC: &str = include_str!("../content/pages/qr_code.md");
const PIE_CHART_DOC: &str = include_str!("../content/pages/pie_chart.md");
const RING_CHART_DOC: &str = include_str!("../content/pages/ring_chart.md");
const RADIO_DOC: &str = include_str!("../content/pages/radio.md");
const RATE_DOC: &str = include_str!("../content/pages/rate.md");
const RESULT_DOC: &str = include_str!("../content/pages/result.md");
const SCROLLBAR_DOC: &str = include_str!("../content/pages/scrollbar.md");
const SEGMENTED_DOC: &str = include_str!("../content/pages/segmented.md");
const SELECT_DOC: &str = include_str!("../content/pages/select.md");
const SKELETON_DOC: &str = include_str!("../content/pages/skeleton.md");
const SPARKLINE_DOC: &str = include_str!("../content/pages/sparkline.md");
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
const TIMER_DOC: &str = include_str!("../content/pages/timer.md");
const TIMELINE_DOC: &str = include_str!("../content/pages/timeline.md");
const TOUR_DOC: &str = include_str!("../content/pages/tour.md");
const TOOLTIP_DOC: &str = include_str!("../content/pages/tooltip.md");
const TRAY_DOC: &str = include_str!("../content/pages/tray.md");
const TRANSFER_DOC: &str = include_str!("../content/pages/transfer.md");
const TREE_SELECT_DOC: &str = include_str!("../content/pages/tree_select.md");
const TREE_DOC: &str = include_str!("../content/pages/tree.md");
const TYPOGRAPHY_DOC: &str = include_str!("../content/pages/typography.md");
const UPLOAD_DOC: &str = include_str!("../content/pages/upload.md");
const WATERMARK_DOC: &str = include_str!("../content/pages/watermark.md");
const VIRTUALIZED_LIST_DOC: &str = include_str!("../content/pages/virtualized_list.md");
const VIRTUALIZED_TABLE_DOC: &str = include_str!("../content/pages/virtualized_table.md");
const VIRTUALIZED_TREE_DOC: &str = include_str!("../content/pages/virtualized_tree.md");

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
        title: "Packaging Workflow",
        markdown: PACKAGING_WORKFLOW_DOC,
    },
    DocPage {
        title: "Adoption Guide",
        markdown: ADOPTION_DOC,
    },
    DocPage {
        title: "Dashboard App",
        markdown: DASHBOARD_APP_DOC,
    },
    DocPage {
        title: "Dashboard Patterns",
        markdown: DASHBOARD_PATTERNS_DOC,
    },
    DocPage {
        title: "Dashboard State",
        markdown: DASHBOARD_STATE_DOC,
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
        title: "Calendar",
        markdown: CALENDAR_DOC,
    },
    DocPage {
        title: "Card",
        markdown: CARD_DOC,
    },
    DocPage {
        title: "Carousel",
        markdown: CAROUSEL_DOC,
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
        title: "CodeEditor",
        markdown: CODE_EDITOR_DOC,
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
        title: "HeatBar",
        markdown: HEAT_BAR_DOC,
    },
    DocPage {
        title: "HorizontalList",
        markdown: HORIZONTAL_LIST_DOC,
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
        title: "InputTag",
        markdown: INPUT_TAG_DOC,
    },
    DocPage {
        title: "Label",
        markdown: LABEL_DOC,
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
        title: "Mention",
        markdown: MENTION_DOC,
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
        title: "Operation",
        markdown: OPERATION_DOC,
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
        title: "QrCode",
        markdown: QR_CODE_DOC,
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
        title: "SegmentRatioBar",
        markdown: SEGMENT_RATIO_BAR_DOC,
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
        title: "SignalMeter",
        markdown: SIGNAL_METER_DOC,
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
        title: "Sparkline",
        markdown: SPARKLINE_DOC,
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
        title: "Timer",
        markdown: TIMER_DOC,
    },
    DocPage {
        title: "Tooltip",
        markdown: TOOLTIP_DOC,
    },
    DocPage {
        title: "Tour",
        markdown: TOUR_DOC,
    },
    DocPage {
        title: "Transfer",
        markdown: TRANSFER_DOC,
    },
    DocPage {
        title: "Tray",
        markdown: TRAY_DOC,
    },
    DocPage {
        title: "Tree",
        markdown: TREE_DOC,
    },
    DocPage {
        title: "TreeSelect",
        markdown: TREE_SELECT_DOC,
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
        title: "VirtualizedList",
        markdown: VIRTUALIZED_LIST_DOC,
    },
    DocPage {
        title: "VirtualizedTable",
        markdown: VIRTUALIZED_TABLE_DOC,
    },
    DocPage {
        title: "VirtualizedTree",
        markdown: VIRTUALIZED_TREE_DOC,
    },
    DocPage {
        title: "Watermark",
        markdown: WATERMARK_DOC,
    },
    DocPage {
        title: "AreaChart",
        markdown: AREA_CHART_DOC,
    },
    DocPage {
        title: "BarChart",
        markdown: BAR_CHART_DOC,
    },
    DocPage {
        title: "LineChart",
        markdown: LINE_CHART_DOC,
    },
    DocPage {
        title: "PieChart",
        markdown: PIE_CHART_DOC,
    },
    DocPage {
        title: "RingChart",
        markdown: RING_CHART_DOC,
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
        "virtualized_list/basic.rs" => Some(include_str!(
            "../content/snippets/virtualized_list/basic.rs"
        )),
        "virtualized_list/draggable.rs" => Some(include_str!(
            "../content/snippets/virtualized_list/draggable.rs"
        )),
        "horizontal_list/draggable.rs" => Some(include_str!(
            "../content/snippets/horizontal_list/draggable.rs"
        )),
        "horizontal_list/divider.rs" => Some(include_str!(
            "../content/snippets/horizontal_list/divider.rs"
        )),
        "horizontal_list/basic.rs" => {
            Some(include_str!("../content/snippets/horizontal_list/basic.rs"))
        }
        "affix/top.rs" => Some(include_str!("../content/snippets/affix/top.rs")),
        "affix/bottom.rs" => Some(include_str!("../content/snippets/affix/bottom.rs")),
        "affix/container.rs" => Some(include_str!("../content/snippets/affix/container.rs")),
        "anchor/basic.rs" => Some(include_str!("../content/snippets/anchor/basic.rs")),
        "anchor/nested.rs" => Some(include_str!("../content/snippets/anchor/nested.rs")),
        "anchor/targets.rs" => Some(include_str!("../content/snippets/anchor/targets.rs")),
        "backtop/basic.rs" => Some(include_str!("../content/snippets/backtop/basic.rs")),
        "backtop/custom.rs" => Some(include_str!("../content/snippets/backtop/custom.rs")),
        "backtop/container.rs" => Some(include_str!("../content/snippets/backtop/container.rs")),
        "form/basic.rs" => Some(include_str!("../content/snippets/form/basic.rs")),
        "form/validation.rs" => Some(include_str!("../content/snippets/form/validation.rs")),
        "form/inline.rs" => Some(include_str!("../content/snippets/form/inline.rs")),
        "preview/image_trigger.rs" => {
            Some(include_str!("../content/snippets/preview/image_trigger.rs"))
        }
        "preview/custom_trigger.rs" => Some(include_str!(
            "../content/snippets/preview/custom_trigger.rs"
        )),
        "preview/escape.rs" => Some(include_str!("../content/snippets/preview/escape.rs")),
        "tour/basic.rs" => Some(include_str!("../content/snippets/tour/basic.rs")),
        "tour/middle.rs" => Some(include_str!("../content/snippets/tour/middle.rs")),
        "tour/no_mask.rs" => Some(include_str!("../content/snippets/tour/no_mask.rs")),
        "tour/close_policy.rs" => Some(include_str!("../content/snippets/tour/close_policy.rs")),
        "calendar/events.rs" => Some(include_str!("../content/snippets/calendar/events.rs")),
        "calendar/range.rs" => Some(include_str!("../content/snippets/calendar/range.rs")),
        "carousel/autoplay.rs" => Some(include_str!("../content/snippets/carousel/autoplay.rs")),
        "carousel/basic.rs" => Some(include_str!("../content/snippets/carousel/basic.rs")),
        "carousel/custom.rs" => Some(include_str!("../content/snippets/carousel/custom.rs")),
        "input_tag/basic.rs" => Some(include_str!("../content/snippets/input_tag/basic.rs")),
        "input_tag/duplicates.rs" => {
            Some(include_str!("../content/snippets/input_tag/duplicates.rs"))
        }
        "input_tag/limited.rs" => Some(include_str!("../content/snippets/input_tag/limited.rs")),
        "mention/disabled.rs" => Some(include_str!("../content/snippets/mention/disabled.rs")),
        "mention/issues.rs" => Some(include_str!("../content/snippets/mention/issues.rs")),
        "mention/people.rs" => Some(include_str!("../content/snippets/mention/people.rs")),
        "progress/gradient_complete.rs" => Some(include_str!(
            "../content/snippets/progress/gradient_complete.rs"
        )),
        "tree_select/filterable.rs" => Some(include_str!(
            "../content/snippets/tree_select/filterable.rs"
        )),
        "tree_select/multiple.rs" => {
            Some(include_str!("../content/snippets/tree_select/multiple.rs"))
        }
        "tree_select/single.rs" => Some(include_str!("../content/snippets/tree_select/single.rs")),
        "virtualized_table/basic.rs" => Some(include_str!(
            "../content/snippets/virtualized_table/basic.rs"
        )),
        "virtualized_table/sortable.rs" => Some(include_str!(
            "../content/snippets/virtualized_table/sortable.rs"
        )),
        "virtualized_tree/basic.rs" => Some(include_str!(
            "../content/snippets/virtualized_tree/basic.rs"
        )),
        "virtualized_tree/checkable.rs" => Some(include_str!(
            "../content/snippets/virtualized_tree/checkable.rs"
        )),
        "watermark/cover.rs" => Some(include_str!("../content/snippets/watermark/cover.rs")),
        "watermark/custom.rs" => Some(include_str!("../content/snippets/watermark/custom.rs")),
        "watermark/header.rs" => Some(include_str!("../content/snippets/watermark/header.rs")),
        "button/types.rs" => Some(include_str!("../content/snippets/button/types.rs")),
        "button/secondary.rs" => Some(include_str!("../content/snippets/button/secondary.rs")),
        "button/text.rs" => Some(include_str!("../content/snippets/button/text.rs")),
        "button/sizes.rs" => Some(include_str!("../content/snippets/button/sizes.rs")),
        "button/states.rs" => Some(include_str!("../content/snippets/button/states.rs")),
        "button/rounded.rs" => Some(include_str!("../content/snippets/button/rounded.rs")),
        "button/custom_colors.rs" => {
            Some(include_str!("../content/snippets/button/custom_colors.rs"))
        }
        "button/gradient.rs" => Some(include_str!("../content/snippets/button/gradient.rs")),
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
        "area_chart/basic.rs" => Some(include_str!("../content/snippets/area_chart/basic.rs")),
        "area_chart/downsample.rs" => {
            Some(include_str!("../content/snippets/area_chart/downsample.rs"))
        }
        "area_chart/overlay.rs" => Some(include_str!("../content/snippets/area_chart/overlay.rs")),
        "area_chart/stacked.rs" => Some(include_str!("../content/snippets/area_chart/stacked.rs")),
        "area_chart/custom.rs" => Some(include_str!("../content/snippets/area_chart/custom.rs")),
        "badge/basic.rs" => Some(include_str!("../content/snippets/badge/basic.rs")),
        "badge/max.rs" => Some(include_str!("../content/snippets/badge/max.rs")),
        "badge/dot.rs" => Some(include_str!("../content/snippets/badge/dot.rs")),
        "input_number/basic.rs" => Some(include_str!("../content/snippets/input_number/basic.rs")),
        "bar_chart/basic.rs" => Some(include_str!("../content/snippets/bar_chart/basic.rs")),
        "bar_chart/grouped.rs" => Some(include_str!("../content/snippets/bar_chart/grouped.rs")),
        "bar_chart/gradient.rs" => Some(include_str!("../content/snippets/bar_chart/gradient.rs")),
        "bar_chart/per_bar_gradient.rs" => Some(include_str!(
            "../content/snippets/bar_chart/per_bar_gradient.rs",
        )),
        "bar_chart/stacked.rs" => Some(include_str!("../content/snippets/bar_chart/stacked.rs")),
        "bar_chart/custom.rs" => Some(include_str!("../content/snippets/bar_chart/custom.rs")),
        "label/basic.rs" => Some(include_str!("../content/snippets/label/basic.rs")),
        "operation/basic.rs" => Some(include_str!("../content/snippets/operation/basic.rs")),
        "segment_ratio_bar/top.rs" => {
            Some(include_str!("../content/snippets/segment_ratio_bar/top.rs"))
        }
        "segment_ratio_bar/bottom.rs" => Some(include_str!(
            "../content/snippets/segment_ratio_bar/bottom.rs"
        )),
        "segment_ratio_bar/both.rs" => Some(include_str!(
            "../content/snippets/segment_ratio_bar/both.rs"
        )),
        "segment_ratio_bar/hidden.rs" => Some(include_str!(
            "../content/snippets/segment_ratio_bar/hidden.rs"
        )),
        "segment_ratio_bar/pattern.rs" => Some(include_str!(
            "../content/snippets/segment_ratio_bar/pattern.rs"
        )),
        "segment_ratio_bar/compact.rs" => Some(include_str!(
            "../content/snippets/segment_ratio_bar/compact.rs"
        )),
        "heat_bar/events.rs" => Some(include_str!("../content/snippets/heat_bar/events.rs")),
        "signal_meter/wifi.rs" => Some(include_str!("../content/snippets/signal_meter/wifi.rs")),
        "signal_meter/levels.rs" => {
            Some(include_str!("../content/snippets/signal_meter/levels.rs"))
        }
        "signal_meter/threshold_colors.rs" => Some(include_str!(
            "../content/snippets/signal_meter/threshold_colors.rs"
        )),
        "signal_meter/mobile.rs" => {
            Some(include_str!("../content/snippets/signal_meter/mobile.rs"))
        }
        "tag/flow.rs" => Some(include_str!("../content/snippets/tag/flow.rs")),
        "bar_chart/standalone.rs" => {
            Some(include_str!("../content/snippets/bar_chart/standalone.rs"))
        }
        "bar_chart/standalone_styles.rs" => Some(include_str!(
            "../content/snippets/bar_chart/standalone_styles.rs"
        )),
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
        "checkbox/custom.rs" => Some(include_str!("../content/snippets/checkbox/custom.rs")),
        "radio/basic.rs" => Some(include_str!("../content/snippets/radio/basic.rs")),
        "radio/group.rs" => Some(include_str!("../content/snippets/radio/group.rs")),
        "radio/buttons.rs" => Some(include_str!("../content/snippets/radio/buttons.rs")),
        "radio/custom.rs" => Some(include_str!("../content/snippets/radio/custom.rs")),
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
        "code_editor/basic.rs" => Some(include_str!("../content/snippets/code_editor/basic.rs")),
        "code_editor/diagnostics.rs" => Some(include_str!(
            "../content/snippets/code_editor/diagnostics.rs"
        )),
        "input/basic.rs" => Some(include_str!("../content/snippets/input/basic.rs")),
        "input/password.rs" => Some(include_str!("../content/snippets/input/password.rs")),
        "input/affix.rs" => Some(include_str!("../content/snippets/input/affix.rs")),
        "input/states.rs" => Some(include_str!("../content/snippets/input/states.rs")),
        "switch/basic.rs" => Some(include_str!("../content/snippets/switch/basic.rs")),
        "switch/disabled.rs" => Some(include_str!("../content/snippets/switch/disabled.rs")),
        "switch/callback.rs" => Some(include_str!("../content/snippets/switch/callback.rs")),
        "message/types.rs" => Some(include_str!("../content/snippets/message/types.rs")),
        "message/formatting.rs" => Some(include_str!("../content/snippets/message/formatting.rs")),
        "qr_code/basic.rs" => Some(include_str!("../content/snippets/qr_code/basic.rs")),
        "qr_code/style.rs" => Some(include_str!("../content/snippets/qr_code/style.rs")),
        "qr_code/decode.rs" => Some(include_str!("../content/snippets/qr_code/decode.rs")),
        "progress/basic.rs" => Some(include_str!("../content/snippets/progress/basic.rs")),
        "progress/inside.rs" => Some(include_str!("../content/snippets/progress/inside.rs")),
        "progress/status.rs" => Some(include_str!("../content/snippets/progress/status.rs")),
        "progress/color.rs" => Some(include_str!("../content/snippets/progress/color.rs")),
        "progress/circle.rs" => Some(include_str!("../content/snippets/progress/circle.rs")),
        "progress/custom.rs" => Some(include_str!("../content/snippets/progress/custom.rs")),
        "progress/circle_gradient.rs" => Some(include_str!(
            "../content/snippets/progress/circle_gradient.rs"
        )),
        "loading/basic.rs" => Some(include_str!("../content/snippets/loading/basic.rs")),
        "loading/fullscreen.rs" => Some(include_str!("../content/snippets/loading/fullscreen.rs")),
        "link/variants.rs" => Some(include_str!("../content/snippets/link/variants.rs")),
        "link/underline.rs" => Some(include_str!("../content/snippets/link/underline.rs")),
        "link/states.rs" => Some(include_str!("../content/snippets/link/states.rs")),
        "link/icons.rs" => Some(include_str!("../content/snippets/link/icons.rs")),
        "line_chart/empty.rs" => Some(include_str!("../content/snippets/line_chart/empty.rs")),
        "line_chart/multi.rs" => Some(include_str!("../content/snippets/line_chart/multi.rs")),
        "line_chart/basic.rs" => Some(include_str!("../content/snippets/line_chart/basic.rs")),
        "line_chart/downsample.rs" => {
            Some(include_str!("../content/snippets/line_chart/downsample.rs"))
        }
        "line_chart/custom.rs" => Some(include_str!("../content/snippets/line_chart/custom.rs")),
        "line_chart/line_styles.rs" => Some(include_str!(
            "../content/snippets/line_chart/line_styles.rs"
        )),
        "pie_chart/basic.rs" => Some(include_str!("../content/snippets/pie_chart/basic.rs")),
        "pie_chart/custom.rs" => Some(include_str!("../content/snippets/pie_chart/custom.rs")),
        "ring_chart/basic.rs" => Some(include_str!("../content/snippets/ring_chart/basic.rs")),
        "ring_chart/custom.rs" => Some(include_str!("../content/snippets/ring_chart/custom.rs")),
        "ring_chart/external.rs" => {
            Some(include_str!("../content/snippets/ring_chart/external.rs"))
        }
        "sparkline/basic.rs" => Some(include_str!("../content/snippets/sparkline/basic.rs")),
        "sparkline/cards.rs" => Some(include_str!("../content/snippets/sparkline/cards.rs")),
        "sparkline/downsample.rs" => {
            Some(include_str!("../content/snippets/sparkline/downsample.rs"))
        }
        "sparkline/area.rs" => Some(include_str!("../content/snippets/sparkline/area.rs")),
        "sparkline/styles.rs" => Some(include_str!("../content/snippets/sparkline/styles.rs")),
        "skeleton/basic.rs" => Some(include_str!("../content/snippets/skeleton/basic.rs")),
        "skeleton/variants.rs" => Some(include_str!("../content/snippets/skeleton/variants.rs")),
        "skeleton/template.rs" => Some(include_str!("../content/snippets/skeleton/template.rs")),
        "result/success.rs" => Some(include_str!("../content/snippets/result/success.rs")),
        "result/statuses.rs" => Some(include_str!("../content/snippets/result/statuses.rs")),
        "statistic/basic.rs" => Some(include_str!("../content/snippets/statistic/basic.rs")),
        "statistic/affix.rs" => Some(include_str!("../content/snippets/statistic/affix.rs")),
        "statistic/icons.rs" => Some(include_str!("../content/snippets/statistic/icons.rs")),
        "statistic/layout.rs" => Some(include_str!("../content/snippets/statistic/layout.rs")),
        "card/basic.rs" => Some(include_str!("../content/snippets/card/basic.rs")),
        "card/footer.rs" => Some(include_str!("../content/snippets/card/footer.rs")),
        "empty/basic.rs" => Some(include_str!("../content/snippets/empty/basic.rs")),
        "empty/description.rs" => Some(include_str!("../content/snippets/empty/description.rs")),
        "empty/image.rs" => Some(include_str!("../content/snippets/empty/image.rs")),
        "empty/extra.rs" => Some(include_str!("../content/snippets/empty/extra.rs")),
        "steps/basic.rs" => Some(include_str!("../content/snippets/steps/basic.rs")),
        "steps/description.rs" => Some(include_str!("../content/snippets/steps/description.rs")),
        "steps/status.rs" => Some(include_str!("../content/snippets/steps/status.rs")),
        "steps/vertical.rs" => Some(include_str!("../content/snippets/steps/vertical.rs")),
        "timeline/basic.rs" => Some(include_str!("../content/snippets/timeline/basic.rs")),
        "timeline/custom.rs" => Some(include_str!("../content/snippets/timeline/custom.rs")),
        "timer/result.rs" => Some(include_str!("../content/snippets/timer/result.rs")),
        "timer/units.rs" => Some(include_str!("../content/snippets/timer/units.rs")),
        "timer/count_down.rs" => Some(include_str!("../content/snippets/timer/count_down.rs")),
        "timer/clock.rs" => Some(include_str!("../content/snippets/timer/clock.rs")),
        "timer/count_up.rs" => Some(include_str!("../content/snippets/timer/count_up.rs")),
        "timeline/placement.rs" => Some(include_str!("../content/snippets/timeline/placement.rs")),
        "timeline/reverse.rs" => Some(include_str!("../content/snippets/timeline/reverse.rs")),
        "breadcrumb/basic.rs" => Some(include_str!("../content/snippets/breadcrumb/basic.rs")),
        "breadcrumb/icon.rs" => Some(include_str!("../content/snippets/breadcrumb/icon.rs")),
        "breadcrumb/separator.rs" => {
            Some(include_str!("../content/snippets/breadcrumb/separator.rs"))
        }
        "breadcrumb/separator_icon.rs" => Some(include_str!(
            "../content/snippets/breadcrumb/separator_icon.rs"
        )),
        "breadcrumb/clickable.rs" => {
            Some(include_str!("../content/snippets/breadcrumb/clickable.rs"))
        }
        "page_header/basic.rs" => Some(include_str!("../content/snippets/page_header/basic.rs")),
        "page_header/extra.rs" => Some(include_str!("../content/snippets/page_header/extra.rs")),
        "page_header/full.rs" => Some(include_str!("../content/snippets/page_header/full.rs")),
        "segmented/basic.rs" => Some(include_str!("../content/snippets/segmented/basic.rs")),
        "segmented/disabled.rs" => Some(include_str!("../content/snippets/segmented/disabled.rs")),
        "segmented/block.rs" => Some(include_str!("../content/snippets/segmented/block.rs")),
        "tooltip/basic.rs" => Some(include_str!("../content/snippets/tooltip/basic.rs")),
        "tooltip/more.rs" => Some(include_str!("../content/snippets/tooltip/more.rs")),
        "popover/basic.rs" => Some(include_str!("../content/snippets/popover/basic.rs")),
        "popover/placements.rs" => Some(include_str!("../content/snippets/popover/placements.rs")),
        "popover/close_strategy.rs" => Some(include_str!(
            "../content/snippets/popover/close_strategy.rs"
        )),
        "popconfirm/basic.rs" => Some(include_str!("../content/snippets/popconfirm/basic.rs")),
        "popconfirm/placements.rs" => {
            Some(include_str!("../content/snippets/popconfirm/placements.rs"))
        }
        "popconfirm/custom_text.rs" => Some(include_str!(
            "../content/snippets/popconfirm/custom_text.rs"
        )),
        "dropdown/basic.rs" => Some(include_str!("../content/snippets/dropdown/basic.rs")),
        "dropdown/close_strategy.rs" => Some(include_str!(
            "../content/snippets/dropdown/close_strategy.rs"
        )),
        "dropdown/placements.rs" => {
            Some(include_str!("../content/snippets/dropdown/placements.rs"))
        }
        "message_box/basic.rs" => Some(include_str!("../content/snippets/message_box/basic.rs")),
        "message_box/manual_close.rs" => Some(include_str!(
            "../content/snippets/message_box/manual_close.rs"
        )),
        "notification/types.rs" => Some(include_str!("../content/snippets/notification/types.rs")),
        "upload/basic.rs" => Some(include_str!("../content/snippets/upload/basic.rs")),
        "upload/drag.rs" => Some(include_str!("../content/snippets/upload/drag.rs")),
        "upload/picture_card.rs" => {
            Some(include_str!("../content/snippets/upload/picture_card.rs"))
        }
        "upload/limits.rs" => Some(include_str!("../content/snippets/upload/limits.rs")),
        "transfer/basic.rs" => Some(include_str!("../content/snippets/transfer/basic.rs")),
        "transfer/filterable.rs" => {
            Some(include_str!("../content/snippets/transfer/filterable.rs"))
        }
        "transfer/disabled.rs" => Some(include_str!("../content/snippets/transfer/disabled.rs")),
        "tree/basic.rs" => Some(include_str!("../content/snippets/tree/basic.rs")),
        "tree/checkable.rs" => Some(include_str!("../content/snippets/tree/checkable.rs")),
        "menu/horizontal.rs" => Some(include_str!("../content/snippets/menu/horizontal.rs")),
        "menu/vertical.rs" => Some(include_str!("../content/snippets/menu/vertical.rs")),
        "menu/collapsed.rs" => Some(include_str!("../content/snippets/menu/collapsed.rs")),
        "pagination/basic.rs" => Some(include_str!("../content/snippets/pagination/basic.rs")),
        "pagination/background.rs" => {
            Some(include_str!("../content/snippets/pagination/background.rs"))
        }
        "pagination/advanced.rs" => {
            Some(include_str!("../content/snippets/pagination/advanced.rs"))
        }
        "tabs/basic.rs" => Some(include_str!("../content/snippets/tabs/basic.rs")),
        "tabs/stretch.rs" => Some(include_str!("../content/snippets/tabs/stretch.rs")),
        "tabs/card.rs" => Some(include_str!("../content/snippets/tabs/card.rs")),
        "tabs/border_card.rs" => Some(include_str!("../content/snippets/tabs/border_card.rs")),
        "tabs/position.rs" => Some(include_str!("../content/snippets/tabs/position.rs")),
        "tabs/editable.rs" => Some(include_str!("../content/snippets/tabs/editable.rs")),
        "layout/divider.rs" => Some(include_str!("../content/snippets/layout/divider.rs")),
        "layout/space.rs" => Some(include_str!("../content/snippets/layout/space.rs")),
        "layout/grid.rs" => Some(include_str!("../content/snippets/layout/grid.rs")),
        "container/space.rs" => Some(include_str!("../content/snippets/container/space.rs")),
        "container/divider.rs" => Some(include_str!("../content/snippets/container/divider.rs")),
        "container/layout.rs" => Some(include_str!("../content/snippets/container/layout.rs")),
        "splitter/basic.rs" => Some(include_str!("../content/snippets/splitter/basic.rs")),
        "scrollbar/basic.rs" => Some(include_str!("../content/snippets/scrollbar/basic.rs")),
        "descriptions/basic.rs" => Some(include_str!("../content/snippets/descriptions/basic.rs")),
        "descriptions/border.rs" => {
            Some(include_str!("../content/snippets/descriptions/border.rs"))
        }
        "descriptions/vertical.rs" => {
            Some(include_str!("../content/snippets/descriptions/vertical.rs"))
        }
        "table/sortable.rs" => Some(include_str!("../content/snippets/table/sortable.rs")),
        "table/basic.rs" => Some(include_str!("../content/snippets/table/basic.rs")),
        "table/stripe_border.rs" => {
            Some(include_str!("../content/snippets/table/stripe_border.rs"))
        }
        "table/fixed_header.rs" => Some(include_str!("../content/snippets/table/fixed_header.rs")),
        "table/loading.rs" => Some(include_str!("../content/snippets/table/loading.rs")),
        "table/empty.rs" => Some(include_str!("../content/snippets/table/empty.rs")),
        "color_picker/basic.rs" => Some(include_str!("../content/snippets/color_picker/basic.rs")),
        "color_picker/presets.rs" => {
            Some(include_str!("../content/snippets/color_picker/presets.rs"))
        }
        "color_picker/compact.rs" => {
            Some(include_str!("../content/snippets/color_picker/compact.rs"))
        }
        "color_picker/disabled.rs" => {
            Some(include_str!("../content/snippets/color_picker/disabled.rs"))
        }
        "time_picker/basic.rs" => Some(include_str!("../content/snippets/time_picker/basic.rs")),
        "time_picker/formatted.rs" => {
            Some(include_str!("../content/snippets/time_picker/formatted.rs"))
        }
        "time_picker/stepped.rs" => {
            Some(include_str!("../content/snippets/time_picker/stepped.rs"))
        }
        "time_picker/no_seconds.rs" => Some(include_str!(
            "../content/snippets/time_picker/no_seconds.rs"
        )),
        "time_picker/disabled.rs" => {
            Some(include_str!("../content/snippets/time_picker/disabled.rs"))
        }
        "tray/basic.rs" => Some(include_str!("../content/snippets/tray/basic.rs")),
        "tray/dynamic_icon.rs" => Some(include_str!("../content/snippets/tray/dynamic_icon.rs")),
        "tray/checkbox.rs" => Some(include_str!("../content/snippets/tray/checkbox.rs")),
        "tray/nested_menu.rs" => Some(include_str!("../content/snippets/tray/nested_menu.rs")),
        "tray/residency.rs" => Some(include_str!("../content/snippets/tray/residency.rs")),
        "tray/close_confirm.rs" => Some(include_str!("../content/snippets/tray/close_confirm.rs")),
        "icon/lucide.rs" => Some(include_str!("../content/snippets/icon/lucide.rs")),
        "icon/colors.rs" => Some(include_str!("../content/snippets/icon/colors.rs")),
        "icon/sizes.rs" => Some(include_str!("../content/snippets/icon/sizes.rs")),
        "image/basic.rs" => Some(include_str!("../content/snippets/image/basic.rs")),
        "image/fit.rs" => Some(include_str!("../content/snippets/image/fit.rs")),
        "image/states.rs" => Some(include_str!("../content/snippets/image/states.rs")),
        "image/preview.rs" => Some(include_str!("../content/snippets/image/preview.rs")),
        "cascader/basic.rs" => Some(include_str!("../content/snippets/cascader/basic.rs")),
        "cascader/selected.rs" => Some(include_str!("../content/snippets/cascader/selected.rs")),
        "cascader/disabled.rs" => Some(include_str!("../content/snippets/cascader/disabled.rs")),
        "cascader/filterable.rs" => {
            Some(include_str!("../content/snippets/cascader/filterable.rs"))
        }
        "cascader/lazy.rs" => Some(include_str!("../content/snippets/cascader/lazy.rs")),
        "collapse/basic.rs" => Some(include_str!("../content/snippets/collapse/basic.rs")),
        "collapse/accordion.rs" => Some(include_str!("../content/snippets/collapse/accordion.rs")),
        "date_picker/basic.rs" => Some(include_str!("../content/snippets/date_picker/basic.rs")),
        "date_picker/formatted.rs" => {
            Some(include_str!("../content/snippets/date_picker/formatted.rs"))
        }
        "date_picker/range.rs" => Some(include_str!("../content/snippets/date_picker/range.rs")),
        "date_picker/month.rs" => Some(include_str!("../content/snippets/date_picker/month.rs")),
        "date_picker/month_range.rs" => Some(include_str!(
            "../content/snippets/date_picker/month_range.rs"
        )),
        "date_picker/year.rs" => Some(include_str!("../content/snippets/date_picker/year.rs")),
        "date_picker/year_range.rs" => Some(include_str!(
            "../content/snippets/date_picker/year_range.rs"
        )),
        "date_picker/disabled.rs" => {
            Some(include_str!("../content/snippets/date_picker/disabled.rs"))
        }
        "date_time_picker/basic.rs" => Some(include_str!(
            "../content/snippets/date_time_picker/basic.rs"
        )),
        "date_time_picker/formatted.rs" => Some(include_str!(
            "../content/snippets/date_time_picker/formatted.rs"
        )),
        "date_time_picker/stepped.rs" => Some(include_str!(
            "../content/snippets/date_time_picker/stepped.rs"
        )),
        "date_time_picker/no_seconds.rs" => Some(include_str!(
            "../content/snippets/date_time_picker/no_seconds.rs"
        )),
        "date_time_picker/range.rs" => Some(include_str!(
            "../content/snippets/date_time_picker/range.rs"
        )),
        "date_time_picker/disabled.rs" => Some(include_str!(
            "../content/snippets/date_time_picker/disabled.rs"
        )),
        "dialog/basic.rs" => Some(include_str!("../content/snippets/dialog/basic.rs")),
        "dialog/manual_close.rs" => {
            Some(include_str!("../content/snippets/dialog/manual_close.rs"))
        }
        "dialog/custom_content.rs" => {
            Some(include_str!("../content/snippets/dialog/custom_content.rs"))
        }
        "drawer/placements.rs" => Some(include_str!("../content/snippets/drawer/placements.rs")),
        "drawer/sizes.rs" => Some(include_str!("../content/snippets/drawer/sizes.rs")),
        "drawer/manual_close.rs" => {
            Some(include_str!("../content/snippets/drawer/manual_close.rs"))
        }
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
        "gallery/area_chart_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/area_chart_demo.rs"
        )),
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
        "gallery/bar_chart_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/bar_chart_demo.rs"
        )),
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
        "gallery/line_chart_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/line_chart_demo.rs"
        )),
        "gallery/pie_chart_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/pie_chart_demo.rs"
        )),
        "gallery/ring_chart_demo.rs" => Some(include_str!(
            "../../aura-gallery/src/demos/ring_chart_demo.rs"
        )),
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
    code_editors: Vec<Entity<CodeEditor>>,
    horizontal_lists: Vec<Entity<HorizontalList>>,
    virtualized_lists: Vec<Entity<VirtualizedList>>,
    virtualized_trees: Vec<Entity<VirtualizedTree>>,
    inputs: Vec<Entity<Input>>,
    radios: Vec<Entity<Radio>>,
    radio_groups: Vec<Entity<RadioGroup>>,
    rates: Vec<Entity<Rate>>,
    selects: Vec<Entity<Select>>,
    sliders: Vec<Entity<Slider>>,
    switches: Vec<Entity<Switch>>,
    segmenteds: Vec<Entity<aura_components::Segmented>>,
    paginations: Vec<Entity<aura_components::Pagination>>,
    tabs: Vec<Entity<aura_components::Tabs>>,
    scrollbars: Vec<Entity<aura_components::Scrollbar>>,
    table_sort_key: Option<SharedString>,
    table_sort_order: Option<aura_components::TableSortOrder>,
    color_pickers: Vec<Entity<aura_components::ColorPicker>>,
    time_pickers: Vec<Entity<aura_components::TimePicker>>,
    cascaders: Vec<Entity<aura_components::Cascader>>,
    collapses: Vec<Entity<aura_components::Collapse>>,
    date_pickers: Vec<Entity<aura_components::DatePicker>>,
    date_time_pickers: Vec<Entity<aura_components::DateTimePicker>>,
    uploads: Vec<Entity<Upload>>,
    transfers: Vec<Entity<Transfer>>,
    trees: Vec<Entity<Tree>>,
    menus: Vec<Entity<Menu>>,
    affixes: Vec<Entity<Affix>>,
    anchors: Vec<Entity<Anchor>>,
    backtops: Vec<Entity<Backtop>>,
    form_inputs: Vec<Entity<Input>>,
    form_selects: Vec<Entity<Select>>,
    form_switches: Vec<Entity<Switch>>,
    form_textareas: Vec<Entity<Textarea>>,
    scroll_handle: ScrollHandle,
}

impl LiveDemoContent {
    fn new(component: SharedString, cx: &mut Context<Self>) -> Self {
        let gallery_demo = aura_gallery::demos::render_doc_demo(component.as_ref(), cx);
        let mut autocompletes = Vec::new();
        let mut input_numbers = Vec::new();
        let mut textareas = Vec::new();
        let mut checkboxes = Vec::new();
        let mut checkbox_groups = Vec::new();
        let mut code_editors = Vec::new();
        let mut horizontal_lists = Vec::new();
        let mut virtualized_lists = Vec::new();
        let mut virtualized_trees = Vec::new();
        let mut inputs = Vec::new();
        let mut radios = Vec::new();
        let mut radio_groups = Vec::new();
        let mut rates = Vec::new();
        let mut selects = Vec::new();
        let mut sliders = Vec::new();
        let mut switches = Vec::new();
        let mut segmenteds = Vec::new();
        let mut paginations = Vec::new();
        let mut tabs = Vec::new();
        let mut scrollbars = Vec::new();
        let mut color_pickers = Vec::new();
        let mut time_pickers = Vec::new();
        let mut cascaders = Vec::new();
        let mut collapses = Vec::new();
        let mut date_pickers = Vec::new();
        let mut date_time_pickers = Vec::new();
        let mut uploads = Vec::new();
        let mut transfers = Vec::new();
        let mut trees = Vec::new();
        let mut menus = Vec::new();
        let mut affixes = Vec::new();
        let mut anchors = Vec::new();
        let mut backtops = Vec::new();
        let mut form_inputs = Vec::new();
        let mut form_selects = Vec::new();
        let mut form_switches = Vec::new();
        let mut form_textareas = Vec::new();
        let scroll_handle = ScrollHandle::new();

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
                        .close_on_click_outside(false)
                        .close_on_escape(false)
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

            "CodeEditorBasic" => {
                code_editors.push(cx.new(|cx| {
                    CodeEditor::new(DOCS_CODE_EDITOR_RUST_SAMPLE, cx)
                        .language(CodeLanguage::Rust)
                        .theme(CodeTheme::OneDark)
                        .line_numbers(true)
                        .tab_size(4)
                        .soft_tabs(true)
                }));
            }
            "CodeEditorDiagnostics" => {
                code_editors.push(cx.new(|cx| {
                    CodeEditor::new(DOCS_CODE_EDITOR_TS_SAMPLE, cx)
                        .language(CodeLanguage::TypeScript)
                        .tab_size(2)
                        .diagnostics_provider(|source| {
                            source
                                .lines()
                                .enumerate()
                                .filter(|(_, line)| {
                                    line.trim_start().starts_with("return")
                                        && !line.trim_end().ends_with(';')
                                })
                                .map(|(index, _)| {
                                    CodeDiagnostic::warning(
                                        index + 1,
                                        1,
                                        "Return line should end with a semicolon.",
                                    )
                                })
                                .collect()
                        })
                        .diagnostics([
                            CodeDiagnostic::warning(2, 3, "Prefer an explicit return type."),
                            CodeDiagnostic::error(
                                2,
                                25,
                                "Missing semicolon according to project style.",
                            ),
                        ])
                }));
            }
            "HorizontalListBasic" => {
                horizontal_lists.push(cx.new(|_| {
                    HorizontalList::new(DOCS_HORIZONTAL_STEPS.len(), docs_horizontal_step_card)
                        .height(px(92.0))
                }));
            }
            "HorizontalListDivider" => {
                horizontal_lists.push(cx.new(|_| {
                    HorizontalList::new(DOCS_HORIZONTAL_FLOW.len(), docs_horizontal_flow_card)
                        .height(px(92.0))
                        .divider(|_| {
                            Icon::new(IconName::ChevronRight)
                                .size(px(18.0))
                                .color(rgb(0x94a3b8).into())
                                .into_any_element()
                        })
                }));
            }
            "HorizontalListDraggable" => {
                horizontal_lists.push(cx.new(|_| {
                    HorizontalList::new(DOCS_HORIZONTAL_LANES.len(), docs_horizontal_lane_card)
                        .height(px(112.0))
                        .draggable(true)
                        .on_reorder(|from, to, _, _| {
                            toast_success!("HorizontalList reordered: {} -> {}", from + 1, to + 1);
                        })
                }));
            }
            "VirtualizedListBasic" => {
                virtualized_lists.push(cx.new(|cx| docs_virtualized_list(cx, false)));
            }
            "VirtualizedListDraggable" => {
                virtualized_lists.push(cx.new(|cx| docs_virtualized_list(cx, true)));
            }
            "VirtualizedTreeBasic" => {
                virtualized_trees.push(cx.new(|cx| docs_virtualized_tree(cx, false)));
            }
            "VirtualizedTreeCheckable" => {
                virtualized_trees.push(cx.new(|cx| docs_virtualized_tree(cx, true)));
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
            "CheckboxCustom" => {
                checkbox_groups.push(cx.new(|cx| styled_checkbox_cards(cx)));
                checkbox_groups.push(cx.new(|cx| styled_checkbox_chips(cx)));
                checkbox_groups.push(cx.new(|cx| rich_checkbox_options(cx)));
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
            "RadioCustom" => {
                radio_groups.push(cx.new(|cx| styled_radio_cards(cx)));
                radio_groups.push(cx.new(|cx| styled_radio_chips(cx)));
                radio_groups.push(cx.new(|cx| rich_radio_options(cx)));
            }
            "SelectBasic" => {
                selects.push(cx.new(|cx| {
                    Select::new(
                        vec!["Apple", "Banana", "Orange", "Grape", "Watermelon"],
                        Some(1),
                        cx,
                    )
                    .close_on_click_outside(false)
                    .close_on_escape(false)
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
            "SegmentedBasic" => {
                segmenteds.push(cx.new(|_| {
                    aura_components::Segmented::new(vec![
                        aura_components::SegmentedOption::new("Daily", "daily"),
                        aura_components::SegmentedOption::new("Weekly", "weekly"),
                        aura_components::SegmentedOption::new("Monthly", "monthly"),
                        aura_components::SegmentedOption::new("Quarterly", "quarterly"),
                        aura_components::SegmentedOption::new("Yearly", "yearly"),
                    ])
                    .id("docs-segmented-basic")
                    .on_change(|value, _, _| toast_info!("Selected: {}", value))
                }));
            }
            "SegmentedDisabled" => {
                segmenteds.push(cx.new(|_| {
                    aura_components::Segmented::new(vec![
                        aura_components::SegmentedOption::new("Map", "Map"),
                        aura_components::SegmentedOption::new("Transit", "Transit"),
                        aura_components::SegmentedOption::new("Satellite", "Satellite")
                            .disabled(true),
                    ])
                    .id("docs-segmented-disabled")
                    .value("Map")
                    .on_change(|value, _, _| toast_info!("Selected: {}", value))
                }));
            }
            "SegmentedBlock" => {
                segmenteds.push(cx.new(|_| {
                    aura_components::Segmented::new(vec![
                        aura_components::SegmentedOption::new("123", "123"),
                        aura_components::SegmentedOption::new("456", "456"),
                        aura_components::SegmentedOption::new("long-text-option", "long"),
                    ])
                    .id("docs-segmented-block")
                    .block(true)
                    .on_change(|value, _, _| toast_info!("Selected: {}", value))
                }));
            }
            "PaginationBasic" => {
                paginations.push(cx.new(|_| {
                    aura_components::Pagination::new(50)
                        .id("docs-pagination-basic")
                        .on_change(|page, _, _| toast_info!("Page changed to: {}", page))
                }));
            }
            "PaginationBackground" => {
                paginations.push(cx.new(|_| {
                    aura_components::Pagination::new(100)
                        .id("docs-pagination-background")
                        .background(true)
                        .on_change(|page, _, _| toast_info!("Page changed to: {}", page))
                }));
            }
            "PaginationAdvanced" => {
                paginations.push(cx.new(|_| {
                    aura_components::Pagination::new(400)
                        .id("docs-pagination-advanced")
                        .page_size(20)
                        .page_sizes(vec![10, 20, 50, 100])
                        .background(true)
                        .layout("total, sizes, prev, pager, next, jumper")
                        .on_change(|page, _, _| toast_info!("Page changed to: {}", page))
                        .on_page_size_change(|size, _, _| {
                            toast_info!("Page size changed to: {}", size)
                        })
                }));
            }
            "TabsBasic" => {
                tabs.push(cx.new(|_| basic_tabs("docs-tabs-basic")));
            }
            "TabsStretch" => {
                tabs.push(cx.new(|_| basic_tabs("docs-tabs-stretch").stretch(true)));
            }
            "TabsCard" => {
                tabs.push(
                    cx.new(|_| basic_tabs("docs-tabs-card").type_(aura_components::TabType::Card)),
                );
            }
            "TabsBorderCard" => {
                tabs.push(cx.new(|_| {
                    basic_tabs("docs-tabs-border-card").type_(aura_components::TabType::BorderCard)
                }));
            }
            "TabsPosition" => {
                tabs.push(cx.new(|_| {
                    short_tabs("docs-tabs-left").position(aura_components::TabPosition::Left)
                }));
                tabs.push(cx.new(|_| {
                    short_tabs("docs-tabs-right").position(aura_components::TabPosition::Right)
                }));
            }
            "TabsEditable" => {
                tabs.push(cx.new(|_| {
                    aura_components::Tabs::new("1")
                        .id("docs-tabs-editable")
                        .editable(true)
                        .pane("1", "Tab 1", |_, _| Text::new("Content of Tab 1"))
                        .pane("2", "Tab 2", |_, _| Text::new("Content of Tab 2"))
                        .on_tab_add(|_, _| toast_info!("Add Tab Clicked"))
                        .on_tab_remove(|name, _, _| toast_info!("Remove Tab: {}", name))
                }));
            }
            "ScrollbarBasic" => {
                scrollbars.push(cx.new(|cx| {
                    aura_components::Scrollbar::new(cx, |_, _| {
                        let items = (1..=20).map(|i| Text::new(format!("Scrollable line {}", i)));
                        Space::new().vertical().gap_lg().children(items)
                    })
                    .height(150.0)
                }));
            }
            "ColorPickerBasic" => {
                color_pickers.push(cx.new(|_| {
                    aura_components::ColorPicker::new("#409eff")
                        .id("docs-color-picker-basic")
                        .width_md()
                }));
            }
            "ColorPickerPresets" => {
                color_pickers.push(cx.new(|_| {
                    aura_components::ColorPicker::new("#13c2c2")
                        .id("docs-color-picker-presets")
                        .width_md()
                        .presets([
                            "#13C2C2", "#52C41A", "#FAAD14", "#F5222D", "#722ED1", "#EB2F96",
                        ])
                }));
            }
            "ColorPickerCompact" => {
                color_pickers.push(cx.new(|_| {
                    aura_components::ColorPicker::new("#F56C6C")
                        .id("docs-color-picker-compact")
                        .show_label(false)
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                }));
            }
            "ColorPickerDisabled" => {
                color_pickers.push(cx.new(|_| {
                    aura_components::ColorPicker::new("#909399")
                        .id("docs-color-picker-disabled")
                        .disabled(true)
                        .width_md()
                }));
            }
            "TimePickerBasic" => {
                time_pickers.push(cx.new(|_| {
                    aura_components::TimePicker::new()
                        .id("docs-time-picker-basic")
                        .width_md()
                }));
            }
            "TimePickerFormatted" => {
                time_pickers.push(cx.new(|_| {
                    aura_components::TimePicker::new()
                        .id("docs-time-picker-formatted")
                        .value(aura_components::TimeValue::new(9, 30, 15).expect("valid time"))
                        .format("HH时mm分ss秒")
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .width_md()
                }));
            }
            "TimePickerStepped" => {
                time_pickers.push(cx.new(|_| {
                    aura_components::TimePicker::new()
                        .id("docs-time-picker-stepped")
                        .value(aura_components::TimeValue::new(14, 30, 0).expect("valid time"))
                        .minute_step(15)
                        .second_step(30)
                        .width_md()
                }));
            }
            "TimePickerNoSeconds" => {
                time_pickers.push(cx.new(|_| {
                    aura_components::TimePicker::new()
                        .id("docs-time-picker-no-seconds")
                        .without_seconds()
                        .value(aura_components::TimeValue::new(18, 45, 0).expect("valid time"))
                        .width_md()
                }));
            }
            "TimePickerDisabled" => {
                time_pickers.push(cx.new(|_| {
                    aura_components::TimePicker::new()
                        .id("docs-time-picker-disabled")
                        .disabled(true)
                        .width_md()
                }));
            }
            "CascaderBasic" => {
                cascaders.push(cx.new(|cx| {
                    aura_components::Cascader::new(docs_region_options(), cx)
                        .placeholder("请选择地区")
                        .clearable(true)
                        .width_md()
                }));
            }
            "CascaderSelected" => {
                cascaders.push(cx.new(|cx| {
                    aura_components::Cascader::new(docs_product_options(), cx)
                        .selected_path(["cloud", "compute", "ecs"])
                        .placeholder("请选择产品")
                        .width_md()
                }));
            }
            "CascaderDisabled" => {
                cascaders.push(cx.new(|cx| {
                    aura_components::Cascader::new(docs_region_options(), cx)
                        .disabled(true)
                        .selected_path(["zhejiang", "hangzhou", "xihu"])
                        .width_md()
                }));
            }
            "CascaderFilterable" => {
                cascaders.push(cx.new(|cx| {
                    aura_components::Cascader::new(docs_region_options(), cx)
                        .filterable(true)
                        .search_query("hang")
                        .placeholder("搜索 hang")
                        .width_md()
                }));
            }
            "CascaderLazy" => {
                cascaders.push(cx.new(|cx| {
                    aura_components::Cascader::new(docs_lazy_options(), cx)
                        .lazy(true)
                        .placeholder("请选择远程节点")
                        .width_md()
                        .on_lazy_load(|cascader, path, _, cx| {
                            cascader.set_children_at_path(&path, docs_lazy_children_for(&path), cx);
                        })
                }));
            }
            "CollapseBasic" => {
                collapses.push(cx.new(|_| docs_collapse("docs-collapse-basic", false)));
            }
            "CollapseAccordion" => {
                collapses.push(cx.new(|_| docs_collapse("docs-collapse-accordion", true)));
            }
            "DatePickerBasic" => {
                date_pickers.push(cx.new(|_| {
                    aura_components::DatePicker::new()
                        .id("docs-date-picker-basic")
                        .width_md()
                }));
            }
            "DatePickerFormatted" => {
                date_pickers.push(cx.new(|_| {
                    aura_components::DatePicker::new()
                        .id("docs-date-picker-formatted")
                        .value(aura_components::DateValue::new(2026, 5, 8).expect("valid date"))
                        .format("YYYY年M月D日")
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .width_md()
                }));
            }
            "DatePickerRange" => {
                date_pickers.push(cx.new(|_| {
                    aura_components::DatePicker::new()
                        .id("docs-date-picker-range")
                        .date_range()
                        .range(
                            aura_components::DateValue::new(2026, 5, 8).expect("valid date"),
                            aura_components::DateValue::new(2026, 5, 18).expect("valid date"),
                        )
                        .width_lg()
                }));
            }
            "DatePickerMonth" => {
                date_pickers.push(cx.new(|_| {
                    aura_components::DatePicker::new()
                        .id("docs-date-picker-month")
                        .month()
                        .value(aura_components::DateValue::new(2026, 5, 1).expect("valid date"))
                        .width_md()
                }));
            }
            "DatePickerMonthRange" => {
                date_pickers.push(cx.new(|_| {
                    aura_components::DatePicker::new()
                        .id("docs-date-picker-month-range")
                        .month_range()
                        .range(
                            aura_components::DateValue::new(2026, 3, 1).expect("valid date"),
                            aura_components::DateValue::new(2026, 9, 1).expect("valid date"),
                        )
                        .width_lg()
                }));
            }
            "DatePickerYear" => {
                date_pickers.push(cx.new(|_| {
                    aura_components::DatePicker::new()
                        .id("docs-date-picker-year")
                        .year()
                        .value(aura_components::DateValue::new(2026, 1, 1).expect("valid date"))
                        .width_md()
                }));
            }
            "DatePickerYearRange" => {
                date_pickers.push(cx.new(|_| {
                    aura_components::DatePicker::new()
                        .id("docs-date-picker-year-range")
                        .year_range()
                        .range(
                            aura_components::DateValue::new(2024, 1, 1).expect("valid date"),
                            aura_components::DateValue::new(2028, 1, 1).expect("valid date"),
                        )
                        .format("YYYY年")
                        .width_lg()
                }));
            }
            "DatePickerDisabled" => {
                date_pickers.push(cx.new(|_| {
                    aura_components::DatePicker::new()
                        .id("docs-date-picker-disabled")
                        .disabled(true)
                        .width_md()
                }));
            }
            "DateTimePickerBasic" => {
                date_time_pickers.push(cx.new(|_| {
                    aura_components::DateTimePicker::new()
                        .id("docs-date-time-picker-basic")
                        .width_md()
                }));
            }
            "DateTimePickerFormatted" => {
                date_time_pickers.push(cx.new(|_| {
                    aura_components::DateTimePicker::new()
                        .id("docs-date-time-picker-formatted")
                        .value(
                            aura_components::DateTimeValue::new(2026, 5, 8, 9, 30, 15)
                                .expect("valid datetime"),
                        )
                        .format("YYYY年M月D日 HH:mm:ss")
                        .width_md()
                }));
            }
            "DateTimePickerStepped" => {
                date_time_pickers.push(cx.new(|_| {
                    aura_components::DateTimePicker::new()
                        .id("docs-date-time-picker-stepped")
                        .value(
                            aura_components::DateTimeValue::new(2026, 5, 8, 14, 30, 0)
                                .expect("valid datetime"),
                        )
                        .minute_step(15)
                        .second_step(30)
                        .width_md()
                }));
            }
            "DateTimePickerNoSeconds" => {
                date_time_pickers.push(cx.new(|_| {
                    aura_components::DateTimePicker::new()
                        .id("docs-date-time-picker-no-seconds")
                        .without_seconds()
                        .value(
                            aura_components::DateTimeValue::new(2026, 5, 8, 18, 45, 0)
                                .expect("valid datetime"),
                        )
                        .width_md()
                }));
            }
            "DateTimePickerRange" => {
                date_time_pickers.push(cx.new(|_| {
                    aura_components::DateTimePicker::new()
                        .id("docs-date-time-picker-range")
                        .date_time_range()
                        .range(
                            aura_components::DateTimeValue::new(2026, 5, 8, 9, 0, 0)
                                .expect("valid datetime"),
                            aura_components::DateTimeValue::new(2026, 5, 18, 18, 30, 0)
                                .expect("valid datetime"),
                        )
                        .width_lg()
                }));
            }
            "DateTimePickerDisabled" => {
                date_time_pickers.push(cx.new(|_| {
                    aura_components::DateTimePicker::new()
                        .id("docs-date-time-picker-disabled")
                        .disabled(true)
                        .width_md()
                }));
            }
            "UploadBasic" => uploads.push(cx.new(|_| docs_upload_basic())),
            "UploadDrag" => uploads.push(cx.new(|_| docs_upload_drag())),
            "UploadPictureCard" => uploads.push(cx.new(|_| docs_upload_picture_card())),
            "UploadLimits" => {
                uploads.push(cx.new(|_| docs_upload_limited()));
                uploads.push(cx.new(|_| docs_upload_disabled()));
            }
            "TransferBasic" => transfers.push(cx.new(|_| docs_transfer_basic())),
            "TransferFilterable" => transfers.push(cx.new(|_| docs_transfer_filterable())),
            "TransferDisabled" => transfers.push(cx.new(|_| docs_transfer_disabled())),
            "TreeBasic" => trees.push(cx.new(|_| docs_tree_basic())),
            "TreeCheckable" => trees.push(cx.new(|_| docs_tree_checkable())),
            "MenuHorizontal" => menus.push(cx.new(|_| docs_menu_horizontal())),
            "MenuVertical" => menus.push(cx.new(|_| docs_menu_vertical())),
            "MenuCollapsed" => menus.push(cx.new(|_| docs_menu_collapsed())),
            "AffixTop" => affixes.push(cx.new(|_| docs_affix_top())),
            "AffixBottom" => affixes.push(cx.new(|_| docs_affix_bottom())),
            "AffixContainer" => affixes.push(cx.new(|_| docs_affix_top())),
            "AnchorBasic" => {
                let handle = scroll_handle.clone();
                anchors.push(cx.new(move |_| docs_anchor_basic(handle)));
            }
            "AnchorNested" | "AnchorTargets" => {
                let handle = scroll_handle.clone();
                anchors.push(cx.new(move |_| docs_anchor_nested(handle)));
            }
            "BacktopBasic" | "BacktopContainer" => {
                let handle = scroll_handle.clone();
                backtops.push(cx.new(move |_| docs_backtop_basic(handle)));
            }
            "BacktopCustom" => {
                let handle = scroll_handle.clone();
                backtops.push(cx.new(move |_| docs_backtop_custom(handle)));
            }
            "FormBasic" => {
                form_inputs.push(cx.new(|cx| Input::new("Aura", cx).placeholder("Name")));
                form_selects
                    .push(cx.new(|cx| Select::new(vec!["Admin", "Editor", "Viewer"], Some(0), cx)));
                form_switches.push(cx.new(|cx| Switch::new(true, cx)));
            }
            "FormValidation" => {
                form_inputs.push(cx.new(|cx| Input::new("", cx).placeholder("请输入标题")));
                form_textareas
                    .push(cx.new(|cx| Textarea::new("Draft", cx).rows(3).max_length(120)));
            }
            "FormInline" => {
                form_inputs.push(cx.new(|cx| Input::new("", cx).placeholder("Search keyword")));
                form_selects
                    .push(cx.new(|cx| Select::new(vec!["All", "Open", "Closed"], Some(0), cx)));
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
            code_editors,
            horizontal_lists,
            virtualized_lists,
            virtualized_trees,
            inputs,
            radios,
            radio_groups,
            rates,
            selects,
            sliders,
            switches,
            segmenteds,
            paginations,
            tabs,
            scrollbars,
            table_sort_key: None,
            table_sort_order: None,
            color_pickers,
            time_pickers,
            cascaders,
            collapses,
            date_pickers,
            date_time_pickers,
            uploads,
            transfers,
            trees,
            menus,
            affixes,
            anchors,
            backtops,
            form_inputs,
            form_selects,
            form_switches,
            form_textareas,
            scroll_handle,
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
            "ButtonCustomColors" => demo_row(vec![
                Button::new("Violet")
                    .custom_color(rgb(0x7c3aed).into(), gpui::white())
                    .pill()
                    .into_any_element(),
                Button::new("Outline")
                    .colors(ButtonColors::outline(
                        rgb(0x0891b2).into(),
                        rgb(0x0f172a).into(),
                        gpui::transparent_black(),
                    ))
                    .rounded_md()
                    .into_any_element(),
                Button::new("Disabled")
                    .custom_color(rgb(0xdb2777).into(), gpui::white())
                    .disabled(true)
                    .into_any_element(),
            ]),
            "ButtonGradient" => demo_row(vec![
                Button::new("Aurora")
                    .gradient(rgb(0x6366f1).into(), rgb(0x06b6d4).into())
                    .pill()
                    .into_any_element(),
                Button::new("Sunset")
                    .gradient_with_angle(110.0, rgb(0xf97316).into(), rgb(0xec4899).into())
                    .large()
                    .rounded_lg()
                    .into_any_element(),
                Button::new("Loading")
                    .gradient(rgb(0x22c55e).into(), rgb(0x14b8a6).into())
                    .loading(true)
                    .into_any_element(),
                Button::new("Disabled")
                    .gradient(rgb(0x8b5cf6).into(), rgb(0x3b82f6).into())
                    .disabled(true)
                    .into_any_element(),
            ]),
            "QrCodeBasic" => demo_row(vec![
                QrCode::new("https://github.com/yhyzgn/aura")
                    .show_text(true)
                    .into_any_element(),
                QrCode::new("aura://component/qr-code")
                    .size(px(140.0))
                    .quiet_zone(2)
                    .into_any_element(),
            ]),
            "QrCodeStyle" => demo_row(vec![
                QrCode::new("Aura primary QR")
                    .size(px(160.0))
                    .colors(rgb(0x2563eb).into(), rgb(0xeff6ff).into())
                    .into_any_element(),
                QrCode::new("Rounded finder")
                    .size(px(170.0))
                    .ec_level(QrEcLevel::High)
                    .module_style(QrModuleStyle::Square)
                    .finder_style(QrFinderStyle::Rounded)
                    .colors(rgb(0x16a34a).into(), rgb(0xf0fdf4).into())
                    .into_any_element(),
                QrCode::new("Gradient diagonal QR")
                    .size(px(180.0))
                    .dot_modules()
                    .circle_finders()
                    .foreground_gradient(
                        vec![
                            rgb(0x7c3aed).into(),
                            rgb(0x06b6d4).into(),
                            rgb(0x22c55e).into(),
                        ],
                        QrGradientDirection::ToBottomRight,
                    )
                    .background(rgb(0xf8fafc).into())
                    .into_any_element(),
                QrCode::new("Gradient left QR")
                    .size(px(180.0))
                    .rounded_modules()
                    .rounded_finders()
                    .foreground_gradient(
                        vec![rgb(0xf97316).into(), rgb(0xec4899).into()],
                        QrGradientDirection::ToLeft,
                    )
                    .background(rgb(0xfffbeb).into())
                    .into_any_element(),
                QrCode::new("Rounded modules with custom logo")
                    .size(px(180.0))
                    .high_recovery()
                    .rounded_modules()
                    .circle_finders()
                    .logo_text("二维")
                    .logo_size_ratio(0.28)
                    .logo_background(rgb(0x22c55e).into())
                    .logo_color(gpui::white())
                    .colors(rgb(0x0f172a).into(), rgb(0xf8fafc).into())
                    .into_any_element(),
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
            "CheckboxGroup" | "CheckboxButtons" | "CheckboxCustom" => demo_stack(
                self.checkbox_groups
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "CodeEditorBasic" | "CodeEditorDiagnostics" => demo_stack(
                self.code_editors
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "HorizontalListBasic" | "HorizontalListDivider" | "HorizontalListDraggable" => {
                demo_stack(
                    self.horizontal_lists
                        .iter()
                        .cloned()
                        .map(Entity::into_any_element)
                        .collect(),
                )
            }
            "VirtualizedListBasic" | "VirtualizedListDraggable" => demo_stack(
                self.virtualized_lists
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "VirtualizedTableBasic" => virtualized_table_demo(false, None, None).into_any_element(),
            "VirtualizedTableSortable" => self.virtualized_table_sortable_demo(_cx),
            "VirtualizedTreeBasic" | "VirtualizedTreeCheckable" => demo_stack(
                self.virtualized_trees
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
            "RadioGroup" | "RadioButtons" | "RadioCustom" => demo_stack(
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

            "AreaChartBasic" => demo_row(vec![
                aura_components::AreaChart::new([aura_components::ChartSeries::new(
                    "Visitors",
                    [
                        aura_components::ChartPoint::new("Mon", 24.0),
                        aura_components::ChartPoint::new("Tue", 32.0),
                        aura_components::ChartPoint::new("Wed", 45.0),
                        aura_components::ChartPoint::new("Thu", 52.0),
                        aura_components::ChartPoint::new("Fri", 61.0),
                        aura_components::ChartPoint::new("Sat", 72.0),
                        aura_components::ChartPoint::new("Sun", 68.0),
                    ],
                )])
                .id("docs-area-chart-basic")
                .height(px(260.0))
                .into_any_element(),
            ]),
            "AreaChartOverlay" => demo_row(vec![
                aura_components::AreaChart::new([
                    aura_components::ChartSeries::new(
                        "Desktop",
                        [
                            aura_components::ChartPoint::new("Mon", 28.0),
                            aura_components::ChartPoint::new("Tue", 34.0),
                            aura_components::ChartPoint::new("Wed", 38.0),
                            aura_components::ChartPoint::new("Thu", 44.0),
                            aura_components::ChartPoint::new("Fri", 50.0),
                        ],
                    ),
                    aura_components::ChartSeries::new(
                        "Mobile",
                        [
                            aura_components::ChartPoint::new("Mon", 18.0),
                            aura_components::ChartPoint::new("Tue", 25.0),
                            aura_components::ChartPoint::new("Wed", 32.0),
                            aura_components::ChartPoint::new("Thu", 39.0),
                            aura_components::ChartPoint::new("Fri", 48.0),
                        ],
                    ),
                ])
                .id("docs-area-chart-overlay")
                .height(px(300.0))
                .y_domain(0.0, 100.0)
                .tooltip_hit_radius(px(18.0))
                .into_any_element(),
            ]),
            "AreaChartStacked" => demo_row(vec![
                aura_components::AreaChart::new([
                    aura_components::ChartSeries::new(
                        "Desktop",
                        [
                            aura_components::ChartPoint::new("Mon", 28.0),
                            aura_components::ChartPoint::new("Tue", 34.0),
                            aura_components::ChartPoint::new("Wed", 38.0),
                            aura_components::ChartPoint::new("Thu", 44.0),
                            aura_components::ChartPoint::new("Fri", 50.0),
                        ],
                    ),
                    aura_components::ChartSeries::new(
                        "Mobile",
                        [
                            aura_components::ChartPoint::new("Mon", 18.0),
                            aura_components::ChartPoint::new("Tue", 25.0),
                            aura_components::ChartPoint::new("Wed", 32.0),
                            aura_components::ChartPoint::new("Thu", 39.0),
                            aura_components::ChartPoint::new("Fri", 48.0),
                        ],
                    ),
                ])
                .id("docs-area-chart-stacked")
                .height(px(300.0))
                .stacked()
                .into_any_element(),
            ]),
            "AreaChartCustom" => demo_row(vec![
                aura_components::AreaChart::new([
                    aura_components::ChartSeries::new(
                        "Desktop",
                        [
                            aura_components::ChartPoint::new("Mon", 28.0),
                            aura_components::ChartPoint::new("Tue", 34.0),
                            aura_components::ChartPoint::new("Wed", 38.0),
                            aura_components::ChartPoint::new("Thu", 44.0),
                            aura_components::ChartPoint::new("Fri", 50.0),
                        ],
                    )
                    .stroke_color(gpui::blue())
                    .fill_color(gpui::blue().opacity(0.36))
                    .stroke_width(px(3.0))
                    .smooth(true),
                    aura_components::ChartSeries::new(
                        "Mobile",
                        [
                            aura_components::ChartPoint::new("Mon", 18.0),
                            aura_components::ChartPoint::new("Tue", 25.0),
                            aura_components::ChartPoint::new("Wed", 32.0),
                            aura_components::ChartPoint::new("Thu", 39.0),
                            aura_components::ChartPoint::new("Fri", 48.0),
                        ],
                    )
                    .stroke_color(gpui::green())
                    .fill_color(gpui::green().opacity(0.24))
                    .stroke_width(px(2.2))
                    .smooth(false),
                ])
                .id("docs-area-chart-custom")
                .height(px(340.0))
                .y_domain(0.0, 100.0)
                .smooth(true)
                .value_label_content(aura_components::ChartValueLabelContent::ValueAndPercentage)
                .percentage_decimals(1)
                .into_any_element(),
            ]),
            "AreaChartDownsample" => demo_row(vec![
                aura_components::AreaChart::new([
                    aura_components::ChartSeries::new(
                        "Desktop",
                        (0..1_800).map(|index| {
                            let wave = ((index as f64) / 32.0).sin() * 14.0;
                            aura_components::ChartPoint::new(format!("T{index}"), 42.0 + wave)
                        }),
                    ),
                    aura_components::ChartSeries::new(
                        "Mobile",
                        (0..1_800).map(|index| {
                            let wave = ((index as f64) / 27.0).cos() * 10.0;
                            let spike = if index % 360 == 0 { 24.0 } else { 0.0 };
                            aura_components::ChartPoint::new(format!("T{index}"), 28.0 + wave + spike)
                        }),
                    ),
                ])
                .id("docs-area-chart-downsample")
                .height(px(320.0))
                .stacked()
                .max_render_points(160)
                .into_any_element(),
            ]),
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
            "ProgressBasic" => demo_stack(vec![
                Progress::new(0.0).into_any_element(),
                Progress::new(30.0).into_any_element(),
                Progress::new(50.0).into_any_element(),
                Progress::new(100.0)
                    .status(ProgressStatus::Success)
                    .into_any_element(),
            ]),
            "ProgressInside" => demo_stack(vec![
                Progress::new(15.0)
                    .thick()
                    .text_inside(true)
                    .into_any_element(),
                Progress::new(70.0)
                    .thick()
                    .text_inside(true)
                    .into_any_element(),
                Progress::new(70.0)
                    .thick()
                    .text_inside_centered()
                    .into_any_element(),
                Progress::new(100.0)
                    .thick()
                    .text_inside(true)
                    .status(ProgressStatus::Success)
                    .into_any_element(),
            ]),
            "ProgressStatus" => demo_stack(vec![
                Progress::new(30.0).into_any_element(),
                Progress::new(50.0)
                    .status(ProgressStatus::Warning)
                    .into_any_element(),
                Progress::new(70.0)
                    .status(ProgressStatus::Exception)
                    .into_any_element(),
                Progress::new(100.0)
                    .status(ProgressStatus::Success)
                    .into_any_element(),
            ]),
            "ProgressColor" => {
                let theme = _cx.global::<Config>().theme.clone();
                demo_stack(vec![
                    Progress::new(50.0).primary().into_any_element(),
                    Progress::new(75.0)
                        .gradient(vec![
                            theme.success.base,
                            theme.warning.base,
                            theme.danger.base,
                            theme.primary.base,
                        ])
                        .into_any_element(),
                    Progress::new(64.0)
                        .color(theme.info.base)
                        .track_color(theme.neutral.hover)
                        .animated(false)
                        .into_any_element(),
                    Progress::new(100.0)
                        .gradient(vec![theme.primary.base, theme.success.base])
                        .complete_color(theme.success.base)
                        .text("Complete")
                        .into_any_element(),
                ])
            }
            "ProgressGradientComplete" => {
                let theme = _cx.global::<Config>().theme.clone();
                demo_stack(vec![
                    Progress::new(88.0)
                        .gradient(vec![theme.info.base, theme.primary.base, theme.success.base])
                        .track_color(theme.neutral.hover)
                        .text("88%")
                        .into_any_element(),
                    Progress::new(100.0)
                        .gradient(vec![theme.info.base, theme.primary.base, theme.success.base])
                        .complete_color(theme.success.base)
                        .track_color(theme.neutral.hover)
                        .text("Completed")
                        .into_any_element(),
                ])
            }
            "ProgressCircle" => demo_row(vec![
                Progress::new(32.0).circle().into_any_element(),
                Progress::new(58.0)
                    .circle()
                    .status(ProgressStatus::Warning)
                    .into_any_element(),
                Progress::new(76.0)
                    .circle()
                    .status(ProgressStatus::Exception)
                    .into_any_element(),
                Progress::new(100.0)
                    .circle()
                    .circle_size(px(132.0))
                    .status(ProgressStatus::Success)
                    .into_any_element(),
            ]),
            "ProgressCustom" => {
                let theme = _cx.global::<Config>().theme.clone();
                demo_row(vec![
                    Progress::new(86.0)
                        .circle()
                        .circle_size(px(148.0))
                        .ring_width(px(12.0))
                        .ring_color(theme.neutral.hover)
                        .progress_color(theme.primary.base)
                        .inner_color(theme.neutral.card)
                        .center_text("Deploy")
                        .text_size(px(22.0))
                        .text_color(theme.primary.base)
                        .into_any_element(),
                    Progress::new(42.0)
                        .circle()
                        .circle_size(px(132.0))
                        .ring_width(px(10.0))
                        .ring_color(theme.success.hover.opacity(0.24))
                        .progress_color(theme.success.base)
                        .inner_color(theme.neutral.body)
                        .center_text("42 / 100")
                        .text_size(px(16.0))
                        .text_weight(FontWeight::NORMAL)
                        .into_any_element(),
                    Progress::new(68.0)
                        .circle()
                        .circle_size(px(132.0))
                        .ring_width(px(14.0))
                        .ring_color(theme.warning.hover.opacity(0.28))
                        .progress_color(theme.warning.base)
                        .inner_color(theme.neutral.card.opacity(0.78))
                        .center_text("CPU")
                        .text_size(px(18.0))
                        .text_color(theme.warning.base)
                        .animated(false)
                        .into_any_element(),
                ])
            }
            "ProgressCircleGradient" => {
                let theme = _cx.global::<Config>().theme.clone();
                demo_row(vec![
                    Progress::new(100.0)
                        .circle()
                        .circle_size(px(148.0))
                        .ring_width(px(12.0))
                        .ring_color(theme.neutral.hover)
                        .gradient(vec![theme.primary.base, theme.success.base])
                        .complete_color(theme.success.base)
                        .inner_color(theme.neutral.card)
                        .center_text("Done")
                        .text_size(px(22.0))
                        .text_color(theme.success.base)
                        .into_any_element(),
                ])
            }
            "LoadingBasic" => demo_row(vec![
                Loading::new().into_any_element(),
                Loading::new().text("Loading...").into_any_element(),
            ]),
            "LoadingFullscreen" => demo_stack(vec![
                Text::new("通常由页面或弹层容器按需渲染全屏 Loading。下面展示同一配置的局部形态，避免遮挡文档页。")
                    .into_any_element(),
                Loading::new()
                    .text("Preparing workspace...")
                    .into_any_element(),
            ]),
            "LinkVariants" => demo_row(vec![
                Link::new("Default").href("https://github.com").into_any_element(),
                Link::new("Primary").primary().href("https://github.com").into_any_element(),
                Link::new("Success").success().href("https://github.com").into_any_element(),
                Link::new("Warning").warning().href("https://github.com").into_any_element(),
                Link::new("Danger").danger().href("https://github.com").into_any_element(),
                Link::new("Info").info().href("https://github.com").into_any_element(),
            ]),
            "LinkUnderline" => demo_row(vec![
                Link::new("With underline").href("https://github.com").into_any_element(),
                Link::new("No underline")
                    .underline(false)
                    .href("https://github.com")
                    .into_any_element(),
            ]),
            "LinkStates" => demo_row(vec![
                Link::new("Disabled")
                    .disabled(true)
                    .href("https://github.com")
                    .into_any_element(),
            ]),
            "LinkIcons" => demo_row(vec![
                Link::new("GitHub")
                    .icon_start(IconName::ExternalLink)
                    .href("https://github.com")
                    .into_any_element(),
                Link::new("Home")
                    .icon_start(IconName::House)
                    .href("https://example.com")
                    .into_any_element(),
            ]),
            "SkeletonBasic" => Skeleton::new().rows(4).into_any_element(),
            "SkeletonVariants" => demo_row(vec![
                SkeletonItem::new(SkeletonVariant::Circle).into_any_element(),
                SkeletonItem::new(SkeletonVariant::Square).into_any_element(),
                SkeletonItem::new(SkeletonVariant::Image).into_any_element(),
            ]),
            "SkeletonTemplate" => {
                let theme = _cx.global::<Config>().theme.clone();
                Skeleton::new()
                    .template(|_, _| {
                        Space::new()
                            .align_start()
                            .gap_lg()
                            .child(SkeletonItem::new(SkeletonVariant::Circle))
                            .child(
                                Space::new()
                                    .vertical()
                                    .grow()
                                    .gap_sm()
                                    .child(
                                        SkeletonItem::new(SkeletonVariant::Paragraph).width_2_5(),
                                    )
                                    .child(Skeleton::new().rows(2)),
                            )
                            .into_any_element()
                    })
                    .child(
                        Space::new()
                            .align_start()
                            .gap_lg()
                            .child(Avatar::new().background(theme.primary.base))
                            .child(
                                Space::new()
                                    .vertical()
                                    .gap_sm()
                                    .child(Text::new("Zed Industries").bold())
                                    .child(Text::new("GPUI renders native Rust views on the GPU.")),
                            ),
                    )
                    .into_any_element()
            }
            "ResultSuccess" => AuraResult::new("成功购买云服务器")
                .status(ResultStatus::Success)
                .sub_title("订单编号：2017182818828182881，请耐心等待审核。")
                .extra(|_, _| {
                    Space::new()
                        .gap_md()
                        .child(Button::new("返回列表"))
                        .child(Button::new("查看详情").primary())
                        .into_any_element()
                })
                .into_any_element(),
            "ResultStatuses" => demo_stack(vec![
                AuraResult::new("您的账户存在安全风险")
                    .status(ResultStatus::Warning)
                    .sub_title("请及时修改密码并开启双重验证。")
                    .extra(|_, _| Button::new("立即处理").primary().into_any_element())
                    .into_any_element(),
                AuraResult::new("提交失败")
                    .status(ResultStatus::Error)
                    .sub_title("请检查网络连接并重试。")
                    .extra(|_, _| Button::new("重新提交").primary().into_any_element())
                    .into_any_element(),
                AuraResult::new("您的申请已提交")
                    .status(ResultStatus::Info)
                    .sub_title("我们将在 3 个工作日内完成审核。")
                    .extra(|_, _| Button::new("知道了").into_any_element())
                    .into_any_element(),
            ]),
            "StatisticBasic" => demo_row(vec![
                Statistic::new("今日活跃用户", "114,514").into_any_element(),
                Statistic::new("总交易额", "¥ 9,999.00").into_any_element(),
            ]),
            "StatisticAffix" => demo_row(vec![
                Statistic::new("增长率", "12.5")
                    .suffix(aura_icons::Icon::new(IconName::TrendingUp))
                    .into_any_element(),
                Statistic::new("月活下降", "5.2")
                    .suffix(aura_icons::Icon::new(IconName::TrendingDown))
                    .into_any_element(),
                Statistic::new("待办事项", "12")
                    .prefix(aura_icons::Icon::new(IconName::ListTodo))
                    .into_any_element(),
            ]),
            "StatisticIcons" => demo_row(vec![
                Statistic::new("转化率", "68%")
                    .value_color(gpui::green())
                    .icon(IconName::TrendingUp)
                    .into_any_element(),
                Statistic::new("告警数", "7")
                    .icon(IconName::Bell)
                    .icon_left()
                    .icon_color(gpui::red())
                    .into_any_element(),
                Statistic::new("完成率", "92%")
                    .icon(IconName::CircleCheck)
                    .icon_right()
                    .icon_color(gpui::blue())
                    .into_any_element(),
            ]),
            "StatisticLayout" => demo_row(vec![
                Card::new(
                    Statistic::new("紧凑水平", "1,280")
                        .icon(IconName::Activity)
                        .horizontal_compact(),
                )
                .width_lg()
                .into_any_element(),
                Card::new(
                    Statistic::new("两端对齐", "¥ 86,420")
                        .icon(IconName::Wallet)
                        .icon_left()
                        .horizontal_between(),
                )
                .width_lg()
                .into_any_element(),
            ]),
            "BarChartBasic" => demo_row(vec![
                aura_components::BarChart::new([aura_components::ChartSeries::new(
                    "Revenue",
                    [
                        aura_components::ChartPoint::new("Q1", 42.0),
                        aura_components::ChartPoint::new("Q2", 58.0),
                        aura_components::ChartPoint::new("Q3", 73.0),
                        aura_components::ChartPoint::new("Q4", 96.0),
                    ],
                )])
                .id("docs-bar-chart-basic")
                .height(px(260.0))
                .tooltip_hit_radius(px(10.0))
                .into_any_element(),
            ]),
            "BarChartGrouped" => demo_row(vec![
                aura_components::BarChart::new([
                    aura_components::ChartSeries::new(
                        "Online",
                        [
                            aura_components::ChartPoint::new("Jan", 42.0),
                            aura_components::ChartPoint::new("Feb", 58.0),
                            aura_components::ChartPoint::new("Mar", 64.0),
                            aura_components::ChartPoint::new("Apr", 72.0),
                        ],
                    ),
                    aura_components::ChartSeries::new(
                        "Retail",
                        [
                            aura_components::ChartPoint::new("Jan", 28.0),
                            aura_components::ChartPoint::new("Feb", 34.0),
                            aura_components::ChartPoint::new("Mar", 39.0),
                            aura_components::ChartPoint::new("Apr", 45.0),
                        ],
                    ),
                ])
                .id("docs-bar-chart-grouped")
                .height(px(300.0))
                .y_domain(0.0, 120.0)
                .tooltip_hit_radius(px(12.0))
                .into_any_element(),
            ]),
            "BarChartGradient" => demo_row(vec![
                aura_components::BarChart::new([aura_components::ChartSeries::new(
                    "Revenue",
                    [
                        aura_components::ChartPoint::new("Q1", 42.0),
                        aura_components::ChartPoint::new("Q2", 58.0),
                        aura_components::ChartPoint::new("Q3", 73.0),
                        aura_components::ChartPoint::new("Q4", 96.0),
                    ],
                )])
                .id("docs-bar-chart-gradient")
                .height(px(300.0))
                .bar_radius(px(6.0))
                .bar_vertical_gradient(gpui::rgb(0x60a5fa).into(), gpui::rgb(0x2563eb).into())
                .value_fill_ranges([
                    aura_components::BarChartValueFillRange::new(
                        0.0,
                        60.0,
                        aura_components::BarChartFill::vertical_gradient(
                            gpui::rgb(0xbfdbfe).into(),
                            gpui::rgb(0x3b82f6).into(),
                        ),
                    ),
                    aura_components::BarChartValueFillRange::new(
                        60.0,
                        100.0,
                        aura_components::BarChartFill::vertical_gradient(
                            gpui::rgb(0xfef08a).into(),
                            gpui::rgb(0xf97316).into(),
                        ),
                    ),
                ])
                .into_any_element(),
            ]),
            "BarChartPerBarGradient" => demo_row(vec![
                aura_components::BarChart::new([aura_components::ChartSeries::new(
                    "Revenue",
                    [
                        aura_components::ChartPoint::new("Q1", 42.0),
                        aura_components::ChartPoint::new("Q2", 58.0),
                        aura_components::ChartPoint::new("Q3", 73.0),
                        aura_components::ChartPoint::new("Q4", 96.0),
                    ],
                )])
                .id("docs-bar-chart-per-bar-gradient")
                .height(px(300.0))
                .bar_radius(px(8.0))
                .bar_fills([
                    aura_components::BarChartFill::vertical_gradient(gpui::rgb(0xdbeafe).into(), gpui::rgb(0x2563eb).into()),
                    aura_components::BarChartFill::vertical_gradient(gpui::rgb(0xdcfce7).into(), gpui::rgb(0x16a34a).into()),
                    aura_components::BarChartFill::vertical_gradient(gpui::rgb(0xffedd5).into(), gpui::rgb(0xea580c).into()),
                    aura_components::BarChartFill::vertical_gradient(gpui::rgb(0xfce7f3).into(), gpui::rgb(0xdb2777).into()),
                ])
                .into_any_element(),
            ]),
            "BarChartStacked" => demo_row(vec![
                aura_components::BarChart::new([
                    aura_components::ChartSeries::new(
                        "Online",
                        [
                            aura_components::ChartPoint::new("Jan", 42.0),
                            aura_components::ChartPoint::new("Feb", 58.0),
                            aura_components::ChartPoint::new("Mar", 64.0),
                            aura_components::ChartPoint::new("Apr", 72.0),
                        ],
                    ),
                    aura_components::ChartSeries::new(
                        "Retail",
                        [
                            aura_components::ChartPoint::new("Jan", 28.0),
                            aura_components::ChartPoint::new("Feb", 34.0),
                            aura_components::ChartPoint::new("Mar", 39.0),
                            aura_components::ChartPoint::new("Apr", 45.0),
                        ],
                    ),
                ])
                .id("docs-bar-chart-stacked")
                .height(px(300.0))
                .tooltip_hit_radius(px(8.0))
                .stacked()
                .into_any_element(),
            ]),
            "BarChartCustom" => demo_row(vec![
                aura_components::BarChart::new([
                    aura_components::ChartSeries::new(
                        "Online",
                        [
                            aura_components::ChartPoint::new("Jan", 42.0),
                            aura_components::ChartPoint::new("Feb", 58.0),
                            aura_components::ChartPoint::new("Mar", 64.0),
                            aura_components::ChartPoint::new("Apr", 72.0),
                        ],
                    )
                    .fill_color(gpui::blue()),
                    aura_components::ChartSeries::new(
                        "Retail",
                        [
                            aura_components::ChartPoint::new("Jan", 28.0),
                            aura_components::ChartPoint::new("Feb", 34.0),
                            aura_components::ChartPoint::new("Mar", 39.0),
                            aura_components::ChartPoint::new("Apr", 45.0),
                        ],
                    )
                    .fill_color(gpui::green()),
                ])
                .id("docs-bar-chart-custom")
                .height(px(340.0))
                .y_domain(0.0, 120.0)
                .bar_gap_ratio(0.32)
                .value_label_content(aura_components::ChartValueLabelContent::ValueAndPercentage)
                .percentage_decimals(1)
                .into_any_element(),
            ]),
            "BarChartStandalone" => demo_row(vec![
                aura_components::BarChart::new([aura_components::ChartSeries::new(
                    "Active",
                    [
                        aura_components::ChartPoint::new("Mon", 18.0),
                        aura_components::ChartPoint::new("Tue", 42.0),
                        aura_components::ChartPoint::new("Wed", 33.0),
                        aura_components::ChartPoint::new("Thu", 76.0),
                        aura_components::ChartPoint::new("Fri", 61.0),
                    ],
                )])
                .id("docs-bar-chart-standalone")
                .standalone()
                .bar_width(px(8.0))
                .bar_gap(px(7.0))
                .bar_radius(px(5.0))
                .value_color_ranges([
                    aura_components::BarChartValueColorRange::new(0.0, 35.0, gpui::rgb(0x86efac).into()),
                    aura_components::BarChartValueColorRange::new(35.0, 70.0, gpui::rgb(0x22c55e).into()),
                    aura_components::BarChartValueColorRange::new(70.0, 100.0, gpui::rgb(0x16a34a).into()),
                ])
                .into_any_element(),
            ]),
            "BarChartStandaloneStyles" => demo_row(vec![
                aura_components::Space::new()
                    .wrap()
                    .gap_lg()
                    .child(
                        aura_components::BarChart::new([aura_components::ChartSeries::new(
                            "Active",
                            [
                                aura_components::ChartPoint::new("Mon", 18.0),
                                aura_components::ChartPoint::new("Tue", 42.0),
                                aura_components::ChartPoint::new("Wed", 33.0),
                                aura_components::ChartPoint::new("Thu", 76.0),
                                aura_components::ChartPoint::new("Fri", 61.0),
                                aura_components::ChartPoint::new("Sat", 88.0),
                                aura_components::ChartPoint::new("Sun", 54.0),
                            ],
                        )])
                        .id("docs-bar-chart-standalone-compact")
                        .standalone()
                        .bar_width(px(8.0))
                        .bar_gap(px(4.0))
                        .bar_radius(px(5.0))
                        .value_color_ranges([
                            aura_components::BarChartValueColorRange::new(0.0, 35.0, gpui::rgb(0x86efac).into()),
                            aura_components::BarChartValueColorRange::new(35.0, 70.0, gpui::rgb(0x22c55e).into()),
                            aura_components::BarChartValueColorRange::new(70.0, 100.0, gpui::rgb(0x16a34a).into()),
                        ]),
                    )
                    .child(
                        aura_components::BarChart::new([aura_components::ChartSeries::new(
                            "Active",
                            [
                                aura_components::ChartPoint::new("Mon", 18.0),
                                aura_components::ChartPoint::new("Tue", 42.0),
                                aura_components::ChartPoint::new("Wed", 33.0),
                                aura_components::ChartPoint::new("Thu", 76.0),
                                aura_components::ChartPoint::new("Fri", 61.0),
                                aura_components::ChartPoint::new("Sat", 88.0),
                                aura_components::ChartPoint::new("Sun", 54.0),
                            ],
                        )])
                        .id("docs-bar-chart-standalone-gradient")
                        .standalone()
                        .bar_width(px(10.0))
                        .bar_gap(px(5.0))
                        .bar_radius(px(8.0))
                        .bar_fills([
                            aura_components::BarChartFill::vertical_gradient(gpui::rgb(0xc4b5fd).into(), gpui::rgb(0x7c3aed).into()),
                            aura_components::BarChartFill::vertical_gradient(gpui::rgb(0xbae6fd).into(), gpui::rgb(0x0284c7).into()),
                            aura_components::BarChartFill::vertical_gradient(gpui::rgb(0xfde68a).into(), gpui::rgb(0xd97706).into()),
                        ]),
                    )
                    .child(
                        aura_components::BarChart::new([aura_components::ChartSeries::new(
                            "Active",
                            [
                                aura_components::ChartPoint::new("Mon", 18.0),
                                aura_components::ChartPoint::new("Tue", 42.0),
                                aura_components::ChartPoint::new("Wed", 33.0),
                                aura_components::ChartPoint::new("Thu", 76.0),
                                aura_components::ChartPoint::new("Fri", 61.0),
                                aura_components::ChartPoint::new("Sat", 88.0),
                                aura_components::ChartPoint::new("Sun", 54.0),
                            ],
                        )])
                        .id("docs-bar-chart-standalone-wide")
                        .standalone()
                        .height(px(96.0))
                        .bar_width(px(14.0))
                        .bar_gap(px(8.0))
                        .bar_radius(px(3.0))
                        .bar_vertical_gradient(gpui::rgb(0xfda4af).into(), gpui::rgb(0xe11d48).into()),
                    )
                    .into_any_element(),
            ]),
            "TagFlow" => demo_row(vec![
                aura_components::TagFlow::new([
                    aura_components::Tag::new("Design").round(true),
                    aura_components::Tag::new("GPUI").success().round(true),
                    aura_components::Tag::new("Animation").warning().round(true),
                    aura_components::Tag::new("Native Rust").danger().round(true),
                    aura_components::Tag::new("Charts").round(true),
                    aura_components::Tag::new("Docs").success().round(true),
                    aura_components::Tag::new("Installer").warning().round(true),
                    aura_components::Tag::new("Tray").round(true),
                ])
                .gap(px(10.0))
                .max_rows(2)
                .estimated_items_per_row(3)
                .overflow_indicator("更多")
                .into_any_element(),
            ]),
            "SignalMeterMobile" => demo_row(vec![
                aura_components::SignalMeter::new(3).height(px(36.0)).into_any_element(),
            ]),
            "SignalMeterWifi" => demo_row(vec![
                aura_components::SignalMeter::new(2)
                    .wifi()
                    .active_color(gpui::rgb(0x3b82f6).into())
                    .inactive_color(gpui::rgb(0xdbeafe).into())
                    .bar_width(px(8.0))
                    .gap(px(5.0))
                    .height(px(44.0))
                    .into_any_element(),
            ]),
            "SignalMeterLevels" => demo_row(vec![
                aura_components::SignalMeter::new(5)
                    .total_signals(6)
                    .level_colors([
                        gpui::rgb(0xef4444).into(),
                        gpui::rgb(0xf97316).into(),
                        gpui::rgb(0xf59e0b).into(),
                        gpui::rgb(0x84cc16).into(),
                        gpui::rgb(0x22c55e).into(),
                        gpui::rgb(0x16a34a).into(),
                    ])
                    .height(px(44.0))
                    .bar_width(px(7.0))
                    .gap(px(5.0))
                    .into_any_element(),
            ]),
            "SignalMeterThresholdColors" => demo_row(vec![
                aura_components::SignalMeter::new(2)
                    .total_signals(5)
                    .threshold_colors([
                        aura_components::SignalLevelColor::new(2, gpui::rgb(0xef4444).into()),
                        aura_components::SignalLevelColor::new(3, gpui::rgb(0xeab308).into()),
                        aura_components::SignalLevelColor::new(4, gpui::rgb(0xf97316).into()),
                        aura_components::SignalLevelColor::new(5, gpui::rgb(0x22c55e).into()),
                    ])
                    .inactive_color(gpui::rgb(0xf1f5f9).into())
                    .height(px(42.0))
                    .bar_width(px(7.0))
                    .gap(px(5.0))
                    .into_any_element(),
                aura_components::SignalMeter::new(4)
                    .total_signals(5)
                    .threshold_colors([
                        aura_components::SignalLevelColor::new(2, gpui::rgb(0xef4444).into()),
                        aura_components::SignalLevelColor::new(3, gpui::rgb(0xeab308).into()),
                        aura_components::SignalLevelColor::new(4, gpui::rgb(0xf97316).into()),
                        aura_components::SignalLevelColor::new(5, gpui::rgb(0x22c55e).into()),
                    ])
                    .inactive_color(gpui::rgb(0xf1f5f9).into())
                    .height(px(42.0))
                    .bar_width(px(7.0))
                    .gap(px(5.0))
                    .into_any_element(),
            ]),
            "HeatBarEvents" => demo_row(vec![
                aura_components::HeatBar::new((0..48).map(|index| {
                    let value = ((index * 7 + 3) % 11) as f64;
                    aura_components::HeatBarItem::new(format!("t{index}"), value, gpui::rgb(0x93c5fd).into())
                }))
                .legends([
                    aura_components::HeatBarLegend::new("错误", 3, gpui::rgb(0xef4444).into()),
                    aura_components::HeatBarLegend::new("警告", 24, gpui::rgb(0xf59e0b).into()),
                ])
                .color_ranges([
                    aura_components::HeatBarColorRange::new(0.0, 3.0, gpui::rgb(0x93c5fd).into()),
                    aura_components::HeatBarColorRange::new(3.0, 7.0, gpui::rgb(0xf59e0b).into()),
                    aura_components::HeatBarColorRange::above(7.0, gpui::rgb(0xef4444).into()),
                ])
                .max_value(10.0)
                .x_labels(["00:00", "06:00", "12:00", "18:00", "24:00"])
                .into_any_element(),
            ]),
            "SegmentRatioBarBottom" => demo_row(vec![
                aura_components::SegmentRatioBar::new([
                    aura_components::SegmentRatioItem::new("Direct", 42.0, gpui::rgb(0x3b82f6).into()),
                    aura_components::SegmentRatioItem::new("Proxy", 51.0, gpui::rgb(0x22c55e).into()),
                    aura_components::SegmentRatioItem::new("Reject", 7.0, gpui::rgb(0xef4444).into()),
                ])
                .height(px(14.0))
                .radius(px(8.0))
                .segment_radius(px(3.0))
                .legend_inset_x(px(8.0))
                .percentage_decimals(0)
                .split_legend(true)
                .into_any_element(),
            ]),
            "SegmentRatioBarTop" => demo_row(vec![
                aura_components::SegmentRatioBar::new([
                    aura_components::SegmentRatioItem::new("Direct", 42.0, gpui::rgb(0x3b82f6).into()),
                    aura_components::SegmentRatioItem::new("Proxy", 51.0, gpui::rgb(0x22c55e).into()),
                    aura_components::SegmentRatioItem::new("Reject", 7.0, gpui::rgb(0xef4444).into()),
                ])
                .legend_position(aura_components::SegmentLegendPosition::Top)
                .height(px(16.0))
                .radius(px(8.0))
                .rounded_segments(px(4.0))
                .legend_inset_x(px(10.0))
                .percentage_decimals(1)
                .into_any_element(),
            ]),
            "SegmentRatioBarBoth" => demo_row(vec![
                aura_components::SegmentRatioBar::new([
                    aura_components::SegmentRatioItem::new("Direct", 42.0, gpui::rgb(0x3b82f6).into()),
                    aura_components::SegmentRatioItem::new("Proxy", 51.0, gpui::rgb(0x22c55e).into()),
                    aura_components::SegmentRatioItem::new("Reject", 7.0, gpui::rgb(0xef4444).into()),
                ])
                .legend_both()
                .height(px(14.0))
                .radius(px(7.0))
                .segment_radius(px(3.0))
                .legend_text_inset(px(8.0))
                .percentage_decimals(1)
                .into_any_element(),
            ]),
            "SegmentRatioBarHidden" => demo_row(vec![
                aura_components::SegmentRatioBar::new([
                    aura_components::SegmentRatioItem::new("Direct", 42.0, gpui::rgb(0x3b82f6).into()),
                    aura_components::SegmentRatioItem::new("Proxy", 51.0, gpui::rgb(0x22c55e).into()),
                    aura_components::SegmentRatioItem::new("Reject", 7.0, gpui::rgb(0xef4444).into()),
                ])
                .hide_legend()
                .height(px(18.0))
                .radius(px(9.0))
                .segment_radius(px(4.0))
                .into_any_element(),
            ]),
            "SegmentRatioBarPattern" => demo_row(vec![
                aura_components::SegmentRatioBar::new([
                    aura_components::SegmentRatioItem::new("Direct", 42.0, gpui::rgb(0x3b82f6).into())
                        .label_pattern("{label}")
                        .value_pattern("{value} req / {percent}"),
                    aura_components::SegmentRatioItem::new("Proxy", 51.0, gpui::rgb(0x22c55e).into())
                        .value_pattern("{percent}"),
                    aura_components::SegmentRatioItem::new("Reject", 7.0, gpui::rgb(0xef4444).into())
                        .value_pattern("{value}"),
                ])
                .legend_both()
                .radius(px(7.0))
                .segment_radius(px(3.0))
                .legend_text_inset(px(10.0))
                .percentage_decimals(1)
                .into_any_element(),
            ]),
            "SegmentRatioBarCompact" => demo_row(vec![
                aura_components::SegmentRatioBar::new([
                    aura_components::SegmentRatioItem::new("API", 18.0, gpui::rgb(0x8b5cf6).into()),
                    aura_components::SegmentRatioItem::new("Web", 33.0, gpui::rgb(0x06b6d4).into()),
                    aura_components::SegmentRatioItem::new("Jobs", 29.0, gpui::rgb(0xf59e0b).into()),
                    aura_components::SegmentRatioItem::new("Other", 20.0, gpui::rgb(0x64748b).into()),
                ])
                .height(px(8.0))
                .radius(px(4.0))
                .rounded_segments(px(2.0))
                .legend_inset_x(px(14.0))
                .percentage_decimals(2)
                .into_any_element(),
            ]),
            "LabelBasic" => demo_row(vec![
                aura_components::Label::new("Build passed").icon(IconName::CircleCheck).color(gpui::green()).into_any_element(),
            ]),
            "OperationBasic" => demo_row(vec![
                aura_components::Operation::new(
                    aura_components::Label::new("执行操作").icon(IconName::Play),
                    aura_components::Button::new("Run").small(),
                )
                .description("左侧可带说明文本，右侧操作区域保持末端对齐。")
                .status("手动")
                .into_any_element(),
            ]),
            "SparklineBasic" => demo_row(vec![
                aura_components::Sparkline::new([12.0, 15.0, 14.0, 18.0, 21.0, 19.0, 24.0, 28.0])
                    .id("docs-sparkline-basic")
                    .width(px(220.0))
                    .height(px(64.0))
                    .color(gpui::rgb(0x2563eb).into())
                    .stroke_width(px(2.4))
                    .into_any_element(),
            ]),
            "SparklineCards" => demo_row(vec![
                aura_components::Card::new(
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(Text::new("Revenue").size(px(12.0)).text_color(gpui::rgb(0x64748b).into()))
                        .child(Text::new("$42.8k").size(px(24.0)).bold())
                        .child(
                            aura_components::Sparkline::new([12.0, 15.0, 14.0, 18.0, 21.0, 19.0, 24.0, 28.0])
                                .id("docs-sparkline-card")
                                .height(px(64.0))
                                .area_fill(true),
                        ),
                )
                .width(px(240.0))
                .into_any_element(),
            ]),
            "SparklineArea" => demo_row(vec![
                aura_components::Sparkline::new([-4.0, -1.0, 3.0, 7.0, 5.0, -2.0, 4.0, 10.0, 8.0])
                    .id("docs-sparkline-area")
                    .width(px(280.0))
                    .height(px(96.0))
                    .area_fill(true)
                    .show_baseline(true)
                    .trend_colors(gpui::rgb(0x14b8a6).into(), gpui::rgb(0xf43f5e).into())
                    .fill_color(gpui::Hsla::from(gpui::rgb(0x14b8a6)).opacity(0.18))
                    .y_domain(-8.0, 12.0)
                    .into_any_element(),
            ]),
            "SparklineStyles" => demo_row(vec![
                aura_components::Sparkline::new([12.0, 15.0, 14.0, 18.0, 21.0, 19.0, 24.0, 28.0])
                    .id("docs-sparkline-style-dashed")
                    .width(px(220.0))
                    .height(px(56.0))
                    .color(gpui::rgb(0x2563eb).into())
                    .line_style(aura_components::ChartLineStyle::Dashed)
                    .smooth(false)
                    .show_last_point(false)
                    .into_any_element(),
                aura_components::Sparkline::new([28.0, 24.0, 25.0, 22.0, 18.0, 17.0, 15.0, 12.0])
                    .id("docs-sparkline-style-dotted")
                    .width(px(220.0))
                    .height(px(56.0))
                    .color(gpui::rgb(0xdc2626).into())
                    .dotted()
                    .show_last_point(false)
                    .into_any_element(),
            ]),
            "SparklineDownsample" => demo_row(vec![
                aura_components::Sparkline::new((0..1_200).map(|index| {
                    let wave = ((index as f64) / 18.0).sin() * 8.0;
                    let spike = if index % 180 == 0 { 16.0 } else { 0.0 };
                    40.0 + wave + spike
                }))
                .id("docs-sparkline-downsample")
                .width(px(280.0))
                .height(px(72.0))
                .color(gpui::rgb(0x7c3aed).into())
                .area_fill(true)
                .max_render_points(96)
                .into_any_element(),
            ]),
            "PieChart" => demo_row(vec![
                aura_components::PieChart::new([
                    aura_components::ChartSeries::new("Desktop", [aura_components::ChartPoint::new("Desktop", 62.0)]),
                    aura_components::ChartSeries::new("Mobile", [aura_components::ChartPoint::new("Mobile", 24.0)]),
                    aura_components::ChartSeries::new("Tablet", [aura_components::ChartPoint::new("Tablet", 9.0)]),
                    aura_components::ChartSeries::new("Other", [aura_components::ChartPoint::new("Other", 5.0)]),
                ])
                .id("docs-pie-chart")
                .height(px(340.0))
                .percentage_decimals(1)
                .outside_label_threshold_degrees(30)
                .value_label_placement(aura_components::ChartValueLabelPlacement::OutsideAligned)
                .tooltip_hit_radius(px(10.0))
                .into_any_element(),
            ]),
            "PieChartCustom" => demo_row(vec![
                aura_components::PieChart::new([
                    aura_components::ChartSeries::new("Desktop", [aura_components::ChartPoint::new("Desktop", 62.0)]).fill_color(gpui::blue()),
                    aura_components::ChartSeries::new("Mobile", [aura_components::ChartPoint::new("Mobile", 24.0)]).fill_color(gpui::green()),
                    aura_components::ChartSeries::new("Tablet", [aura_components::ChartPoint::new("Tablet", 9.0)]).fill_color(gpui::yellow()),
                    aura_components::ChartSeries::new("Other", [aura_components::ChartPoint::new("Other", 5.0)]).fill_color(gpui::red()),
                ])
                .id("docs-pie-chart-custom")
                .height(px(360.0))
                .value_label_content(aura_components::ChartValueLabelContent::Percentage)
                .value_label_placement(aura_components::ChartValueLabelPlacement::OutsideFree)
                .percentage_decimals(2)
                .outside_label_threshold_degrees(120)
                .show_tooltip(false)
                .into_any_element(),
            ]),
            "RingChart" => demo_row(vec![
                aura_components::RingChart::new([
                    aura_components::ChartSeries::new("Desktop", [aura_components::ChartPoint::new("Desktop", 62.0)]),
                    aura_components::ChartSeries::new("Mobile", [aura_components::ChartPoint::new("Mobile", 24.0)]),
                    aura_components::ChartSeries::new("Tablet", [aura_components::ChartPoint::new("Tablet", 9.0)]),
                    aura_components::ChartSeries::new("Other", [aura_components::ChartPoint::new("Other", 5.0)]),
                ])
                .id("docs-ring-chart")
                .height(px(340.0))
                .inner_ratio(0.5)
                .percentage_decimals(1)
                .outside_label_threshold_degrees(30)
                .value_label_placement(aura_components::ChartValueLabelPlacement::OutsideAligned)
                .tooltip_hit_radius(px(10.0))
                .into_any_element(),
            ]),
            "RingChartCustom" => demo_row(vec![
                aura_components::RingChart::new([
                    aura_components::ChartSeries::new("Desktop", [aura_components::ChartPoint::new("Desktop", 62.0)]).fill_color(gpui::blue()),
                    aura_components::ChartSeries::new("Mobile", [aura_components::ChartPoint::new("Mobile", 24.0)]).fill_color(gpui::green()),
                    aura_components::ChartSeries::new("Tablet", [aura_components::ChartPoint::new("Tablet", 9.0)]).fill_color(gpui::yellow()),
                    aura_components::ChartSeries::new("Other", [aura_components::ChartPoint::new("Other", 5.0)]).fill_color(gpui::red()),
                ])
                .id("docs-ring-chart-custom")
                .height(px(360.0))
                .inner_ratio(0.48)
                .value_label_content(aura_components::ChartValueLabelContent::ValueOverTotalAndPercentage)
                .value_label_placement(aura_components::ChartValueLabelPlacement::OutsideAligned)
                .percentage_decimals(1)
                .outside_label_threshold_degrees(120)
                .into_any_element(),
            ]),
            "RingChartExternal" => demo_row(vec![
                aura_components::RingChart::new([
                    aura_components::ChartSeries::new("Desktop", [aura_components::ChartPoint::new("Desktop", 62.0)]).fill_color(gpui::blue()),
                    aura_components::ChartSeries::new("Mobile", [aura_components::ChartPoint::new("Mobile", 24.0)]).fill_color(gpui::green()),
                    aura_components::ChartSeries::new("Tablet", [aura_components::ChartPoint::new("Tablet", 9.0)]).fill_color(gpui::yellow()),
                    aura_components::ChartSeries::new("Other", [aura_components::ChartPoint::new("Other", 5.0)]).fill_color(gpui::red()),
                ])
                .id("docs-ring-chart-external-vertical")
                .height(px(340.0))
                .inner_ratio(0.58)
                .external_vertical_legend()
                .external_legend_right()
                .external_legend_max_items(3)
                .external_legend_content(aura_components::ChartValueLabelContent::Percentage)
                .external_legend_percentage_decimals(2)
                .into_any_element(),
                aura_components::RingChart::new([
                    aura_components::ChartSeries::new("Desktop", [aura_components::ChartPoint::new("Desktop", 62.0)]).fill_color(gpui::blue()),
                    aura_components::ChartSeries::new("Mobile", [aura_components::ChartPoint::new("Mobile", 24.0)]).fill_color(gpui::green()),
                    aura_components::ChartSeries::new("Tablet", [aura_components::ChartPoint::new("Tablet", 9.0)]).fill_color(gpui::yellow()),
                    aura_components::ChartSeries::new("Other", [aura_components::ChartPoint::new("Other", 5.0)]).fill_color(gpui::red()),
                ])
                .id("docs-ring-chart-external-horizontal")
                .height(px(340.0))
                .inner_ratio(0.7)
                .external_horizontal_legend()
                .external_legend_content(aura_components::ChartValueLabelContent::ValueOverTotalAndPercentage)
                .external_legend_percentage_decimals(1)
                .show_tooltip(false)
                .into_any_element(),
            ]),

            "LineChartBasic" => demo_row(vec![
                aura_components::LineChart::new([aura_components::ChartSeries::new(
                    "CPU Usage",
                    [
                        aura_components::ChartPoint::new("10:00", 24.0),
                        aura_components::ChartPoint::new("10:05", 36.0),
                        aura_components::ChartPoint::new("10:10", 32.0),
                        aura_components::ChartPoint::new("10:15", 52.0),
                        aura_components::ChartPoint::new("10:20", 46.0),
                        aura_components::ChartPoint::new("10:25", 64.0),
                    ],
                )])
                .id("docs-line-chart-basic")
                .height(px(260.0))
                .tooltip_hit_radius(px(16.0))
                .into_any_element(),
            ]),
            "LineChartMulti" => demo_row(vec![
                aura_components::LineChart::new([
                    aura_components::ChartSeries::new(
                        "CPU",
                        [
                            aura_components::ChartPoint::new("Mon", 25.0),
                            aura_components::ChartPoint::new("Tue", 38.0),
                            aura_components::ChartPoint::new("Wed", 42.0),
                            aura_components::ChartPoint::new("Thu", 58.0),
                            aura_components::ChartPoint::new("Fri", 49.0),
                            aura_components::ChartPoint::new("Sat", 72.0),
                            aura_components::ChartPoint::new("Sun", 61.0),
                        ],
                    ),
                    aura_components::ChartSeries::new(
                        "Memory",
                        [
                            aura_components::ChartPoint::new("Mon", 48.0),
                            aura_components::ChartPoint::new("Tue", 52.0),
                            aura_components::ChartPoint::new("Wed", 57.0),
                            aura_components::ChartPoint::new("Thu", 63.0),
                            aura_components::ChartPoint::new("Fri", 66.0),
                            aura_components::ChartPoint::new("Sat", 69.0),
                            aura_components::ChartPoint::new("Sun", 74.0),
                        ],
                    ),
                ])
                .id("docs-line-chart-multi")
                .height(px(300.0))
                .y_domain(0.0, 100.0)
                .into_any_element(),
            ]),
            "LineChartCustom" => demo_row(vec![
                aura_components::LineChart::new([
                    aura_components::ChartSeries::new(
                        "CPU",
                        [
                            aura_components::ChartPoint::new("Mon", 25.0),
                            aura_components::ChartPoint::new("Tue", 38.0),
                            aura_components::ChartPoint::new("Wed", 42.0),
                            aura_components::ChartPoint::new("Thu", 58.0),
                            aura_components::ChartPoint::new("Fri", 49.0),
                        ],
                    )
                    .stroke_color(gpui::blue())
                    .fill_color(gpui::blue().opacity(0.22))
                    .stroke_width(px(3.2))
                    .smooth(true),
                    aura_components::ChartSeries::new(
                        "Memory",
                        [
                            aura_components::ChartPoint::new("Mon", 48.0),
                            aura_components::ChartPoint::new("Tue", 52.0),
                            aura_components::ChartPoint::new("Wed", 57.0),
                            aura_components::ChartPoint::new("Thu", 63.0),
                            aura_components::ChartPoint::new("Fri", 66.0),
                        ],
                    )
                    .stroke_color(gpui::green())
                    .fill_color(gpui::green().opacity(0.18))
                    .stroke_width(px(2.4))
                    .smooth(false),
                ])
                .id("docs-line-chart-custom")
                .height(px(380.0))
                .y_domain(0.0, 100.0)
                .area_fill(true)
                .value_label_content(aura_components::ChartValueLabelContent::Percentage)
                .percentage_decimals(1)
                .into_any_element(),
            ]),
            "LineChartLineStyles" => demo_row(vec![
                aura_components::LineChart::new([
                    aura_components::ChartSeries::new(
                        "Solid Smooth",
                        [
                            aura_components::ChartPoint::new("Mon", 32.0),
                            aura_components::ChartPoint::new("Tue", 44.0),
                            aura_components::ChartPoint::new("Wed", 38.0),
                            aura_components::ChartPoint::new("Thu", 70.0),
                        ],
                    )
                    .stroke_color(gpui::blue())
                    .stroke_width(px(3.2))
                    .line_style(aura_components::ChartLineStyle::Solid)
                    .smooth(true),
                    aura_components::ChartSeries::new(
                        "Dashed",
                        [
                            aura_components::ChartPoint::new("Mon", 22.0),
                            aura_components::ChartPoint::new("Tue", 35.0),
                            aura_components::ChartPoint::new("Wed", 52.0),
                            aura_components::ChartPoint::new("Thu", 58.0),
                        ],
                    )
                    .stroke_color(gpui::green())
                    .stroke_width(px(2.6))
                    .dashed()
                    .smooth(false),
                    aura_components::ChartSeries::new(
                        "Dotted",
                        [
                            aura_components::ChartPoint::new("Mon", 60.0),
                            aura_components::ChartPoint::new("Tue", 54.0),
                            aura_components::ChartPoint::new("Wed", 49.0),
                            aura_components::ChartPoint::new("Thu", 45.0),
                        ],
                    )
                    .stroke_color(gpui::red())
                    .stroke_width(px(2.8))
                    .dotted()
                    .smooth(true),
                ])
                .id("docs-line-chart-line-styles")
                .height(px(320.0))
                .y_domain(0.0, 100.0)
                .area_fill(false)
                .point_markers(false)
                .show_tooltip(false)
                .into_any_element(),
            ]),
            "LineChartEmpty" => demo_row(vec![
                aura_components::LineChart::new(Vec::<aura_components::ChartSeries>::new())
                    .id("docs-line-chart-empty")
                    .height(px(220.0))
                    .into_any_element(),
            ]),
            "LineChartDownsample" => demo_row(vec![
                aura_components::LineChart::new([aura_components::ChartSeries::new(
                    "Latency",
                    (0..2_000).map(|index| {
                        let wave = ((index as f64) / 24.0).sin() * 18.0;
                        let spike = if index % 240 == 0 { 32.0 } else { 0.0 };
                        aura_components::ChartPoint::new(format!("T{index}"), 48.0 + wave + spike)
                    }),
                )])
                .id("docs-line-chart-downsample")
                .height(px(320.0))
                .y_domain(0.0, 100.0)
                .point_markers(false)
                .area_fill(true)
                .max_render_points(180)
                .into_any_element(),
            ]),
            "CardBasic" => demo_row(vec![
                Card::new("Standard card content goes here.")
                    .title("Standard Card")
                    .width_md()
                    .into_any_element(),
                Card::new("This card will change shadow on hover.")
                    .title("Hoverable Card")
                    .hoverable()
                    .width_md()
                    .into_any_element(),
            ]),
            "CardFooter" => Card::new("Card body with a custom footer.")
                .title("Card with Footer")
                .width_lg()
                .footer(
                    aura_components::Row::new()
                        .justify(aura_components::RowJustify::End)
                        .child(Button::new("Cancel").small())
                        .child(Button::new("Save").primary().small()),
                )
                .into_any_element(),
            "EmptyBasic" => Card::new(aura_components::Empty::new())
                .width_md()
                .into_any_element(),
            "EmptyDescription" => Card::new(
                aura_components::Empty::new().description("自定义描述文字"),
            )
            .width_md()
            .into_any_element(),
            "EmptyImage" => Card::new(
                aura_components::Empty::new()
                    .image(aura_icons::Icon::new(IconName::Search))
                    .description("没有找到相关内容"),
            )
            .width_md()
            .into_any_element(),
            "EmptyExtra" => Card::new(aura_components::Empty::new().extra(|_, _| {
                Button::new("去添加").primary().into_any_element()
            }))
            .width_md()
            .into_any_element(),
            "StepsBasic" => aura_components::Steps::new()
                .active(1)
                .step(aura_components::StepItem::new("步骤 1"))
                .step(aura_components::StepItem::new("步骤 2"))
                .step(aura_components::StepItem::new("步骤 3"))
                .into_any_element(),
            "StepsDescription" => aura_components::Steps::new()
                .active(1)
                .step(
                    aura_components::StepItem::new("步骤 1")
                        .description("这是一段描述性文字")
                        .icon(IconName::User),
                )
                .step(
                    aura_components::StepItem::new("步骤 2")
                        .description("这是一段描述性文字")
                        .icon(IconName::Settings),
                )
                .step(
                    aura_components::StepItem::new("步骤 3")
                        .description("这是一段描述性文字")
                        .icon(IconName::Check),
                )
                .into_any_element(),
            "StepsStatus" => aura_components::Steps::new()
                .active(1)
                .step(
                    aura_components::StepItem::new("已完成")
                        .status(aura_components::StepStatus::Finish),
                )
                .step(
                    aura_components::StepItem::new("发生错误")
                        .status(aura_components::StepStatus::Error),
                )
                .step(aura_components::StepItem::new("等待中"))
                .into_any_element(),
            "StepsVertical" => aura_components::Steps::new()
                .active(1)
                .direction(aura_components::StepsDirection::Vertical)
                .step(
                    aura_components::StepItem::new("步骤 1")
                        .description("这是一段很长很长很长的描述性文字"),
                )
                .step(aura_components::StepItem::new("步骤 2"))
                .step(aura_components::StepItem::new("步骤 3"))
                .into_any_element(),
            "TimelineBasic" => Card::new(
                aura_components::Timeline::new()
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-01")
                            .content("创建成功"),
                    )
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-02")
                            .content("通过审核"),
                    )
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-03")
                            .content("项目发布"),
                    ),
            )
            .into_any_element(),
            "TimelineCustom" => Card::new(
                aura_components::Timeline::new()
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-01")
                            .content("成功状态")
                            .success(),
                    )
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-02")
                            .content("警告状态")
                            .warning()
                            .hollow(true),
                    )
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-03")
                            .content("错误状态")
                            .danger()
                            .icon(IconName::CircleX),
                    )
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-04")
                            .content("自定义图标")
                            .primary()
                            .icon(IconName::Star),
                    ),
            )
            .into_any_element(),
            "TimelinePlacement" => Card::new(
                aura_components::Timeline::new()
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-01")
                            .content("时间戳在顶部")
                            .placement(aura_components::TimelinePlacement::Top),
                    )
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-02")
                            .content("时间戳在底部")
                            .placement(aura_components::TimelinePlacement::Bottom),
                    ),
            )
            .into_any_element(),
            "TimelineReverse" => Card::new(
                aura_components::Timeline::new()
                    .reverse(true)
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-01")
                            .content("事件 1"),
                    )
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-02")
                            .content("事件 2"),
                    )
                    .item(
                        aura_components::TimelineItem::new()
                            .timestamp("2026-05-03")
                            .content("事件 3"),
                    ),
            )
            .into_any_element(),
            "BreadcrumbBasic" => aura_components::Breadcrumb::new()
                .item(aura_components::BreadcrumbItem::new("首页"))
                .item(aura_components::BreadcrumbItem::new("活动管理"))
                .item(aura_components::BreadcrumbItem::new("活动列表"))
                .item(aura_components::BreadcrumbItem::new("活动详情"))
                .into_any_element(),
            "BreadcrumbIcon" => aura_components::Breadcrumb::new()
                .item(aura_components::BreadcrumbItem::new("首页").icon(IconName::House))
                .item(aura_components::BreadcrumbItem::new("推广管理"))
                .item(aura_components::BreadcrumbItem::new("推广列表"))
                .item(aura_components::BreadcrumbItem::new("推广详情"))
                .into_any_element(),
            "BreadcrumbSeparator" => aura_components::Breadcrumb::new()
                .separator(">")
                .item(aura_components::BreadcrumbItem::new("首页"))
                .item(aura_components::BreadcrumbItem::new("推广管理"))
                .item(aura_components::BreadcrumbItem::new("推广列表"))
                .item(aura_components::BreadcrumbItem::new("推广详情"))
                .into_any_element(),
            "BreadcrumbSeparatorIcon" => aura_components::Breadcrumb::new()
                .separator_icon(IconName::ChevronRight)
                .item(aura_components::BreadcrumbItem::new("首页"))
                .item(aura_components::BreadcrumbItem::new("推广管理"))
                .item(aura_components::BreadcrumbItem::new("推广列表"))
                .item(aura_components::BreadcrumbItem::new("推广详情"))
                .into_any_element(),
            "BreadcrumbClickable" => aura_components::Breadcrumb::new()
                .item(
                    aura_components::BreadcrumbItem::new("首页")
                        .on_click(|_, _| toast_info!("Home Clicked")),
                )
                .item(
                    aura_components::BreadcrumbItem::new("推广管理")
                        .on_click(|_, _| toast_info!("Management Clicked")),
                )
                .item(aura_components::BreadcrumbItem::new("推广列表"))
                .into_any_element(),
            "PageHeaderBasic" => aura_components::PageHeader::new("详情页面")
                .on_back(|_, _| toast_info!("Back Clicked"))
                .into_any_element(),
            "PageHeaderExtra" => aura_components::PageHeader::new("详情页面")
                .sub_title("子标题")
                .on_back(|_, _| toast_info!("Back Clicked"))
                .extra(|_, _| {
                    Space::new()
                        .gap_sm()
                        .child(Button::new("编辑"))
                        .child(Button::new("主要操作").primary())
                        .into_any_element()
                })
                .into_any_element(),
            "PageHeaderFull" => Card::new(
                aura_components::PageHeader::new("详情页面")
                    .sub_title("子标题")
                    .on_back(|_, _| toast_info!("Back Clicked"))
                    .extra(|_, _| {
                        Space::new()
                            .gap_sm()
                            .child(Button::new("刷新"))
                            .child(Button::new("提交").primary())
                            .into_any_element()
                    })
                    .content(|_, _| {
                        aura_components::Row::new()
                            .child(
                                Space::new()
                                    .vertical()
                                    .gap_xs()
                                    .child(Text::new("创建人"))
                                    .child(Text::new("张三").bold()),
                            )
                            .child(
                                Space::new()
                                    .vertical()
                                    .gap_xs()
                                    .child(Text::new("创建时间"))
                                    .child(Text::new("2026-05-06").bold()),
                            )
                            .into_any_element()
                    })
                    .footer(|_, _| Text::new("页脚内容区域").into_any_element()),
            )
            .no_shadow()
            .into_any_element(),
            "SegmentedBasic" => self
                .segmenteds
                .first()
                .cloned()
                .map(Entity::into_any_element)
                .unwrap_or_else(|| Paragraph::with_text("Missing Segmented demo").into_any_element()),
            "SegmentedDisabled" => self
                .segmenteds
                .first()
                .cloned()
                .map(Entity::into_any_element)
                .unwrap_or_else(|| Paragraph::with_text("Missing Segmented demo").into_any_element()),
            "SegmentedBlock" => self
                .segmenteds
                .first()
                .cloned()
                .map(Entity::into_any_element)
                .unwrap_or_else(|| Paragraph::with_text("Missing Segmented demo").into_any_element()),
            "TooltipBasic" => demo_row(vec![
                aura_components::Tooltip::new(Button::new("Top"))
                    .content("Prompt info")
                    .placement(aura_core::Placement::Top)
                    .into_any_element(),
                aura_components::Tooltip::new(Button::new("Bottom"))
                    .content("Prompt info")
                    .placement(aura_core::Placement::Bottom)
                    .into_any_element(),
                aura_components::Tooltip::new(Button::new("Left"))
                    .content("Prompt info")
                    .placement(aura_core::Placement::Left)
                    .into_any_element(),
                aura_components::Tooltip::new(Button::new("Right"))
                    .content("Prompt info")
                    .placement(aura_core::Placement::Right)
                    .into_any_element(),
            ]),
            "TooltipMore" => demo_row(vec![
                aura_components::Tooltip::new(Button::new("Top Start"))
                    .content("Top Start")
                    .placement(aura_core::Placement::TopStart)
                    .into_any_element(),
                aura_components::Tooltip::new(Button::new("Top End"))
                    .content("Top End")
                    .placement(aura_core::Placement::TopEnd)
                    .into_any_element(),
                aura_components::Tooltip::new(Button::new("Bottom Start"))
                    .content("Bottom Start")
                    .placement(aura_core::Placement::BottomStart)
                    .into_any_element(),
                aura_components::Tooltip::new(Button::new("Bottom End"))
                    .content("Bottom End")
                    .placement(aura_core::Placement::BottomEnd)
                    .into_any_element(),
            ]),
            "PaginationBasic" | "PaginationBackground" | "PaginationAdvanced" => self
                .paginations
                .first()
                .cloned()
                .map(|pagination| Card::new(pagination).into_any_element())
                .unwrap_or_else(|| Paragraph::with_text("Missing Pagination demo").into_any_element()),
            "TabsBasic" | "TabsStretch" | "TabsCard" | "TabsBorderCard" | "TabsEditable" => self
                .tabs
                .first()
                .cloned()
                .map(Entity::into_any_element)
                .unwrap_or_else(|| Paragraph::with_text("Missing Tabs demo").into_any_element()),
            "TabsPosition" => demo_row(
                self.tabs
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "LayoutDivider" => layout_divider_demo(),
            "LayoutSpace" => layout_space_demo(),
            "LayoutGrid" => layout_grid_demo(_cx),
            "ContainerSpace" => container_space_demo(),
            "ContainerDivider" => container_divider_demo(),
            "ContainerLayout" => container_layout_demo(),
            "SplitterBasic" => aura_components::Splitter::new()
                .height_md()
                .bordered()
                .left(Card::new(Text::new("Left panel")).no_shadow())
                .right(Card::new(Text::new("Right panel")).no_shadow())
                .into_any_element(),
            "ScrollbarBasic" => self
                .scrollbars
                .first()
                .cloned()
                .map(Entity::into_any_element)
                .unwrap_or_else(|| Paragraph::with_text("Missing Scrollbar demo").into_any_element()),
            "DescriptionsBasic" => descriptions_basic_demo(),
            "DescriptionsBorder" => descriptions_border_demo(),
            "DescriptionsVertical" => descriptions_vertical_demo(),
            "TableSortable" => self.sortable_table_demo(_cx),
            "TableBasic" => table_basic_demo(),
            "TableStripeBorder" => table_basic_table().stripe(true).border(true).into_any_element(),
            "TableFixedHeader" => table_fixed_header_demo(),
            "TableLoading" => table_basic_table().loading(true).into_any_element(),
            "TableEmpty" => table_basic_table().empty_text("暂无订单数据").into_any_element(),
            "IconLucide" => icon_lucide_demo(_cx),
            "IconColors" => icon_colors_demo(_cx),
            "IconSizes" => icon_sizes_demo(),
            "ImageBasic" => image_basic_demo(),
            "ImageFit" => image_fit_demo(),
            "ImageStates" => image_states_demo(),
            "ImagePreview" => aura_components::Image::new(REMOTE_DEMO_IMAGE)
                .thumbnail()
                .cover()
                .preview(true)
                .into_any_element(),
            "ColorPickerBasic" => demo_stack(vec![
                self.color_picker_element(),
                Text::new("点击颜色方块打开 popup；在大色板中选择颜色，右侧切换 hue，下方选择 alpha。")
                    .into_any_element(),
            ]),
            "ColorPickerPresets" | "ColorPickerCompact" | "ColorPickerDisabled" => {
                self.color_picker_element()
            }
            "TimePickerBasic" => demo_stack(vec![
                self.time_picker_element(),
                Text::new(format!("当前选择：{}", self.time_picker_selected_text(_cx)))
                    .into_any_element(),
            ]),
            "TimePickerFormatted"
            | "TimePickerStepped"
            | "TimePickerNoSeconds"
            | "TimePickerDisabled" => self.time_picker_element(),
            "CascaderBasic" => demo_stack(vec![
                self.cascader_element(),
                Text::new("点击含子级的选项会展开下一列，点击叶子节点完成选择。")
                    .into_any_element(),
            ]),
            "CascaderFilterable" => demo_stack(vec![
                self.cascader_element(),
                Text::new(r#"示例预置 search_query="hang"，展开后可查看路径匹配结果。"#)
                    .into_any_element(),
            ]),
            "CascaderLazy" => demo_stack(vec![
                self.cascader_element(),
                Text::new("点击空子级分支时触发 on_lazy_load，并通过 set_children_at_path 写回子节点。")
                    .into_any_element(),
            ]),
            "CascaderSelected" | "CascaderDisabled" => self.cascader_element(),
            "CollapseBasic" | "CollapseAccordion" => self.collapse_element(),
            "DatePickerBasic" => demo_stack(vec![
                self.date_picker_element(),
                Text::new(format!(
                    "当前选择：{}",
                    self.date_picker_selected_text(_cx)
                ))
                .into_any_element(),
            ]),
            "DatePickerFormatted"
            | "DatePickerRange"
            | "DatePickerMonth"
            | "DatePickerMonthRange"
            | "DatePickerYear"
            | "DatePickerYearRange"
            | "DatePickerDisabled" => self.date_picker_element(),
            "DateTimePickerBasic" => demo_stack(vec![
                self.date_time_picker_element(),
                Text::new(format!(
                    "当前选择：{}",
                    self.date_time_picker_selected_text(_cx)
                ))
                .into_any_element(),
            ]),
            "DateTimePickerFormatted"
            | "DateTimePickerStepped"
            | "DateTimePickerNoSeconds"
            | "DateTimePickerRange"
            | "DateTimePickerDisabled" => self.date_time_picker_element(),
            "DialogBasic" => Button::new("Open Dialog")
                .primary()
                .on_click(|_, _, cx| {
                    aura_components::Dialog::new()
                        .title("Tips")
                        .content(|_, _| dialog_body("This is a message from the dialog."))
                        .show(cx);
                })
                .into_any_element(),
            "DialogManualClose" => Button::new("Manual Close Only")
                .warning()
                .on_click(|_, _, cx| {
                    aura_components::Dialog::new()
                        .title("Manual close dialog")
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .content(|_, _| {
                            Space::new()
                                .vertical()
                                .gap_lg()
                                .child(Text::new(
                                    "点击遮罩和按 ESC 都不会关闭，只能点击按钮手动关闭。",
                                ))
                                .child(
                                    aura_components::Row::new()
                                        .justify(aura_components::RowJustify::End)
                                        .child(Button::new("I understand").primary().on_click(
                                            |_, _, cx| aura_components::Dialog::close(cx),
                                        )),
                                )
                        })
                        .show(cx);
                })
                .into_any_element(),
            "DialogCustomContent" => Button::new("Form-like Content")
                .on_click(|_, _, cx| {
                    aura_components::Dialog::new()
                        .title("Edit profile")
                        .content(|_, _| {
                            Space::new()
                                .vertical()
                                .gap_md()
                                .child(Text::new("Name: Aura User"))
                                .child(Text::new("Role: Designer"))
                                .child(
                                    aura_components::Row::new()
                                        .justify(aura_components::RowJustify::End)
                                        .child(Button::new("Cancel").on_click(|_, _, cx| {
                                            aura_components::Dialog::close(cx)
                                        }))
                                        .child(Button::new("Save").primary().on_click(
                                            |_, _, cx| aura_components::Dialog::close(cx),
                                        )),
                                )
                        })
                        .show(cx);
                })
                .into_any_element(),
            "DrawerPlacements" => demo_row(vec![
                Button::new("Right Drawer")
                    .primary()
                    .on_click(|_, _, cx| docs_drawer("Right Drawer", aura_components::DrawerPlacement::Right).show(cx))
                    .into_any_element(),
                Button::new("Left Drawer")
                    .on_click(|_, _, cx| docs_drawer("Left Drawer", aura_components::DrawerPlacement::Left).show(cx))
                    .into_any_element(),
                Button::new("Top Drawer")
                    .on_click(|_, _, cx| docs_drawer("Top Drawer", aura_components::DrawerPlacement::Top).height_sm().show(cx))
                    .into_any_element(),
                Button::new("Bottom Drawer")
                    .on_click(|_, _, cx| docs_drawer("Bottom Drawer", aura_components::DrawerPlacement::Bottom).height_sm().show(cx))
                    .into_any_element(),
            ]),
            "DrawerSizes" => demo_row(vec![
                Button::new("Wide Drawer")
                    .on_click(|_, _, cx| docs_drawer("Wide Drawer", aura_components::DrawerPlacement::Right).width_lg().show(cx))
                    .into_any_element(),
                Button::new("Tall Top Drawer")
                    .on_click(|_, _, cx| docs_drawer("Tall Top Drawer", aura_components::DrawerPlacement::Top).height_lg().show(cx))
                    .into_any_element(),
            ]),
            "DrawerManualClose" => Button::new("Manual Close Only")
                .warning()
                .on_click(|_, _, cx| {
                    aura_components::Drawer::new()
                        .title("Manual close drawer")
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .content(|_, _| {
                            Space::new()
                                .vertical()
                                .gap_lg()
                                .child(Text::new("点击遮罩和按 ESC 都不会关闭。"))
                                .child(Button::new("Close Drawer").primary().on_click(
                                    |_, _, cx| aura_components::Drawer::close(cx),
                                ))
                        })
                        .show(cx);
                })
                .into_any_element(),
            "PopoverBasic" => docs_popover_basic().into_any_element(),
            "PopoverPlacements" => docs_popover_placements().into_any_element(),
            "PopoverCloseStrategy" => docs_popover_close_strategy().into_any_element(),
            "PopconfirmBasic" => docs_popconfirm_basic().into_any_element(),
            "PopconfirmPlacements" => docs_popconfirm_placements().into_any_element(),
            "PopconfirmCustomText" => docs_popconfirm_custom_text().into_any_element(),
            "DropdownBasic" => docs_dropdown_basic().into_any_element(),
            "DropdownPlacements" => docs_dropdown_placements().into_any_element(),
            "DropdownCloseStrategy" => docs_dropdown_close_strategy().into_any_element(),
            "MessageBoxBasic" => docs_message_box_basic().into_any_element(),
            "MessageBoxManualClose" => docs_message_box_manual_close().into_any_element(),
            "NotificationTypes" => docs_notification_types().into_any_element(),
            "UploadBasic" | "UploadDrag" | "UploadPictureCard" => self
                .uploads
                .first()
                .cloned()
                .map(Entity::into_any_element)
                .unwrap_or_else(|| Paragraph::with_text("Upload demo is not initialized.").into_any_element()),
            "UploadLimits" => demo_stack(
                self.uploads
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
            "TransferBasic" | "TransferFilterable" | "TransferDisabled" => self
                .transfers
                .first()
                .cloned()
                .map(Entity::into_any_element)
                .unwrap_or_else(|| Paragraph::with_text("Transfer demo is not initialized.").into_any_element()),
            "TreeBasic" | "TreeCheckable" => self
                .trees
                .first()
                .cloned()
                .map(Entity::into_any_element)
                .unwrap_or_else(|| Paragraph::with_text("Tree demo is not initialized.").into_any_element()),
            "MenuHorizontal" | "MenuVertical" | "MenuCollapsed" => self
                .menus
                .first()
                .cloned()
                .map(Entity::into_any_element)
                .unwrap_or_else(|| {
                    Paragraph::with_text("Menu demo is not initialized.").into_any_element()
                }),
            "AffixTop" | "AffixBottom" => docs_affix_scroll_shell(
                self.affixes.first().cloned().map(Entity::into_any_element),
                false,
                _cx,
            ),
            "AffixContainer" => docs_affix_scroll_shell(
                self.affixes.first().cloned().map(Entity::into_any_element),
                true,
                _cx,
            ),
            "AnchorBasic" | "AnchorNested" | "AnchorTargets" => self
                .anchors
                .first()
                .cloned()
                .map(|anchor| docs_anchor_shell(self.scroll_handle.clone(), anchor, _cx))
                .unwrap_or_else(|| {
                    Paragraph::with_text("Anchor demo is not initialized.").into_any_element()
                }),
            "BacktopBasic" | "BacktopCustom" | "BacktopContainer" => self
                .backtops
                .first()
                .cloned()
                .map(|backtop| docs_backtop_shell(self.scroll_handle.clone(), backtop, _cx))
                .unwrap_or_else(|| {
                    Paragraph::with_text("Backtop demo is not initialized.").into_any_element()
                }),
            "FormBasic" => {
                docs_form_basic(&self.form_inputs, &self.form_selects, &self.form_switches)
            }
            "FormValidation" => docs_form_validation(&self.form_inputs, &self.form_textareas),
            "FormInline" => docs_form_inline(&self.form_inputs, &self.form_selects),
            "PreviewImageTrigger" => docs_preview_image_trigger().into_any_element(),
            "PreviewCustomTrigger" => docs_preview_custom_trigger(_cx).into_any_element(),
            "PreviewEscape" => docs_preview_escape().into_any_element(),
            "TimerCountUp" => Timer::count_up(std::time::Duration::ZERO)
                .id("docs-live-timer-count-up")
                .start()
                .title("Build elapsed")
                .display_unit(TimerUnit::Seconds)
                .into_any_element(),
            "TimerCountDown" => Timer::count_down(
                std::time::Duration::from_secs(300),
                std::time::Duration::from_secs(84),
            )
            .id("docs-live-timer-count-down")
            .start()
            .title("Deploy window")
            .display_unit(TimerUnit::Minutes)
            .prefix("剩余")
            .into_any_element(),
            "TimerUnits" => demo_row(vec![
                Timer::count_up(std::time::Duration::from_millis(1532))
                    .title("Latency")
                    .display_unit(TimerUnit::Milliseconds)
                    .compact()
                    .into_any_element(),
                Timer::count_up(std::time::Duration::from_secs(64))
                    .display_unit(TimerUnit::Seconds)
                    .show_unit(false)
                    .prefix("T+")
                    .suffix("seconds")
                    .compact()
                    .into_any_element(),
            ]),
            "TimerClock" => demo_row(vec![
                Timer::count_up(std::time::Duration::from_secs(3661))
                    .id("docs-live-timer-clock-up")
                    .start()
                    .title("Elapsed clock")
                    .clock_format()
                    .into_any_element(),
                Timer::count_down(
                    std::time::Duration::from_secs(7200),
                    std::time::Duration::from_secs(139),
                )
                .id("docs-live-timer-clock-down")
                .start()
                .title("Remaining clock")
                .format(TimerFormat::Clock)
                .prefix("剩余")
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

fn layout_divider_demo() -> AnyElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(Text::new("Horizontal (default)"))
        .child(aura_components::Divider::new())
        .child(Text::new("With label"))
        .child(aura_components::Divider::new().label("Center Text"))
        .child(Text::new("Vertical"))
        .child(
            aura_components::Flex::new()
                .row()
                .height_units(60.0)
                .gap_lg()
                .align_center()
                .child(Text::new("Left"))
                .child(aura_components::Divider::new().vertical())
                .child(Text::new("Right")),
        )
        .into_any_element()
}

fn layout_space_demo() -> AnyElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(Text::new("Horizontal gap (default 8px):"))
        .child(
            Space::new()
                .child(Button::new("Button 1"))
                .child(Button::new("Button 2"))
                .child(Button::new("Button 3")),
        )
        .child(Text::new("Vertical gap:"))
        .child(
            Space::new()
                .vertical()
                .gap_xl()
                .child(Button::new("Vertical 1").primary())
                .child(Button::new("Vertical 2").primary()),
        )
        .into_any_element()
}

fn layout_grid_demo(cx: &mut Context<LiveDemoContent>) -> AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    Space::new()
        .vertical()
        .gap_sm()
        .child(
            aura_components::Row::new().column(aura_components::Col::new(24).child(grid_box(
                &theme,
                "span 24",
                gpui::blue(),
            ))),
        )
        .child(
            aura_components::Row::new()
                .column(aura_components::Col::new(12).child(grid_box(
                    &theme,
                    "span 12",
                    gpui::red(),
                )))
                .column(aura_components::Col::new(12).child(grid_box(
                    &theme,
                    "span 12",
                    gpui::green(),
                ))),
        )
        .child(
            aura_components::Row::new()
                .column(aura_components::Col::new(8).child(grid_box(
                    &theme,
                    "span 8",
                    gpui::blue(),
                )))
                .column(aura_components::Col::new(8).child(grid_box(&theme, "span 8", gpui::red())))
                .column(aura_components::Col::new(8).child(grid_box(
                    &theme,
                    "span 8",
                    gpui::green(),
                ))),
        )
        .into_any_element()
}

fn container_space_demo() -> AnyElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(
            Space::new()
                .child(Button::new("Button 1"))
                .child(Button::new("Button 2"))
                .child(Button::new("Button 3")),
        )
        .child(
            Space::new()
                .vertical()
                .child(Button::new("Vertical 1").primary())
                .child(Button::new("Vertical 2").primary()),
        )
        .into_any_element()
}

fn container_divider_demo() -> AnyElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(
            Space::new()
                .vertical()
                .child(Text::new("Above divider"))
                .child(aura_components::Divider::new())
                .child(Text::new("Below divider")),
        )
        .child(aura_components::Divider::new().label("Center Label"))
        .child(
            aura_components::Flex::new()
                .row()
                .align_center()
                .gap_lg()
                .height_units(48.0)
                .child(Text::new("Section 1"))
                .child(aura_components::Divider::new().vertical())
                .child(Text::new("Section 2"))
                .child(aura_components::Divider::new().vertical())
                .child(Text::new("Section 3")),
        )
        .into_any_element()
}

fn container_layout_demo() -> AnyElement {
    aura_components::Flex::new()
        .height_units(300.0)
        .w_full()
        .border()
        .child(
            aura_components::Container::new()
                .header(Title::new("Header").h5())
                .aside(
                    aura_components::Flex::new()
                        .padding_md()
                        .child(Text::new("Aside Sidebar")),
                )
                .footer(Text::new("Footer"))
                .child(
                    aura_components::Flex::new()
                        .padding_md()
                        .child(Text::new("Main Content Area")),
                ),
        )
        .into_any_element()
}

fn grid_box(theme: &aura_theme::Theme, text: &str, color: gpui::Hsla) -> impl IntoElement {
    aura_components::Flex::new()
        .row()
        .bg(color.opacity(0.5))
        .height_units(36.0)
        .rounded_units(4.0)
        .center()
        .text_color(theme.neutral.text_1)
        .text_xs()
        .child(text.to_string())
}

impl LiveDemoContent {
    fn sortable_table_demo(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        let view = cx.entity().clone();
        let sort_key = self.table_sort_key.clone();
        let sort_order = self.table_sort_order;

        let mut table = aura_components::Table::new(table_sortable_columns(&theme))
            .rows(table_sorted_rows(sort_key.as_ref(), sort_order))
            .stripe(true)
            .border(true)
            .on_sort_change(move |state, _, cx| {
                view.update(cx, |this, cx| {
                    this.table_sort_key = state.order.map(|_| state.key.clone());
                    this.table_sort_order = state.order;
                    cx.notify();
                });
            });

        if let Some(key) = sort_key {
            table = table.sort(key, sort_order);
        }

        table.into_any_element()
    }

    fn virtualized_table_sortable_demo(&self, cx: &mut Context<Self>) -> AnyElement {
        let view = cx.entity().clone();
        let sort_key = self.table_sort_key.clone();
        let sort_order = self.table_sort_order;
        let mut table = virtualized_table_demo(true, sort_key.clone(), sort_order).on_sort_change(
            move |state, _, cx| {
                view.update(cx, |this, cx| {
                    this.table_sort_key = state.order.map(|_| state.key.clone());
                    this.table_sort_order = state.order;
                    cx.notify();
                });
            },
        );
        if let Some(key) = sort_key {
            table = table.sort(key, sort_order);
        }
        table.into_any_element()
    }

    fn color_picker_element(&self) -> AnyElement {
        self.color_pickers
            .first()
            .cloned()
            .map(Entity::into_any_element)
            .unwrap_or_else(|| Paragraph::with_text("Missing ColorPicker demo").into_any_element())
    }

    fn time_picker_element(&self) -> AnyElement {
        self.time_pickers
            .first()
            .cloned()
            .map(Entity::into_any_element)
            .unwrap_or_else(|| Paragraph::with_text("Missing TimePicker demo").into_any_element())
    }

    fn time_picker_selected_text(&self, cx: &Context<Self>) -> String {
        self.time_pickers
            .first()
            .and_then(|picker| picker.read(cx).value_ref())
            .map(|value| value.format())
            .unwrap_or_else(|| "尚未选择".to_string())
    }

    fn cascader_element(&self) -> AnyElement {
        self.cascaders
            .first()
            .cloned()
            .map(Entity::into_any_element)
            .unwrap_or_else(|| Paragraph::with_text("Missing Cascader demo").into_any_element())
    }

    fn collapse_element(&self) -> AnyElement {
        self.collapses
            .first()
            .cloned()
            .map(Entity::into_any_element)
            .unwrap_or_else(|| Paragraph::with_text("Missing Collapse demo").into_any_element())
    }

    fn date_picker_element(&self) -> AnyElement {
        self.date_pickers
            .first()
            .cloned()
            .map(Entity::into_any_element)
            .unwrap_or_else(|| Paragraph::with_text("Missing DatePicker demo").into_any_element())
    }

    fn date_time_picker_element(&self) -> AnyElement {
        self.date_time_pickers
            .first()
            .cloned()
            .map(Entity::into_any_element)
            .unwrap_or_else(|| {
                Paragraph::with_text("Missing DateTimePicker demo").into_any_element()
            })
    }

    fn date_picker_selected_text(&self, cx: &Context<Self>) -> String {
        self.date_pickers
            .first()
            .and_then(|picker| picker.read(cx).value_ref())
            .map(|value| value.format())
            .unwrap_or_else(|| "尚未选择".to_string())
    }

    fn date_time_picker_selected_text(&self, cx: &Context<Self>) -> String {
        self.date_time_pickers
            .first()
            .and_then(|picker| picker.read(cx).value_ref())
            .map(|value| value.format())
            .unwrap_or_else(|| "尚未选择".to_string())
    }
}

const REMOTE_DEMO_IMAGE: &str =
    "https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg";

fn local_demo_image() -> String {
    format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR"))
}

fn icon_labeled(icon: aura_icons::Icon, label: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .align_center()
        .gap_xs()
        .child(icon)
        .child(Text::new(label).nowrap())
}

fn icon_lucide_demo(cx: &mut Context<LiveDemoContent>) -> AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    demo_row(
        [
            (IconName::House, "Home"),
            (IconName::User, "User"),
            (IconName::Search, "Search"),
            (IconName::Check, "Check"),
            (IconName::ChevronDown, "ChevronDown"),
            (IconName::Settings, "Settings"),
        ]
        .into_iter()
        .map(|(icon, label)| {
            icon_labeled(
                aura_icons::Icon::new(icon)
                    .size_lg()
                    .color(theme.primary.base),
                label,
            )
            .into_any_element()
        })
        .collect(),
    )
}

fn icon_colors_demo(cx: &mut Context<LiveDemoContent>) -> AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    demo_row(vec![
        icon_labeled(
            aura_icons::Icon::new(IconName::Star)
                .size_lg()
                .color(theme.primary.base),
            "Primary",
        )
        .into_any_element(),
        icon_labeled(
            aura_icons::Icon::new(IconName::Star)
                .size_lg()
                .color(theme.success.base),
            "Success",
        )
        .into_any_element(),
        icon_labeled(
            aura_icons::Icon::new(IconName::Star)
                .size_lg()
                .color(theme.warning.base),
            "Warning",
        )
        .into_any_element(),
        icon_labeled(
            aura_icons::Icon::new(IconName::Star)
                .size_lg()
                .color(theme.danger.base),
            "Danger",
        )
        .into_any_element(),
    ])
}

fn icon_sizes_demo() -> AnyElement {
    demo_row(vec![
        icon_labeled(aura_icons::Icon::new(IconName::House).size_xs(), "12px").into_any_element(),
        icon_labeled(aura_icons::Icon::new(IconName::House).size_md(), "18px").into_any_element(),
        icon_labeled(aura_icons::Icon::new(IconName::House).size_lg(), "24px").into_any_element(),
        icon_labeled(aura_icons::Icon::new(IconName::House).size_xl(), "32px").into_any_element(),
    ])
}

fn image_basic_demo() -> AnyElement {
    let local = local_demo_image();
    demo_row(vec![
        aura_components::Image::new(REMOTE_DEMO_IMAGE)
            .thumbnail()
            .cover()
            .into_any_element(),
        aura_components::Image::new(local.clone())
            .thumbnail()
            .cover()
            .into_any_element(),
        aura_components::Image::new(local)
            .thumbnail()
            .contain()
            .into_any_element(),
    ])
}

fn image_fit_demo() -> AnyElement {
    let local = local_demo_image();
    demo_row(
        [
            ("Fill", aura_components::ImageFit::Fill),
            ("Contain", aura_components::ImageFit::Contain),
            ("Cover", aura_components::ImageFit::Cover),
            ("ScaleDown", aura_components::ImageFit::ScaleDown),
        ]
        .into_iter()
        .map(|(label, fit)| {
            Card::new(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(
                        aura_components::Image::new(local.clone())
                            .thumbnail_sm()
                            .fit(fit),
                    )
                    .child(Text::new(label).nowrap()),
            )
            .no_shadow()
            .into_any_element()
        })
        .collect(),
    )
}

fn image_states_demo() -> AnyElement {
    let local = local_demo_image();
    demo_row(vec![
        labeled_image(
            aura_components::Image::new(local.clone())
                .square_lg()
                .cover()
                .round(),
            "Circle",
        ),
        labeled_image(
            aura_components::Image::new(local.clone())
                .thumbnail_sm()
                .cover()
                .round_options(aura_components::ImageRoundOptions::without_square_crop()),
            "Round bounds",
        ),
        labeled_image(
            aura_components::Image::new(local.clone())
                .square_lg()
                .cover()
                .round_sleeve(),
            "Ring sleeve",
        ),
        labeled_image(
            aura_components::Image::new(local)
                .thumbnail()
                .cover()
                .radius(aura_components::ImageRadius::Large)
                .shadow(true),
            "Shadow",
        ),
        labeled_image(
            aura_components::Image::new("aura://missing-image.png")
                .thumbnail()
                .alt("加载失败"),
            "Fallback",
        ),
        labeled_image(aura_components::Image::empty().thumbnail(), "Empty"),
    ])
}

fn labeled_image(image: aura_components::Image, label: &'static str) -> AnyElement {
    Space::new()
        .vertical()
        .align_center()
        .gap_sm()
        .child(image)
        .child(Text::new(label).nowrap())
        .into_any_element()
}

fn descriptions_basic_demo() -> AnyElement {
    aura_components::Descriptions::new()
        .title("用户信息")
        .item("用户名", "kooriookami", 1)
        .item("手机号", "18100000000", 1)
        .item("居住地", "苏州市", 1)
        .item("备注", Text::new("学校").bg(gpui::blue().opacity(0.1)), 1)
        .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2)
        .into_any_element()
}

fn descriptions_border_demo() -> AnyElement {
    aura_components::Descriptions::new()
        .title("用户信息")
        .border(true)
        .extra(Button::new("操作").primary().small())
        .item("用户名", "kooriookami", 1)
        .item("手机号", "18100000000", 1)
        .item("居住地", "苏州市", 1)
        .item("备注", "学校", 1)
        .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2)
        .into_any_element()
}

fn descriptions_vertical_demo() -> AnyElement {
    aura_components::Descriptions::new()
        .title("垂直布局")
        .border(true)
        .direction(aura_components::DescriptionsDirection::Vertical)
        .item("用户名", "kooriookami", 1)
        .item("手机号", "18100000000", 1)
        .item("居住地", "苏州市", 1)
        .item("备注", "学校", 1)
        .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2)
        .into_any_element()
}

#[derive(Clone)]
struct DocsOrderRecord {
    date: &'static str,
    name: &'static str,
    address: &'static str,
    status: &'static str,
}

fn table_records() -> Vec<DocsOrderRecord> {
    vec![
        DocsOrderRecord {
            date: "2016-05-03",
            name: "Tom",
            address: "上海市普陀区金沙江路 1518 弄",
            status: "已完成",
        },
        DocsOrderRecord {
            date: "2016-05-02",
            name: "Jack",
            address: "上海市普陀区金沙江路 1517 弄",
            status: "进行中",
        },
        DocsOrderRecord {
            date: "2016-05-04",
            name: "Alice",
            address: "上海市普陀区金沙江路 1519 弄",
            status: "已完成",
        },
        DocsOrderRecord {
            date: "2016-05-01",
            name: "Bob",
            address: "上海市普陀区金沙江路 1516 弄",
            status: "待处理",
        },
    ]
}

fn table_basic_columns() -> Vec<aura_components::TableColumn> {
    vec![
        aura_components::TableColumn::new("date", "日期").width_sm(),
        aura_components::TableColumn::new("name", "姓名").width_sm(),
        aura_components::TableColumn::new("address", "地址").min_width_lg(),
        aura_components::TableColumn::new("status", "状态")
            .width_sm()
            .align(aura_components::TableAlign::Center),
        aura_components::TableColumn::new("action", "操作")
            .width_sm()
            .align(aura_components::TableAlign::Right),
    ]
}

fn table_sortable_columns(theme: &aura_theme::Theme) -> Vec<aura_components::TableColumn> {
    vec![
        aura_components::TableColumn::new("date", "日期")
            .width_sm()
            .sortable(),
        aura_components::TableColumn::new("name", "姓名")
            .header(
                Text::new("客户")
                    .bold()
                    .text_color(theme.primary.base)
                    .nowrap(),
            )
            .width_sm()
            .sortable(),
        aura_components::TableColumn::new("address", "地址").min_width_lg(),
        aura_components::TableColumn::new("status", "状态")
            .width_sm()
            .align(aura_components::TableAlign::Center)
            .sortable(),
        aura_components::TableColumn::new("action", "操作")
            .width_sm()
            .align(aura_components::TableAlign::Right),
    ]
}

fn table_basic_table() -> aura_components::Table {
    aura_components::Table::new(table_basic_columns()).rows(table_basic_rows())
}

fn table_basic_demo() -> AnyElement {
    table_basic_table().into_any_element()
}

fn table_fixed_header_demo() -> AnyElement {
    aura_components::Table::new(table_basic_columns())
        .rows(table_long_rows())
        .stripe(true)
        .fixed_header(true)
        .height_md()
        .into_any_element()
}

fn table_basic_rows() -> Vec<aura_components::TableRow> {
    table_records().into_iter().map(table_record_row).collect()
}

fn table_sorted_rows(
    sort_key: Option<&SharedString>,
    sort_order: Option<aura_components::TableSortOrder>,
) -> Vec<aura_components::TableRow> {
    let mut records = table_records();
    if let (Some(key), Some(order)) = (sort_key, sort_order) {
        records.sort_by(|a, b| table_field_value(a, key).cmp(table_field_value(b, key)));
        if order == aura_components::TableSortOrder::Descending {
            records.reverse();
        }
    }
    records.into_iter().map(table_record_row).collect()
}

fn table_field_value<'a>(record: &'a DocsOrderRecord, key: &SharedString) -> &'a str {
    match key.as_ref() {
        "date" => record.date,
        "name" => record.name,
        "status" => record.status,
        "address" => record.address,
        _ => "",
    }
}

fn table_long_rows() -> Vec<aura_components::TableRow> {
    (1..=16)
        .map(|i| {
            table_row(
                match i % 4 {
                    0 => "2016-05-04",
                    1 => "2016-05-01",
                    2 => "2016-05-02",
                    _ => "2016-05-03",
                },
                match i % 4 {
                    0 => "Tom",
                    1 => "Jack",
                    2 => "Alice",
                    _ => "Bob",
                },
                "上海市普陀区金沙江路 1518 弄",
                if i % 3 == 0 { "待处理" } else { "已完成" },
            )
        })
        .collect()
}

fn table_record_row(record: DocsOrderRecord) -> aura_components::TableRow {
    table_row(record.date, record.name, record.address, record.status)
}

fn table_row(
    date: &'static str,
    name: &'static str,
    address: &'static str,
    status: &'static str,
) -> aura_components::TableRow {
    aura_components::TableRow::new()
        .cell("date", date)
        .cell("name", name)
        .cell("address", address)
        .cell("status", table_status_tag(status))
        .cell("action", Button::new("查看").primary().small())
}

fn table_status_tag(status: &'static str) -> aura_components::Tag {
    let tag = aura_components::Tag::new(status).round(true).small();
    match status {
        "已完成" => tag.success(),
        "进行中" => tag.info(),
        _ => tag.warning(),
    }
}

fn virtualized_table_demo(
    sortable: bool,
    sort_key: Option<SharedString>,
    sort_order: Option<aura_components::TableSortOrder>,
) -> VirtualizedTable {
    let reverse = sort_order == Some(aura_components::TableSortOrder::Descending);
    let mut table = VirtualizedTable::new(
        virtualized_table_columns(sortable),
        10_000,
        move |row, key, _window, _cx| virtualized_table_cell(row, key, reverse),
    )
    .height(px(360.0))
    .row_height(px(52.0))
    .stripe(true)
    .border(true);

    if let Some(key) = sort_key {
        table = table.sort(key, sort_order);
    }

    table
}

fn virtualized_table_columns(sortable: bool) -> Vec<aura_components::TableColumn> {
    let columns = vec![
        aura_components::TableColumn::new("date", "日期").width_sm(),
        aura_components::TableColumn::new("name", "客户").width_sm(),
        aura_components::TableColumn::new("region", "区域").width_sm(),
        aura_components::TableColumn::new("amount", "金额")
            .width_sm()
            .align(aura_components::TableAlign::Right),
        aura_components::TableColumn::new("status", "状态")
            .width_sm()
            .align(aura_components::TableAlign::Center),
        aura_components::TableColumn::new("action", "操作")
            .width_sm()
            .align(aura_components::TableAlign::Right),
    ];
    if sortable {
        columns
            .into_iter()
            .map(|column| column.sortable())
            .collect()
    } else {
        columns
    }
}

fn virtualized_table_cell(row: usize, key: &SharedString, reverse: bool) -> gpui::AnyElement {
    let index = if reverse { 9_999 - row } else { row };
    match key.as_ref() {
        "date" => Text::new(format!("2026-06-{:02}", index % 28 + 1)).into_any_element(),
        "name" => Text::new(format!("客户 #{:04}", index + 1))
            .bold()
            .into_any_element(),
        "region" => Text::new(["华东", "华南", "华北", "西南"][index % 4]).into_any_element(),
        "amount" => Text::new(format!(
            "¥{:>6.2}",
            (1_000 + index * 17 % 90_000) as f32 / 10.0
        ))
        .into_any_element(),
        "status" => table_status_tag(["已完成", "进行中", "待处理"][index % 3]).into_any_element(),
        "action" => Button::new("查看").primary().small().into_any_element(),
        _ => Text::new("-").into_any_element(),
    }
}

const DOCS_CODE_EDITOR_RUST_SAMPLE: &str = r#"fn main() {
    println!("Hello Aura");
}
"#;

const DOCS_CODE_EDITOR_TS_SAMPLE: &str = r#"export function run(value: number) {
  return value.toString()
}
"#;

const DOCS_HORIZONTAL_STEPS: &[(&str, &str)] =
    &[("01", "Discover"), ("02", "Design"), ("03", "Build")];
const DOCS_HORIZONTAL_FLOW: &[&str] = &["Input", "Validate", "Transform", "Export"];
const DOCS_HORIZONTAL_LANES: &[(&str, &str)] = &[
    ("Inbox", "8 tasks"),
    ("Ready", "5 tasks"),
    ("Doing", "3 tasks"),
    ("Done", "12 tasks"),
];

fn docs_horizontal_step_card(index: usize) -> AnyElement {
    let (number, label) = DOCS_HORIZONTAL_STEPS[index];
    div()
        .w(px(132.0))
        .h(px(72.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xdbeafe))
        .bg(rgb(0xeff6ff))
        .p_3()
        .flex()
        .flex_col()
        .justify_between()
        .child(
            Text::new(number)
                .size(px(12.0))
                .text_color(rgb(0x2563eb).into()),
        )
        .child(Text::new(label).bold().text_color(rgb(0x1e3a8a).into()))
        .into_any_element()
}

fn docs_horizontal_flow_card(index: usize) -> AnyElement {
    div()
        .w(px(144.0))
        .h(px(68.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xffffff))
        .p_3()
        .child(Text::new(DOCS_HORIZONTAL_FLOW[index]).bold())
        .into_any_element()
}

fn docs_horizontal_lane_card(index: usize) -> AnyElement {
    let (label, desc) = DOCS_HORIZONTAL_LANES[index];
    div()
        .w(px(156.0))
        .h(px(92.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xffffff))
        .p_3()
        .flex()
        .flex_col()
        .gap_2()
        .child(Text::new(label).bold())
        .child(
            Text::new(desc)
                .size(px(12.0))
                .text_color(rgb(0x64748b).into()),
        )
        .into_any_element()
}

fn docs_virtualized_list(cx: &mut Context<VirtualizedList>, draggable: bool) -> VirtualizedList {
    let count = if draggable { 48 } else { 1_000 };
    let mut list = VirtualizedList::new(count, cx, move |index, _window, _cx| {
        div()
            .flex()
            .items_center()
            .justify_between()
            .p_3()
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(0xe5e7eb))
            .child(
                Space::new()
                    .vertical()
                    .gap_xs()
                    .child(
                        Text::new(format!(
                            "{} #{:04}",
                            if draggable { "Task" } else { "Row" },
                            index + 1
                        ))
                        .bold(),
                    )
                    .child(Text::new(if draggable {
                        "Drag this row to reorder visible data."
                    } else {
                        "Rendered inside the native virtual viewport."
                    })),
            )
            .child(AuraTag::new(if index % 2 == 0 { "even" } else { "odd" }).info())
            .into_any_element()
    });

    list.set_height(Some(px(320.0)));
    list.set_item_spacing(px(12.0));
    list.measure_all_items_for_scrollbar();
    if draggable {
        list.set_draggable(true);
        list.set_on_reorder(|from, to, _, _| {
            toast_success!("VirtualizedList reordered: {} -> {}", from + 1, to + 1);
        });
    }
    list
}

fn docs_virtualized_tree(cx: &mut Context<VirtualizedTree>, checkable: bool) -> VirtualizedTree {
    let mut tree = VirtualizedTree::new(docs_virtualized_tree_data(), cx)
        .height(px(360.0))
        .row_height(px(36.0))
        .default_expanded_keys(if checkable {
            vec!["dept-1".into(), "dept-1-team-1".into()]
        } else {
            vec!["dept-0".into(), "dept-0-team-0".into()]
        });

    if checkable {
        tree = tree
            .show_checkbox(true)
            .multiple(true)
            .default_selected_keys(vec!["dept-1-team-1-member-3".into()])
            .on_node_click(|id, _, _| {
                toast_success!("VirtualizedTree selected: {}", id);
            });
    }

    tree
}

fn docs_virtualized_tree_data() -> Vec<TreeNode> {
    (0..24)
        .map(|dept| {
            let mut node = TreeNode::new(
                format!("dept-{dept}"),
                format!("Department {:02}", dept + 1),
            );
            for team in 0..8 {
                let mut team_node = TreeNode::new(
                    format!("dept-{dept}-team-{team}"),
                    format!("Team {:02}-{:02}", dept + 1, team + 1),
                );
                for member in 0..16 {
                    team_node = team_node.child(TreeNode::new(
                        format!("dept-{dept}-team-{team}-member-{member}"),
                        format!("Member {:02}-{:02}-{:02}", dept + 1, team + 1, member + 1),
                    ));
                }
                node = node.child(team_node);
            }
            node
        })
        .collect()
}

fn demo_row(children: Vec<AnyElement>) -> AnyElement {
    Space::new()
        .wrap()
        .gap_sm()
        .children(children)
        .into_any_element()
}

fn dialog_body(message: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_lg()
        .child(Text::new(message))
        .child(
            aura_components::Row::new()
                .justify(aura_components::RowJustify::End)
                .child(
                    Button::new("Close")
                        .primary()
                        .on_click(|_, _, cx| aura_components::Dialog::close(cx)),
                ),
        )
}

fn docs_affix_top() -> Affix {
    Affix::new().offset_lg().content(|_, _| {
        Button::new("固钉在距离顶部 80px 的位置")
            .primary()
            .into_any_element()
    })
}

fn docs_affix_bottom() -> Affix {
    Affix::new()
        .position(AffixPosition::Bottom)
        .offset_md()
        .content(|_, _| {
            Button::new("固钉在距离底部 20px 的位置")
                .success()
                .into_any_element()
        })
}

fn docs_affix_scroll_shell(
    affix: Option<AnyElement>,
    show_container_note: bool,
    cx: &mut Context<LiveDemoContent>,
) -> AnyElement {
    let theme = cx.global::<Config>().theme.clone();

    Flex::new()
        .relative()
        .height_units(360.0)
        .overflow_hidden()
        .border()
        .border_color(theme.neutral.border)
        .rounded_md()
        .bg(theme.neutral.hover)
        .child(
            Flex::new()
                .size_full()
                .id(if show_container_note {
                    "docs-affix-container-scroll-view"
                } else {
                    "docs-affix-scroll-view"
                })
                .overflow_y_scroll()
                .padding_md()
                .child(
                    Flex::new()
                        .height_units(120.0)
                        .center()
                        .child(Text::new("向下滚动查看固钉效果").text_color(theme.neutral.text_3)),
                )
                .when_some(affix, |this, affix| this.child(affix))
                .child(
                    Flex::new()
                        .height_units(520.0)
                        .bg(theme.neutral.card)
                        .margin_y_units(16.0)
                        .border()
                        .border_color(theme.neutral.border)
                        .center()
                        .child(Text::new(if show_container_note {
                            "滚动容器负责触发 Affix 测量"
                        } else {
                            "长内容占位"
                        })),
                )
                .child(Flex::new().height_units(180.0)),
        )
        .into_any_element()
}

fn docs_anchor_basic(scroll_handle: ScrollHandle) -> Anchor {
    Anchor::new(scroll_handle)
        .offset_sm()
        .link(AnchorLink::new("基础用法", "basic"))
        .link(AnchorLink::new("API", "api"))
}

fn docs_anchor_nested(scroll_handle: ScrollHandle) -> Anchor {
    Anchor::new(scroll_handle)
        .offset_sm()
        .link(AnchorLink::new("基础用法", "basic"))
        .link(
            AnchorLink::new("API", "api")
                .child(AnchorLink::new("Attributes", "attributes"))
                .child(AnchorLink::new("Events", "events")),
        )
}

fn docs_anchor_shell(
    scroll_handle: ScrollHandle,
    anchor: Entity<Anchor>,
    cx: &mut Context<LiveDemoContent>,
) -> AnyElement {
    let theme = cx.global::<Config>().theme.clone();

    Flex::new()
        .row()
        .gap_lg()
        .height_units(420.0)
        .overflow_hidden()
        .border()
        .border_color(theme.neutral.border)
        .rounded_md()
        .padding_md()
        .child(
            Flex::new().column().width_percent(72.0).h_full().child(
                Flex::new()
                    .flex_1()
                    .id("docs-anchor-scroll-view")
                    .overflow_y_scroll()
                    .track_scroll(&scroll_handle)
                    .child(
                        Space::new()
                            .vertical()
                            .gap_xl()
                            .child(AnchorTarget::new(
                                "basic",
                                anchor.clone(),
                                docs_anchor_panel(&theme, "基础用法内容区域", 240.0),
                            ))
                            .child(AnchorTarget::new(
                                "api",
                                anchor.clone(),
                                docs_anchor_panel(&theme, "API 内容区域", 160.0),
                            ))
                            .child(AnchorTarget::new(
                                "attributes",
                                anchor.clone(),
                                docs_anchor_panel(&theme, "Attributes 内容区域", 260.0),
                            ))
                            .child(AnchorTarget::new(
                                "events",
                                anchor.clone(),
                                docs_anchor_panel(&theme, "Events 内容区域", 260.0),
                            ))
                            .child(Flex::new().height_units(180.0)),
                    ),
            ),
        )
        .child(Flex::new().width_percent(28.0).child(anchor))
        .into_any_element()
}

fn docs_anchor_panel(
    theme: &aura_theme::Theme,
    label: &'static str,
    height: f32,
) -> impl IntoElement {
    Flex::new()
        .height_units(height)
        .bg(theme.neutral.hover)
        .rounded_md()
        .center()
        .child(Text::new(label))
}

fn docs_backtop_basic(scroll_handle: ScrollHandle) -> Backtop {
    Backtop::new(scroll_handle)
        .id("docs-backtop-basic")
        .visibility_height_sm()
}

fn docs_backtop_custom(scroll_handle: ScrollHandle) -> Backtop {
    Backtop::new(scroll_handle)
        .id("docs-backtop-custom")
        .right_lg()
        .content(|_, cx| {
            let theme = cx.global::<Config>().theme.clone();
            Flex::new()
                .size_full()
                .center()
                .bg(theme.primary.base)
                .child(
                    Icon::new(IconName::ArrowUp)
                        .size_md()
                        .color(theme.neutral.card),
                )
                .into_any_element()
        })
}

fn docs_backtop_shell(
    scroll_handle: ScrollHandle,
    backtop: Entity<Backtop>,
    cx: &mut Context<LiveDemoContent>,
) -> AnyElement {
    let theme = cx.global::<Config>().theme.clone();

    Flex::new()
        .relative()
        .height_units(360.0)
        .overflow_hidden()
        .border()
        .border_color(theme.neutral.border)
        .rounded_md()
        .child(
            Flex::new()
                .size_full()
                .id("docs-backtop-scroll-view")
                .overflow_y_scroll()
                .track_scroll(&scroll_handle)
                .child(Space::new().vertical().gap_sm().children((0..32).map(|i| {
                    Flex::new()
                        .height_units(40.0)
                        .row()
                        .align_center()
                        .padding_x_units(16.0)
                        .bg(theme.neutral.hover)
                        .rounded_units(4.0)
                        .child(Text::new(format!("Scroll Item {i}")))
                }))),
        )
        .child(backtop)
        .into_any_element()
}

fn docs_form_basic(
    inputs: &[Entity<Input>],
    selects: &[Entity<Select>],
    switches: &[Entity<Switch>],
) -> AnyElement {
    Form::new()
        .child(
            FormItem::new()
                .label("Name")
                .required(true)
                .child(inputs[0].clone()),
        )
        .child(FormItem::new().label("Role").child(selects[0].clone()))
        .child(FormItem::new().label("Enabled").child(switches[0].clone()))
        .into_any_element()
}

fn docs_form_validation(inputs: &[Entity<Input>], textareas: &[Entity<Textarea>]) -> AnyElement {
    Form::new()
        .child(
            FormItem::new()
                .label("Title")
                .required(true)
                .error("Title is required")
                .child(inputs[0].clone()),
        )
        .child(
            FormItem::new()
                .label("Description")
                .child(textareas[0].clone()),
        )
        .into_any_element()
}

fn docs_form_inline(inputs: &[Entity<Input>], selects: &[Entity<Select>]) -> AnyElement {
    Form::new()
        .inline(true)
        .child(FormItem::new().label("Keyword").child(inputs[0].clone()))
        .child(FormItem::new().label("Status").child(selects[0].clone()))
        .child(FormItem::new().child(Button::new("Search").primary()))
        .into_any_element()
}

fn docs_preview_image_trigger() -> impl IntoElement {
    let remote = "https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg";

    Space::new()
        .wrap()
        .gap_md()
        .child(Preview::new(remote).child(Image::new(remote).thumbnail().cover().preview(false)))
}

fn docs_preview_custom_trigger(cx: &mut Context<LiveDemoContent>) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();
    let local = format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR"));

    Preview::new(local).child(
        Card::new(
            Space::new()
                .gap_md()
                .child(
                    Icon::new(IconName::Image)
                        .size_lg()
                        .color(theme.primary.base),
                )
                .child(
                    Space::new()
                        .vertical()
                        .gap_xs()
                        .child(Text::new("点击查看大图").bold())
                        .child(Text::new("Preview 可以包裹卡片、按钮或其他元素。")),
                ),
        )
        .no_shadow(),
    )
}

fn docs_preview_escape() -> Preview {
    let remote = "https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg";

    Preview::new(remote)
        .close_on_escape(false)
        .close_on_click_outside(false)
        .hover_effect(false)
        .child(Button::new("打开预览（ESC / 外部点击不关闭）").primary())
}

fn docs_popover_basic() -> Popover {
    Popover::new(Button::new("Bottom Center").primary())
        .id("docs-popover-basic")
        .placement(Placement::Bottom)
        .content(|_, _| {
            Space::new()
                .vertical()
                .gap_sm()
                .child(Text::new("Title").bold())
                .child(Text::new("This is native GPUI popover content."))
                .child(Button::new("Confirm").primary().small())
        })
}

fn docs_popover_placements() -> impl IntoElement {
    Space::new().wrap().gap_sm().children([
        docs_popover_at("TopStart", Placement::TopStart),
        docs_popover_at("Top", Placement::Top),
        docs_popover_at("TopEnd", Placement::TopEnd),
        docs_popover_at("Left", Placement::Left),
        docs_popover_at("Right", Placement::Right),
        docs_popover_at("BottomStart", Placement::BottomStart),
        docs_popover_at("Bottom", Placement::Bottom),
        docs_popover_at("BottomEnd", Placement::BottomEnd),
    ])
}

fn docs_popover_at(label: &'static str, placement: Placement) -> Popover {
    Popover::new(Button::new(label).small())
        .id(format!("docs-popover-placement-{label}"))
        .placement(placement)
        .content(move |_, _| Text::new(format!("Placement: {placement:?}")))
}

fn docs_popover_close_strategy() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_lg()
        .child(
            Popover::new(Button::new("Manual Close Only").warning())
                .id("docs-popover-manual-close")
                .placement(Placement::Bottom)
                .close_on_click_outside(false)
                .close_on_escape(false)
                .content(|_, _| {
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Text::new("Manual close").bold())
                        .child(Text::new("Backdrop and ESC are disabled for this popover."))
                        .child(Button::new("Close Popover").primary().small().on_click(
                            |_, _, cx| {
                                clear_popover(&"docs-popover-manual-close".into(), cx);
                            },
                        ))
                }),
        )
        .child(
            Popover::new(Button::new("Custom Offset"))
                .id("docs-popover-custom-offset")
                .placement(Placement::Bottom)
                .offset_lg()
                .content(|_, _| Text::new("Offset = 20px")),
        )
}

fn docs_popconfirm_basic() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(
            Popconfirm::new(Button::new("Delete").danger())
                .id("docs-popconfirm-delete")
                .title("Are you sure to delete this task?")
                .on_confirm(|_, _| toast_success!("Deleted"))
                .on_cancel(|_, _| toast_warning!("Cancelled")),
        )
        .child(
            Popconfirm::new(Button::new("Archive"))
                .id("docs-popconfirm-archive")
                .title("Archive this item?")
                .confirm_text("Yes")
                .cancel_text("No"),
        )
}

fn docs_popconfirm_placements() -> impl IntoElement {
    Space::new().wrap().gap_md().children([
        docs_popconfirm_at("Top", Placement::Top),
        docs_popconfirm_at("Bottom", Placement::Bottom),
        docs_popconfirm_at("Left", Placement::Left),
        docs_popconfirm_at("Right", Placement::Right),
        docs_popconfirm_at("BottomEnd", Placement::BottomEnd),
    ])
}

fn docs_popconfirm_at(label: &'static str, placement: Placement) -> Popconfirm {
    Popconfirm::new(Button::new(label).small())
        .id(format!("docs-popconfirm-placement-{label}"))
        .title(format!("Confirm at {placement:?}?"))
        .placement(placement)
}

fn docs_popconfirm_custom_text() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(
            Popconfirm::new(Button::new("Publish").success())
                .id("docs-popconfirm-publish")
                .title("Publish current draft?")
                .confirm_text("Publish")
                .cancel_text("Keep editing")
                .placement(Placement::Top),
        )
        .child(
            Popconfirm::new(Button::new("Danger action").danger())
                .id("docs-popconfirm-danger")
                .title("This action cannot be undone.")
                .confirm_text("I understand")
                .cancel_text("Abort")
                .close_on_escape(false)
                .close_on_click_outside(false)
                .placement(Placement::BottomStart),
        )
}

fn docs_dropdown_basic() -> Dropdown {
    Dropdown::new(Button::new("Actions"))
        .id("docs-dropdown-actions")
        .placement(Placement::BottomStart)
        .item("Create", |_, _| toast_info!("Create clicked"))
        .item("Duplicate", |_, _| toast_info!("Duplicate clicked"))
        .item("Archive", |_, _| toast_info!("Archive clicked"))
}

fn docs_dropdown_placements() -> impl IntoElement {
    Space::new().wrap().gap_md().children([
        docs_dropdown_at("docs-dropdown-top", "Top", Placement::Top),
        docs_dropdown_at("docs-dropdown-bottom", "Bottom", Placement::Bottom),
        docs_dropdown_at("docs-dropdown-left", "Left", Placement::Left),
        docs_dropdown_at("docs-dropdown-right", "Right", Placement::Right),
    ])
}

fn docs_dropdown_at(id: &'static str, label: &'static str, placement: Placement) -> Dropdown {
    Dropdown::new(Button::new(label))
        .id(id)
        .placement(placement)
        .item("Action 1", |_, _| toast_info!("Action 1"))
        .item("Action 2", |_, _| toast_info!("Action 2"))
}

fn docs_dropdown_close_strategy() -> Dropdown {
    Dropdown::new(Button::new("Manual close menu"))
        .id("docs-dropdown-manual-close")
        .placement(Placement::BottomStart)
        .close_on_click_outside(false)
        .close_on_escape(false)
        .item("Save draft", |_, _| toast_info!("Save draft"))
        .item("Duplicate", |_, _| toast_info!("Duplicate"))
}

fn docs_message_box_basic() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Button::new("Open Alert").on_click(|_, _, cx| {
            aura_components::alert("Alert Title", "This is an alert message.", cx);
        }))
        .child(Button::new("Open Confirm").primary().on_click(|_, _, cx| {
            aura_components::confirm(
                "Confirm Title",
                "Are you sure you want to proceed?",
                |_, _| toast_success!("Confirmed"),
                cx,
            );
        }))
}

fn docs_message_box_manual_close() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Button::new("Manual Alert").warning().on_click(|_, _, cx| {
            aura_components::MessageBox::new(
                "Manual Alert",
                "Only the OK button can close this message box.",
            )
            .close_on_click_outside(false)
            .close_on_escape(false)
            .alert(cx);
        }))
        .child(Button::new("Manual Confirm").danger().on_click(|_, _, cx| {
            aura_components::MessageBox::new(
                "Manual Confirm",
                "Only Cancel or Confirm can close this message box.",
            )
            .close_on_click_outside(false)
            .close_on_escape(false)
            .confirm(|_, _| toast_success!("Manual confirm accepted"), cx);
        }))
}

fn docs_notification_types() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(
            Button::new("Success Notification")
                .primary()
                .on_click(|_, _, cx| {
                    show_notification(
                        "Success",
                        Some("This is a success description".into()),
                        NotificationType::Success,
                        cx,
                    );
                }),
        )
        .child(Button::new("Info Notification").on_click(|_, _, cx| {
            show_notification(
                "Notification Title",
                Some("This is the content".into()),
                NotificationType::Info,
                cx,
            );
        }))
        .child(
            Button::new("Warning Notification")
                .warning()
                .on_click(|_, _, cx| {
                    show_notification("Warning", None, NotificationType::Warning, cx);
                }),
        )
        .child(
            Button::new("Error Notification")
                .danger()
                .on_click(|_, _, cx| {
                    show_notification(
                        "Error",
                        Some("Detailed error message goes here".into()),
                        NotificationType::Error,
                        cx,
                    );
                }),
        )
}

fn docs_upload_basic() -> Upload {
    Upload::new()
        .button_text("选择文件")
        .accept(".pdf,.fig,.txt")
        .max_size(5 * 1024 * 1024)
        .tip("仅接受 pdf/fig/txt，单文件 ≤ 5MB。")
        .width_lg()
        .add_file(
            UploadFile::new("spec", "产品需求说明.pdf")
                .size(428_000)
                .status(UploadStatus::Success),
        )
        .add_file(
            UploadFile::new("draft", "设计稿-v2.fig")
                .size(2_480_000)
                .status(UploadStatus::Uploading)
                .progress(68),
        )
}

fn docs_upload_drag() -> Upload {
    Upload::new()
        .drag(true)
        .multiple(true)
        .accept(".png,.jpg,.jpeg,.pdf")
        .max_size(2 * 1024 * 1024)
        .button_text("拖拽文件到这里上传")
        .tip("真实拖放接入可由宿主扩展；组件提供原生拖拽区域样式。")
        .width_lg()
        .add_file(
            UploadFile::new("error", "合同扫描件.jpg")
                .size(820_000)
                .status(UploadStatus::Error)
                .description("网络中断"),
        )
}

fn docs_upload_picture_card() -> Upload {
    Upload::new()
        .picture_card()
        .button_text("上传图片")
        .multiple(true)
        .accept("image/*")
        .max_size(2 * 1024 * 1024)
        .width_lg()
        .files([
            UploadFile::new("cover", "cover.png")
                .size(512_000)
                .status(UploadStatus::Success),
            UploadFile::new("banner", "banner.jpg")
                .size(1_240_000)
                .status(UploadStatus::Uploading)
                .progress(42),
        ])
}

fn docs_upload_limited() -> Upload {
    Upload::new()
        .limit(1)
        .accept(".zip,.txt")
        .max_size(10 * 1024 * 1024)
        .button_text("达到数量限制")
        .tip("limit=1 时已有文件，入口自动禁用。")
        .width_lg()
        .add_file(
            UploadFile::new("only", "唯一附件.zip")
                .size(5_120_000)
                .status(UploadStatus::Ready),
        )
}

fn docs_upload_disabled() -> Upload {
    Upload::new()
        .disabled(true)
        .button_text("禁用上传")
        .tip("禁用状态下入口不可用。")
        .width_lg()
}

fn docs_transfer_basic() -> Transfer {
    Transfer::new(docs_city_items())
        .titles("待选城市", "已选城市")
        .target_keys(["shanghai"])
        .checked_source_keys(["beijing", "shenzhen"])
}

fn docs_transfer_filterable() -> Transfer {
    Transfer::new(docs_role_items())
        .titles("全部角色", "已授权")
        .filterable(true)
        .source_filter("admin")
        .target_filter("ops")
        .target_keys(["ops"])
        .checked_source_keys(["admin"])
        .width_lg()
}

fn docs_transfer_disabled() -> Transfer {
    Transfer::new(vec![
        TransferItem::new("beijing", "北京"),
        TransferItem::new("shanghai", "上海"),
        TransferItem::new("disabled", "成都（禁用）")
            .description("不可移动")
            .disabled(true),
    ])
    .titles("源列表", "目标列表")
    .target_keys(["disabled"])
    .checked_target_keys(["disabled"])
}

fn docs_city_items() -> Vec<TransferItem> {
    vec![
        TransferItem::new("beijing", "北京").description("华北区域"),
        TransferItem::new("shanghai", "上海").description("华东区域"),
        TransferItem::new("shenzhen", "深圳").description("华南区域"),
        TransferItem::new("guangzhou", "广州").description("华南区域"),
    ]
}

fn docs_role_items() -> Vec<TransferItem> {
    vec![
        TransferItem::new("admin", "Admin 管理员").description("admin / full access"),
        TransferItem::new("editor", "Editor 编辑").description("content write"),
        TransferItem::new("viewer", "Viewer 只读").description("read only"),
        TransferItem::new("ops", "Ops 运维").description("ops / deploy"),
        TransferItem::new("auditor", "Auditor 审计").description("compliance"),
    ]
}

fn docs_tree_basic() -> Tree {
    Tree::new(vec![
        TreeNode::new("1", "一级 1")
            .child(TreeNode::new("1-1", "二级 1-1").child(TreeNode::new("1-1-1", "三级 1-1-1")))
            .child(TreeNode::new("1-2", "二级 1-2")),
        TreeNode::new("2", "一级 2").child(TreeNode::new("2-1", "二级 2-1")),
    ])
}

fn docs_tree_checkable() -> Tree {
    Tree::new(vec![
        TreeNode::new("docs", "docs").child(TreeNode::new("quick-start", "quick_start.md")),
        TreeNode::new("src", "src").child(TreeNode::new("components", "aura-components")),
    ])
    .show_checkbox(true)
    .multiple(true)
    .on_node_click(|id, _, _| toast_info!("selected node: {}", id))
}

fn docs_menu_horizontal() -> Menu {
    Menu::new()
        .id("docs-menu-horizontal")
        .mode(MenuMode::Horizontal)
        .default_active("1")
        .on_select(|id, _, _| toast_info!("active menu: {}", id))
        .item("1", "处理中心", Some(IconName::List))
        .submenu("2", "我的工作台", Some(IconName::Briefcase), |s| {
            s.item("2-1", "选项1", None).item("2-2", "选项2", None)
        })
        .item("3", "消息中心", Some(IconName::Bell))
}

fn docs_menu_vertical() -> Menu {
    Menu::new()
        .id("docs-menu-vertical")
        .mode(MenuMode::Vertical)
        .default_active("1")
        .on_select(|id, _, _| toast_info!("active menu: {}", id))
        .item("1", "导航一", Some(IconName::House))
        .submenu("2", "导航二", Some(IconName::Settings), |s| {
            s.item("2-1", "选项1", None)
                .item("2-2", "选项2", None)
                .group("分组一", |g| {
                    g.item("2-3", "选项3", None).item("2-4", "选项4", None)
                })
        })
        .item("3", "导航三", Some(IconName::MessageSquare))
}

fn docs_menu_collapsed() -> Menu {
    Menu::new()
        .id("docs-menu-collapsed")
        .mode(MenuMode::Vertical)
        .collapse(true)
        .default_active("1")
        .on_select(|id, _, _| toast_info!("active menu: {}", id))
        .item("1", "导航一", Some(IconName::House))
        .submenu("2", "导航二", Some(IconName::Settings), |s| {
            s.item("2-1", "选项1", None).item("2-2", "选项2", None)
        })
        .item("3", "导航三", Some(IconName::MessageSquare))
}

fn docs_drawer(
    title: &'static str,
    placement: aura_components::DrawerPlacement,
) -> aura_components::Drawer {
    aura_components::Drawer::new()
        .title(title)
        .placement(placement)
        .content(move |_, _| {
            Space::new()
                .vertical()
                .gap_lg()
                .child(Text::new(format!("This is a {:?} drawer.", placement)))
                .child(
                    Button::new("Close")
                        .primary()
                        .on_click(|_, _, cx| aura_components::Drawer::close(cx)),
                )
        })
}

fn docs_region_options() -> Vec<aura_components::CascaderOption> {
    vec![
        aura_components::CascaderOption::new("zhejiang", "浙江")
            .child(
                aura_components::CascaderOption::new("hangzhou", "杭州")
                    .child(aura_components::CascaderOption::new("xihu", "西湖区"))
                    .child(aura_components::CascaderOption::new("yuhang", "余杭区")),
            )
            .child(
                aura_components::CascaderOption::new("ningbo", "宁波")
                    .child(aura_components::CascaderOption::new("haishu", "海曙区"))
                    .child(aura_components::CascaderOption::new("jiangbei", "江北区")),
            ),
        aura_components::CascaderOption::new("jiangsu", "江苏")
            .child(
                aura_components::CascaderOption::new("nanjing", "南京")
                    .child(aura_components::CascaderOption::new("xuanwu", "玄武区"))
                    .child(aura_components::CascaderOption::new("gulou", "鼓楼区")),
            )
            .child(
                aura_components::CascaderOption::new("suzhou", "苏州")
                    .child(aura_components::CascaderOption::new("gusu", "姑苏区"))
                    .child(
                        aura_components::CascaderOption::new("wuzhong", "吴中区").disabled(true),
                    ),
            ),
    ]
}

fn docs_product_options() -> Vec<aura_components::CascaderOption> {
    vec![
        aura_components::CascaderOption::new("cloud", "云产品")
            .child(
                aura_components::CascaderOption::new("compute", "计算")
                    .child(aura_components::CascaderOption::new("ecs", "云服务器 ECS"))
                    .child(aura_components::CascaderOption::new("fc", "函数计算")),
            )
            .child(
                aura_components::CascaderOption::new("storage", "存储")
                    .child(aura_components::CascaderOption::new("oss", "对象存储 OSS"))
                    .child(aura_components::CascaderOption::new("nas", "文件存储 NAS")),
            ),
        aura_components::CascaderOption::new("data", "数据服务").child(
            aura_components::CascaderOption::new("database", "数据库")
                .child(aura_components::CascaderOption::new(
                    "mysql",
                    "云数据库 MySQL",
                ))
                .child(aura_components::CascaderOption::new("redis", "Redis")),
        ),
    ]
}

fn docs_lazy_options() -> Vec<aura_components::CascaderOption> {
    vec![
        aura_components::CascaderOption::new("remote-a", "远程分组 A"),
        aura_components::CascaderOption::new("remote-b", "远程分组 B"),
        aura_components::CascaderOption::new("ready", "本地叶子").leaf(true),
    ]
}

fn docs_lazy_children_for(path: &[SharedString]) -> Vec<aura_components::CascaderOption> {
    let key = path
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("/");

    match key.as_str() {
        "remote-a" => vec![
            aura_components::CascaderOption::new("team", "团队")
                .child(aura_components::CascaderOption::new("design", "设计组").leaf(true)),
            aura_components::CascaderOption::new("project", "项目")
                .child(aura_components::CascaderOption::new("aura", "Aura UI").leaf(true)),
        ],
        "remote-b" => vec![
            aura_components::CascaderOption::new("north", "华北").leaf(true),
            aura_components::CascaderOption::new("south", "华南").leaf(true),
        ],
        _ => vec![aura_components::CascaderOption::new("loaded", "加载结果").leaf(true)],
    }
}

fn docs_collapse(id: &'static str, accordion: bool) -> aura_components::Collapse {
    let collapse = aura_components::Collapse::new()
        .id(id)
        .item("consistency", "Consistency", |_, _| {
            Text::new("Consistent with real life: in line with process and intuition.")
        })
        .item("feedback", "Feedback", |_, _| {
            Text::new("Operation feedback: users clearly perceive style updates.")
        });

    if accordion {
        collapse.accordion()
    } else {
        collapse
    }
}

fn basic_tabs(id: &'static str) -> aura_components::Tabs {
    aura_components::Tabs::new("first")
        .id(id)
        .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
        .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
        .pane("third", "角色管理", |_, _| Text::new("角色管理内容"))
        .pane("fourth", "定时任务", |_, _| Text::new("定时任务内容"))
}

fn short_tabs(id: &'static str) -> aura_components::Tabs {
    aura_components::Tabs::new("first")
        .id(id)
        .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
        .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
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

fn styled_checkbox_cards(cx: &mut Context<CheckboxGroup>) -> CheckboxGroup {
    CheckboxGroup::new(vec!["CPU", "Memory", "Network"], vec![0, 2], cx)
        .horizontal()
        .option_style(
            CheckboxOptionStyle::new()
                .bg(rgb(0xf8fafc).into())
                .selected_bg(rgb(0xdbeafe).into())
                .selected_text_color(rgb(0x1d4ed8).into())
                .selected_border_color(rgb(0x3b82f6).into())
                .hover_bg(rgb(0xeff6ff).into())
                .radius(px(12.0))
                .padding(px(14.0), px(10.0)),
        )
}

fn styled_checkbox_chips(cx: &mut Context<CheckboxGroup>) -> CheckboxGroup {
    CheckboxGroup::new(vec!["Fast", "Stable", "Secure"], vec![1], cx)
        .horizontal()
        .option_style(
            CheckboxOptionStyle::new()
                .bg(gpui::transparent_black())
                .selected_bg(rgb(0x111827).into())
                .selected_text_color(gpui::white())
                .selected_border_color(rgb(0x111827).into())
                .radius(px(999.0))
                .padding(px(16.0), px(8.0))
                .show_indicator(false),
        )
}

fn rich_checkbox_options(cx: &mut Context<CheckboxGroup>) -> CheckboxGroup {
    CheckboxGroup::new(vec!["Analytics", "Alerts", "Exports"], vec![0, 1], cx)
        .horizontal()
        .option_style(
            CheckboxOptionStyle::new()
                .selected_bg(rgb(0xf0fdf4).into())
                .selected_text_color(rgb(0x166534).into())
                .selected_border_color(rgb(0x22c55e).into())
                .hover_bg(rgb(0xf8fafc).into())
                .radius(px(14.0))
                .padding(px(14.0), px(12.0))
                .gap(px(10.0)),
        )
        .option_renderer(|option| {
            let description = match option.index {
                0 => "趋势、漏斗和指标面板",
                1 => "阈值触发与通知策略",
                _ => "CSV / JSON 批量导出",
            };
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(option.label.clone())
                        .when(option.selected, |s| {
                            s.child(Icon::new(IconName::BadgeCheck).size_xs())
                        }),
                )
                .child(div().text_xs().child(description))
                .into_any_element()
        })
}

fn styled_radio_cards(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(vec!["Daily", "Weekly", "Monthly"], 1, cx)
        .horizontal()
        .option_style(
            RadioOptionStyle::new()
                .bg(rgb(0xf8fafc).into())
                .selected_bg(rgb(0xecfeff).into())
                .selected_text_color(rgb(0x0e7490).into())
                .selected_border_color(rgb(0x06b6d4).into())
                .hover_bg(rgb(0xf0fdfa).into())
                .radius(px(12.0))
                .padding(px(14.0), px(10.0)),
        )
}

fn rich_radio_options(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(vec!["Starter", "Team", "Enterprise"], 1, cx)
        .horizontal()
        .option_style(
            RadioOptionStyle::new()
                .selected_bg(rgb(0xfffbeb).into())
                .selected_text_color(rgb(0x92400e).into())
                .selected_border_color(rgb(0xf59e0b).into())
                .hover_bg(rgb(0xfffbeb).into())
                .radius(px(14.0))
                .padding(px(14.0), px(12.0))
                .gap(px(10.0)),
        )
        .option_renderer(|option| {
            let (icon, description) = match option.index {
                0 => (IconName::Rocket, "个人试用与轻量项目"),
                1 => (IconName::Users, "团队协作与权限控制"),
                _ => (IconName::Building2, "审计、SLA 和专属支持"),
            };
            div()
                .flex()
                .items_start()
                .gap_2()
                .child(Icon::new(icon).size_md())
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(option.label.clone())
                                .when(option.selected, |s| {
                                    s.child(Icon::new(IconName::CircleCheck).size_xs())
                                }),
                        )
                        .child(div().text_xs().child(description)),
                )
                .into_any_element()
        })
}

fn styled_radio_chips(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(vec!["Low", "Medium", "High"], 2, cx)
        .horizontal()
        .option_style(
            RadioOptionStyle::new()
                .bg(gpui::transparent_black())
                .selected_bg(rgb(0x7c3aed).into())
                .selected_text_color(gpui::white())
                .selected_border_color(rgb(0x7c3aed).into())
                .radius(px(999.0))
                .padding(px(16.0), px(8.0))
                .show_indicator(false),
        )
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
    code_block = code_block.selectable(true);
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
    stable_path: &str,
) -> AnyElement {
    match block {
        Block::Paragraph(segments) => render_paragraph_with_id(
            segments.clone(),
            theme,
            format!("docs-md-{stable_path}-paragraph"),
        ),
        Block::Heading { level, content } => {
            let heading =
                Title::new(inline_plain_text(content)).id(format!("docs-md-{stable_path}-heading"));
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
                        .enumerate()
                        .map(|(index, block)| {
                            render_persistent_block(
                                block,
                                theme,
                                live_demos,
                                demo_index,
                                &format!("{stable_path}-quote-{index}"),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            )
            .into_any_element(),
        Block::List {
            ordered,
            start,
            items,
        } => render_persistent_list(
            *ordered,
            *start,
            items,
            theme,
            live_demos,
            demo_index,
            stable_path,
        ),
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
    stable_path: &str,
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
            .enumerate()
            .map(|(block_index, block)| {
                render_persistent_block(
                    block,
                    theme,
                    live_demos,
                    demo_index,
                    &format!("{stable_path}-item-{index}-block-{block_index}"),
                )
            })
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
    render_paragraph_with_id(segments, theme, aura_core::unique_id("markdown-paragraph"))
}

fn render_paragraph_with_id(
    segments: Vec<InlineSegment>,
    theme: &aura_theme::Theme,
    id: impl Into<SharedString>,
) -> AnyElement {
    Paragraph::new()
        .id(id)
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
        let theme = cx.global::<Config>().theme.clone();

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
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_4()
                    .rounded(px(theme.radius.md))
                    .border_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.card)
                    .overflow_hidden()
                    .child(Title::new(page.title).h3())
                    .child(div().flex_1().min_h_0().child(page_view)),
            )
            .overlay(DocsPortalLayer)
    }
}

struct DocsPageView {
    virtual_list: Entity<VirtualizedList>,
}

impl DocsPageView {
    fn new(markdown: &'static str, cx: &mut Context<Self>) -> Self {
        let document = MarkdownDocument::parse(markdown);
        let mut demo_components = Vec::new();
        collect_live_demo_components(&document.blocks, &mut demo_components);
        let live_demos: Vec<Entity<LiveDemoHost>> = demo_components
            .into_iter()
            .map(|component| cx.new(|cx| LiveDemoHost::new(component, cx)))
            .collect();

        let virtual_blocks = document.blocks.clone();
        let virtual_live_demos = live_demos.clone();
        let virtual_list = cx.new(|cx| {
            let mut list =
                VirtualizedList::new(document.blocks.len(), cx, move |index, _window, cx| {
                    let theme = cx.global::<Config>().theme.clone();
                    let Some(block) = virtual_blocks.get(index) else {
                        return div().into_any_element();
                    };
                    let mut demo_index = live_demo_index_before(&virtual_blocks, index);
                    render_persistent_block(
                        block,
                        &theme,
                        &virtual_live_demos,
                        &mut demo_index,
                        &format!("block-{index}"),
                    )
                });
            list.set_item_spacing(px(20.0));
            list.measure_all_items_for_scrollbar();
            list
        });

        Self { virtual_list }
    }
}

impl Render for DocsPageView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.virtual_list.clone()
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
    fn adoption_docs_cover_minimal_app_and_public_entrypoints() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        assert!(titles.contains(&"Adoption Guide"));
        assert!(ADOPTION_DOC.contains("cargo run -p aura-minimal-app"));
        assert!(ADOPTION_DOC.contains("examples/minimal-app/src/main.rs"));
        assert!(ADOPTION_DOC.contains("init_aura(cx, Theme::light())"));
        assert!(ADOPTION_DOC.contains("Entity<T>"));
        assert!(include_str!("../../../README.md").contains("cargo run -p aura-minimal-app"));
        assert!(
            include_str!("../../../CONTRIBUTING.md").contains("cargo doc --workspace --no-deps")
        );
        assert!(
            include_str!("../../../examples/minimal-app/Cargo.toml").contains("aura-minimal-app")
        );
    }

    #[test]
    fn dashboard_dogfood_app_is_documented_and_workspace_registered() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        assert!(titles.contains(&"Dashboard App"));
        assert!(DASHBOARD_APP_DOC.contains("cargo run -p aura-dashboard-app"));
        assert!(DASHBOARD_APP_DOC.contains("examples/dashboard-app"));
        assert!(DASHBOARD_APP_DOC.contains("LineChart"));
        assert!(DASHBOARD_APP_DOC.contains("Table"));
        assert!(DASHBOARD_APP_DOC.contains("CodeBlock"));
        assert!(DASHBOARD_APP_DOC.contains("DashboardGrid"));
        assert!(include_str!("../../../README.md").contains("cargo run -p aura-dashboard-app"));
        assert!(include_str!("../../../Cargo.toml").contains("examples/dashboard-app"));
        assert!(
            include_str!("../../../examples/dashboard-app/Cargo.toml")
                .contains("aura-dashboard-app")
        );
    }

    #[test]
    fn dashboard_patterns_cover_composition_helpers_and_theme_switching() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        assert!(titles.contains(&"Dashboard Patterns"));
        assert!(DASHBOARD_PATTERNS_DOC.contains("DashboardGrid"));
        assert!(DASHBOARD_PATTERNS_DOC.contains("dashboard_card"));
        assert!(DASHBOARD_PATTERNS_DOC.contains("metric_card"));
        assert!(DASHBOARD_PATTERNS_DOC.contains("window.refresh()"));
        assert!(
            include_str!("../../../examples/dashboard-app/src/main.rs")
                .contains("DashboardGrid::metrics()")
        );
        assert!(
            include_str!("../../../crates/aura-components/src/dashboard.rs")
                .contains("pub struct DashboardGrid")
        );
    }

    #[test]
    fn dashboard_state_docs_cover_data_flow_model() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        assert!(titles.contains(&"Dashboard State"));
        assert!(DASHBOARD_STATE_DOC.contains("DashboardData"));
        assert!(DASHBOARD_STATE_DOC.contains("DashboardFilters"));
        assert!(DASHBOARD_STATE_DOC.contains("apply_filters"));
        assert!(DASHBOARD_STATE_DOC.contains("DashboardStatus"));
        assert!(
            include_str!("../../../examples/dashboard-app/src/model.rs")
                .contains("pub fn apply_filters")
        );
        assert!(
            include_str!("../../../examples/dashboard-app/src/main.rs")
                .contains("refresh_dashboard")
        );
    }

    #[test]
    fn packaging_docs_explain_ci_and_release_workflow_boundaries() {
        assert!(PACKAGING_WORKFLOW_DOC.contains(".github/workflows/ci.yml"));
        assert!(PACKAGING_WORKFLOW_DOC.contains(".github/workflows/package.yml"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("Should publish release assets?"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("`rust-quality`"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("`packaging-dry-run`"));
        assert!(
            PACKAGING_WORKFLOW_DOC.contains("Only `v*` tag runs publish GitHub Release assets")
        );
        assert!(PACKAGING_WORKFLOW_DOC.contains("If a step builds installers, uploads artifacts, or calls `gh release`, it belongs only in `package.yml`."));
    }

    #[test]
    fn ci_workflow_splits_workspace_and_packaging_dry_run_jobs() {
        let ci = include_str!("../../../.github/workflows/ci.yml");

        assert!(ci.contains("rust-quality:"));
        assert!(ci.contains("packaging-dry-run:"));
        assert!(ci.contains("cargo check --workspace --all-targets"));
        assert!(ci.contains("cargo run -p xtask -- package validate"));
        assert!(ci.contains("cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run"));
        assert!(ci.contains("Install Linux native build dependencies"));
        assert!(ci.contains("Install packaging dry-run tools"));
        assert!(!ci.contains("rpm"));
        assert!(!ci.contains("zsync"));
    }

    #[test]
    fn packaging_docs_and_workflows_include_release_readiness_gate() {
        let ci = include_str!("../../../.github/workflows/ci.yml");
        let package = include_str!("../../../.github/workflows/package.yml");

        assert!(PACKAGING_WORKFLOW_DOC.contains("package release-readiness"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("packaging/signing-policy.md"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("AURA_REQUIRE_SIGNING=true"));
        assert!(ci.contains("Release readiness dry-run policy check"));
        assert!(ci.contains("cargo run -p xtask -- package release-readiness"));
        assert!(package.contains("Check release readiness policy"));
        assert!(package.contains("AURA_REQUIRE_SIGNING"));
        assert!(package.contains("AURA_MACOS_CODESIGN_IDENTITY"));
        assert!(package.contains("AURA_WINDOWS_SIGNTOOL_CERT_PATH"));
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
    fn authored_page_snippets_are_available_to_docs_loader() {
        for page in DOC_PAGES {
            let document = MarkdownDocument::parse(page.markdown);
            for block in document.blocks() {
                assert_page_block_snippets_load(page.title, block);
            }
        }
    }

    fn assert_page_block_snippets_load(page_title: &str, block: &Block) {
        match block {
            Block::CodeBlock {
                source: Some(source),
                ..
            } => {
                assert!(
                    load_code_snippet(source.as_ref()).is_some(),
                    "{page_title} references missing docs loader snippet {source}"
                );
            }
            Block::BlockQuote(children) => {
                for child in children {
                    assert_page_block_snippets_load(page_title, child);
                }
            }
            Block::List { items, .. } => {
                for item in items {
                    for child in item {
                        assert_page_block_snippets_load(page_title, child);
                    }
                }
            }
            _ => {}
        }
    }

    #[test]
    fn quick_start_registers_core_app_key_bindings() {
        let quick_start = include_str!("../content/snippets/quick_start/main_window.rs");
        let gallery = include_str!("../../aura-gallery/src/main.rs");
        let docs = include_str!("main.rs");

        for registration in [
            "Input::register_key_bindings(cx)",
            "CodeBlock::register_key_bindings(cx)",
            "CodeEditor::register_key_bindings(cx)",
            "Preview::register_key_bindings(cx)",
            "Text::register_key_bindings(cx)",
            "Paragraph::register_key_bindings(cx)",
            "Title::register_key_bindings(cx)",
            "Tour::register_key_bindings(cx)",
        ] {
            assert!(
                quick_start.contains(registration),
                "QuickStart main_window.rs missing {registration}"
            );
            assert!(
                gallery.contains(registration),
                "Gallery main.rs missing {registration}"
            );
            assert!(
                docs.contains(registration),
                "Docs main.rs missing {registration}"
            );
        }
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
            "virtualized_list/basic.rs",
            "line_chart/custom.rs",
            "bar_chart/custom.rs",
            "area_chart/custom.rs",
            "pie_chart/custom.rs",
        ] {
            assert!(harness.contains(&format!("../../content/snippets/{snippet}")));
            assert!(load_code_snippet(snippet).is_some());
        }
        assert!(!harness.contains("quick_start/run.sh"));
    }

    #[test]
    fn p13_docs_live_demo_markers_have_precise_renderers() {
        let renderer = include_str!("markdown.rs");
        for (page_name, page) in [
            ("qr_code", QR_CODE_DOC),
            ("code_editor", CODE_EDITOR_DOC),
            ("horizontal_list", HORIZONTAL_LIST_DOC),
            ("virtualized_list", VIRTUALIZED_LIST_DOC),
        ] {
            assert!(
                !page.contains("::::AuraDemo"),
                "{page_name} contains malformed live demo marker"
            );
            let document = MarkdownDocument::parse(page);
            let mut components = Vec::new();
            collect_live_demo_components(document.blocks(), &mut components);
            for component in components {
                assert!(
                    renderer.contains(&format!("\"{}\" =>", component.as_ref())),
                    "{page_name} live demo {component} should have a precise renderer"
                );
            }
        }
    }

    #[test]
    fn docs_rust_code_block_sources_are_compile_checked() {
        let harness = include_str!("bin/check_snippets.rs");
        let docs = [
            BUTTON_DOC,
            INPUT_DOC,
            SWITCH_DOC,
            MESSAGE_DOC,
            TYPOGRAPHY_DOC,
            VIRTUALIZED_LIST_DOC,
            LINE_CHART_DOC,
            BAR_CHART_DOC,
            AREA_CHART_DOC,
            PIE_CHART_DOC,
            PROGRESS_DOC,
        ];

        for doc in docs {
            let mut remaining = doc;
            while let Some(index) = remaining.find("```rust src=\"") {
                let after_marker = &remaining[index + "```rust src=\"".len()..];
                let Some(end_index) = after_marker.find('\"') else {
                    break;
                };
                let snippet = &after_marker[..end_index];
                assert!(
                    harness.contains(&format!("../../content/snippets/{snippet}")),
                    "{snippet} is referenced by docs but missing from check_snippets.rs"
                );
                remaining = &after_marker[end_index + 1..];
            }
        }
    }

    #[test]
    fn code_blocks_render_with_horizontal_scroll_shell() {
        let source = include_str!("../content/pages/code_block.md");

        assert!(source.contains("src=\"code_block/basic.rs\""));
        assert!(source.contains("src=\"code_block/theme.rs\""));
        assert!(load_code_snippet("code_block/basic.rs").is_some());
    }

    #[test]
    fn docs_markdown_code_blocks_keep_selectable_text_enabled() {
        let source = include_str!("markdown.rs");
        let render_code_block = &source[source
            .find("fn render_code_block(")
            .expect("render_code_block should exist")
            ..source
                .find("fn collect_live_demo_components(")
                .expect("collect_live_demo_components should follow")];

        assert!(render_code_block.contains("selectable(true)"));
    }

    #[test]
    fn docs_markdown_text_uses_stable_selection_ids() {
        let source = include_str!("markdown.rs");
        let persistent_renderer = &source[source
            .find("fn render_persistent_block(")
            .expect("render_persistent_block should exist")
            ..source
                .find("fn render_persistent_list(")
                .expect("render_persistent_list should follow")];

        assert!(persistent_renderer.contains("stable_path: &str"));
        assert!(persistent_renderer.contains("docs-md-{stable_path}-paragraph"));
        assert!(persistent_renderer.contains("docs-md-{stable_path}-heading"));
        assert!(
            !persistent_renderer.contains("render_paragraph(segments.clone(), theme)"),
            "docs markdown paragraphs need stable ids so selection survives notify/repaint"
        );
    }

    #[test]
    fn docs_page_uses_gpui_virtual_list_for_visible_area_rendering() {
        let source = include_str!("markdown.rs");
        let docs_page_view = &source[source
            .find("struct DocsPageView")
            .expect("DocsPageView should exist")
            ..source
                .find("fn live_demo_index_before")
                .expect("live_demo_index_before should follow DocsPageView")];

        assert!(docs_page_view.contains("VirtualizedList::new(document.blocks.len()"));
        assert!(docs_page_view.contains("list.set_item_spacing(px(20.0))"));
        assert!(!docs_page_view.contains(".set_render_item"));
        assert!(docs_page_view.contains("live_demo_index_before"));
    }

    #[test]
    fn docs_shell_uses_native_container_and_menu() {
        let source = include_str!("markdown.rs");

        assert!(source.contains("Container::new()"));
        assert!(source.contains("Menu::new()"));
        assert!(source.contains(".aside_scroll()"));
        assert!(source.contains("VirtualizedList::new"));
        assert!(source.contains("virtual_list: Entity<VirtualizedList>"));
        assert!(source.contains("measure_all_items_for_scrollbar"));
        assert!(source.contains(".flex_1().min_h_0().child(page_view)"));
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
                    "checkbox/custom.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/radio.md"),
                "RadioBasic",
                &[
                    "radio/basic.rs",
                    "radio/group.rs",
                    "radio/buttons.rs",
                    "radio/custom.rs",
                ][..],
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
            (
                include_str!("../content/pages/progress.md"),
                "ProgressBasic",
                &[
                    "progress/basic.rs",
                    "progress/inside.rs",
                    "progress/status.rs",
                    "progress/color.rs",
                    "progress/circle.rs",
                    "progress/custom.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/loading.md"),
                "LoadingBasic",
                &["loading/basic.rs", "loading/fullscreen.rs"][..],
            ),
            (
                include_str!("../content/pages/link.md"),
                "LinkVariants",
                &[
                    "link/variants.rs",
                    "link/underline.rs",
                    "link/states.rs",
                    "link/icons.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/skeleton.md"),
                "SkeletonBasic",
                &[
                    "skeleton/basic.rs",
                    "skeleton/variants.rs",
                    "skeleton/template.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/result.md"),
                "ResultSuccess",
                &["result/success.rs", "result/statuses.rs"][..],
            ),
            (
                include_str!("../content/pages/statistic.md"),
                "StatisticBasic",
                &[
                    "statistic/basic.rs",
                    "statistic/affix.rs",
                    "statistic/icons.rs",
                    "statistic/layout.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/card.md"),
                "CardBasic",
                &["card/basic.rs", "card/footer.rs"][..],
            ),
            (
                include_str!("../content/pages/empty.md"),
                "EmptyBasic",
                &[
                    "empty/basic.rs",
                    "empty/description.rs",
                    "empty/image.rs",
                    "empty/extra.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/steps.md"),
                "StepsBasic",
                &[
                    "steps/basic.rs",
                    "steps/description.rs",
                    "steps/status.rs",
                    "steps/vertical.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/timeline.md"),
                "TimelineBasic",
                &[
                    "timeline/basic.rs",
                    "timeline/custom.rs",
                    "timeline/placement.rs",
                    "timeline/reverse.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/breadcrumb.md"),
                "BreadcrumbBasic",
                &[
                    "breadcrumb/basic.rs",
                    "breadcrumb/icon.rs",
                    "breadcrumb/separator.rs",
                    "breadcrumb/separator_icon.rs",
                    "breadcrumb/clickable.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/page_header.md"),
                "PageHeaderBasic",
                &[
                    "page_header/basic.rs",
                    "page_header/extra.rs",
                    "page_header/full.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/segmented.md"),
                "SegmentedBasic",
                &[
                    "segmented/basic.rs",
                    "segmented/disabled.rs",
                    "segmented/block.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/tooltip.md"),
                "TooltipBasic",
                &["tooltip/basic.rs", "tooltip/more.rs"][..],
            ),
            (
                include_str!("../content/pages/pagination.md"),
                "PaginationBasic",
                &[
                    "pagination/basic.rs",
                    "pagination/background.rs",
                    "pagination/advanced.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/tabs.md"),
                "TabsBasic",
                &[
                    "tabs/basic.rs",
                    "tabs/stretch.rs",
                    "tabs/card.rs",
                    "tabs/border_card.rs",
                    "tabs/position.rs",
                    "tabs/editable.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/layout.md"),
                "LayoutDivider",
                &["layout/divider.rs", "layout/space.rs", "layout/grid.rs"][..],
            ),
            (
                include_str!("../content/pages/container.md"),
                "ContainerSpace",
                &[
                    "container/space.rs",
                    "container/divider.rs",
                    "container/layout.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/splitter.md"),
                "SplitterBasic",
                &["splitter/basic.rs"][..],
            ),
            (
                include_str!("../content/pages/scrollbar.md"),
                "ScrollbarBasic",
                &["scrollbar/basic.rs"][..],
            ),
            (
                include_str!("../content/pages/descriptions.md"),
                "DescriptionsBasic",
                &[
                    "descriptions/basic.rs",
                    "descriptions/border.rs",
                    "descriptions/vertical.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/table.md"),
                "TableSortable",
                &[
                    "table/sortable.rs",
                    "table/basic.rs",
                    "table/stripe_border.rs",
                    "table/fixed_header.rs",
                    "table/loading.rs",
                    "table/empty.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/color_picker.md"),
                "ColorPickerBasic",
                &[
                    "color_picker/basic.rs",
                    "color_picker/presets.rs",
                    "color_picker/compact.rs",
                    "color_picker/disabled.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/time_picker.md"),
                "TimePickerBasic",
                &[
                    "time_picker/basic.rs",
                    "time_picker/formatted.rs",
                    "time_picker/stepped.rs",
                    "time_picker/no_seconds.rs",
                    "time_picker/disabled.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/icon.md"),
                "IconLucide",
                &["icon/lucide.rs", "icon/colors.rs", "icon/sizes.rs"][..],
            ),
            (
                include_str!("../content/pages/image.md"),
                "ImageBasic",
                &[
                    "image/basic.rs",
                    "image/fit.rs",
                    "image/states.rs",
                    "image/preview.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/cascader.md"),
                "CascaderBasic",
                &[
                    "cascader/basic.rs",
                    "cascader/selected.rs",
                    "cascader/disabled.rs",
                    "cascader/filterable.rs",
                    "cascader/lazy.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/collapse.md"),
                "CollapseBasic",
                &["collapse/basic.rs", "collapse/accordion.rs"][..],
            ),
            (
                include_str!("../content/pages/date_picker.md"),
                "DatePickerBasic",
                &[
                    "date_picker/basic.rs",
                    "date_picker/formatted.rs",
                    "date_picker/range.rs",
                    "date_picker/month.rs",
                    "date_picker/month_range.rs",
                    "date_picker/year.rs",
                    "date_picker/year_range.rs",
                    "date_picker/disabled.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/date_time_picker.md"),
                "DateTimePickerBasic",
                &[
                    "date_time_picker/basic.rs",
                    "date_time_picker/formatted.rs",
                    "date_time_picker/stepped.rs",
                    "date_time_picker/no_seconds.rs",
                    "date_time_picker/range.rs",
                    "date_time_picker/disabled.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/dialog.md"),
                "DialogBasic",
                &[
                    "dialog/basic.rs",
                    "dialog/manual_close.rs",
                    "dialog/custom_content.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/drawer.md"),
                "DrawerPlacements",
                &[
                    "drawer/placements.rs",
                    "drawer/sizes.rs",
                    "drawer/manual_close.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/affix.md"),
                "AffixTop",
                &["affix/top.rs", "affix/bottom.rs", "affix/container.rs"][..],
            ),
            (
                include_str!("../content/pages/anchor.md"),
                "AnchorBasic",
                &["anchor/basic.rs", "anchor/nested.rs", "anchor/targets.rs"][..],
            ),
            (
                include_str!("../content/pages/backtop.md"),
                "BacktopBasic",
                &[
                    "backtop/basic.rs",
                    "backtop/custom.rs",
                    "backtop/container.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/form.md"),
                "FormBasic",
                &["form/basic.rs", "form/validation.rs", "form/inline.rs"][..],
            ),
            (
                include_str!("../content/pages/preview.md"),
                "PreviewImageTrigger",
                &[
                    "preview/image_trigger.rs",
                    "preview/custom_trigger.rs",
                    "preview/escape.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/popover.md"),
                "PopoverBasic",
                &[
                    "popover/basic.rs",
                    "popover/placements.rs",
                    "popover/close_strategy.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/popconfirm.md"),
                "PopconfirmBasic",
                &[
                    "popconfirm/basic.rs",
                    "popconfirm/placements.rs",
                    "popconfirm/custom_text.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/dropdown.md"),
                "DropdownBasic",
                &[
                    "dropdown/basic.rs",
                    "dropdown/placements.rs",
                    "dropdown/close_strategy.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/message_box.md"),
                "MessageBoxBasic",
                &["message_box/basic.rs", "message_box/manual_close.rs"][..],
            ),
            (
                include_str!("../content/pages/notification.md"),
                "NotificationTypes",
                &["notification/types.rs"][..],
            ),
            (
                include_str!("../content/pages/upload.md"),
                "UploadBasic",
                &[
                    "upload/basic.rs",
                    "upload/drag.rs",
                    "upload/picture_card.rs",
                    "upload/limits.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/transfer.md"),
                "TransferBasic",
                &[
                    "transfer/basic.rs",
                    "transfer/filterable.rs",
                    "transfer/disabled.rs",
                ][..],
            ),
            (
                include_str!("../content/pages/tree.md"),
                "TreeBasic",
                &["tree/basic.rs", "tree/checkable.rs"][..],
            ),
            (
                include_str!("../content/pages/menu.md"),
                "MenuHorizontal",
                &[
                    "menu/horizontal.rs",
                    "menu/vertical.rs",
                    "menu/collapsed.rs",
                ][..],
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
