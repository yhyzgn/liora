use gpui::{
    AnyElement, AnyView, App, Component, Context, Entity, FontWeight, Hsla, IntoElement, Render,
    RenderImage, RenderOnce, ScrollHandle, SharedString, WeakEntity, Window, div, img, prelude::*,
    px, rgb,
};
use liora_components::{
    Affix, AffixPosition, Alert, AlertType, Anchor, AnchorLink, AnchorTarget, AppWindowFrame,
    Autocomplete, AutocompleteItem, Avatar, Backtop, Badge, BadgeType, Button, ButtonColors,
    Calendar, CalendarDate, CalendarEvent, Card, Carousel, CarouselIndicatorPosition, CarouselItem,
    Checkbox, CheckboxGroup, CheckboxOptionStyle, CodeBlock as LioraCodeBlock, CodeDiagnostic,
    CodeEditor, CodeHighlighter, CodeLanguage, CodeTheme, Container, Dropdown, DropdownButton,
    DropdownButtonItem, Flex, Form, FormItem, HorizontalList, Image, Input, InputNumber,
    InputNumberControlsPosition, Link, Loading, Menu, MenuMode, NotificationType, Paragraph,
    Popconfirm, Popover, Preview, Progress, ProgressStatus, QrCode, QrEcLevel, QrFinderStyle,
    QrGradientDirection, QrModuleStyle, Radio, RadioGroup, RadioOptionStyle, Rate,
    Result as LioraResult, ResultStatus, Segmented, SegmentedOption, Select, SelectableTextGroup,
    Shell, Sidebar, Skeleton, SkeletonItem, SkeletonVariant, Slider, Space, Statistic, Switch,
    Tag as LioraTag, Text, Textarea, Timer, TimerFormat, TimerUnit, Title, TitleBar,
    TitleBarContentAlign, Tour, TourPlacement, TourStep, Transfer, TransferItem, Tree, TreeNode,
    TreeSelect, TreeSelectNode, Upload, UploadFile, UploadStatus, VirtualizedList,
    VirtualizedTable, VirtualizedTree, Watermark, WindowControlsPosition, WindowFrameMode,
    frame_mode_switch_row, show_notification, toast_error, toast_info, toast_success,
    toast_warning,
};
use liora_core::{
    Config, PassivePortal, Placement, Portal, ThemeMode, apply_theme_mode, clear_popover,
};
use liora_gallery::category;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::Theme;
use liora_tray::{TrayCloseAction, TrayCommand, TrayMenuItemSpec, default_liora_tray_menu};
use liora_updater::{
    AssetKind, InstallAction, InstallPlan, LioraApp, Platform, UpdateRequest, Updater,
    liora_asset_selector,
};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::{
    process::Command,
    sync::{Arc, OnceLock},
};

const INTRO_DOC: &str = include_str!("../content/pages/overview.md");
const QUICK_START_DOC: &str = include_str!("../content/pages/quick_start.md");
const ARCHITECTURE_DOC: &str = include_str!("../content/pages/architecture.md");
const PACKAGING_WORKFLOW_DOC: &str = include_str!("../content/pages/packaging_workflow.md");
const RELEASE_CANDIDATE_DOC: &str = include_str!("../../../docs/release-candidate-checklist.md");
const ADOPTION_DOC: &str = include_str!("../content/pages/adoption.md");
const DASHBOARD_APP_DOC: &str = include_str!("../content/pages/dashboard_app.md");
const DASHBOARD_PATTERNS_DOC: &str = include_str!("../content/pages/dashboard_patterns.md");
const DASHBOARD_STATE_DOC: &str = include_str!("../content/pages/dashboard_state.md");
const THEME_SYSTEM_DOC: &str = include_str!("../content/pages/theme_system.md");
const TITLEBAR_DOC: &str = include_str!("../content/pages/titlebar.md");
const ABOUT_DOC: &str = include_str!("../content/pages/about.md");

const ACCORDION_DOC: &str = include_str!("../content/pages/accordion.md");
const AFFIX_DOC: &str = include_str!("../content/pages/affix.md");
const ALERT_DOC: &str = include_str!("../content/pages/alert.md");
const AREA_CHART_DOC: &str = include_str!("../content/pages/area_chart.md");
const ANCHOR_DOC: &str = include_str!("../content/pages/anchor.md");
const AUTOCOMPLETE_DOC: &str = include_str!("../content/pages/autocomplete.md");
const AVATAR_DOC: &str = include_str!("../content/pages/avatar.md");
const BACKTOP_DOC: &str = include_str!("../content/pages/backtop.md");
const BADGE_DOC: &str = include_str!("../content/pages/badge.md");
const BAR_CHART_DOC: &str = include_str!("../content/pages/bar_chart.md");
const CANDLESTICK_CHART_DOC: &str = include_str!("../content/pages/candlestick_chart.md");
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
const DOCK_LAYOUT_DOC: &str = include_str!("../content/pages/dock_layout.md");
const TOGGLE_DOC: &str = include_str!("../content/pages/toggle.md");
const GROUP_BOX_DOC: &str = include_str!("../content/pages/group_box.md");
const HOVER_CARD_DOC: &str = include_str!("../content/pages/hover_card.md");
const SCROLLABLE_MASK_DOC: &str = include_str!("../content/pages/scrollable_mask.md");
const CLIPBOARD_DOC: &str = include_str!("../content/pages/clipboard.md");
const FOCUS_TRAP_DOC: &str = include_str!("../content/pages/focus_trap.md");
const NATIVE_MENU_DOC: &str = include_str!("../content/pages/native_menu.md");
const DROPDOWN_DOC: &str = include_str!("../content/pages/dropdown.md");
const DROPDOWN_BUTTON_DOC: &str = include_str!("../content/pages/dropdown_button.md");
const EMPTY_DOC: &str = include_str!("../content/pages/empty.md");
const FORM_DOC: &str = include_str!("../content/pages/form.md");
const HEAT_BAR_DOC: &str = include_str!("../content/pages/heat_bar.md");
const LABEL_DOC: &str = include_str!("../content/pages/label.md");
const SPINNER_DOC: &str = include_str!("../content/pages/spinner.md");
const STATUS_BAR_DOC: &str = include_str!("../content/pages/status_bar.md");
const OTP_INPUT_DOC: &str = include_str!("../content/pages/otp_input.md");
const KBD_DOC: &str = include_str!("../content/pages/kbd.md");
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
const PARAGRAPH_DOC: &str = include_str!("../content/pages/paragraph.md");
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
const SEARCHABLE_LIST_DOC: &str = include_str!("../content/pages/searchable_list.md");
const SIDEBAR_DOC: &str = include_str!("../content/pages/sidebar.md");
const SHELL_DOC: &str = include_str!("../content/pages/shell.md");
const SETTINGS_DOC: &str = include_str!("../content/pages/settings.md");
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
const TEXT_DOC: &str = include_str!("../content/pages/text.md");
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
        title: "Release Candidate",
        markdown: RELEASE_CANDIDATE_DOC,
    },
    DocPage {
        title: "Adoption Guide",
        markdown: ADOPTION_DOC,
    },
    DocPage {
        title: "Gallery Dogfooding",
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
        title: "Accordion",
        markdown: ACCORDION_DOC,
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
        title: "Toggle",
        markdown: TOGGLE_DOC,
    },
    DocPage {
        title: "GroupBox",
        markdown: GROUP_BOX_DOC,
    },
    DocPage {
        title: "HoverCard",
        markdown: HOVER_CARD_DOC,
    },
    DocPage {
        title: "ScrollableMask",
        markdown: SCROLLABLE_MASK_DOC,
    },
    DocPage {
        title: "Clipboard",
        markdown: CLIPBOARD_DOC,
    },
    DocPage {
        title: "FocusTrap",
        markdown: FOCUS_TRAP_DOC,
    },
    DocPage {
        title: "NativeMenu",
        markdown: NATIVE_MENU_DOC,
    },
    DocPage {
        title: "DockLayout",
        markdown: DOCK_LAYOUT_DOC,
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
        title: "DropdownButton",
        markdown: DROPDOWN_BUTTON_DOC,
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
        title: "Kbd",
        markdown: KBD_DOC,
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
        title: "OtpInput",
        markdown: OTP_INPUT_DOC,
    },
    DocPage {
        title: "PageHeader",
        markdown: PAGE_HEADER_DOC,
    },
    DocPage {
        title: "Paragraph",
        markdown: PARAGRAPH_DOC,
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
        title: "SearchableList",
        markdown: SEARCHABLE_LIST_DOC,
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
        title: "Settings",
        markdown: SETTINGS_DOC,
    },
    DocPage {
        title: "Select",
        markdown: SELECT_DOC,
    },
    DocPage {
        title: "Shell",
        markdown: SHELL_DOC,
    },
    DocPage {
        title: "Sidebar",
        markdown: SIDEBAR_DOC,
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
        title: "Spinner",
        markdown: SPINNER_DOC,
    },
    DocPage {
        title: "StatusBar",
        markdown: STATUS_BAR_DOC,
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
        title: "Text",
        markdown: TEXT_DOC,
    },
    DocPage {
        title: "Textarea",
        markdown: TEXTAREA_DOC,
    },
    DocPage {
        title: "Theme",
        markdown: THEME_SYSTEM_DOC,
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
        title: "TitleBar",
        markdown: TITLEBAR_DOC,
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
        title: "CandlestickChart",
        markdown: CANDLESTICK_CHART_DOC,
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
    const START: &str = "::LioraDemo{";
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
        "accordion/basic.rs" => Some(include_str!("../content/snippets/accordion/basic.rs")),
        "accordion/multiple.rs" => Some(include_str!("../content/snippets/accordion/multiple.rs")),
        "accordion/states.rs" => Some(include_str!("../content/snippets/accordion/states.rs")),
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
        "quick_start/fonts.rs" => Some(include_str!("../content/snippets/quick_start/fonts.rs")),
        "quick_start/components.rs" => Some(include_str!(
            "../content/snippets/quick_start/components.rs"
        )),
        "architecture/render_pipeline.rs" => Some(include_str!(
            "../content/snippets/architecture/render_pipeline.rs"
        )),
        "typography/text.rs" => Some(include_str!("../content/snippets/typography/text.rs")),
        "typography/paragraph.rs" => {
            Some(include_str!("../content/snippets/typography/paragraph.rs"))
        }
        "typography/selectable_text_group.rs" => Some(include_str!(
            "../content/snippets/typography/selectable_text_group.rs"
        )),
        "typography/document_blocks.rs" => Some(include_str!(
            "../content/snippets/typography/document_blocks.rs"
        )),
        "typography/markdown.rs" => {
            Some(include_str!("../content/snippets/typography/markdown.rs"))
        }
        "typography/document_inline.rs" => Some(include_str!(
            "../content/snippets/typography/document_inline.rs"
        )),
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
        "theme/system_mode.rs" => Some(include_str!("../content/snippets/theme/system_mode.rs")),
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
        "virtualized_table/data_table.rs" => Some(include_str!(
            "../content/snippets/virtualized_table/data_table.rs"
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
        "searchable_list/basic.rs" => {
            Some(include_str!("../content/snippets/searchable_list/basic.rs"))
        }
        "searchable_list/filtered.rs" => Some(include_str!(
            "../content/snippets/searchable_list/filtered.rs"
        )),
        "searchable_list/empty.rs" => {
            Some(include_str!("../content/snippets/searchable_list/empty.rs"))
        }
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
        "candlestick_chart/basic.rs" => Some(include_str!(
            "../content/snippets/candlestick_chart/basic.rs"
        )),
        "candlestick_chart/custom.rs" => Some(include_str!(
            "../content/snippets/candlestick_chart/custom.rs"
        )),
        "candlestick_chart/dense.rs" => Some(include_str!(
            "../content/snippets/candlestick_chart/dense.rs"
        )),
        "bar_chart/grouped.rs" => Some(include_str!("../content/snippets/bar_chart/grouped.rs")),
        "bar_chart/gradient.rs" => Some(include_str!("../content/snippets/bar_chart/gradient.rs")),
        "bar_chart/per_bar_gradient.rs" => Some(include_str!(
            "../content/snippets/bar_chart/per_bar_gradient.rs",
        )),
        "bar_chart/stacked.rs" => Some(include_str!("../content/snippets/bar_chart/stacked.rs")),
        "bar_chart/custom.rs" => Some(include_str!("../content/snippets/bar_chart/custom.rs")),
        "label/basic.rs" => Some(include_str!("../content/snippets/label/basic.rs")),
        "kbd/basic.rs" => Some(include_str!("../content/snippets/kbd/basic.rs")),
        "kbd/composition.rs" => Some(include_str!("../content/snippets/kbd/composition.rs")),
        "kbd/sizes.rs" => Some(include_str!("../content/snippets/kbd/sizes.rs")),
        "otp_input/basic.rs" => Some(include_str!("../content/snippets/otp_input/basic.rs")),
        "otp_input/states.rs" => Some(include_str!("../content/snippets/otp_input/states.rs")),
        "otp_input/masked.rs" => Some(include_str!("../content/snippets/otp_input/masked.rs")),
        "otp_input/interactive.rs" => {
            Some(include_str!("../content/snippets/otp_input/interactive.rs"))
        }
        "spinner/basic.rs" => Some(include_str!("../content/snippets/spinner/basic.rs")),
        "spinner/composition.rs" => {
            Some(include_str!("../content/snippets/spinner/composition.rs"))
        }
        "spinner/colors.rs" => Some(include_str!("../content/snippets/spinner/colors.rs")),
        "spinner/sizes.rs" => Some(include_str!("../content/snippets/spinner/sizes.rs")),
        "status_bar/shell.rs" => Some(include_str!("../content/snippets/status_bar/shell.rs")),
        "status_bar/tones.rs" => Some(include_str!("../content/snippets/status_bar/tones.rs")),
        "status_bar/custom.rs" => Some(include_str!("../content/snippets/status_bar/custom.rs")),
        "status_bar/advanced.rs" => {
            Some(include_str!("../content/snippets/status_bar/advanced.rs"))
        }
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
        "select/searchable.rs" => Some(include_str!("../content/snippets/select/searchable.rs")),
        "select/grouped.rs" => Some(include_str!("../content/snippets/select/grouped.rs")),
        "select/multiple.rs" => Some(include_str!("../content/snippets/select/multiple.rs")),
        "select/footer.rs" => Some(include_str!("../content/snippets/select/footer.rs")),
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
        "code_editor/advanced.rs" => {
            Some(include_str!("../content/snippets/code_editor/advanced.rs"))
        }
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
        "settings/page.rs" => Some(include_str!("../content/snippets/settings/page.rs")),
        "settings/sensitive.rs" => Some(include_str!("../content/snippets/settings/sensitive.rs")),
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
        "dropdown_button/basic.rs" => {
            Some(include_str!("../content/snippets/dropdown_button/basic.rs"))
        }
        "dropdown_button/split.rs" => {
            Some(include_str!("../content/snippets/dropdown_button/split.rs"))
        }
        "dropdown_button/item_states.rs" => Some(include_str!(
            "../content/snippets/dropdown_button/item_states.rs"
        )),
        "dropdown_button/sizes.rs" => {
            Some(include_str!("../content/snippets/dropdown_button/sizes.rs"))
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
        "shell/basic.rs" => Some(include_str!("../content/snippets/shell/basic.rs")),
        "shell/full_product.rs" => Some(include_str!("../content/snippets/shell/full_product.rs")),
        "shell/content_first.rs" => {
            Some(include_str!("../content/snippets/shell/content_first.rs"))
        }
        "shell/minimal.rs" => Some(include_str!("../content/snippets/shell/minimal.rs")),
        "sidebar/basic.rs" => Some(include_str!("../content/snippets/sidebar/basic.rs")),
        "sidebar/brand.rs" => Some(include_str!("../content/snippets/sidebar/brand.rs")),
        "sidebar/scrollable.rs" => Some(include_str!("../content/snippets/sidebar/scrollable.rs")),
        "sidebar/inspector.rs" => Some(include_str!("../content/snippets/sidebar/inspector.rs")),
        "sidebar/icon_rail.rs" => Some(include_str!("../content/snippets/sidebar/icon_rail.rs")),
        "sidebar/custom_slots.rs" => {
            Some(include_str!("../content/snippets/sidebar/custom_slots.rs"))
        }
        "titlebar/basic.rs" => Some(include_str!("../content/snippets/titlebar/basic.rs")),
        "titlebar/window_controls.rs" => Some(include_str!(
            "../content/snippets/titlebar/window_controls.rs"
        )),
        "titlebar/window_controls_right.rs" => Some(include_str!(
            "../content/snippets/titlebar/window_controls_right.rs"
        )),
        "titlebar/window_controls_left.rs" => Some(include_str!(
            "../content/snippets/titlebar/window_controls_left.rs"
        )),
        "titlebar/command_center.rs" => Some(include_str!(
            "../content/snippets/titlebar/command_center.rs"
        )),
        "titlebar/borderless.rs" => {
            Some(include_str!("../content/snippets/titlebar/borderless.rs"))
        }
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
        "toggle/basic.rs" => Some(include_str!("../content/snippets/toggle/basic.rs")),
        "group_box/basic.rs" => Some(include_str!("../content/snippets/group_box/basic.rs")),
        "hover_card/basic.rs" => Some(include_str!("../content/snippets/hover_card/basic.rs")),
        "scrollable_mask/basic.rs" => {
            Some(include_str!("../content/snippets/scrollable_mask/basic.rs"))
        }
        "clipboard/helper.rs" => Some(include_str!("../content/snippets/clipboard/helper.rs")),
        "focus_trap/policy.rs" => Some(include_str!("../content/snippets/focus_trap/policy.rs")),
        "native_menu/descriptor.rs" => Some(include_str!(
            "../content/snippets/native_menu/descriptor.rs"
        )),
        "native_menu/actions.rs" => {
            Some(include_str!("../content/snippets/native_menu/actions.rs"))
        }
        "drawer/placements.rs" => Some(include_str!("../content/snippets/drawer/placements.rs")),
        "drawer/sizes.rs" => Some(include_str!("../content/snippets/drawer/sizes.rs")),
        "drawer/sheet_placements.rs" => Some(include_str!(
            "../content/snippets/drawer/sheet_placements.rs"
        )),
        "drawer/sheet_controlled.rs" => Some(include_str!(
            "../content/snippets/drawer/sheet_controlled.rs"
        )),
        "dock_layout/workbench.rs" => {
            Some(include_str!("../content/snippets/dock_layout/workbench.rs"))
        }
        "dock_layout/inspector.rs" => {
            Some(include_str!("../content/snippets/dock_layout/inspector.rs"))
        }
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
        "gallery/affix_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/affix_demo.rs"))
        }
        "gallery/alert_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/alert_demo.rs"))
        }
        "gallery/anchor_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/anchor_demo.rs"))
        }
        "gallery/area_chart_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/area_chart_demo.rs"
        )),
        "gallery/autocomplete_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/autocomplete_demo.rs"
        )),
        "gallery/avatar_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/avatar_demo.rs"))
        }
        "gallery/backtop_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/backtop_demo.rs"
        )),
        "gallery/badge_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/badge_demo.rs"))
        }
        "gallery/bar_chart_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/bar_chart_demo.rs"
        )),
        "gallery/breadcrumb_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/breadcrumb_demo.rs"
        )),
        "gallery/button_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/button_demo.rs"))
        }
        "gallery/card_demo.rs" => Some(include_str!("../../liora-gallery/src/demos/card_demo.rs")),
        "gallery/cascader_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/cascader_demo.rs"
        )),
        "gallery/code_block_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/code_block_demo.rs"
        )),
        "gallery/collapse_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/collapse_demo.rs"
        )),
        "gallery/color_picker_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/color_picker_demo.rs"
        )),
        "gallery/container_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/container_demo.rs"
        )),
        "gallery/date_picker_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/date_picker_demo.rs"
        )),
        "gallery/date_time_picker_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/date_time_picker_demo.rs"
        )),
        "gallery/descriptions_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/descriptions_demo.rs"
        )),
        "gallery/dialog_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/dialog_demo.rs"))
        }
        "gallery/drawer_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/drawer_demo.rs"))
        }
        "gallery/dropdown_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/dropdown_demo.rs"
        )),
        "gallery/dropdown_button_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/dropdown_button_demo.rs"
        )),
        "gallery/empty_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/empty_demo.rs"))
        }
        "gallery/form_controls_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/form_controls_demo.rs"
        )),
        "gallery/form_demo.rs" => Some(include_str!("../../liora-gallery/src/demos/form_demo.rs")),
        "gallery/icon_demo.rs" => Some(include_str!("../../liora-gallery/src/demos/icon_demo.rs")),
        "gallery/image_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/image_demo.rs"))
        }
        "gallery/layout_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/layout_demo.rs"))
        }
        "gallery/link_demo.rs" => Some(include_str!("../../liora-gallery/src/demos/link_demo.rs")),
        "gallery/line_chart_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/line_chart_demo.rs"
        )),
        "gallery/pie_chart_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/pie_chart_demo.rs"
        )),
        "gallery/ring_chart_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/ring_chart_demo.rs"
        )),
        "gallery/loading_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/loading_demo.rs"
        )),
        "gallery/menu_demo.rs" => Some(include_str!("../../liora-gallery/src/demos/menu_demo.rs")),
        "gallery/message_box_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/message_box_demo.rs"
        )),
        "gallery/message_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/message_demo.rs"
        )),
        "gallery/notification_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/notification_demo.rs"
        )),
        "gallery/page_header_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/page_header_demo.rs"
        )),
        "gallery/paragraph_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/paragraph_demo.rs"
        )),
        "gallery/pagination_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/pagination_demo.rs"
        )),
        "gallery/popconfirm_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/popconfirm_demo.rs"
        )),
        "gallery/popover_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/popover_demo.rs"
        )),
        "gallery/preview_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/preview_demo.rs"
        )),
        "gallery/progress_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/progress_demo.rs"
        )),
        "gallery/result_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/result_demo.rs"))
        }
        "gallery/scrollbar_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/scrollbar_demo.rs"
        )),
        "gallery/segmented_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/segmented_demo.rs"
        )),
        "gallery/skeleton_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/skeleton_demo.rs"
        )),
        "gallery/splitter_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/splitter_demo.rs"
        )),
        "gallery/statistic_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/statistic_demo.rs"
        )),
        "gallery/steps_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/steps_demo.rs"))
        }
        "gallery/table_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/table_demo.rs"))
        }
        "gallery/tabs_demo.rs" => Some(include_str!("../../liora-gallery/src/demos/tabs_demo.rs")),
        "gallery/tag_demo.rs" => Some(include_str!("../../liora-gallery/src/demos/tag_demo.rs")),
        "gallery/text_demo.rs" => Some(include_str!("../../liora-gallery/src/demos/text_demo.rs")),
        "gallery/time_picker_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/time_picker_demo.rs"
        )),
        "gallery/timeline_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/timeline_demo.rs"
        )),
        "gallery/tooltip_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/tooltip_demo.rs"
        )),
        "gallery/transfer_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/transfer_demo.rs"
        )),
        "gallery/tree_demo.rs" => Some(include_str!("../../liora-gallery/src/demos/tree_demo.rs")),
        "gallery/typography_demo.rs" => Some(include_str!(
            "../../liora-gallery/src/demos/typography_demo.rs"
        )),
        "gallery/upload_demo.rs" => {
            Some(include_str!("../../liora-gallery/src/demos/upload_demo.rs"))
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
    fn render(self, theme: &liora_theme::Theme, cx: &mut App) -> AnyElement {
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

        Flex::new()
            .column()
            .w_full()
            .gap_md()
            .child(Text::new(self.component.clone()).size(px(theme.font_size.sm)))
            .child(self.demo.clone())
    }
}

struct LiveDemoContent {
    component: SharedString,
    gallery_demo: Option<AnyView>,
    autocompletes: Vec<Entity<Autocomplete>>,
    select_advanceds: Vec<Entity<Select>>,
    input_numbers: Vec<Entity<InputNumber>>,
    textareas: Vec<Entity<Textarea>>,
    checkboxes: Vec<Entity<Checkbox>>,
    checkbox_groups: Vec<Entity<CheckboxGroup>>,
    code_editors: Vec<Entity<CodeEditor>>,
    horizontal_lists: Vec<Entity<HorizontalList>>,
    virtualized_lists: Vec<Entity<VirtualizedList>>,
    virtualized_trees: Vec<Entity<VirtualizedTree>>,
    inputs: Vec<Entity<Input>>,
    input_tags: Vec<Entity<liora_components::InputTag>>,
    mentions: Vec<Entity<liora_components::Mention>>,
    tree_selects: Vec<Entity<TreeSelect>>,
    radios: Vec<Entity<Radio>>,
    radio_groups: Vec<Entity<RadioGroup>>,
    rates: Vec<Entity<Rate>>,
    selects: Vec<Entity<Select>>,
    sliders: Vec<Entity<Slider>>,
    switches: Vec<Entity<Switch>>,
    settings_auto_save: Option<Entity<Switch>>,
    settings_telemetry: Option<Entity<Switch>>,
    settings_theme: Option<Entity<Select>>,
    settings_font_size: Option<Entity<Input>>,
    segmenteds: Vec<Entity<liora_components::Segmented>>,
    paginations: Vec<Entity<liora_components::Pagination>>,
    tabs: Vec<Entity<liora_components::Tabs>>,
    scrollbars: Vec<Entity<liora_components::Scrollbar>>,
    accordions: Vec<Entity<liora_components::Accordion>>,
    table_sort_key: Option<SharedString>,
    table_sort_order: Option<liora_components::TableSortOrder>,
    color_pickers: Vec<Entity<liora_components::ColorPicker>>,
    time_pickers: Vec<Entity<liora_components::TimePicker>>,
    cascaders: Vec<Entity<liora_components::Cascader>>,
    collapses: Vec<Entity<liora_components::Collapse>>,
    date_pickers: Vec<Entity<liora_components::DatePicker>>,
    date_time_pickers: Vec<Entity<liora_components::DateTimePicker>>,
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
    otp_inputs: Vec<Entity<liora_components::OtpInput>>,
    scroll_handle: ScrollHandle,
}

impl LiveDemoContent {
    fn new(component: SharedString, cx: &mut Context<Self>) -> Self {
        let gallery_demo = liora_gallery::demos::render_doc_demo(component.as_ref(), cx);
        let mut autocompletes = Vec::new();
        let mut select_advanceds = Vec::new();
        let mut input_numbers = Vec::new();
        let mut textareas = Vec::new();
        let mut checkboxes = Vec::new();
        let mut checkbox_groups = Vec::new();
        let mut code_editors = Vec::new();
        let mut horizontal_lists = Vec::new();
        let mut virtualized_lists = Vec::new();
        let mut virtualized_trees = Vec::new();
        let mut inputs = Vec::new();
        let mut input_tags = Vec::new();
        let mut mentions = Vec::new();
        let mut tree_selects = Vec::new();
        let mut radios = Vec::new();
        let mut radio_groups = Vec::new();
        let mut rates = Vec::new();
        let mut selects = Vec::new();
        let mut sliders = Vec::new();
        let mut switches = Vec::new();
        let mut settings_auto_save = None;
        let mut settings_telemetry = None;
        let mut settings_theme = None;
        let mut settings_font_size = None;
        let mut segmenteds = Vec::new();
        let mut paginations = Vec::new();
        let mut tabs = Vec::new();
        let mut scrollbars = Vec::new();
        let mut accordions = Vec::new();
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
        let mut otp_inputs = Vec::new();
        let scroll_handle = ScrollHandle::new();

        match component.as_ref() {
            "ShellBasic" | "ShellFullProduct" => {
                menus.push(cx.new(|_| docs_product_menu("docs-shell-main-menu-live")));
                menus.push(cx.new(|_| docs_inspector_menu("docs-shell-inspector-menu-live")));
            }
            "ShellContentFirst" => {
                menus.push(cx.new(|_| docs_compact_menu("docs-shell-compact-menu-live")));
            }
            "SidebarBasic" | "SidebarBrand" => {
                menus.push(cx.new(|_| docs_workspace_menu("docs-sidebar-brand-menu-live")));
            }
            "SidebarScrollable" => {
                menus.push(cx.new(|_| docs_long_workspace_menu("docs-sidebar-long-menu-live")));
            }
            "SidebarInspector" => {
                menus.push(cx.new(|_| docs_inspector_menu("docs-sidebar-inspector-menu-live")));
            }
            "SidebarIconRail" => {
                menus.push(cx.new(|_| docs_icon_rail_menu("docs-sidebar-icon-menu-live")));
            }
            "Accordion" | "AccordionBasic" => {
                accordions.push(cx.new(|_| docs_accordion_basic_demo()));
            }
            "AccordionMultiple" => {
                accordions.push(cx.new(|_| docs_accordion_multiple_demo()));
            }
            "AccordionStates" => {
                accordions.push(cx.new(|_| docs_accordion_states_demo()));
            }
            "QrCodeDecode" => {
                uploads.push(cx.new(|_| docs_qr_code_decode_upload()));
            }
            "InputTagBasic" => {
                input_tags.push(cx.new(|cx| docs_input_tag_basic(cx)));
            }
            "InputTagLimited" => {
                input_tags.push(cx.new(|cx| docs_input_tag_limited(cx)));
            }
            "InputTagDuplicates" => {
                input_tags.push(cx.new(|cx| docs_input_tag_duplicates(cx)));
            }
            "MentionPeople" => {
                mentions.push(cx.new(|cx| docs_mention_people(cx)));
            }
            "MentionIssues" => {
                mentions.push(cx.new(|cx| docs_mention_issues(cx)));
            }
            "MentionDisabled" => {
                mentions.push(cx.new(|cx| docs_mention_disabled(cx)));
            }
            "TreeSelectSingle" => {
                tree_selects.push(cx.new(|cx| docs_tree_select_single(cx)));
            }
            "TreeSelectMultiple" => {
                tree_selects.push(cx.new(|cx| docs_tree_select_multiple(cx)));
            }
            "TreeSelectFilterable" => {
                tree_selects.push(cx.new(|cx| docs_tree_select_filterable(cx)));
            }
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

            "SelectSearchable" => {
                select_advanceds.push(cx.new(|cx| {
                    Select::searchable(docs_combobox_framework_items(), cx)
                        .placeholder("Choose framework")
                }));
            }
            "SelectGrouped" => {
                select_advanceds.push(cx.new(|cx| {
                    Select::searchable(docs_combobox_component_items(), cx)
                        .placeholder("Search components")
                        .width(px(340.0))
                }));
            }
            "SelectMultiple" => {
                select_advanceds.push(cx.new(|cx| {
                    Select::searchable(docs_combobox_component_items(), cx)
                        .multiple()
                        .selected_values(vec!["button", "select-search"])
                        .placeholder("Pick multiple components")
                        .width(px(340.0))
                }));
            }
            "SelectFooter" => {
                select_advanceds.push(cx.new(|cx| {
                    Select::searchable(docs_combobox_component_items(), cx)
                        .placeholder("Create or select")
                        .width(px(340.0))
                        .footer(|_, _| {
                            Button::new("Create component")
                                .small()
                                .icon_start(IconName::Plus)
                                .into_any_element()
                        })
                }));
            }
            "SettingsPageBasic" => {
                settings_auto_save = Some(cx.new(|cx| Switch::new(true, cx)));
                settings_theme =
                    Some(cx.new(|cx| Select::new(vec!["System", "Light", "Dark"], Some(0), cx)));
                settings_font_size = Some(cx.new(|cx| Input::new("14", cx).width(px(88.0))));
            }
            "SettingsSensitive" => {
                settings_telemetry = Some(cx.new(|cx| Switch::new(false, cx)));
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
            "CodeEditorAdvanced" => {
                code_editors.push(cx.new(|cx| {
                    CodeEditor::new(DOCS_CODE_EDITOR_RUST_SAMPLE, cx)
                        .language(CodeLanguage::Rust)
                        .theme(CodeTheme::OneDark)
                        .search_query("Space")
                        .completions([
                            liora_components::CodeCompletionItem::new("Space::new")
                                .kind("struct")
                                .detail("layout container"),
                            liora_components::CodeCompletionItem::new("Button::new")
                                .kind("function")
                                .detail("action control"),
                            liora_components::CodeCompletionItem::new("toast_info!")
                                .kind("macro")
                                .detail("show message"),
                        ])
                        .hover(liora_components::CodeHover::new(
                            "Space::new",
                            "Creates a flexible native layout container.",
                        ))
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

            "OtpInputInteractive" => {
                otp_inputs.push(cx.new(|cx| liora_components::OtpInput::new("", cx)));
            }
            "OtpInputMasked" => {
                otp_inputs.push(cx.new(|cx| {
                    liora_components::OtpInput::new("42", cx)
                        .length(4, cx)
                        .masked(true)
                }));
            }
            "OtpInputStates" => {
                otp_inputs
                    .push(cx.new(|cx| liora_components::OtpInput::new("934201", cx).success()));
                otp_inputs.push(cx.new(|cx| {
                    liora_components::OtpInput::new("128", cx)
                        .length(4, cx)
                        .error()
                }));
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
                    liora_components::Segmented::new(vec![
                        liora_components::SegmentedOption::new("Daily", "daily"),
                        liora_components::SegmentedOption::new("Weekly", "weekly"),
                        liora_components::SegmentedOption::new("Monthly", "monthly"),
                        liora_components::SegmentedOption::new("Quarterly", "quarterly"),
                        liora_components::SegmentedOption::new("Yearly", "yearly"),
                    ])
                    .id("docs-segmented-basic")
                    .on_change(|value, _, _| toast_info!("Selected: {}", value))
                }));
            }
            "SegmentedDisabled" => {
                segmenteds.push(cx.new(|_| {
                    liora_components::Segmented::new(vec![
                        liora_components::SegmentedOption::new("Map", "Map"),
                        liora_components::SegmentedOption::new("Transit", "Transit"),
                        liora_components::SegmentedOption::new("Satellite", "Satellite")
                            .disabled(true),
                    ])
                    .id("docs-segmented-disabled")
                    .value("Map")
                    .on_change(|value, _, _| toast_info!("Selected: {}", value))
                }));
            }
            "SegmentedBlock" => {
                segmenteds.push(cx.new(|_| {
                    liora_components::Segmented::new(vec![
                        liora_components::SegmentedOption::new("123", "123"),
                        liora_components::SegmentedOption::new("456", "456"),
                        liora_components::SegmentedOption::new("long-text-option", "long"),
                    ])
                    .id("docs-segmented-block")
                    .block(true)
                    .on_change(|value, _, _| toast_info!("Selected: {}", value))
                }));
            }
            "PaginationBasic" => {
                paginations.push(cx.new(|_| {
                    liora_components::Pagination::new(50)
                        .id("docs-pagination-basic")
                        .on_change(|page, _, _| toast_info!("Page changed to: {}", page))
                }));
            }
            "PaginationBackground" => {
                paginations.push(cx.new(|_| {
                    liora_components::Pagination::new(100)
                        .id("docs-pagination-background")
                        .background(true)
                        .on_change(|page, _, _| toast_info!("Page changed to: {}", page))
                }));
            }
            "PaginationAdvanced" => {
                paginations.push(cx.new(|_| {
                    liora_components::Pagination::new(400)
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
                    cx.new(|_| basic_tabs("docs-tabs-card").type_(liora_components::TabType::Card)),
                );
            }
            "TabsBorderCard" => {
                tabs.push(cx.new(|_| {
                    basic_tabs("docs-tabs-border-card").type_(liora_components::TabType::BorderCard)
                }));
            }
            "TabsPosition" => {
                tabs.push(cx.new(|_| {
                    short_tabs("docs-tabs-left").position(liora_components::TabPosition::Left)
                }));
                tabs.push(cx.new(|_| {
                    short_tabs("docs-tabs-right").position(liora_components::TabPosition::Right)
                }));
            }
            "TabsEditable" => {
                tabs.push(cx.new(|_| {
                    liora_components::Tabs::new("1")
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
                    liora_components::Scrollbar::new(cx, |_, _| {
                        let items = (1..=20).map(|i| Text::new(format!("Scrollable line {}", i)));
                        Space::new().vertical().gap_lg().children(items)
                    })
                    .height(150.0)
                }));
            }
            "ColorPickerBasic" => {
                color_pickers.push(cx.new(|_| {
                    liora_components::ColorPicker::new("#409eff")
                        .id("docs-color-picker-basic")
                        .width_md()
                }));
            }
            "ColorPickerPresets" => {
                color_pickers.push(cx.new(|_| {
                    liora_components::ColorPicker::new("#13c2c2")
                        .id("docs-color-picker-presets")
                        .width_md()
                        .presets([
                            "#13C2C2", "#52C41A", "#FAAD14", "#F5222D", "#722ED1", "#EB2F96",
                        ])
                }));
            }
            "ColorPickerCompact" => {
                color_pickers.push(cx.new(|_| {
                    liora_components::ColorPicker::new("#F56C6C")
                        .id("docs-color-picker-compact")
                        .show_label(false)
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                }));
            }
            "ColorPickerDisabled" => {
                color_pickers.push(cx.new(|_| {
                    liora_components::ColorPicker::new("#909399")
                        .id("docs-color-picker-disabled")
                        .disabled(true)
                        .width_md()
                }));
            }
            "TimePickerBasic" => {
                time_pickers.push(cx.new(|_| {
                    liora_components::TimePicker::new()
                        .id("docs-time-picker-basic")
                        .width_md()
                }));
            }
            "TimePickerFormatted" => {
                time_pickers.push(cx.new(|_| {
                    liora_components::TimePicker::new()
                        .id("docs-time-picker-formatted")
                        .value(liora_components::TimeValue::new(9, 30, 15).expect("valid time"))
                        .format("HH时mm分ss秒")
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .width_md()
                }));
            }
            "TimePickerStepped" => {
                time_pickers.push(cx.new(|_| {
                    liora_components::TimePicker::new()
                        .id("docs-time-picker-stepped")
                        .value(liora_components::TimeValue::new(14, 30, 0).expect("valid time"))
                        .minute_step(15)
                        .second_step(30)
                        .width_md()
                }));
            }
            "TimePickerNoSeconds" => {
                time_pickers.push(cx.new(|_| {
                    liora_components::TimePicker::new()
                        .id("docs-time-picker-no-seconds")
                        .without_seconds()
                        .value(liora_components::TimeValue::new(18, 45, 0).expect("valid time"))
                        .width_md()
                }));
            }
            "TimePickerDisabled" => {
                time_pickers.push(cx.new(|_| {
                    liora_components::TimePicker::new()
                        .id("docs-time-picker-disabled")
                        .disabled(true)
                        .width_md()
                }));
            }
            "CascaderBasic" => {
                cascaders.push(cx.new(|cx| {
                    liora_components::Cascader::new(docs_region_options(), cx)
                        .placeholder("请选择地区")
                        .clearable(true)
                        .width_md()
                }));
            }
            "CascaderSelected" => {
                cascaders.push(cx.new(|cx| {
                    liora_components::Cascader::new(docs_product_options(), cx)
                        .selected_path(["cloud", "compute", "ecs"])
                        .placeholder("请选择产品")
                        .width_md()
                }));
            }
            "CascaderDisabled" => {
                cascaders.push(cx.new(|cx| {
                    liora_components::Cascader::new(docs_region_options(), cx)
                        .disabled(true)
                        .selected_path(["zhejiang", "hangzhou", "xihu"])
                        .width_md()
                }));
            }
            "CascaderFilterable" => {
                cascaders.push(cx.new(|cx| {
                    liora_components::Cascader::new(docs_region_options(), cx)
                        .filterable(true)
                        .search_query("hang")
                        .placeholder("搜索 hang")
                        .width_md()
                }));
            }
            "CascaderLazy" => {
                cascaders.push(cx.new(|cx| {
                    liora_components::Cascader::new(docs_lazy_options(), cx)
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
                    liora_components::DatePicker::new()
                        .id("docs-date-picker-basic")
                        .width_md()
                }));
            }
            "DatePickerFormatted" => {
                date_pickers.push(cx.new(|_| {
                    liora_components::DatePicker::new()
                        .id("docs-date-picker-formatted")
                        .value(liora_components::DateValue::new(2026, 5, 8).expect("valid date"))
                        .format("YYYY年M月D日")
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .width_md()
                }));
            }
            "DatePickerRange" => {
                date_pickers.push(cx.new(|_| {
                    liora_components::DatePicker::new()
                        .id("docs-date-picker-range")
                        .date_range()
                        .range(
                            liora_components::DateValue::new(2026, 5, 8).expect("valid date"),
                            liora_components::DateValue::new(2026, 5, 18).expect("valid date"),
                        )
                        .width_lg()
                }));
            }
            "DatePickerMonth" => {
                date_pickers.push(cx.new(|_| {
                    liora_components::DatePicker::new()
                        .id("docs-date-picker-month")
                        .month()
                        .value(liora_components::DateValue::new(2026, 5, 1).expect("valid date"))
                        .width_md()
                }));
            }
            "DatePickerMonthRange" => {
                date_pickers.push(cx.new(|_| {
                    liora_components::DatePicker::new()
                        .id("docs-date-picker-month-range")
                        .month_range()
                        .range(
                            liora_components::DateValue::new(2026, 3, 1).expect("valid date"),
                            liora_components::DateValue::new(2026, 9, 1).expect("valid date"),
                        )
                        .width_lg()
                }));
            }
            "DatePickerYear" => {
                date_pickers.push(cx.new(|_| {
                    liora_components::DatePicker::new()
                        .id("docs-date-picker-year")
                        .year()
                        .value(liora_components::DateValue::new(2026, 1, 1).expect("valid date"))
                        .width_md()
                }));
            }
            "DatePickerYearRange" => {
                date_pickers.push(cx.new(|_| {
                    liora_components::DatePicker::new()
                        .id("docs-date-picker-year-range")
                        .year_range()
                        .range(
                            liora_components::DateValue::new(2024, 1, 1).expect("valid date"),
                            liora_components::DateValue::new(2028, 1, 1).expect("valid date"),
                        )
                        .format("YYYY年")
                        .width_lg()
                }));
            }
            "DatePickerDisabled" => {
                date_pickers.push(cx.new(|_| {
                    liora_components::DatePicker::new()
                        .id("docs-date-picker-disabled")
                        .disabled(true)
                        .width_md()
                }));
            }
            "DateTimePickerBasic" => {
                date_time_pickers.push(cx.new(|_| {
                    liora_components::DateTimePicker::new()
                        .id("docs-date-time-picker-basic")
                        .width_md()
                }));
            }
            "DateTimePickerFormatted" => {
                date_time_pickers.push(cx.new(|_| {
                    liora_components::DateTimePicker::new()
                        .id("docs-date-time-picker-formatted")
                        .value(
                            liora_components::DateTimeValue::new(2026, 5, 8, 9, 30, 15)
                                .expect("valid datetime"),
                        )
                        .format("YYYY年M月D日 HH:mm:ss")
                        .width_md()
                }));
            }
            "DateTimePickerStepped" => {
                date_time_pickers.push(cx.new(|_| {
                    liora_components::DateTimePicker::new()
                        .id("docs-date-time-picker-stepped")
                        .value(
                            liora_components::DateTimeValue::new(2026, 5, 8, 14, 30, 0)
                                .expect("valid datetime"),
                        )
                        .minute_step(15)
                        .second_step(30)
                        .width_md()
                }));
            }
            "DateTimePickerNoSeconds" => {
                date_time_pickers.push(cx.new(|_| {
                    liora_components::DateTimePicker::new()
                        .id("docs-date-time-picker-no-seconds")
                        .without_seconds()
                        .value(
                            liora_components::DateTimeValue::new(2026, 5, 8, 18, 45, 0)
                                .expect("valid datetime"),
                        )
                        .width_md()
                }));
            }
            "DateTimePickerRange" => {
                date_time_pickers.push(cx.new(|_| {
                    liora_components::DateTimePicker::new()
                        .id("docs-date-time-picker-range")
                        .date_time_range()
                        .range(
                            liora_components::DateTimeValue::new(2026, 5, 8, 9, 0, 0)
                                .expect("valid datetime"),
                            liora_components::DateTimeValue::new(2026, 5, 18, 18, 30, 0)
                                .expect("valid datetime"),
                        )
                        .width_lg()
                }));
            }
            "DateTimePickerDisabled" => {
                date_time_pickers.push(cx.new(|_| {
                    liora_components::DateTimePicker::new()
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
                form_inputs.push(cx.new(|cx| Input::new("Liora", cx).placeholder("Name")));
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
            select_advanceds,
            input_numbers,
            textareas,
            checkboxes,
            checkbox_groups,
            code_editors,
            horizontal_lists,
            virtualized_lists,
            virtualized_trees,
            inputs,
            input_tags,
            mentions,
            tree_selects,
            radios,
            radio_groups,
            rates,
            selects,
            sliders,
            switches,
            settings_auto_save,
            settings_telemetry,
            settings_theme,
            settings_font_size,
            segmenteds,
            paginations,
            tabs,
            scrollbars,
            accordions,
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
            otp_inputs,
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
                QrCode::new("https://github.com/yhyzgn/liora")
                    .show_text(true)
                    .into_any_element(),
                QrCode::new("liora://component/qr-code")
                    .size(px(140.0))
                    .quiet_zone(2)
                    .into_any_element(),
            ]),
            "QrCodeStyle" => demo_row(vec![
                QrCode::new("Liora primary QR")
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
                LioraTag::new("Tag 1").into_any_element(),
                LioraTag::new("Tag 2").success().into_any_element(),
                LioraTag::new("Tag 3").warning().into_any_element(),
                LioraTag::new("Tag 4").danger().into_any_element(),
            ]),
            "TagClosable" => demo_row(vec![
                LioraTag::new("Tag 1").closable(true).into_any_element(),
                LioraTag::new("Tag 2")
                    .success()
                    .closable(true)
                    .into_any_element(),
                LioraTag::new("Tag 3")
                    .warning()
                    .closable(true)
                    .into_any_element(),
                LioraTag::new("Tag 4")
                    .danger()
                    .closable(true)
                    .into_any_element(),
            ]),
            "TagThemes" => demo_stack(vec![
                demo_row(vec![
                    LioraTag::new("Dark").dark().into_any_element(),
                    LioraTag::new("Success").success().dark().into_any_element(),
                    LioraTag::new("Warning").warning().dark().into_any_element(),
                    LioraTag::new("Danger").danger().dark().into_any_element(),
                ]),
                demo_row(vec![
                    LioraTag::new("Plain").plain().into_any_element(),
                    LioraTag::new("Success").success().plain().into_any_element(),
                    LioraTag::new("Warning").warning().plain().into_any_element(),
                    LioraTag::new("Danger").danger().plain().into_any_element(),
                ]),
            ]),
            "TagSizes" => demo_row(vec![
                LioraTag::new("Default").into_any_element(),
                LioraTag::new("Large").large().into_any_element(),
                LioraTag::new("Small").small().into_any_element(),
            ]),
            "TagRound" => demo_row(vec![
                LioraTag::new("Tag 1").round(true).into_any_element(),
                LioraTag::new("Tag 2")
                    .success()
                    .round(true)
                    .into_any_element(),
                LioraTag::new("Tag 3")
                    .warning()
                    .round(true)
                    .into_any_element(),
                LioraTag::new("Tag 4")
                    .danger()
                    .round(true)
                    .into_any_element(),
            ]),
            "CalendarEvents" => Card::new(docs_calendar_events()).no_shadow().into_any_element(),
            "CalendarRange" => Card::new(docs_calendar_range()).no_shadow().into_any_element(),
            "CarouselBasic" => Card::new(docs_carousel_basic()).no_shadow().into_any_element(),
            "CarouselAutoplay" => Card::new(docs_carousel_autoplay()).no_shadow().into_any_element(),
            "CarouselCustom" => Card::new(docs_carousel_custom()).no_shadow().into_any_element(),
            "CodeBlockBasic" => Card::new(docs_code_block_basic()).no_shadow().into_any_element(),
            "CodeBlockLanguage" => Card::new(docs_code_block_language()).no_shadow().into_any_element(),
            "CodeBlockTheme" => Card::new(docs_code_block_theme()).no_shadow().into_any_element(),
            "CodeBlockInline" => Card::new(docs_code_block_inline()).no_shadow().into_any_element(),
            "QrCodeDecode" => self
                .uploads
                .first()
                .cloned()
                .map(|upload| Card::new(docs_qr_code_decode_demo(upload)).no_shadow().into_any_element())
                .unwrap_or_else(|| Paragraph::with_text("QrCode decode demo is not initialized.").into_any_element()),
            "InputTagBasic" | "InputTagLimited" | "InputTagDuplicates" => self
                .input_tags
                .first()
                .cloned()
                .map(|input_tag| Card::new(input_tag).no_shadow().into_any_element())
                .unwrap_or_else(|| Paragraph::with_text("InputTag demo is not initialized.").into_any_element()),
            "MentionPeople" | "MentionIssues" | "MentionDisabled" => self
                .mentions
                .first()
                .cloned()
                .map(|mention| Card::new(mention).no_shadow().into_any_element())
                .unwrap_or_else(|| Paragraph::with_text("Mention demo is not initialized.").into_any_element()),
            "TreeSelectSingle" | "TreeSelectMultiple" | "TreeSelectFilterable" => self
                .tree_selects
                .first()
                .cloned()
                .map(|tree_select| Card::new(tree_select).no_shadow().into_any_element())
                .unwrap_or_else(|| Paragraph::with_text("TreeSelect demo is not initialized.").into_any_element()),
            "WatermarkCover" => Card::new(docs_watermark_cover()).no_shadow().into_any_element(),
            "WatermarkHeader" => Card::new(docs_watermark_header()).no_shadow().into_any_element(),
            "WatermarkCustom" => Card::new(docs_watermark_custom()).no_shadow().into_any_element(),
            "TimerResult" => Card::new(docs_timer_result()).no_shadow().into_any_element(),
            "TourMiddle" => Card::new(docs_tour_launcher(
                "从第二步启动",
                "打开一个 active_index(1) 的 Tour。",
                docs_middle_tour,
            ))
            .no_shadow()
            .into_any_element(),
            "TourNoMask" => Card::new(docs_tour_launcher(
                "启动无遮罩 Tour",
                "show_mask(false) 后页面仍保持可见。",
                docs_no_mask_tour,
            ))
            .no_shadow()
            .into_any_element(),
            "TourClosePolicy" => Card::new(docs_tour_launcher(
                "启动受控 Tour",
                "ESC 和外部点击不会关闭此引导。",
                docs_close_policy_tour,
            ))
            .no_shadow()
            .into_any_element(),
            "TrayResidency" => Card::new(docs_tray_residency()).no_shadow().into_any_element(),
            "TrayInstall" => Card::new(docs_tray_install()).no_shadow().into_any_element(),
            "TrayDynamicIcon" => Card::new(docs_tray_dynamic_icon()).no_shadow().into_any_element(),
            "TrayCheckbox" => Card::new(docs_tray_checkbox()).no_shadow().into_any_element(),
            "TrayCloseConfirm" => Card::new(docs_tray_close_confirm()).no_shadow().into_any_element(),
            "TrayNestedMenu" => Card::new(docs_tray_nested_menu()).no_shadow().into_any_element(),
            "TextBasic" => Card::new(demo_stack(vec![
                Text::new("Drag to select this default Text value.").into_any_element(),
                Text::new("Primary bold text").text_color(_cx.global::<Config>().theme.primary.base).bold().into_any_element(),
                Text::new("Inline code").code_style(&_cx.global::<Config>().theme).into_any_element(),
                Text::new("Decorative label").selectable(false).into_any_element(),
            ])).no_shadow().into_any_element(),
            "ParagraphWrapped" => Card::new(docs_paragraph_wrapped()).no_shadow().into_any_element(),
            "TypographyParagraph" => Card::new(docs_typography_paragraph(_cx)).no_shadow().into_any_element(),
            "SelectableTextGroupBasic" => Card::new(docs_selectable_text_group(_cx)).no_shadow().into_any_element(),


            "ToggleBasic" => demo_stack(vec![
                liora_components::Toggle::new("Bold", true).into_any_element(),
                liora_components::ToggleGroup::new([
                    liora_components::ToggleOption::new("preview", "Preview"),
                    liora_components::ToggleOption::new("code", "Code"),
                    liora_components::ToggleOption::new("split", "Split"),
                ]).selected("preview").into_any_element(),
            ]),
            "GroupBoxBasic" => liora_components::GroupBox::new("Editor", demo_stack(vec![Text::new("Tab size: 4").into_any_element(), Text::new("Soft tabs enabled").into_any_element()])).description("Project-level editor preferences.").into_any_element(),
            "HoverCardBasic" => liora_components::HoverCard::new(Text::new("Hover target").underline()).content(|_, _| demo_stack(vec![Text::new("Preview card").bold().into_any_element(), Text::new("Use this for profile or link previews.").into_any_element()])).into_any_element(),
            "ScrollableMaskBasic" => liora_components::ScrollableMask::new(demo_stack((1..=16).map(|i| Text::new(format!("Scrollable row {i}")).into_any_element()).collect())).height(gpui::px(160.0)).into_any_element(),
            "ClipboardHelper" => Text::new("Use write_text_to_clipboard(cx, text) inside event handlers.").into_any_element(),
            "FocusTrapPolicy" => { let policy = liora_components::FocusTrap::new().restore_focus(true).close_on_escape(false); Text::new(format!("trap={}, restore={}, esc={}", policy.enabled, policy.restore_focus, policy.close_on_escape)).into_any_element() },
            "NativeMenuDescriptor" => liora_components::NativeMenu::new("File")
                .perform_builtin_actions(false)
                .item(liora_components::NativeMenuItem::new_window())
                .item(liora_components::NativeMenuItem::open())
                .item(liora_components::NativeMenuItem::open_file())
                .item(liora_components::NativeMenuItem::open_files())
                .item(liora_components::NativeMenuItem::open_folder())
                .item(liora_components::NativeMenuItem::open_folders())
                .item(
                    liora_components::NativeMenuItem::new("recent", "Open Recent")
                        .child(liora_components::NativeMenuItem::new("recent-gallery", "liora-gallery"))
                        .child(liora_components::NativeMenuItem::new("recent-docs", "liora-docs")),
                )
                .item(liora_components::NativeMenuItem::separator())
                .item(liora_components::NativeMenuItem::save())
                .item(liora_components::NativeMenuItem::save_as())
                .item(liora_components::NativeMenuItem::open_url("Open GitHub Repository", "https://github.com/yhyzgn/liora"))
                .item(liora_components::NativeMenuItem::new("check-updates", "Check for Updates").with_action(liora_components::NativeMenuAction::Custom("check-updates".into())))
                .item(liora_components::NativeMenuItem::new("publish", "Publish Release").disabled(true))
                .on_paths_selected(|action, paths, _| {
                    toast_info!("{} paths: {:?}", action.info().name, paths);
                })
                .into_any_element(),
            "NativeMenuActions" => native_menu_action_catalog().into_any_element(),
            "DockLayoutWorkbench" => demo_row(vec![
                liora_components::DockLayout::new()
                    .height_lg()
                    .panel_gap(gpui::px(6.0))
                    .panel(liora_components::DockPanel::new("explorer", "Explorer", liora_components::DockEdge::Left, demo_stack(vec![Text::new("src").into_any_element(), Text::new("crates").into_any_element(), Text::new("apps").into_any_element()])).size(gpui::px(220.0)))
                    .panel(liora_components::DockPanel::new("terminal", "Terminal", liora_components::DockEdge::Bottom, Text::new("cargo check --workspace --all-targets")).size(gpui::px(132.0)))
                    .tab(liora_components::DockTab::new("main", "main.rs", Text::new("fn main() { init_liora(cx); }").wrap()))
                    .tab(liora_components::DockTab::new("readme", "README.md", Text::new("# Liora\nNative GPUI component library.").wrap()))
                    .into_any_element(),
            ]),
            "DockLayoutInspector" => demo_row(vec![
                liora_components::DockLayout::new()
                    .height(gpui::px(460.0))
                    .panel_gap(gpui::px(6.0))
                    .panel(liora_components::DockPanel::new("outline", "Outline", liora_components::DockEdge::Left, demo_stack(vec![Text::new("App").into_any_element(), Text::new("Shell").into_any_element(), Text::new("Content").into_any_element()])).size(gpui::px(180.0)))
                    .panel(liora_components::DockPanel::new("props", "Properties", liora_components::DockEdge::Right, demo_stack(vec![Text::new("theme").into_any_element(), Text::new("state").into_any_element(), Text::new("events").into_any_element()])).size(gpui::px(220.0)))
                    .panel(liora_components::DockPanel::new("logs", "Logs", liora_components::DockEdge::Bottom, Text::new("Ready • 0 errors")).size(gpui::px(96.0)))
                    .tab(liora_components::DockTab::new("preview", "Preview", demo_stack(vec![liora_components::Tag::new("Live").success().into_any_element(), Text::new("Center content remains flexible.").into_any_element()])))
                    .into_any_element(),
            ]),
            "TextDocumentBlocks" => demo_row(vec![
                liora_components::Text::document([
                    liora_components::TextBlock::heading(2, "Application bootstrap"),
                    liora_components::TextBlock::paragraph("Initialize Liora once, then compose native GPUI windows with reusable components."),
                    liora_components::TextBlock::quote("Text document mode is lightweight; use Docs for full documentation chrome."),
                    liora_components::TextBlock::unordered([
                        "Native selectable text",
                        "Theme-aware quote and code surfaces",
                        "Copyable code blocks",
                    ]),
                    liora_components::TextBlock::code("liora_components::init_liora(cx);", "rust"),
                ])
                .framed(true)
                .max_width(gpui::px(760.0))
                .into_any_element(),
            ]),
            "TextMarkdown" => demo_row(vec![
                liora_components::Text::markdown(
                    "# Release notes\n\nLiora renders app documents as native GPUI elements.\n\n> Keep SDK docs close to product behavior.\n\n1. Parse a small Markdown subset\n2. Render reusable component blocks\n\n```rust\nText::markdown(markdown)\n```",
                )
                .framed(true)
                .max_width(gpui::px(760.0))
                .into_any_element(),
            ]),
            "TextDocumentInline" => demo_row(vec![
                liora_components::Text::document([
                    liora_components::TextBlock::heading(3, "Inline help"),
                    liora_components::TextBlock::paragraph("Use max_width for readable line length and selectable(false) for decorative guidance."),
                    liora_components::TextBlock::Divider,
                    liora_components::TextBlock::ordered([
                        "Keep content concise",
                        "Use real components",
                        "Avoid browser runtime",
                    ]),
                ])
                .selectable(false)
                .max_width(gpui::px(680.0))
                .into_any_element(),
            ]),
            "SettingsPageBasic" => docs_settings_page_basic(self),
            "SettingsSensitive" => docs_settings_sensitive(self),
            "DrawerSheetPlacements" => demo_row(vec![
                Button::new("Right").icon_start(IconName::PanelRightOpen).on_click(|_, _, cx| {
                    liora_components::Drawer::sheet().title("Inspector").right().content_view(|_| docs_sheet_body("Right inspector")).show(cx);
                }).into_any_element(),
                Button::new("Left").icon_start(IconName::PanelLeftOpen).on_click(|_, _, cx| {
                    liora_components::Drawer::sheet().title("Navigator").left().content_view(|_| docs_sheet_body("Left navigator")).show(cx);
                }).into_any_element(),
                Button::new("Top").icon_start(IconName::PanelTopOpen).on_click(|_, _, cx| {
                    liora_components::Drawer::sheet().title("Command").top().height_sm().content_view(|_| docs_sheet_body("Top command")).show(cx);
                }).into_any_element(),
                Button::new("Bottom").icon_start(IconName::PanelBottomOpen).on_click(|_, _, cx| {
                    liora_components::Drawer::sheet().title("Actions").bottom().height_sm().content_view(|_| docs_sheet_body("Bottom actions")).show(cx);
                }).into_any_element(),
            ]),
            "DrawerSheetControlled" => Button::new("Open blocking review")
                .primary()
                .icon_start(IconName::ShieldCheck)
                .on_click(|_, _, cx| {
                    liora_components::Drawer::sheet()
                        .id("docs-blocking-review")
                        .title("Blocking review")
                        .width_lg()
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .content_view(|_| docs_sheet_body("Explicit close only"))
                        .show(cx);
                })
                .into_any_element(),
            "StatusBarShell" => docs_status_bar_shell(),
            "StatusBarTones" => docs_status_bar_tones(),
            "StatusBarCustom" => docs_status_bar_custom(),
            "StatusBarAdvanced" => docs_status_bar_advanced(&_cx.global::<Config>().theme),
            "SearchableListBasic" => docs_searchable_list_basic(),
            "SearchableListFiltered" => docs_searchable_list_filtered(),
            "SearchableListEmpty" => docs_searchable_list_empty(),
            "SelectSearchable" | "SelectGrouped" | "SelectMultiple" | "SelectFooter" => demo_stack(
                self.select_advanceds
                    .iter()
                    .cloned()
                    .map(Entity::into_any_element)
                    .collect(),
            ),
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
            "CodeEditorBasic" | "CodeEditorDiagnostics" | "CodeEditorAdvanced" => demo_stack(
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
            "VirtualizedTableDataTable" => virtualized_table_demo(false, None, None)
                .selected_rows([1, 3, 5])
                .active_row(Some(8))
                .load_more("加载更多数据", |_, _| {})
                .into_any_element(),
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
                liora_components::AreaChart::new([liora_components::ChartSeries::new(
                    "Visitors",
                    [
                        liora_components::ChartPoint::new("Mon", 24.0),
                        liora_components::ChartPoint::new("Tue", 32.0),
                        liora_components::ChartPoint::new("Wed", 45.0),
                        liora_components::ChartPoint::new("Thu", 52.0),
                        liora_components::ChartPoint::new("Fri", 61.0),
                        liora_components::ChartPoint::new("Sat", 72.0),
                        liora_components::ChartPoint::new("Sun", 68.0),
                    ],
                )])
                .id("docs-area-chart-basic")
                .height(px(260.0))
                .into_any_element(),
            ]),
            "AreaChartOverlay" => demo_row(vec![
                liora_components::AreaChart::new([
                    liora_components::ChartSeries::new(
                        "Desktop",
                        [
                            liora_components::ChartPoint::new("Mon", 28.0),
                            liora_components::ChartPoint::new("Tue", 34.0),
                            liora_components::ChartPoint::new("Wed", 38.0),
                            liora_components::ChartPoint::new("Thu", 44.0),
                            liora_components::ChartPoint::new("Fri", 50.0),
                        ],
                    ),
                    liora_components::ChartSeries::new(
                        "Mobile",
                        [
                            liora_components::ChartPoint::new("Mon", 18.0),
                            liora_components::ChartPoint::new("Tue", 25.0),
                            liora_components::ChartPoint::new("Wed", 32.0),
                            liora_components::ChartPoint::new("Thu", 39.0),
                            liora_components::ChartPoint::new("Fri", 48.0),
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
                liora_components::AreaChart::new([
                    liora_components::ChartSeries::new(
                        "Desktop",
                        [
                            liora_components::ChartPoint::new("Mon", 28.0),
                            liora_components::ChartPoint::new("Tue", 34.0),
                            liora_components::ChartPoint::new("Wed", 38.0),
                            liora_components::ChartPoint::new("Thu", 44.0),
                            liora_components::ChartPoint::new("Fri", 50.0),
                        ],
                    ),
                    liora_components::ChartSeries::new(
                        "Mobile",
                        [
                            liora_components::ChartPoint::new("Mon", 18.0),
                            liora_components::ChartPoint::new("Tue", 25.0),
                            liora_components::ChartPoint::new("Wed", 32.0),
                            liora_components::ChartPoint::new("Thu", 39.0),
                            liora_components::ChartPoint::new("Fri", 48.0),
                        ],
                    ),
                ])
                .id("docs-area-chart-stacked")
                .height(px(300.0))
                .stacked()
                .into_any_element(),
            ]),
            "AreaChartCustom" => demo_row(vec![
                liora_components::AreaChart::new([
                    liora_components::ChartSeries::new(
                        "Desktop",
                        [
                            liora_components::ChartPoint::new("Mon", 28.0),
                            liora_components::ChartPoint::new("Tue", 34.0),
                            liora_components::ChartPoint::new("Wed", 38.0),
                            liora_components::ChartPoint::new("Thu", 44.0),
                            liora_components::ChartPoint::new("Fri", 50.0),
                        ],
                    )
                    .stroke_color(gpui::blue())
                    .fill_color(gpui::blue().opacity(0.36))
                    .stroke_width(px(3.0))
                    .smooth(true),
                    liora_components::ChartSeries::new(
                        "Mobile",
                        [
                            liora_components::ChartPoint::new("Mon", 18.0),
                            liora_components::ChartPoint::new("Tue", 25.0),
                            liora_components::ChartPoint::new("Wed", 32.0),
                            liora_components::ChartPoint::new("Thu", 39.0),
                            liora_components::ChartPoint::new("Fri", 48.0),
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
                .value_label_content(liora_components::ChartValueLabelContent::ValueAndPercentage)
                .percentage_decimals(1)
                .into_any_element(),
            ]),
            "AreaChartDownsample" => demo_row(vec![
                liora_components::AreaChart::new([
                    liora_components::ChartSeries::new(
                        "Desktop",
                        (0..1_800).map(|index| {
                            let wave = ((index as f64) / 32.0).sin() * 14.0;
                            liora_components::ChartPoint::new(format!("T{index}"), 42.0 + wave)
                        }),
                    ),
                    liora_components::ChartSeries::new(
                        "Mobile",
                        (0..1_800).map(|index| {
                            let wave = ((index as f64) / 27.0).cos() * 10.0;
                            let spike = if index % 360 == 0 { 24.0 } else { 0.0 };
                            liora_components::ChartPoint::new(format!("T{index}"), 28.0 + wave + spike)
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
                        let name = "Liora";
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
            "SpinnerSizes" | "SpinnerBasic" => {
                let theme = _cx.global::<Config>().theme.clone();
                let card_bg = theme.neutral.hover.opacity(0.56);
                let card_alt_bg = theme.neutral.card;
                let card_border = theme.neutral.border;
                spinner_live_grid(vec![
                    spinner_live_card(
                        "Small / inline",
                        "Button labels and status bars",
                        liora_components::Spinner::new().small(),
                        card_bg,
                        card_border,
                    ),
                    spinner_live_card(
                        "Default / row",
                        "List rows and toolbar jobs",
                        liora_components::Spinner::new(),
                        card_alt_bg,
                        card_border,
                    ),
                    spinner_live_card(
                        "Large / panel",
                        "Card-level refresh state",
                        liora_components::Spinner::new().large(),
                        card_bg,
                        card_border,
                    ),
                    spinner_live_card(
                        "Custom icon",
                        "RefreshCw with spin motion",
                        liora_components::Spinner::new().icon(IconName::RefreshCw).size(px(22.0)),
                        card_alt_bg,
                        card_border,
                    ),
                ])
            }
            "SpinnerColors" => {
                let theme = _cx.global::<Config>().theme.clone();
                let card_bg = theme.neutral.hover.opacity(0.56);
                let card_alt_bg = theme.neutral.card;
                let card_border = theme.neutral.border;
                spinner_live_grid(vec![
                    spinner_live_status_card(
                        "Syncing",
                        "同步远端配置中",
                        0x2563eb,
                        card_alt_bg,
                        card_border,
                    ),
                    spinner_live_status_card(
                        "Verifying",
                        "等待校验服务返回",
                        0x16a34a,
                        card_bg,
                        card_border,
                    ),
                    spinner_live_status_card(
                        "Retrying",
                        "网络不稳定，正在重试",
                        0xf59e0b,
                        card_alt_bg,
                        card_border,
                    ),
                    spinner_live_status_card(
                        "Recovering",
                        "错误恢复任务仍在运行",
                        0xdc2626,
                        card_bg,
                        card_border,
                    ),
                ])
            }
            "SpinnerComposition" => {
                let theme = _cx.global::<Config>().theme.clone();
                let card_bg = theme.neutral.hover.opacity(0.56);
                let card_alt_bg = theme.neutral.card;
                let card_border = theme.neutral.border;
                demo_stack(vec![
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(Button::new("Syncing").primary().icon_start(
                            liora_components::Spinner::new().small().into_any_element(),
                        ))
                        .child(Button::new("Exporting").icon_start(
                            liora_components::Spinner::new().small().into_any_element(),
                        ))
                        .into_any_element(),
                    div()
                        .w(px(320.0))
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap_4()
                        .rounded_lg()
                        .border_1()
                        .border_color(card_border)
                        .bg(card_bg)
                        .p_4()
                        .child(div().flex_1().min_w(px(0.0)).child(
                            liora_components::Label::new("Fetching metrics")
                                .custom_icon(liora_components::Spinner::new().small()),
                        ))
                        .child(
                            div()
                                .flex_none()
                                .child(Text::new("12 jobs queued").xs().nowrap()),
                        )
                        .into_any_element(),
                    spinner_live_card(
                        "Background export",
                        "Exporting reports.zip · 42%",
                        liora_components::Spinner::new().icon(IconName::LoaderCircle).large(),
                        card_alt_bg,
                        card_border,
                    ),
                ])
            }
            "KbdBasic" => demo_row(vec![
                liora_components::Kbd::new("⌘K").into_any_element(),
                liora_components::Kbd::new("Ctrl").into_any_element(),
                liora_components::Kbd::new("Shift").into_any_element(),
                liora_components::Kbd::new("Enter").into_any_element(),
                liora_components::Kbd::new("Esc").into_any_element(),
            ]),
            "KbdSizes" => demo_row(vec![
                liora_components::Kbd::new("⌘").small().into_any_element(),
                liora_components::Kbd::new("Tab").into_any_element(),
                liora_components::Kbd::new("Space").large().into_any_element(),
            ]),
            "KbdComposition" => demo_row(vec![
                liora_components::Kbd::new("Esc").color(gpui::rgb(0xdc2626).into()).into_any_element(),
                liora_components::Kbd::new("⌘S")
                    .bg(gpui::rgb(0xdcfce7).into())
                    .color(gpui::rgb(0x166534).into())
                    .into_any_element(),
                Space::new()
                    .gap_xs()
                    .child(liora_components::Kbd::new("⌘"))
                    .child(liora_components::Kbd::new("K"))
                    .into_any_element(),
            ]),
            "OtpInputInteractive" | "OtpInputBasic" => self.otp_inputs.first().cloned().map_or_else(
                || Text::new("OtpInput demo unavailable").into_any_element(),
                |input| input.into_any_element(),
            ),
            "OtpInputMasked" => self.otp_inputs.first().cloned().map_or_else(
                || Text::new("OtpInput masked demo unavailable").into_any_element(),
                |input| input.into_any_element(),
            ),
            "OtpInputStates" => demo_stack(
                self.otp_inputs
                    .iter()
                    .cloned()
                    .map(IntoElement::into_any_element)
                    .collect(),
            ),
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
            "ResultSuccess" => LioraResult::new("成功购买云服务器")
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
                LioraResult::new("您的账户存在安全风险")
                    .status(ResultStatus::Warning)
                    .sub_title("请及时修改密码并开启双重验证。")
                    .extra(|_, _| Button::new("立即处理").primary().into_any_element())
                    .into_any_element(),
                LioraResult::new("提交失败")
                    .status(ResultStatus::Error)
                    .sub_title("请检查网络连接并重试。")
                    .extra(|_, _| Button::new("重新提交").primary().into_any_element())
                    .into_any_element(),
                LioraResult::new("您的申请已提交")
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
                    .suffix(liora_icons::Icon::new(IconName::TrendingUp))
                    .into_any_element(),
                Statistic::new("月活下降", "5.2")
                    .suffix(liora_icons::Icon::new(IconName::TrendingDown))
                    .into_any_element(),
                Statistic::new("待办事项", "12")
                    .prefix(liora_icons::Icon::new(IconName::ListTodo))
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
            "CandlestickChartBasic" => demo_row(vec![
                liora_components::CandlestickChart::new([
                    liora_components::CandlestickPoint::new("Mon", 102.0, 112.0, 98.0, 109.0).volume(12_400.0),
                    liora_components::CandlestickPoint::new("Tue", 109.0, 115.0, 104.0, 106.0).volume(15_800.0),
                    liora_components::CandlestickPoint::new("Wed", 106.0, 121.0, 105.0, 118.0).volume(18_600.0),
                    liora_components::CandlestickPoint::new("Thu", 118.0, 124.0, 111.0, 114.0).volume(16_100.0),
                    liora_components::CandlestickPoint::new("Fri", 114.0, 128.0, 113.0, 126.0).volume(21_900.0),
                ])
                .height(gpui::px(320.0))
                .show_legend(true)
                .tooltip_hit_radius(gpui::px(12.0))
                .into_any_element(),
            ]),
            "CandlestickChartCustom" => demo_row(vec![
                liora_components::CandlestickChart::new([
                    liora_components::CandlestickPoint::new("09:30", 88.0, 94.0, 85.0, 92.0),
                    liora_components::CandlestickPoint::new("10:00", 92.0, 96.0, 89.0, 90.0),
                    liora_components::CandlestickPoint::new("10:30", 90.0, 101.0, 88.0, 99.0),
                    liora_components::CandlestickPoint::new("11:00", 99.0, 103.0, 95.0, 97.0),
                    liora_components::CandlestickPoint::new("11:30", 97.0, 108.0, 96.0, 106.0),
                ])
                .height(gpui::px(300.0))
                .up_color(gpui::rgb(0x14b8a6).into())
                .down_color(gpui::rgb(0xf43f5e).into())
                .body_width(gpui::px(12.0))
                .wick_width(gpui::px(2.0))
                .max_axis_labels(5)
                .into_any_element(),
            ]),
            "CandlestickChartDense" => demo_row(vec![
                liora_components::CandlestickChart::new((0..48).map(|index| {
                    let base = 110.0 + (index as f64 * 0.36) + ((index % 7) as f64 - 3.0) * 1.6;
                    let open = base + ((index % 5) as f64 - 2.0) * 0.9;
                    let close = base + (((index + 2) % 5) as f64 - 2.0) * 1.2;
                    let high = open.max(close) + 3.0 + (index % 4) as f64;
                    let low = open.min(close) - 2.6 - (index % 3) as f64;
                    liora_components::CandlestickPoint::new(format!("D{}", index + 1), open, high, low, close)
                }))
                .height(gpui::px(340.0))
                .up_color(gpui::green())
                .down_color(gpui::red())
                .max_render_points(28)
                .max_axis_labels(8)
                .max_value_labels(8)
                .show_value_labels(true)
                .value_label_content(liora_components::ChartValueLabelContent::Value)
                .into_any_element(),
            ]),
            "BarChartBasic" => demo_row(vec![
                liora_components::BarChart::new([liora_components::ChartSeries::new(
                    "Revenue",
                    [
                        liora_components::ChartPoint::new("Q1", 42.0),
                        liora_components::ChartPoint::new("Q2", 58.0),
                        liora_components::ChartPoint::new("Q3", 73.0),
                        liora_components::ChartPoint::new("Q4", 96.0),
                    ],
                )])
                .id("docs-bar-chart-basic")
                .height(px(260.0))
                .tooltip_hit_radius(px(10.0))
                .into_any_element(),
            ]),
            "BarChartGrouped" => demo_row(vec![
                liora_components::BarChart::new([
                    liora_components::ChartSeries::new(
                        "Online",
                        [
                            liora_components::ChartPoint::new("Jan", 42.0),
                            liora_components::ChartPoint::new("Feb", 58.0),
                            liora_components::ChartPoint::new("Mar", 64.0),
                            liora_components::ChartPoint::new("Apr", 72.0),
                        ],
                    ),
                    liora_components::ChartSeries::new(
                        "Retail",
                        [
                            liora_components::ChartPoint::new("Jan", 28.0),
                            liora_components::ChartPoint::new("Feb", 34.0),
                            liora_components::ChartPoint::new("Mar", 39.0),
                            liora_components::ChartPoint::new("Apr", 45.0),
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
                liora_components::BarChart::new([liora_components::ChartSeries::new(
                    "Revenue",
                    [
                        liora_components::ChartPoint::new("Q1", 42.0),
                        liora_components::ChartPoint::new("Q2", 58.0),
                        liora_components::ChartPoint::new("Q3", 73.0),
                        liora_components::ChartPoint::new("Q4", 96.0),
                    ],
                )])
                .id("docs-bar-chart-gradient")
                .height(px(300.0))
                .bar_radius(px(6.0))
                .bar_vertical_gradient(gpui::rgb(0x60a5fa).into(), gpui::rgb(0x2563eb).into())
                .value_fill_ranges([
                    liora_components::BarChartValueFillRange::new(
                        0.0,
                        60.0,
                        liora_components::BarChartFill::vertical_gradient(
                            gpui::rgb(0xbfdbfe).into(),
                            gpui::rgb(0x3b82f6).into(),
                        ),
                    ),
                    liora_components::BarChartValueFillRange::new(
                        60.0,
                        100.0,
                        liora_components::BarChartFill::vertical_gradient(
                            gpui::rgb(0xfef08a).into(),
                            gpui::rgb(0xf97316).into(),
                        ),
                    ),
                ])
                .into_any_element(),
            ]),
            "BarChartPerBarGradient" => demo_row(vec![
                liora_components::BarChart::new([liora_components::ChartSeries::new(
                    "Revenue",
                    [
                        liora_components::ChartPoint::new("Q1", 42.0),
                        liora_components::ChartPoint::new("Q2", 58.0),
                        liora_components::ChartPoint::new("Q3", 73.0),
                        liora_components::ChartPoint::new("Q4", 96.0),
                    ],
                )])
                .id("docs-bar-chart-per-bar-gradient")
                .height(px(300.0))
                .bar_radius(px(8.0))
                .bar_fills([
                    liora_components::BarChartFill::vertical_gradient(gpui::rgb(0xdbeafe).into(), gpui::rgb(0x2563eb).into()),
                    liora_components::BarChartFill::vertical_gradient(gpui::rgb(0xdcfce7).into(), gpui::rgb(0x16a34a).into()),
                    liora_components::BarChartFill::vertical_gradient(gpui::rgb(0xffedd5).into(), gpui::rgb(0xea580c).into()),
                    liora_components::BarChartFill::vertical_gradient(gpui::rgb(0xfce7f3).into(), gpui::rgb(0xdb2777).into()),
                ])
                .into_any_element(),
            ]),
            "BarChartStacked" => demo_row(vec![
                liora_components::BarChart::new([
                    liora_components::ChartSeries::new(
                        "Online",
                        [
                            liora_components::ChartPoint::new("Jan", 42.0),
                            liora_components::ChartPoint::new("Feb", 58.0),
                            liora_components::ChartPoint::new("Mar", 64.0),
                            liora_components::ChartPoint::new("Apr", 72.0),
                        ],
                    ),
                    liora_components::ChartSeries::new(
                        "Retail",
                        [
                            liora_components::ChartPoint::new("Jan", 28.0),
                            liora_components::ChartPoint::new("Feb", 34.0),
                            liora_components::ChartPoint::new("Mar", 39.0),
                            liora_components::ChartPoint::new("Apr", 45.0),
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
                liora_components::BarChart::new([
                    liora_components::ChartSeries::new(
                        "Online",
                        [
                            liora_components::ChartPoint::new("Jan", 42.0),
                            liora_components::ChartPoint::new("Feb", 58.0),
                            liora_components::ChartPoint::new("Mar", 64.0),
                            liora_components::ChartPoint::new("Apr", 72.0),
                        ],
                    )
                    .fill_color(gpui::blue()),
                    liora_components::ChartSeries::new(
                        "Retail",
                        [
                            liora_components::ChartPoint::new("Jan", 28.0),
                            liora_components::ChartPoint::new("Feb", 34.0),
                            liora_components::ChartPoint::new("Mar", 39.0),
                            liora_components::ChartPoint::new("Apr", 45.0),
                        ],
                    )
                    .fill_color(gpui::green()),
                ])
                .id("docs-bar-chart-custom")
                .height(px(340.0))
                .y_domain(0.0, 120.0)
                .bar_gap_ratio(0.32)
                .value_label_content(liora_components::ChartValueLabelContent::ValueAndPercentage)
                .percentage_decimals(1)
                .into_any_element(),
            ]),
            "BarChartStandalone" => demo_row(vec![
                liora_components::BarChart::new([liora_components::ChartSeries::new(
                    "Active",
                    [
                        liora_components::ChartPoint::new("Mon", 18.0),
                        liora_components::ChartPoint::new("Tue", 42.0),
                        liora_components::ChartPoint::new("Wed", 33.0),
                        liora_components::ChartPoint::new("Thu", 76.0),
                        liora_components::ChartPoint::new("Fri", 61.0),
                    ],
                )])
                .id("docs-bar-chart-standalone")
                .standalone()
                .bar_width(px(8.0))
                .bar_gap(px(7.0))
                .bar_radius(px(5.0))
                .value_color_ranges([
                    liora_components::BarChartValueColorRange::new(0.0, 35.0, gpui::rgb(0x86efac).into()),
                    liora_components::BarChartValueColorRange::new(35.0, 70.0, gpui::rgb(0x22c55e).into()),
                    liora_components::BarChartValueColorRange::new(70.0, 100.0, gpui::rgb(0x16a34a).into()),
                ])
                .into_any_element(),
            ]),
            "BarChartStandaloneStyles" => demo_row(vec![
                liora_components::Space::new()
                    .wrap()
                    .gap_lg()
                    .child(
                        liora_components::BarChart::new([liora_components::ChartSeries::new(
                            "Active",
                            [
                                liora_components::ChartPoint::new("Mon", 18.0),
                                liora_components::ChartPoint::new("Tue", 42.0),
                                liora_components::ChartPoint::new("Wed", 33.0),
                                liora_components::ChartPoint::new("Thu", 76.0),
                                liora_components::ChartPoint::new("Fri", 61.0),
                                liora_components::ChartPoint::new("Sat", 88.0),
                                liora_components::ChartPoint::new("Sun", 54.0),
                            ],
                        )])
                        .id("docs-bar-chart-standalone-compact")
                        .standalone()
                        .bar_width(px(8.0))
                        .bar_gap(px(4.0))
                        .bar_radius(px(5.0))
                        .value_color_ranges([
                            liora_components::BarChartValueColorRange::new(0.0, 35.0, gpui::rgb(0x86efac).into()),
                            liora_components::BarChartValueColorRange::new(35.0, 70.0, gpui::rgb(0x22c55e).into()),
                            liora_components::BarChartValueColorRange::new(70.0, 100.0, gpui::rgb(0x16a34a).into()),
                        ]),
                    )
                    .child(
                        liora_components::BarChart::new([liora_components::ChartSeries::new(
                            "Active",
                            [
                                liora_components::ChartPoint::new("Mon", 18.0),
                                liora_components::ChartPoint::new("Tue", 42.0),
                                liora_components::ChartPoint::new("Wed", 33.0),
                                liora_components::ChartPoint::new("Thu", 76.0),
                                liora_components::ChartPoint::new("Fri", 61.0),
                                liora_components::ChartPoint::new("Sat", 88.0),
                                liora_components::ChartPoint::new("Sun", 54.0),
                            ],
                        )])
                        .id("docs-bar-chart-standalone-gradient")
                        .standalone()
                        .bar_width(px(10.0))
                        .bar_gap(px(5.0))
                        .bar_radius(px(8.0))
                        .bar_fills([
                            liora_components::BarChartFill::vertical_gradient(gpui::rgb(0xc4b5fd).into(), gpui::rgb(0x7c3aed).into()),
                            liora_components::BarChartFill::vertical_gradient(gpui::rgb(0xbae6fd).into(), gpui::rgb(0x0284c7).into()),
                            liora_components::BarChartFill::vertical_gradient(gpui::rgb(0xfde68a).into(), gpui::rgb(0xd97706).into()),
                        ]),
                    )
                    .child(
                        liora_components::BarChart::new([liora_components::ChartSeries::new(
                            "Active",
                            [
                                liora_components::ChartPoint::new("Mon", 18.0),
                                liora_components::ChartPoint::new("Tue", 42.0),
                                liora_components::ChartPoint::new("Wed", 33.0),
                                liora_components::ChartPoint::new("Thu", 76.0),
                                liora_components::ChartPoint::new("Fri", 61.0),
                                liora_components::ChartPoint::new("Sat", 88.0),
                                liora_components::ChartPoint::new("Sun", 54.0),
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
                liora_components::TagFlow::new([
                    liora_components::Tag::new("Design").round(true),
                    liora_components::Tag::new("GPUI").success().round(true),
                    liora_components::Tag::new("Animation").warning().round(true),
                    liora_components::Tag::new("Native Rust").danger().round(true),
                    liora_components::Tag::new("Charts").round(true),
                    liora_components::Tag::new("Docs").success().round(true),
                    liora_components::Tag::new("Installer").warning().round(true),
                    liora_components::Tag::new("Tray").round(true),
                ])
                .gap(px(10.0))
                .max_rows(2)
                .estimated_items_per_row(3)
                .overflow_indicator("更多")
                .into_any_element(),
            ]),
            "SignalMeterMobile" => demo_row(vec![
                liora_components::SignalMeter::new(3).height(px(36.0)).into_any_element(),
            ]),
            "SignalMeterWifi" => demo_row(vec![
                liora_components::SignalMeter::new(2)
                    .wifi()
                    .active_color(gpui::rgb(0x3b82f6).into())
                    .inactive_color(gpui::rgb(0xdbeafe).into())
                    .bar_width(px(8.0))
                    .gap(px(5.0))
                    .height(px(44.0))
                    .into_any_element(),
            ]),
            "SignalMeterLevels" => demo_row(vec![
                liora_components::SignalMeter::new(5)
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
                liora_components::SignalMeter::new(2)
                    .total_signals(5)
                    .threshold_colors([
                        liora_components::SignalLevelColor::new(2, gpui::rgb(0xef4444).into()),
                        liora_components::SignalLevelColor::new(3, gpui::rgb(0xeab308).into()),
                        liora_components::SignalLevelColor::new(4, gpui::rgb(0xf97316).into()),
                        liora_components::SignalLevelColor::new(5, gpui::rgb(0x22c55e).into()),
                    ])
                    .inactive_color(gpui::rgb(0xf1f5f9).into())
                    .height(px(42.0))
                    .bar_width(px(7.0))
                    .gap(px(5.0))
                    .into_any_element(),
                liora_components::SignalMeter::new(4)
                    .total_signals(5)
                    .threshold_colors([
                        liora_components::SignalLevelColor::new(2, gpui::rgb(0xef4444).into()),
                        liora_components::SignalLevelColor::new(3, gpui::rgb(0xeab308).into()),
                        liora_components::SignalLevelColor::new(4, gpui::rgb(0xf97316).into()),
                        liora_components::SignalLevelColor::new(5, gpui::rgb(0x22c55e).into()),
                    ])
                    .inactive_color(gpui::rgb(0xf1f5f9).into())
                    .height(px(42.0))
                    .bar_width(px(7.0))
                    .gap(px(5.0))
                    .into_any_element(),
            ]),
            "HeatBarEvents" => demo_row(vec![
                liora_components::HeatBar::new((0..48).map(|index| {
                    let value = ((index * 7 + 3) % 11) as f64;
                    liora_components::HeatBarItem::new(format!("t{index}"), value, gpui::rgb(0x93c5fd).into())
                }))
                .legends([
                    liora_components::HeatBarLegend::new("错误", 3, gpui::rgb(0xef4444).into()),
                    liora_components::HeatBarLegend::new("警告", 24, gpui::rgb(0xf59e0b).into()),
                ])
                .color_ranges([
                    liora_components::HeatBarColorRange::new(0.0, 3.0, gpui::rgb(0x93c5fd).into()),
                    liora_components::HeatBarColorRange::new(3.0, 7.0, gpui::rgb(0xf59e0b).into()),
                    liora_components::HeatBarColorRange::above(7.0, gpui::rgb(0xef4444).into()),
                ])
                .max_value(10.0)
                .x_labels(["00:00", "06:00", "12:00", "18:00", "24:00"])
                .into_any_element(),
            ]),
            "SegmentRatioBarBottom" => demo_row(vec![
                liora_components::SegmentRatioBar::new([
                    liora_components::SegmentRatioItem::new("Direct", 42.0, gpui::rgb(0x3b82f6).into()),
                    liora_components::SegmentRatioItem::new("Proxy", 51.0, gpui::rgb(0x22c55e).into()),
                    liora_components::SegmentRatioItem::new("Reject", 7.0, gpui::rgb(0xef4444).into()),
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
                liora_components::SegmentRatioBar::new([
                    liora_components::SegmentRatioItem::new("Direct", 42.0, gpui::rgb(0x3b82f6).into()),
                    liora_components::SegmentRatioItem::new("Proxy", 51.0, gpui::rgb(0x22c55e).into()),
                    liora_components::SegmentRatioItem::new("Reject", 7.0, gpui::rgb(0xef4444).into()),
                ])
                .legend_position(liora_components::SegmentLegendPosition::Top)
                .height(px(16.0))
                .radius(px(8.0))
                .rounded_segments(px(4.0))
                .legend_inset_x(px(10.0))
                .percentage_decimals(1)
                .into_any_element(),
            ]),
            "SegmentRatioBarBoth" => demo_row(vec![
                liora_components::SegmentRatioBar::new([
                    liora_components::SegmentRatioItem::new("Direct", 42.0, gpui::rgb(0x3b82f6).into()),
                    liora_components::SegmentRatioItem::new("Proxy", 51.0, gpui::rgb(0x22c55e).into()),
                    liora_components::SegmentRatioItem::new("Reject", 7.0, gpui::rgb(0xef4444).into()),
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
                liora_components::SegmentRatioBar::new([
                    liora_components::SegmentRatioItem::new("Direct", 42.0, gpui::rgb(0x3b82f6).into()),
                    liora_components::SegmentRatioItem::new("Proxy", 51.0, gpui::rgb(0x22c55e).into()),
                    liora_components::SegmentRatioItem::new("Reject", 7.0, gpui::rgb(0xef4444).into()),
                ])
                .hide_legend()
                .height(px(18.0))
                .radius(px(9.0))
                .segment_radius(px(4.0))
                .into_any_element(),
            ]),
            "SegmentRatioBarPattern" => demo_row(vec![
                liora_components::SegmentRatioBar::new([
                    liora_components::SegmentRatioItem::new("Direct", 42.0, gpui::rgb(0x3b82f6).into())
                        .label_pattern("{label}")
                        .value_pattern("{value} req / {percent}"),
                    liora_components::SegmentRatioItem::new("Proxy", 51.0, gpui::rgb(0x22c55e).into())
                        .value_pattern("{percent}"),
                    liora_components::SegmentRatioItem::new("Reject", 7.0, gpui::rgb(0xef4444).into())
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
                liora_components::SegmentRatioBar::new([
                    liora_components::SegmentRatioItem::new("API", 18.0, gpui::rgb(0x8b5cf6).into()),
                    liora_components::SegmentRatioItem::new("Web", 33.0, gpui::rgb(0x06b6d4).into()),
                    liora_components::SegmentRatioItem::new("Jobs", 29.0, gpui::rgb(0xf59e0b).into()),
                    liora_components::SegmentRatioItem::new("Other", 20.0, gpui::rgb(0x64748b).into()),
                ])
                .height(px(8.0))
                .radius(px(4.0))
                .rounded_segments(px(2.0))
                .legend_inset_x(px(14.0))
                .percentage_decimals(2)
                .into_any_element(),
            ]),
            "LabelBasic" => demo_row(vec![
                liora_components::Label::new("Build passed").icon(IconName::CircleCheck).color(gpui::green()).into_any_element(),
            ]),
            "OperationBasic" => demo_row(vec![
                liora_components::Operation::new(
                    liora_components::Label::new("执行操作").icon(IconName::Play),
                    liora_components::Button::new("Run").small(),
                )
                .description("左侧可带说明文本，右侧操作区域保持末端对齐。")
                .status("手动")
                .into_any_element(),
            ]),
            "SparklineBasic" => demo_row(vec![
                liora_components::Sparkline::new([12.0, 15.0, 14.0, 18.0, 21.0, 19.0, 24.0, 28.0])
                    .id("docs-sparkline-basic")
                    .width(px(220.0))
                    .height(px(64.0))
                    .color(gpui::rgb(0x2563eb).into())
                    .stroke_width(px(2.4))
                    .into_any_element(),
            ]),
            "SparklineCards" => demo_row(vec![
                liora_components::Card::new(
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(Text::new("Revenue").size(px(12.0)).text_color(gpui::rgb(0x64748b).into()))
                        .child(Text::new("$42.8k").size(px(24.0)).bold())
                        .child(
                            liora_components::Sparkline::new([12.0, 15.0, 14.0, 18.0, 21.0, 19.0, 24.0, 28.0])
                                .id("docs-sparkline-card")
                                .height(px(64.0))
                                .area_fill(true),
                        ),
                )
                .width(px(240.0))
                .into_any_element(),
            ]),
            "SparklineArea" => demo_row(vec![
                liora_components::Sparkline::new([-4.0, -1.0, 3.0, 7.0, 5.0, -2.0, 4.0, 10.0, 8.0])
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
                liora_components::Sparkline::new([12.0, 15.0, 14.0, 18.0, 21.0, 19.0, 24.0, 28.0])
                    .id("docs-sparkline-style-dashed")
                    .width(px(220.0))
                    .height(px(56.0))
                    .color(gpui::rgb(0x2563eb).into())
                    .line_style(liora_components::ChartLineStyle::Dashed)
                    .smooth(false)
                    .show_last_point(false)
                    .into_any_element(),
                liora_components::Sparkline::new([28.0, 24.0, 25.0, 22.0, 18.0, 17.0, 15.0, 12.0])
                    .id("docs-sparkline-style-dotted")
                    .width(px(220.0))
                    .height(px(56.0))
                    .color(gpui::rgb(0xdc2626).into())
                    .dotted()
                    .show_last_point(false)
                    .into_any_element(),
            ]),
            "SparklineDownsample" => demo_row(vec![
                liora_components::Sparkline::new((0..1_200).map(|index| {
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
                liora_components::PieChart::new([
                    liora_components::ChartSeries::new("Desktop", [liora_components::ChartPoint::new("Desktop", 62.0)]),
                    liora_components::ChartSeries::new("Mobile", [liora_components::ChartPoint::new("Mobile", 24.0)]),
                    liora_components::ChartSeries::new("Tablet", [liora_components::ChartPoint::new("Tablet", 9.0)]),
                    liora_components::ChartSeries::new("Other", [liora_components::ChartPoint::new("Other", 5.0)]),
                ])
                .id("docs-pie-chart")
                .height(px(340.0))
                .percentage_decimals(1)
                .outside_label_threshold_degrees(30)
                .value_label_placement(liora_components::ChartValueLabelPlacement::OutsideAligned)
                .tooltip_hit_radius(px(10.0))
                .into_any_element(),
            ]),
            "PieChartCustom" => demo_row(vec![
                liora_components::PieChart::new([
                    liora_components::ChartSeries::new("Desktop", [liora_components::ChartPoint::new("Desktop", 62.0)]).fill_color(gpui::blue()),
                    liora_components::ChartSeries::new("Mobile", [liora_components::ChartPoint::new("Mobile", 24.0)]).fill_color(gpui::green()),
                    liora_components::ChartSeries::new("Tablet", [liora_components::ChartPoint::new("Tablet", 9.0)]).fill_color(gpui::yellow()),
                    liora_components::ChartSeries::new("Other", [liora_components::ChartPoint::new("Other", 5.0)]).fill_color(gpui::red()),
                ])
                .id("docs-pie-chart-custom")
                .height(px(360.0))
                .value_label_content(liora_components::ChartValueLabelContent::Percentage)
                .value_label_placement(liora_components::ChartValueLabelPlacement::OutsideFree)
                .percentage_decimals(2)
                .outside_label_threshold_degrees(120)
                .show_tooltip(false)
                .into_any_element(),
            ]),
            "RingChart" => demo_row(vec![
                liora_components::RingChart::new([
                    liora_components::ChartSeries::new("Desktop", [liora_components::ChartPoint::new("Desktop", 62.0)]),
                    liora_components::ChartSeries::new("Mobile", [liora_components::ChartPoint::new("Mobile", 24.0)]),
                    liora_components::ChartSeries::new("Tablet", [liora_components::ChartPoint::new("Tablet", 9.0)]),
                    liora_components::ChartSeries::new("Other", [liora_components::ChartPoint::new("Other", 5.0)]),
                ])
                .id("docs-ring-chart")
                .height(px(340.0))
                .inner_ratio(0.5)
                .percentage_decimals(1)
                .outside_label_threshold_degrees(30)
                .value_label_placement(liora_components::ChartValueLabelPlacement::OutsideAligned)
                .tooltip_hit_radius(px(10.0))
                .into_any_element(),
            ]),
            "RingChartCustom" => demo_row(vec![
                liora_components::RingChart::new([
                    liora_components::ChartSeries::new("Desktop", [liora_components::ChartPoint::new("Desktop", 62.0)]).fill_color(gpui::blue()),
                    liora_components::ChartSeries::new("Mobile", [liora_components::ChartPoint::new("Mobile", 24.0)]).fill_color(gpui::green()),
                    liora_components::ChartSeries::new("Tablet", [liora_components::ChartPoint::new("Tablet", 9.0)]).fill_color(gpui::yellow()),
                    liora_components::ChartSeries::new("Other", [liora_components::ChartPoint::new("Other", 5.0)]).fill_color(gpui::red()),
                ])
                .id("docs-ring-chart-custom")
                .height(px(360.0))
                .inner_ratio(0.48)
                .value_label_content(liora_components::ChartValueLabelContent::ValueOverTotalAndPercentage)
                .value_label_placement(liora_components::ChartValueLabelPlacement::OutsideAligned)
                .percentage_decimals(1)
                .outside_label_threshold_degrees(120)
                .into_any_element(),
            ]),
            "RingChartExternal" => demo_row(vec![
                liora_components::RingChart::new([
                    liora_components::ChartSeries::new("Desktop", [liora_components::ChartPoint::new("Desktop", 62.0)]).fill_color(gpui::blue()),
                    liora_components::ChartSeries::new("Mobile", [liora_components::ChartPoint::new("Mobile", 24.0)]).fill_color(gpui::green()),
                    liora_components::ChartSeries::new("Tablet", [liora_components::ChartPoint::new("Tablet", 9.0)]).fill_color(gpui::yellow()),
                    liora_components::ChartSeries::new("Other", [liora_components::ChartPoint::new("Other", 5.0)]).fill_color(gpui::red()),
                ])
                .id("docs-ring-chart-external-vertical")
                .height(px(340.0))
                .inner_ratio(0.58)
                .external_vertical_legend()
                .external_legend_right()
                .external_legend_max_items(3)
                .external_legend_content(liora_components::ChartValueLabelContent::Percentage)
                .external_legend_percentage_decimals(2)
                .into_any_element(),
                liora_components::RingChart::new([
                    liora_components::ChartSeries::new("Desktop", [liora_components::ChartPoint::new("Desktop", 62.0)]).fill_color(gpui::blue()),
                    liora_components::ChartSeries::new("Mobile", [liora_components::ChartPoint::new("Mobile", 24.0)]).fill_color(gpui::green()),
                    liora_components::ChartSeries::new("Tablet", [liora_components::ChartPoint::new("Tablet", 9.0)]).fill_color(gpui::yellow()),
                    liora_components::ChartSeries::new("Other", [liora_components::ChartPoint::new("Other", 5.0)]).fill_color(gpui::red()),
                ])
                .id("docs-ring-chart-external-horizontal")
                .height(px(340.0))
                .inner_ratio(0.7)
                .external_horizontal_legend()
                .external_legend_content(liora_components::ChartValueLabelContent::ValueOverTotalAndPercentage)
                .external_legend_percentage_decimals(1)
                .show_tooltip(false)
                .into_any_element(),
            ]),

            "LineChartBasic" => demo_row(vec![
                liora_components::LineChart::new([liora_components::ChartSeries::new(
                    "CPU Usage",
                    [
                        liora_components::ChartPoint::new("10:00", 24.0),
                        liora_components::ChartPoint::new("10:05", 36.0),
                        liora_components::ChartPoint::new("10:10", 32.0),
                        liora_components::ChartPoint::new("10:15", 52.0),
                        liora_components::ChartPoint::new("10:20", 46.0),
                        liora_components::ChartPoint::new("10:25", 64.0),
                    ],
                )])
                .id("docs-line-chart-basic")
                .height(px(260.0))
                .tooltip_hit_radius(px(16.0))
                .into_any_element(),
            ]),
            "LineChartMulti" => demo_row(vec![
                liora_components::LineChart::new([
                    liora_components::ChartSeries::new(
                        "CPU",
                        [
                            liora_components::ChartPoint::new("Mon", 25.0),
                            liora_components::ChartPoint::new("Tue", 38.0),
                            liora_components::ChartPoint::new("Wed", 42.0),
                            liora_components::ChartPoint::new("Thu", 58.0),
                            liora_components::ChartPoint::new("Fri", 49.0),
                            liora_components::ChartPoint::new("Sat", 72.0),
                            liora_components::ChartPoint::new("Sun", 61.0),
                        ],
                    ),
                    liora_components::ChartSeries::new(
                        "Memory",
                        [
                            liora_components::ChartPoint::new("Mon", 48.0),
                            liora_components::ChartPoint::new("Tue", 52.0),
                            liora_components::ChartPoint::new("Wed", 57.0),
                            liora_components::ChartPoint::new("Thu", 63.0),
                            liora_components::ChartPoint::new("Fri", 66.0),
                            liora_components::ChartPoint::new("Sat", 69.0),
                            liora_components::ChartPoint::new("Sun", 74.0),
                        ],
                    ),
                ])
                .id("docs-line-chart-multi")
                .height(px(300.0))
                .y_domain(0.0, 100.0)
                .into_any_element(),
            ]),
            "LineChartCustom" => demo_row(vec![
                liora_components::LineChart::new([
                    liora_components::ChartSeries::new(
                        "CPU",
                        [
                            liora_components::ChartPoint::new("Mon", 25.0),
                            liora_components::ChartPoint::new("Tue", 38.0),
                            liora_components::ChartPoint::new("Wed", 42.0),
                            liora_components::ChartPoint::new("Thu", 58.0),
                            liora_components::ChartPoint::new("Fri", 49.0),
                        ],
                    )
                    .stroke_color(gpui::blue())
                    .fill_color(gpui::blue().opacity(0.22))
                    .stroke_width(px(3.2))
                    .smooth(true),
                    liora_components::ChartSeries::new(
                        "Memory",
                        [
                            liora_components::ChartPoint::new("Mon", 48.0),
                            liora_components::ChartPoint::new("Tue", 52.0),
                            liora_components::ChartPoint::new("Wed", 57.0),
                            liora_components::ChartPoint::new("Thu", 63.0),
                            liora_components::ChartPoint::new("Fri", 66.0),
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
                .value_label_content(liora_components::ChartValueLabelContent::Percentage)
                .percentage_decimals(1)
                .into_any_element(),
            ]),
            "LineChartLineStyles" => demo_row(vec![
                liora_components::LineChart::new([
                    liora_components::ChartSeries::new(
                        "Solid Smooth",
                        [
                            liora_components::ChartPoint::new("Mon", 32.0),
                            liora_components::ChartPoint::new("Tue", 44.0),
                            liora_components::ChartPoint::new("Wed", 38.0),
                            liora_components::ChartPoint::new("Thu", 70.0),
                        ],
                    )
                    .stroke_color(gpui::blue())
                    .stroke_width(px(3.2))
                    .line_style(liora_components::ChartLineStyle::Solid)
                    .smooth(true),
                    liora_components::ChartSeries::new(
                        "Dashed",
                        [
                            liora_components::ChartPoint::new("Mon", 22.0),
                            liora_components::ChartPoint::new("Tue", 35.0),
                            liora_components::ChartPoint::new("Wed", 52.0),
                            liora_components::ChartPoint::new("Thu", 58.0),
                        ],
                    )
                    .stroke_color(gpui::green())
                    .stroke_width(px(2.6))
                    .dashed()
                    .smooth(false),
                    liora_components::ChartSeries::new(
                        "Dotted",
                        [
                            liora_components::ChartPoint::new("Mon", 60.0),
                            liora_components::ChartPoint::new("Tue", 54.0),
                            liora_components::ChartPoint::new("Wed", 49.0),
                            liora_components::ChartPoint::new("Thu", 45.0),
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
                liora_components::LineChart::new(Vec::<liora_components::ChartSeries>::new())
                    .id("docs-line-chart-empty")
                    .height(px(220.0))
                    .into_any_element(),
            ]),
            "LineChartDownsample" => demo_row(vec![
                liora_components::LineChart::new([liora_components::ChartSeries::new(
                    "Latency",
                    (0..2_000).map(|index| {
                        let wave = ((index as f64) / 24.0).sin() * 18.0;
                        let spike = if index % 240 == 0 { 32.0 } else { 0.0 };
                        liora_components::ChartPoint::new(format!("T{index}"), 48.0 + wave + spike)
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
                    liora_components::Row::new()
                        .justify(liora_components::RowJustify::End)
                        .child(Button::new("Cancel").small())
                        .child(Button::new("Save").primary().small()),
                )
                .into_any_element(),
            "EmptyBasic" => Card::new(liora_components::Empty::new())
                .width_md()
                .into_any_element(),
            "EmptyDescription" => Card::new(
                liora_components::Empty::new().description("自定义描述文字"),
            )
            .width_md()
            .into_any_element(),
            "EmptyImage" => Card::new(
                liora_components::Empty::new()
                    .image(liora_icons::Icon::new(IconName::Search))
                    .description("没有找到相关内容"),
            )
            .width_md()
            .into_any_element(),
            "EmptyExtra" => Card::new(liora_components::Empty::new().extra(|_, _| {
                Button::new("去添加").primary().into_any_element()
            }))
            .width_md()
            .into_any_element(),
            "StepsBasic" => liora_components::Steps::new()
                .active(1)
                .step(liora_components::StepItem::new("步骤 1"))
                .step(liora_components::StepItem::new("步骤 2"))
                .step(liora_components::StepItem::new("步骤 3"))
                .into_any_element(),
            "StepsDescription" => liora_components::Steps::new()
                .active(1)
                .step(
                    liora_components::StepItem::new("步骤 1")
                        .description("这是一段描述性文字")
                        .icon(IconName::User),
                )
                .step(
                    liora_components::StepItem::new("步骤 2")
                        .description("这是一段描述性文字")
                        .icon(IconName::Settings),
                )
                .step(
                    liora_components::StepItem::new("步骤 3")
                        .description("这是一段描述性文字")
                        .icon(IconName::Check),
                )
                .into_any_element(),
            "StepsStatus" => liora_components::Steps::new()
                .active(1)
                .step(
                    liora_components::StepItem::new("已完成")
                        .status(liora_components::StepStatus::Finish),
                )
                .step(
                    liora_components::StepItem::new("发生错误")
                        .status(liora_components::StepStatus::Error),
                )
                .step(liora_components::StepItem::new("等待中"))
                .into_any_element(),
            "StepsVertical" => liora_components::Steps::new()
                .active(1)
                .direction(liora_components::StepsDirection::Vertical)
                .step(
                    liora_components::StepItem::new("步骤 1")
                        .description("这是一段很长很长很长的描述性文字"),
                )
                .step(liora_components::StepItem::new("步骤 2"))
                .step(liora_components::StepItem::new("步骤 3"))
                .into_any_element(),
            "TimelineBasic" => Card::new(
                liora_components::Timeline::new()
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-01")
                            .content("创建成功"),
                    )
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-02")
                            .content("通过审核"),
                    )
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-03")
                            .content("项目发布"),
                    ),
            )
            .into_any_element(),
            "TimelineCustom" => Card::new(
                liora_components::Timeline::new()
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-01")
                            .content("成功状态")
                            .success(),
                    )
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-02")
                            .content("警告状态")
                            .warning()
                            .hollow(true),
                    )
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-03")
                            .content("错误状态")
                            .danger()
                            .icon(IconName::CircleX),
                    )
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-04")
                            .content("自定义图标")
                            .primary()
                            .icon(IconName::Star),
                    ),
            )
            .into_any_element(),
            "TimelinePlacement" => Card::new(
                liora_components::Timeline::new()
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-01")
                            .content("时间戳在顶部")
                            .placement(liora_components::TimelinePlacement::Top),
                    )
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-02")
                            .content("时间戳在底部")
                            .placement(liora_components::TimelinePlacement::Bottom),
                    ),
            )
            .into_any_element(),
            "TimelineReverse" => Card::new(
                liora_components::Timeline::new()
                    .reverse(true)
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-01")
                            .content("事件 1"),
                    )
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-02")
                            .content("事件 2"),
                    )
                    .item(
                        liora_components::TimelineItem::new()
                            .timestamp("2026-05-03")
                            .content("事件 3"),
                    ),
            )
            .into_any_element(),
            "BreadcrumbBasic" => liora_components::Breadcrumb::new()
                .item(liora_components::BreadcrumbItem::new("首页"))
                .item(liora_components::BreadcrumbItem::new("活动管理"))
                .item(liora_components::BreadcrumbItem::new("活动列表"))
                .item(liora_components::BreadcrumbItem::new("活动详情"))
                .into_any_element(),
            "BreadcrumbIcon" => liora_components::Breadcrumb::new()
                .item(liora_components::BreadcrumbItem::new("首页").icon(IconName::House))
                .item(liora_components::BreadcrumbItem::new("推广管理"))
                .item(liora_components::BreadcrumbItem::new("推广列表"))
                .item(liora_components::BreadcrumbItem::new("推广详情"))
                .into_any_element(),
            "BreadcrumbSeparator" => liora_components::Breadcrumb::new()
                .separator(">")
                .item(liora_components::BreadcrumbItem::new("首页"))
                .item(liora_components::BreadcrumbItem::new("推广管理"))
                .item(liora_components::BreadcrumbItem::new("推广列表"))
                .item(liora_components::BreadcrumbItem::new("推广详情"))
                .into_any_element(),
            "BreadcrumbSeparatorIcon" => liora_components::Breadcrumb::new()
                .separator_icon(IconName::ChevronRight)
                .item(liora_components::BreadcrumbItem::new("首页"))
                .item(liora_components::BreadcrumbItem::new("推广管理"))
                .item(liora_components::BreadcrumbItem::new("推广列表"))
                .item(liora_components::BreadcrumbItem::new("推广详情"))
                .into_any_element(),
            "BreadcrumbClickable" => liora_components::Breadcrumb::new()
                .item(
                    liora_components::BreadcrumbItem::new("首页")
                        .on_click(|_, _| toast_info!("Home Clicked")),
                )
                .item(
                    liora_components::BreadcrumbItem::new("推广管理")
                        .on_click(|_, _| toast_info!("Management Clicked")),
                )
                .item(liora_components::BreadcrumbItem::new("推广列表"))
                .into_any_element(),
            "PageHeaderBasic" => liora_components::PageHeader::new("详情页面")
                .on_back(|_, _| toast_info!("Back Clicked"))
                .into_any_element(),
            "PageHeaderExtra" => liora_components::PageHeader::new("详情页面")
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
                liora_components::PageHeader::new("详情页面")
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
                        liora_components::Row::new()
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
                liora_components::Tooltip::new(Button::new("Top"))
                    .content("Prompt info")
                    .placement(liora_core::Placement::Top)
                    .into_any_element(),
                liora_components::Tooltip::new(Button::new("Bottom"))
                    .content("Prompt info")
                    .placement(liora_core::Placement::Bottom)
                    .into_any_element(),
                liora_components::Tooltip::new(Button::new("Left"))
                    .content("Prompt info")
                    .placement(liora_core::Placement::Left)
                    .into_any_element(),
                liora_components::Tooltip::new(Button::new("Right"))
                    .content("Prompt info")
                    .placement(liora_core::Placement::Right)
                    .into_any_element(),
            ]),
            "TooltipMore" => demo_row(vec![
                liora_components::Tooltip::new(Button::new("Top Start"))
                    .content("Top Start")
                    .placement(liora_core::Placement::TopStart)
                    .into_any_element(),
                liora_components::Tooltip::new(Button::new("Top End"))
                    .content("Top End")
                    .placement(liora_core::Placement::TopEnd)
                    .into_any_element(),
                liora_components::Tooltip::new(Button::new("Bottom Start"))
                    .content("Bottom Start")
                    .placement(liora_core::Placement::BottomStart)
                    .into_any_element(),
                liora_components::Tooltip::new(Button::new("Bottom End"))
                    .content("Bottom End")
                    .placement(liora_core::Placement::BottomEnd)
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
            "ShellBasic" | "ShellFullProduct" => self.docs_shell_full_product_demo(_cx),
            "ShellContentFirst" => self.docs_shell_content_first_demo(_cx),
            "ShellMinimal" => docs_shell_minimal_demo(&_cx.global::<Config>().theme),
            "SidebarBasic" | "SidebarBrand" => self.docs_sidebar_brand_demo(_cx),
            "SidebarScrollable" => self.docs_sidebar_scrollable_demo(_cx),
            "SidebarInspector" => self.docs_sidebar_inspector_demo(_cx),
            "SidebarIconRail" => self.docs_sidebar_icon_rail_demo(_cx),
            "SidebarCustomSlots" => docs_sidebar_custom_slots_demo(&_cx.global::<Config>().theme),
            "Accordion" | "AccordionBasic" => self
                .accordions
                .first()
                .cloned()
                .map(|accordion| Card::new(accordion).no_shadow().into_any_element())
                .unwrap_or_else(|| Paragraph::with_text("Missing Accordion demo").into_any_element()),
            "AccordionMultiple" => self
                .accordions
                .first()
                .cloned()
                .map(|accordion| Card::new(accordion).no_shadow().into_any_element())
                .unwrap_or_else(|| Paragraph::with_text("Missing Accordion demo").into_any_element()),
            "AccordionStates" => self
                .accordions
                .first()
                .cloned()
                .map(|accordion| Card::new(accordion).no_shadow().into_any_element())
                .unwrap_or_else(|| Paragraph::with_text("Missing Accordion demo").into_any_element()),
            "TitleBarBasic" | "TitleBarControls" | "TitleBarControlsRight" => {
                docs_titlebar_controls_right_demo(&_cx.global::<Config>().theme)
            }
            "TitleBarControlsLeft" => docs_titlebar_controls_left_demo(&_cx.global::<Config>().theme),
            "TitleBarCommandCenter" => docs_titlebar_command_demo(&_cx.global::<Config>().theme),
            "TitleBarBorderless" => docs_titlebar_borderless_demo(&_cx.global::<Config>().theme),
            "SplitterBasic" => liora_components::Splitter::new()
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
            "ImagePreview" => liora_components::Image::new(REMOTE_DEMO_IMAGE)
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
                    liora_components::Dialog::new()
                        .title("Tips")
                        .content(|_, _| dialog_body("This is a message from the dialog."))
                        .show(cx);
                })
                .into_any_element(),
            "DialogManualClose" => Button::new("Manual Close Only")
                .warning()
                .on_click(|_, _, cx| {
                    liora_components::Dialog::new()
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
                                    liora_components::Row::new()
                                        .justify(liora_components::RowJustify::End)
                                        .child(Button::new("I understand").primary().on_click(
                                            |_, _, cx| liora_components::Dialog::close(cx),
                                        )),
                                )
                        })
                        .show(cx);
                })
                .into_any_element(),
            "DialogCustomContent" => Button::new("Form-like Content")
                .on_click(|_, _, cx| {
                    liora_components::Dialog::new()
                        .title("Edit profile")
                        .content(|_, _| {
                            Space::new()
                                .vertical()
                                .gap_md()
                                .child(Text::new("Name: Liora User"))
                                .child(Text::new("Role: Designer"))
                                .child(
                                    liora_components::Row::new()
                                        .justify(liora_components::RowJustify::End)
                                        .child(Button::new("Cancel").on_click(|_, _, cx| {
                                            liora_components::Dialog::close(cx)
                                        }))
                                        .child(Button::new("Save").primary().on_click(
                                            |_, _, cx| liora_components::Dialog::close(cx),
                                        )),
                                )
                        })
                        .show(cx);
                })
                .into_any_element(),
            "DrawerPlacements" => demo_row(vec![
                Button::new("Right Drawer")
                    .primary()
                    .on_click(|_, _, cx| docs_drawer("Right Drawer", liora_components::DrawerPlacement::Right).show(cx))
                    .into_any_element(),
                Button::new("Left Drawer")
                    .on_click(|_, _, cx| docs_drawer("Left Drawer", liora_components::DrawerPlacement::Left).show(cx))
                    .into_any_element(),
                Button::new("Top Drawer")
                    .on_click(|_, _, cx| docs_drawer("Top Drawer", liora_components::DrawerPlacement::Top).height_sm().show(cx))
                    .into_any_element(),
                Button::new("Bottom Drawer")
                    .on_click(|_, _, cx| docs_drawer("Bottom Drawer", liora_components::DrawerPlacement::Bottom).height_sm().show(cx))
                    .into_any_element(),
            ]),
            "DrawerSizes" => demo_row(vec![
                Button::new("Wide Drawer")
                    .on_click(|_, _, cx| docs_drawer("Wide Drawer", liora_components::DrawerPlacement::Right).width_lg().show(cx))
                    .into_any_element(),
                Button::new("Tall Top Drawer")
                    .on_click(|_, _, cx| docs_drawer("Tall Top Drawer", liora_components::DrawerPlacement::Top).height_lg().show(cx))
                    .into_any_element(),
            ]),
            "DrawerManualClose" => Button::new("Manual Close Only")
                .warning()
                .on_click(|_, _, cx| {
                    liora_components::Drawer::new()
                        .title("Manual close drawer")
                        .close_on_click_outside(false)
                        .close_on_escape(false)
                        .content(|_, _| {
                            Space::new()
                                .vertical()
                                .gap_lg()
                                .child(Text::new("点击遮罩和按 ESC 都不会关闭。"))
                                .child(Button::new("Close Drawer").primary().on_click(
                                    |_, _, cx| liora_components::Drawer::close(cx),
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
            "DropdownButtonBasic" => docs_dropdown_button_basic().into_any_element(),
            "DropdownButtonSplit" => docs_dropdown_button_split().into_any_element(),
            "DropdownButtonItemStates" => docs_dropdown_button_item_states().into_any_element(),
            "DropdownButtonSizes" => docs_dropdown_button_sizes().into_any_element(),
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
                        "Unsupported Liora demo component: {}",
                        self.component.as_ref()
                    ))
                    .into_any_element()
                },
                |demo| demo.into_any_element(),
            ),
        }
    }
}

fn docs_settings_page_basic(content: &LiveDemoContent) -> AnyElement {
    let Some(auto_save) = content.settings_auto_save.clone() else {
        return Text::new("Settings demo unavailable").into_any_element();
    };
    let Some(theme_select) = content.settings_theme.clone() else {
        return Text::new("Settings demo unavailable").into_any_element();
    };
    let Some(font_size) = content.settings_font_size.clone() else {
        return Text::new("Settings demo unavailable").into_any_element();
    };
    liora_components::SettingsPage::new("Application Settings")
        .description("Settings rows can host Switch, Select, Input, Button, or custom content.")
        .group(
            liora_components::SettingsGroup::new("Editor")
                .description("Editing and save behavior")
                .item(
                    liora_components::SettingsItem::new("Auto save")
                        .description("Save files when focus leaves the editor.")
                        .icon(IconName::Save)
                        .control(auto_save)
                        .primary(),
                )
                .item(
                    liora_components::SettingsItem::new("Font size")
                        .description("Controls editor UI font size.")
                        .icon(IconName::CaseSensitive)
                        .control(font_size),
                ),
        )
        .group(
            liora_components::SettingsGroup::new("Appearance")
                .item(
                    liora_components::SettingsItem::new("Theme mode")
                        .description("Follow system or force a light/dark theme.")
                        .icon(IconName::Palette)
                        .control(theme_select),
                )
                .item(
                    liora_components::SettingsItem::new("Preview")
                        .description("Open a small preview action.")
                        .control(Button::new("Preview").small()),
                ),
        )
        .into_any_element()
}

fn docs_settings_sensitive(content: &LiveDemoContent) -> AnyElement {
    let Some(telemetry) = content.settings_telemetry.clone() else {
        return Text::new("Settings demo unavailable").into_any_element();
    };
    liora_components::SettingsPage::new("Sensitive settings")
        .group(
            liora_components::SettingsGroup::new("Privacy")
                .item(
                    liora_components::SettingsItem::new("Telemetry")
                        .description("Share anonymous product diagnostics.")
                        .icon(IconName::Activity)
                        .control(telemetry),
                )
                .item(
                    liora_components::SettingsItem::new("Delete local cache")
                        .description("Remove generated indexes and temporary package files.")
                        .icon(IconName::Trash2)
                        .danger()
                        .control(Button::new("Delete").danger().small()),
                )
                .item(
                    liora_components::SettingsItem::new("Enterprise policy")
                        .description("Managed by organization policy.")
                        .icon(IconName::Lock)
                        .disabled(true)
                        .control(Button::new("Locked").small().disabled(true)),
                ),
        )
        .into_any_element()
}

fn native_menu_action_catalog() -> impl IntoElement {
    Space::new().vertical().gap_md().children(
        liora_components::NativeMenuAction::catalog()
            .into_iter()
            .map(|action| {
                let info = action.info();
                Space::new()
                    .vertical()
                    .gap_xs()
                    .child(
                        Space::new()
                            .gap_sm()
                            .wrap()
                            .child(Text::new(info.name).bold())
                            .child(LioraTag::new(info.id).info().round(true))
                            .child(if info.handled_by_liora {
                                LioraTag::new("Liora handles").success().round(true)
                            } else {
                                LioraTag::new("App dispatch").warning().round(true)
                            }),
                    )
                    .child(Text::new(info.description).sm().wrap())
                    .child(Text::new(info.effect).xs().wrap())
            }),
    )
}

fn docs_sheet_body(title: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Text::new(title).bold())
        .child(Text::new("Use Drawer::sheet for a short contextual flow."))
        .child(
            Space::new()
                .gap_sm()
                .child(Button::new("Cancel"))
                .child(Button::new("Apply").primary()),
        )
}

fn docs_status_bar_shell() -> AnyElement {
    docs_status_bar_preview(
        liora_components::StatusBar::new()
            .left_item(
                liora_components::StatusBarItem::new("Ready")
                    .success()
                    .icon(IconName::CircleCheck)
                    .pill(),
            )
            .left_item(
                liora_components::StatusBarItem::new("Syncing")
                    .loading(true)
                    .info(),
            )
            .center_item(
                liora_components::StatusBarItem::new("src/main.rs")
                    .primary()
                    .icon(IconName::FileCode),
            )
            .right_item(liora_components::StatusBarItem::new("UTF-8").compact())
            .right_item(liora_components::StatusBarItem::new("Ln 42, Col 7").compact())
            .right_item(liora_components::StatusBarItem::new("v0.1.12").pill()),
    )
}

fn docs_status_bar_tones() -> AnyElement {
    docs_status_bar_preview(
        liora_components::StatusBar::new()
            .left_item(
                liora_components::StatusBarItem::new("Connected")
                    .success()
                    .icon(IconName::Wifi)
                    .detail("42ms")
                    .pill(),
            )
            .left_item(
                liora_components::StatusBarItem::new("Queue")
                    .warning()
                    .icon(IconName::Clock3)
                    .detail("3 jobs"),
            )
            .center_item(
                liora_components::StatusBarItem::new("Preview mode")
                    .primary()
                    .icon(IconName::Monitor),
            )
            .right_item(
                liora_components::StatusBarItem::new("Offline cache")
                    .danger()
                    .icon(IconName::WifiOff)
                    .pill(),
            ),
    )
}

fn docs_status_bar_custom() -> AnyElement {
    docs_status_bar_preview(
        liora_components::StatusBar::new()
            .height(px(38.0))
            .left_item(
                liora_components::StatusBarItem::new("Workspace: Liora").icon(IconName::FolderOpen),
            )
            .center_item(liora_components::StatusBarItem::custom(
                Space::new()
                    .gap_sm()
                    .child(
                        Button::new("Run")
                            .small()
                            .primary()
                            .icon_start(IconName::Play),
                    )
                    .child(Button::new("Build").small().icon_start(IconName::Hammer)),
            ))
            .right_item(
                liora_components::StatusBarItem::new("Native GPUI")
                    .info()
                    .pill(),
            ),
    )
}

fn docs_status_bar_advanced(theme: &Theme) -> AnyElement {
    docs_status_bar_preview(
        liora_components::StatusBar::new()
            .height(px(40.0))
            .borderless()
            .background(theme.primary.base.opacity(0.10))
            .left_item(
                liora_components::StatusBarItem::new("Deploy")
                    .icon(IconName::Rocket)
                    .dot()
                    .min_width(px(108.0))
                    .background(theme.primary.base.opacity(0.16))
                    .text_color(theme.primary.base)
                    .pill()
                    .on_click(|_, _| {}),
            )
            .left_item(liora_components::StatusBarItem::separator())
            .left_item(liora_components::StatusBarItem::new("main").icon(IconName::GitBranch))
            .center_item(liora_components::StatusBarItem::spacer())
            .right_item(
                liora_components::StatusBarItem::new("Updates ready")
                    .info()
                    .icon(IconName::Download)
                    .on_click(|_, _| {})
                    .pill(),
            ),
    )
}

fn docs_status_bar_preview(status_bar: liora_components::StatusBar) -> AnyElement {
    div()
        .w(px(720.0))
        .rounded_lg()
        .border_1()
        .overflow_hidden()
        .child(
            div()
                .min_h(px(112.0))
                .flex()
                .items_center()
                .justify_center()
                .child(Text::new("Application workspace").sm()),
        )
        .child(status_bar)
        .into_any_element()
}

fn docs_combobox_framework_items() -> Vec<liora_components::SearchableListItem> {
    vec![
        liora_components::SearchableListItem::labeled("gpui", "GPUI")
            .description("Native Rust UI runtime"),
        liora_components::SearchableListItem::labeled("liora", "Liora")
            .description("Component SDK"),
        liora_components::SearchableListItem::labeled("iced", "Iced")
            .description("Cross-platform Rust GUI"),
    ]
}

fn docs_combobox_component_items() -> Vec<liora_components::SearchableListItem> {
    vec![
        liora_components::SearchableListItem::labeled("button", "Button").group("Basic"),
        liora_components::SearchableListItem::labeled("input", "Input").group("Basic"),
        liora_components::SearchableListItem::labeled("select-search", "Searchable Select")
            .group("Input"),
        liora_components::SearchableListItem::labeled("sidebar", "Sidebar").group("Shell"),
        liora_components::SearchableListItem::labeled("status-bar", "StatusBar").group("Shell"),
        liora_components::SearchableListItem::labeled("dock-layout", "DockLayout")
            .group("Shell")
            .disabled(true),
    ]
}

fn docs_searchable_list_basic() -> AnyElement {
    liora_components::SearchableList::new(docs_combobox_component_items())
        .selected("select-search")
        .width(px(340.0))
        .into_any_element()
}

fn docs_searchable_list_filtered() -> AnyElement {
    liora_components::SearchableList::new(docs_combobox_component_items())
        .query("shell")
        .selected_values(vec!["status-bar"])
        .width(px(340.0))
        .into_any_element()
}

fn docs_searchable_list_empty() -> AnyElement {
    liora_components::SearchableList::new(docs_combobox_component_items())
        .query("not-found")
        .empty_text("No component found")
        .max_items(2)
        .width(px(340.0))
        .into_any_element()
}

fn layout_divider_demo() -> AnyElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(Text::new("Horizontal (default)"))
        .child(liora_components::Divider::new())
        .child(Text::new("With label"))
        .child(liora_components::Divider::new().label("Center Text"))
        .child(Text::new("Vertical"))
        .child(
            liora_components::Flex::new()
                .row()
                .height_units(60.0)
                .gap_lg()
                .align_center()
                .child(Text::new("Left"))
                .child(liora_components::Divider::new().vertical())
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
            liora_components::Row::new().column(liora_components::Col::new(24).child(grid_box(
                &theme,
                "span 24",
                gpui::blue(),
            ))),
        )
        .child(
            liora_components::Row::new()
                .column(liora_components::Col::new(12).child(grid_box(
                    &theme,
                    "span 12",
                    gpui::red(),
                )))
                .column(liora_components::Col::new(12).child(grid_box(
                    &theme,
                    "span 12",
                    gpui::green(),
                ))),
        )
        .child(
            liora_components::Row::new()
                .column(liora_components::Col::new(8).child(grid_box(
                    &theme,
                    "span 8",
                    gpui::blue(),
                )))
                .column(liora_components::Col::new(8).child(grid_box(
                    &theme,
                    "span 8",
                    gpui::red(),
                )))
                .column(liora_components::Col::new(8).child(grid_box(
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
                .child(liora_components::Divider::new())
                .child(Text::new("Below divider")),
        )
        .child(liora_components::Divider::new().label("Center Label"))
        .child(
            liora_components::Flex::new()
                .row()
                .align_center()
                .gap_lg()
                .height_units(48.0)
                .child(Text::new("Section 1"))
                .child(liora_components::Divider::new().vertical())
                .child(Text::new("Section 2"))
                .child(liora_components::Divider::new().vertical())
                .child(Text::new("Section 3")),
        )
        .into_any_element()
}

fn container_layout_demo() -> AnyElement {
    liora_components::Flex::new()
        .height_units(300.0)
        .w_full()
        .border()
        .child(
            liora_components::Container::new()
                .header(Title::new("Header").h5())
                .aside(
                    liora_components::Flex::new()
                        .padding_md()
                        .child(Text::new("Aside Sidebar")),
                )
                .footer(Text::new("Footer"))
                .child(
                    liora_components::Flex::new()
                        .padding_md()
                        .child(Text::new("Main Content Area")),
                ),
        )
        .into_any_element()
}

fn grid_box(theme: &liora_theme::Theme, text: &str, color: gpui::Hsla) -> impl IntoElement {
    liora_components::Flex::new()
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

        let mut table = liora_components::Table::new(table_sortable_columns(&theme))
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

    fn docs_shell_full_product_demo(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        Card::new(
            Flex::new().height_units(520.0).overflow_hidden().child(
                Shell::new(
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .child(Title::new("Dashboard workspace").h4())
                            .child(Text::new("The main region scrolls independently."))
                            .child(Button::new("Primary action").primary()),
                    )
                    .no_shadow(),
                )
                .id("docs-shell-full-product-live")
                .mode(WindowFrameMode::Custom)
                .titlebar(
                    TitleBar::new()
                        .title("Liora Product")
                        .subtitle("Full shell composition")
                        .icon(Icon::new(IconName::Sparkles).size_units(16.0))
                        .height_units(56.0)
                        .background(theme.neutral.card)
                        .border_color(theme.neutral.border)
                        .title_color(theme.neutral.text_1)
                        .subtitle_color(theme.neutral.text_3)
                        .window_controls(true)
                        .action(Button::new("Sync").small())
                        .action(Button::new("Publish").small().primary()),
                )
                .header(
                    Space::new()
                        .gap_sm()
                        .child(Text::new("Production").bold())
                        .child(Text::new("All systems nominal").sm()),
                )
                .header_height_units(54.0)
                .header_background(theme.neutral.card)
                .header_border_color(theme.primary.light_7)
                .sidebar(
                    Sidebar::new()
                        .id("docs-shell-left-sidebar-live")
                        .expanded_width_units(238.0)
                        .brand("Workspace")
                        .brand_subtitle("Product navigation")
                        .logo(Icon::new(IconName::Blocks).size_units(18.0))
                        .background(theme.neutral.card)
                        .border_color(theme.neutral.border)
                        .header_padding_units(12.0)
                        .content_padding_units(6.0)
                        .footer_padding_units(10.0)
                        .gap_units(8.0)
                        .scrollable()
                        .child(self.menus[0].clone())
                        .footer(Text::new("v0.1 shell").xs()),
                )
                .right_sidebar(
                    Sidebar::new()
                        .id("docs-shell-right-sidebar-live")
                        .right()
                        .expanded_width_units(210.0)
                        .brand("Inspector")
                        .brand_subtitle("Context panel")
                        .logo(Icon::new(IconName::PanelRight).size_units(18.0))
                        .background(theme.neutral.popover)
                        .border_color(theme.neutral.border)
                        .header_padding_units(10.0)
                        .content_padding_units(8.0)
                        .footer_padding_units(10.0)
                        .gap_units(8.0)
                        .scrollable()
                        .child(self.menus[1].clone())
                        .content(Text::new("Selection properties render here.").sm())
                        .footer(Button::new("Open details").small()),
                )
                .main(Text::new("Additional dashboard content can be appended."))
                .footer(Text::new("Workspace ready").xs())
                .footer_height_units(42.0)
                .footer_background(theme.neutral.card)
                .footer_border_color(theme.primary.light_7)
                .main_scroll()
                .main_padding_units(16.0)
                .body_background(theme.neutral.body)
                .main_background(theme.neutral.card)
                .main_rounded_units(18.0)
                .overlay(Text::new("Overlay slot").xs())
                .overlay_position(liora_components::ShellOverlayPosition::BottomRight)
                .overlay_inset_units(18.0),
            ),
        )
        .no_shadow()
        .into_any_element()
    }

    fn docs_shell_content_first_demo(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        Card::new(
            Flex::new().height_units(420.0).overflow_hidden().child(
                Shell::new(
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .child(Title::new("Document canvas").h4())
                            .child(Text::new("System frame mode keeps platform decorations."))
                            .child(Button::new("Edit page").primary()),
                    )
                    .no_shadow(),
                )
                .id("docs-shell-content-first-live")
                .mode(WindowFrameMode::System)
                .header(
                    Space::new()
                        .gap_sm()
                        .child(Text::new("Docs workspace").bold())
                        .child(Text::new("System frame + app shell").sm()),
                )
                .header_height_units(58.0)
                .header_background(theme.primary.light_9)
                .header_border_color(theme.primary.light_7)
                .sidebar(
                    Sidebar::new()
                        .id("docs-shell-content-sidebar-live")
                        .expanded_width_units(210.0)
                        .brand("Docs")
                        .brand_subtitle("Compact nav")
                        .logo(Icon::new(IconName::BookOpen).size_units(18.0))
                        .background(theme.primary.light_9)
                        .border_color(theme.primary.light_7)
                        .header_padding_units(12.0)
                        .content_padding_units(6.0)
                        .footer_padding_units(10.0)
                        .gap_units(8.0)
                        .scrollable()
                        .child(self.menus.first().cloned().unwrap_or_else(|| {
                            cx.new(|_| docs_compact_menu("docs-shell-compact-menu-fallback"))
                        })),
                )
                .right_sidebar(
                    Sidebar::new()
                        .id("docs-shell-context-note-live")
                        .right()
                        .expanded_width_units(190.0)
                        .brand("Context")
                        .brand_subtitle("Page metadata")
                        .logo(Icon::new(IconName::Info).size_units(18.0))
                        .content(Text::new("Use this region for metadata or help.")),
                )
                .footer(Text::new("Autosaved just now").xs())
                .footer_height_units(38.0)
                .footer_background(theme.primary.light_9)
                .footer_border_color(theme.primary.light_7)
                .main_padding_units(18.0)
                .main_scroll()
                .body_gap_units(10.0)
                .body_background(theme.neutral.body)
                .main_background(theme.neutral.card)
                .main_rounded_units(18.0),
            ),
        )
        .no_shadow()
        .into_any_element()
    }

    fn docs_sidebar_brand_demo(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        Card::new(
            Flex::new()
                .height_units(392.0)
                .row()
                .overflow_hidden()
                .child(
                    Sidebar::new()
                        .id("docs-sidebar-brand-live")
                        .left()
                        .expanded_width_units(286.0)
                        .min_width_units(220.0)
                        .max_width_units(360.0)
                        .resizable()
                        .header_padding_units(14.0)
                        .content_padding_units(8.0)
                        .footer_padding_units(12.0)
                        .gap_units(8.0)
                        .rounded_units(16.0)
                        .background(theme.neutral.card)
                        .border_color(theme.neutral.border)
                        .brand("Liora Workspace")
                        .brand_subtitle("Native GPUI shell")
                        .logo(
                            Icon::new(IconName::Sparkles)
                                .size_units(20.0)
                                .color(theme.primary.base),
                        )
                        .brand_action(Button::new("+").small().primary())
                        .scrollable()
                        .child(self.menus.first().cloned().unwrap_or_else(|| {
                            cx.new(|_| docs_workspace_menu("docs-sidebar-brand-menu-fallback"))
                        }))
                        .footer(
                            Space::new()
                                .gap_sm()
                                .child(Button::new("New").small())
                                .child(Button::new("Settings").small()),
                        ),
                ),
        )
        .no_shadow()
        .into_any_element()
    }

    fn docs_sidebar_scrollable_demo(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        Card::new(
            Flex::new()
                .height_units(320.0)
                .row()
                .overflow_hidden()
                .child(
                    Sidebar::new()
                        .id("docs-sidebar-long-scroll-live")
                        .expanded_width_units(316.0)
                        .header_padding_units(16.0)
                        .content_padding_units(6.0)
                        .footer_padding_units(10.0)
                        .gap_units(10.0)
                        .rounded_units(18.0)
                        .background(theme.neutral.card)
                        .border_color(theme.neutral.border)
                        .header(
                            Space::new()
                                .gap_sm()
                                .child(
                                    Icon::new(IconName::Sparkles)
                                        .size_units(22.0)
                                        .color(theme.primary.base),
                                )
                                .child(Text::new("Liora Product").bold())
                                .child(Button::new("Pro").small().primary()),
                        )
                        .scrollable()
                        .child(self.menus.first().cloned().unwrap_or_else(|| {
                            cx.new(|_| docs_long_workspace_menu("docs-sidebar-long-menu-fallback"))
                        }))
                        .footer(
                            Space::new()
                                .gap_sm()
                                .child(Text::new("Pinned footer").xs())
                                .child(Button::new("Upgrade").small().primary()),
                        ),
                ),
        )
        .no_shadow()
        .into_any_element()
    }

    fn docs_sidebar_inspector_demo(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        Card::new(
            Flex::new()
                .height_units(340.0)
                .row()
                .justify_end()
                .overflow_hidden()
                .child(
                    Sidebar::new()
                        .id("docs-sidebar-inspector-live")
                        .right()
                        .expanded_width_units(268.0)
                        .brand("Inspector")
                        .brand_subtitle("Selection details")
                        .logo(Icon::new(IconName::PanelRight).size_units(20.0))
                        .brand_action(Button::new("Pin").small())
                        .background(theme.neutral.popover)
                        .border_color(theme.neutral.border)
                        .rounded_units(14.0)
                        .header_padding_units(12.0)
                        .content_padding_units(10.0)
                        .footer_padding_units(12.0)
                        .gap_units(6.0)
                        .scrollable()
                        .child(self.menus.first().cloned().unwrap_or_else(|| {
                            cx.new(|_| docs_inspector_menu("docs-sidebar-inspector-menu-fallback"))
                        }))
                        .content(
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Text::new("Width 268 px"))
                                .child(Text::new("Mode Full"))
                                .child(Text::new("Pinned Yes")),
                        )
                        .footer(Text::new("Updates when selection changes.").xs()),
                ),
        )
        .no_shadow()
        .into_any_element()
    }

    fn docs_sidebar_icon_rail_demo(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        Card::new(
            Flex::new()
                .height_units(316.0)
                .row()
                .overflow_hidden()
                .child(
                    Sidebar::new()
                        .id("docs-sidebar-icon-rail-live")
                        .collapse_mode(liora_components::SidebarCollapseMode::IconsOnly)
                        .collapsed_width_units(72.0)
                        .expanded_width_units(260.0)
                        .header_padding_units(10.0)
                        .content_padding_units(8.0)
                        .footer_padding_units(10.0)
                        .gap_units(8.0)
                        .background(theme.primary.light_9)
                        .border_color(theme.primary.light_7)
                        .rounded_units(18.0)
                        .logo(
                            Icon::new(IconName::Sparkles)
                                .size_units(20.0)
                                .color(theme.primary.base),
                        )
                        .scrollable()
                        .child(self.menus.first().cloned().unwrap_or_else(|| {
                            cx.new(|_| docs_icon_rail_menu("docs-sidebar-icon-menu-fallback"))
                        }))
                        .footer(Icon::new(IconName::Settings).size_units(18.0)),
                ),
        )
        .no_shadow()
        .into_any_element()
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

fn icon_labeled(icon: liora_icons::Icon, label: &'static str) -> impl IntoElement {
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
                liora_icons::Icon::new(icon)
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
            liora_icons::Icon::new(IconName::Star)
                .size_lg()
                .color(theme.primary.base),
            "Primary",
        )
        .into_any_element(),
        icon_labeled(
            liora_icons::Icon::new(IconName::Star)
                .size_lg()
                .color(theme.success.base),
            "Success",
        )
        .into_any_element(),
        icon_labeled(
            liora_icons::Icon::new(IconName::Star)
                .size_lg()
                .color(theme.warning.base),
            "Warning",
        )
        .into_any_element(),
        icon_labeled(
            liora_icons::Icon::new(IconName::Star)
                .size_lg()
                .color(theme.danger.base),
            "Danger",
        )
        .into_any_element(),
    ])
}

fn icon_sizes_demo() -> AnyElement {
    demo_row(vec![
        icon_labeled(liora_icons::Icon::new(IconName::House).size_xs(), "12px").into_any_element(),
        icon_labeled(liora_icons::Icon::new(IconName::House).size_md(), "18px").into_any_element(),
        icon_labeled(liora_icons::Icon::new(IconName::House).size_lg(), "24px").into_any_element(),
        icon_labeled(liora_icons::Icon::new(IconName::House).size_xl(), "32px").into_any_element(),
    ])
}

fn image_basic_demo() -> AnyElement {
    let local = local_demo_image();
    demo_row(vec![
        liora_components::Image::new(REMOTE_DEMO_IMAGE)
            .thumbnail()
            .cover()
            .into_any_element(),
        liora_components::Image::new(local.clone())
            .thumbnail()
            .cover()
            .into_any_element(),
        liora_components::Image::new(local)
            .thumbnail()
            .contain()
            .into_any_element(),
    ])
}

fn image_fit_demo() -> AnyElement {
    let local = local_demo_image();
    demo_row(
        [
            ("Fill", liora_components::ImageFit::Fill),
            ("Contain", liora_components::ImageFit::Contain),
            ("Cover", liora_components::ImageFit::Cover),
            ("ScaleDown", liora_components::ImageFit::ScaleDown),
        ]
        .into_iter()
        .map(|(label, fit)| {
            Card::new(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(
                        liora_components::Image::new(local.clone())
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
            liora_components::Image::new(local.clone())
                .square_lg()
                .cover()
                .round(),
            "Circle",
        ),
        labeled_image(
            liora_components::Image::new(local.clone())
                .thumbnail_sm()
                .cover()
                .round_options(liora_components::ImageRoundOptions::without_square_crop()),
            "Round bounds",
        ),
        labeled_image(
            liora_components::Image::new(local.clone())
                .square_lg()
                .cover()
                .round_sleeve(),
            "Ring sleeve",
        ),
        labeled_image(
            liora_components::Image::new(local)
                .thumbnail()
                .cover()
                .radius(liora_components::ImageRadius::Large)
                .shadow(true),
            "Shadow",
        ),
        labeled_image(
            liora_components::Image::new("liora://missing-image.png")
                .thumbnail()
                .alt("加载失败"),
            "Fallback",
        ),
        labeled_image(liora_components::Image::empty().thumbnail(), "Empty"),
    ])
}

fn labeled_image(image: liora_components::Image, label: &'static str) -> AnyElement {
    Space::new()
        .vertical()
        .align_center()
        .gap_sm()
        .child(image)
        .child(Text::new(label).nowrap())
        .into_any_element()
}

fn descriptions_basic_demo() -> AnyElement {
    liora_components::Descriptions::new()
        .title("用户信息")
        .item("用户名", "kooriookami", 1)
        .item("手机号", "18100000000", 1)
        .item("居住地", "苏州市", 1)
        .item("备注", Text::new("学校").bg(gpui::blue().opacity(0.1)), 1)
        .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2)
        .into_any_element()
}

fn descriptions_border_demo() -> AnyElement {
    liora_components::Descriptions::new()
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
    liora_components::Descriptions::new()
        .title("垂直布局")
        .border(true)
        .direction(liora_components::DescriptionsDirection::Vertical)
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

fn table_basic_columns() -> Vec<liora_components::TableColumn> {
    vec![
        liora_components::TableColumn::new("date", "日期").width_sm(),
        liora_components::TableColumn::new("name", "姓名").width_sm(),
        liora_components::TableColumn::new("address", "地址").min_width_lg(),
        liora_components::TableColumn::new("status", "状态")
            .width_sm()
            .align(liora_components::TableAlign::Center),
        liora_components::TableColumn::new("action", "操作")
            .width_sm()
            .align(liora_components::TableAlign::Right),
    ]
}

fn table_sortable_columns(theme: &liora_theme::Theme) -> Vec<liora_components::TableColumn> {
    vec![
        liora_components::TableColumn::new("date", "日期")
            .width_sm()
            .sortable(),
        liora_components::TableColumn::new("name", "姓名")
            .header(
                Text::new("客户")
                    .bold()
                    .text_color(theme.primary.base)
                    .nowrap(),
            )
            .width_sm()
            .sortable(),
        liora_components::TableColumn::new("address", "地址").min_width_lg(),
        liora_components::TableColumn::new("status", "状态")
            .width_sm()
            .align(liora_components::TableAlign::Center)
            .sortable(),
        liora_components::TableColumn::new("action", "操作")
            .width_sm()
            .align(liora_components::TableAlign::Right),
    ]
}

fn table_basic_table() -> liora_components::Table {
    liora_components::Table::new(table_basic_columns()).rows(table_basic_rows())
}

fn table_basic_demo() -> AnyElement {
    table_basic_table().into_any_element()
}

fn table_fixed_header_demo() -> AnyElement {
    liora_components::Table::new(table_basic_columns())
        .rows(table_long_rows())
        .stripe(true)
        .fixed_header(true)
        .height_md()
        .into_any_element()
}

fn table_basic_rows() -> Vec<liora_components::TableRow> {
    table_records().into_iter().map(table_record_row).collect()
}

fn table_sorted_rows(
    sort_key: Option<&SharedString>,
    sort_order: Option<liora_components::TableSortOrder>,
) -> Vec<liora_components::TableRow> {
    let mut records = table_records();
    if let (Some(key), Some(order)) = (sort_key, sort_order) {
        records.sort_by(|a, b| table_field_value(a, key).cmp(table_field_value(b, key)));
        if order == liora_components::TableSortOrder::Descending {
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

fn table_long_rows() -> Vec<liora_components::TableRow> {
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

fn table_record_row(record: DocsOrderRecord) -> liora_components::TableRow {
    table_row(record.date, record.name, record.address, record.status)
}

fn table_row(
    date: &'static str,
    name: &'static str,
    address: &'static str,
    status: &'static str,
) -> liora_components::TableRow {
    liora_components::TableRow::new()
        .cell("date", date)
        .cell("name", name)
        .cell("address", address)
        .cell("status", table_status_tag(status))
        .cell("action", Button::new("查看").primary().small())
}

fn table_status_tag(status: &'static str) -> liora_components::Tag {
    let tag = liora_components::Tag::new(status).round(true).small();
    match status {
        "已完成" => tag.success(),
        "进行中" => tag.info(),
        _ => tag.warning(),
    }
}

fn virtualized_table_demo(
    sortable: bool,
    sort_key: Option<SharedString>,
    sort_order: Option<liora_components::TableSortOrder>,
) -> VirtualizedTable {
    let reverse = sort_order == Some(liora_components::TableSortOrder::Descending);
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

fn virtualized_table_columns(sortable: bool) -> Vec<liora_components::TableColumn> {
    let columns = vec![
        liora_components::TableColumn::new("date", "日期").width_sm(),
        liora_components::TableColumn::new("name", "客户").width_sm(),
        liora_components::TableColumn::new("region", "区域").width_sm(),
        liora_components::TableColumn::new("amount", "金额")
            .width_sm()
            .align(liora_components::TableAlign::Right),
        liora_components::TableColumn::new("status", "状态")
            .width_sm()
            .align(liora_components::TableAlign::Center),
        liora_components::TableColumn::new("action", "操作")
            .width_sm()
            .align(liora_components::TableAlign::Right),
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
    println!("Hello Liora");
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
            .child(LioraTag::new(if index % 2 == 0 { "even" } else { "odd" }).info())
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

fn docs_date(day: u32) -> CalendarDate {
    CalendarDate::new(2026, 6, day).expect("docs demo date should be valid")
}

fn docs_calendar_events() -> impl IntoElement {
    Calendar::new(2026, 6).selected(docs_date(16)).events([
        CalendarEvent::new(docs_date(3), "Design review").color(rgb(0x2563eb).into()),
        CalendarEvent::new(docs_date(18), "Docs polish").color(rgb(0xf97316).into()),
    ])
}

fn docs_calendar_range() -> impl IntoElement {
    Calendar::new(2026, 6)
        .range(docs_date(10), docs_date(18))
        .disabled_dates([docs_date(6), docs_date(7), docs_date(21)])
}

fn docs_carousel_basic() -> impl IntoElement {
    Carousel::new(vec![
        CarouselItem::new("Native Rust UI")
            .description("Pure GPUI rendering with Liora components.")
            .accent(rgb(0x2563eb).into()),
        CarouselItem::new("Charts & Dashboards")
            .description("Native chart primitives for desktop apps.")
            .accent(rgb(0x16a34a).into()),
    ])
    .height(px(240.0))
}

fn docs_carousel_autoplay() -> impl IntoElement {
    Carousel::new(vec![
        CarouselItem::new("Preview").accent(rgb(0x2563eb).into()),
        CarouselItem::new("Release").accent(rgb(0x9333ea).into()),
    ])
    .autoplay(true)
    .interval_ms(1800)
    .pause_on_hover(true)
    .indicator_position(CarouselIndicatorPosition::Outside)
    .height(px(220.0))
}

fn docs_carousel_custom() -> impl IntoElement {
    Carousel::new(vec![
        CarouselItem::new("Custom content")
            .description("The slot below is another Liora element.")
            .accent(rgb(0xf97316).into())
            .content(
                Flex::new()
                    .row()
                    .center()
                    .rounded_pill()
                    .bg(rgb(0xfff7ed).into())
                    .padding_x_units(12.0)
                    .padding_y_px(4.0)
                    .text_sm()
                    .text_color(rgb(0x9a3412).into())
                    .child("Composable slot"),
            ),
    ])
    .show_arrows(false)
    .hide_indicators()
}

fn docs_code_block_basic() -> impl IntoElement {
    LioraCodeBlock::new("cargo run -p liora-docs")
        .shell()
        .copyable(true)
        .selectable(true)
}

fn docs_code_block_language() -> impl IntoElement {
    LioraCodeBlock::new(r#"fn main() { println!("Liora"); }"#)
        .language("rust")
        .copyable(true)
}

fn docs_code_block_theme() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(
            LioraCodeBlock::new("cargo run -p liora-docs")
                .shell()
                .auto_theme(),
        )
        .child(
            LioraCodeBlock::new(r#"fn main() { println!("Liora"); }"#)
                .rust()
                .github_dark_theme(),
        )
        .child(
            LioraCodeBlock::new("[package]\nname = \"liora\"")
                .toml()
                .highlighter(CodeHighlighter::Syntect)
                .theme(CodeTheme::Nord)
                .selectable(true),
        )
}

fn docs_code_block_inline() -> impl IntoElement {
    Flex::new()
        .row()
        .wrap()
        .align_center()
        .gap_sm()
        .child(Text::new("Run").sm())
        .child(LioraCodeBlock::new("cargo check").shell().inline())
        .child(Text::new("before publishing docs snippets.").sm())
}

fn docs_qr_code_decode_upload() -> Upload {
    Upload::new()
        .accept(".png,.jpg,.jpeg,.gif,.bmp,.webp,image/*")
        .limit(1)
        .button_text("选择二维码图片")
        .tip("选择本地图片后自动调用 QrCode::decode_file")
        .width_lg()
        .files([UploadFile::new("qr-sample", "release-qr.png")
            .status(UploadStatus::Ready)
            .description("等待选择本地二维码图片")])
}

fn docs_qr_code_decode_demo(upload: Entity<Upload>) -> impl IntoElement {
    Space::new().vertical().gap_md().child(upload).child(
        Flex::new()
            .column()
            .gap_px(4.0)
            .padding_md()
            .rounded_units(12.0)
            .border()
            .child(Text::new("识别结果").sm().bold())
            .child(Text::new("选择图片后会显示二维码内容、纠错等级和版本。").sm()),
    )
}

fn docs_input_tag_basic(
    cx: &mut Context<liora_components::InputTag>,
) -> liora_components::InputTag {
    liora_components::InputTag::new(vec!["Rust", "GPUI", "Liora"], cx).placeholder("Add skill")
}

fn docs_input_tag_limited(
    cx: &mut Context<liora_components::InputTag>,
) -> liora_components::InputTag {
    liora_components::InputTag::new(vec!["Design", "Docs"], cx)
        .placeholder("Max 4")
        .max_tags(4)
}

fn docs_input_tag_duplicates(
    cx: &mut Context<liora_components::InputTag>,
) -> liora_components::InputTag {
    liora_components::InputTag::new(vec!["blue", "blue"], cx)
        .allow_duplicates(true)
        .placeholder("Duplicates allowed")
}

fn docs_mention_people(cx: &mut Context<liora_components::Mention>) -> liora_components::Mention {
    liora_components::Mention::new(
        vec![
            liora_components::MentionItem::new("alice", "Alice Chen").description("Design systems"),
            liora_components::MentionItem::new("bob", "Bob Smith")
                .description("Release engineering"),
            liora_components::MentionItem::new("carol", "Carol Li")
                .description("Docs and examples"),
            liora_components::MentionItem::new("dora", "Dora Wang")
                .description("Quality assurance"),
        ],
        cx,
    )
    .placeholder("Type @ to mention a teammate")
    .on_select(|item, _, _| toast_info!("Mention {}", item.label))
}

fn docs_mention_issues(cx: &mut Context<liora_components::Mention>) -> liora_components::Mention {
    liora_components::Mention::new(
        vec![
            liora_components::MentionItem::new("128", "#128 Improve chart hover"),
            liora_components::MentionItem::new("142", "#142 Package smoke scripts"),
            liora_components::MentionItem::new("176", "#176 Add TreeSelect"),
            liora_components::MentionItem::new("201", "#201 Polish docs navigation"),
        ],
        cx,
    )
    .trigger('#')
    .placeholder("Type # to reference an issue")
    .max_suggestions(4)
}

fn docs_mention_disabled(cx: &mut Context<liora_components::Mention>) -> liora_components::Mention {
    liora_components::Mention::new(
        vec![liora_components::MentionItem::new("alice", "Alice Chen")],
        cx,
    )
    .placeholder("Disabled mention")
    .disabled(true)
}

fn docs_tree_select_nodes() -> Vec<TreeSelectNode> {
    vec![
        TreeSelectNode::new("guide", "Guide")
            .child(TreeSelectNode::new("overview", "Overview"))
            .child(TreeSelectNode::new("quick_start", "Quick Start")),
        TreeSelectNode::new("components", "Components")
            .child(TreeSelectNode::new("button", "Button"))
            .child(TreeSelectNode::new("internal", "Internal Draft")),
        TreeSelectNode::new("charts", "Charts")
            .child(TreeSelectNode::new("line_chart", "LineChart"))
            .child(TreeSelectNode::new("ring_chart", "RingChart")),
    ]
}

fn docs_tree_select_single(cx: &mut Context<TreeSelect>) -> TreeSelect {
    TreeSelect::new(docs_tree_select_nodes(), cx)
        .placeholder("Choose documentation page")
        .selected(["quick_start"])
}

fn docs_tree_select_multiple(cx: &mut Context<TreeSelect>) -> TreeSelect {
    TreeSelect::new(docs_tree_select_nodes(), cx)
        .multiple(true)
        .selected(["button", "line_chart"])
        .disabled_keys(["internal"])
}

fn docs_tree_select_filterable(cx: &mut Context<TreeSelect>) -> TreeSelect {
    TreeSelect::new(docs_tree_select_nodes(), cx)
        .filterable(true)
        .placeholder("Search docs tree")
}

fn docs_watermark_surface(text: &'static str) -> impl IntoElement {
    Flex::new()
        .height_units(160.0)
        .padding_md()
        .rounded_units(12.0)
        .border()
        .child(Text::new(text))
}

fn docs_watermark_cover() -> impl IntoElement {
    Watermark::new(
        docs_watermark_surface("Internal report"),
        "LIORA CONFIDENTIAL",
    )
    .density(3, 4)
    .opacity(0.18)
}

fn docs_watermark_header() -> impl IntoElement {
    Watermark::new(docs_watermark_surface("Preview asset"), "PREVIEW")
        .header()
        .density(1, 3)
        .color(rgb(0x2563eb).into())
        .opacity(0.24)
}

fn docs_watermark_custom() -> impl IntoElement {
    Watermark::new(docs_watermark_surface("Design draft"), "DRAFT")
        .density(4, 5)
        .gap(px(72.0), px(48.0))
        .color(rgb(0xf97316).into())
        .opacity(0.22)
        .rotate(-32.0)
}

fn docs_timer_result() -> impl IntoElement {
    let timer = Timer::count_down(
        std::time::Duration::from_secs(300),
        std::time::Duration::from_secs(84),
    );
    let snapshot = timer.snapshot();

    Flex::new()
        .column()
        .gap_md()
        .child(
            timer
                .title("Deploy window")
                .display_unit(TimerUnit::Minutes),
        )
        .child(
            Flex::new()
                .row()
                .wrap()
                .gap_sm()
                .child(LioraTag::new(format!(
                    "elapsed: {:.0}s",
                    snapshot.elapsed_as(TimerUnit::Seconds)
                )))
                .child(LioraTag::new(format!(
                    "remaining: {:.1}m",
                    snapshot
                        .remaining_as(TimerUnit::Minutes)
                        .unwrap_or_default()
                )))
                .child(LioraTag::new(format!("finished: {}", snapshot.finished))),
        )
}

fn docs_tour_launcher(
    label: &'static str,
    description: &'static str,
    starter: fn(&mut App),
) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Text::new(description).sm())
        .child(Button::new(label).primary().on_click(move |_, _, cx| {
            starter(cx);
        }))
}

fn docs_middle_tour(cx: &mut App) {
    Tour::new(vec![
        TourStep::new("第一步", "介绍入口。"),
        TourStep::new("第二步", "当前步骤前后按钮都可用。"),
        TourStep::new("第三步", "结束引导。"),
    ])
    .active_index(1)
    .previous_text("上一步")
    .next_text("下一步")
    .finish_text("完成")
    .show(cx);
}

fn docs_no_mask_tour(cx: &mut App) {
    Tour::new(vec![
        TourStep::new("透明遮罩", "Tour 仍然在顶层浮动，但不显示半透明遮罩。")
            .placement(TourPlacement::Center),
    ])
    .show_mask(false)
    .close_on_click_outside(true)
    .finish_text("完成")
    .show(cx);
}

fn docs_close_policy_tour(cx: &mut App) {
    Tour::new(vec![
        TourStep::new("确认引导", "ESC 和外部点击都不会关闭。"),
        TourStep::new("显式完成", "用户需要点击关闭图标或完成按钮。"),
    ])
    .close_on_escape(false)
    .close_on_click_outside(false)
    .finish_text("我已了解")
    .show(cx);
}

fn docs_tray_residency() -> impl IntoElement {
    Flex::new()
        .column()
        .gap_md()
        .child(docs_status_row("驻留模式", "enabled", rgb(0x16a34a).into()))
        .child(docs_status_row("托盘可见", "true", rgb(0x2563eb).into()))
        .child(docs_status_row(
            "关闭窗口",
            "hide-to-tray",
            rgb(0xf97316).into(),
        ))
}

fn docs_tray_install() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Text::new("TrayConfig::new(\"liora-gallery\")").sm().bold())
        .child(Text::new("tooltip + app-owned icon + default_liora_tray_menu()").sm())
        .child(docs_tray_menu_preview(default_liora_tray_menu()))
}

fn docs_tray_dynamic_icon() -> impl IntoElement {
    Flex::new()
        .row()
        .wrap()
        .gap_sm()
        .child(docs_icon_chip("default", rgb(0x2563eb).into()))
        .child(docs_icon_chip("syncing", rgb(0xf97316).into()))
        .child(docs_icon_chip("error", rgb(0xdc2626).into()))
        .child(LioraTag::new(TrayCommand::SetIcon("syncing".into()).id()).round(true))
}

fn docs_tray_checkbox() -> impl IntoElement {
    let items = vec![
        TrayMenuItemSpec::check(
            "启动时自动显示主窗口",
            TrayCommand::Custom("auto-show-window".into()),
            true,
        ),
        TrayMenuItemSpec::check(
            "静音通知",
            TrayCommand::Custom("mute-notifications".into()),
            false,
        ),
    ];
    docs_tray_menu_preview(items)
}

fn docs_tray_close_confirm() -> impl IntoElement {
    Flex::new()
        .column()
        .gap_md()
        .child(docs_close_action(TrayCloseAction::ExitProcess))
        .child(docs_close_action(TrayCloseAction::HideToTray))
        .child(docs_close_action(TrayCloseAction::Ask))
}

fn docs_tray_nested_menu() -> impl IntoElement {
    let items = vec![TrayMenuItemSpec::submenu(
        "项目",
        vec![
            TrayMenuItemSpec::action("打开最近项目", TrayCommand::Custom("open-recent".into())),
            TrayMenuItemSpec::submenu(
                "工作区",
                vec![TrayMenuItemSpec::submenu(
                    "生产环境",
                    vec![
                        TrayMenuItemSpec::action(
                            "打开仪表盘",
                            TrayCommand::Custom("workspace-prod-dashboard".into()),
                        ),
                        TrayMenuItemSpec::action(
                            "打开日志",
                            TrayCommand::Custom("workspace-prod-logs".into()),
                        ),
                    ],
                )],
            ),
        ],
    )];
    docs_tray_menu_preview(items)
}

fn docs_status_row(label: &'static str, value: &'static str, color: Hsla) -> impl IntoElement {
    Flex::new()
        .row()
        .align_center()
        .justify_between()
        .gap_md()
        .padding_md()
        .rounded_units(12.0)
        .border()
        .child(Text::new(label).sm())
        .child(
            Flex::new()
                .padding_x_px(10.0)
                .padding_y_px(3.0)
                .rounded_pill()
                .bg(color)
                .text_color(gpui::white())
                .text_xs()
                .child(value),
        )
}

fn docs_icon_chip(label: &'static str, color: Hsla) -> impl IntoElement {
    Flex::new()
        .row()
        .align_center()
        .gap_sm()
        .padding_md()
        .rounded_pill()
        .border()
        .child(Icon::new(IconName::Circle).size_units(14.0).color(color))
        .child(Text::new(label).sm())
}

fn docs_close_action(action: TrayCloseAction) -> impl IntoElement {
    let (label, value, color) = match action {
        TrayCloseAction::Ask => ("询问用户", "Ask", rgb(0x64748b).into()),
        TrayCloseAction::ExitProcess => ("关闭进程", "ExitProcess", rgb(0xdc2626).into()),
        TrayCloseAction::HideToTray => ("隐藏到托盘", "HideToTray", rgb(0x16a34a).into()),
    };
    docs_status_row(label, value, color)
}

fn docs_tray_menu_preview(items: Vec<TrayMenuItemSpec>) -> impl IntoElement {
    Flex::new()
        .column()
        .gap_sm()
        .padding_md()
        .rounded_units(14.0)
        .border()
        .children(
            items
                .iter()
                .flat_map(|item| docs_tray_menu_rows(item, 0))
                .collect::<Vec<_>>(),
        )
}

fn docs_tray_menu_rows(item: &TrayMenuItemSpec, depth: usize) -> Vec<AnyElement> {
    match item {
        TrayMenuItemSpec::Action {
            label,
            command,
            enabled,
        } => vec![docs_tray_menu_row(
            depth,
            label,
            command.id(),
            *enabled,
            IconName::MousePointerClick,
        )],
        TrayMenuItemSpec::Check {
            label,
            command,
            checked,
            enabled,
        } => vec![docs_tray_menu_row(
            depth,
            label,
            format!("{} · checked={checked}", command.id()),
            *enabled,
            IconName::Check,
        )],
        TrayMenuItemSpec::Submenu {
            label,
            enabled,
            children,
        } => {
            let mut rows = vec![docs_tray_menu_row(
                depth,
                label,
                "submenu".to_string(),
                *enabled,
                IconName::ChevronRight,
            )];
            rows.extend(
                children
                    .iter()
                    .flat_map(|child| docs_tray_menu_rows(child, depth + 1)),
            );
            rows
        }
        TrayMenuItemSpec::Separator => vec![
            Flex::new()
                .height_units(1.0)
                .bg(rgb(0xe2e8f0).into())
                .into_any_element(),
        ],
    }
}

fn docs_tray_menu_row(
    depth: usize,
    label: &str,
    command: String,
    enabled: bool,
    icon: IconName,
) -> AnyElement {
    Flex::new()
        .row()
        .align_center()
        .justify_between()
        .gap_md()
        .padding_x_px(12.0 + depth as f32 * 18.0)
        .padding_y_px(7.0)
        .rounded_units(10.0)
        .child(
            Flex::new()
                .row()
                .align_center()
                .gap_sm()
                .child(Icon::new(icon).size_units(14.0))
                .child(Text::new(label.to_string()).sm()),
        )
        .child(
            Text::new(if enabled {
                command
            } else {
                "disabled".to_string()
            })
            .sm()
            .text_color(rgb(0x64748b).into()),
        )
        .into_any_element()
}

fn docs_paragraph_wrapped() -> impl IntoElement {
    Paragraph::with_text(
        "Liora Paragraph renders prose as one selectable native text flow. This long paragraph intentionally wraps across multiple visual lines, so dragging from the first visual line into later visual lines should keep one continuous highlighted selection range.",
    )
}

fn docs_typography_paragraph(cx: &mut Context<LiveDemoContent>) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();
    Paragraph::new()
        .child(Text::new("Liora UI is a professional "))
        .child(
            Text::new("desktop UI library")
                .bold()
                .text_color(theme.primary.base),
        )
        .child(Text::new(" for Rust, built on top of ").text_color(theme.info.base))
        .child(Text::new("GPUI").italic().bg(theme.neutral.hover))
        .child(Text::new(". Inline "))
        .child(Text::new("code").code_style(&theme))
        .child(Text::new(" stays in the same wrapped paragraph."))
}

fn docs_selectable_text_group(cx: &mut Context<LiveDemoContent>) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();
    SelectableTextGroup::new()
        .id("docs-selectable-text-group-basic")
        .text(Text::new("Release notes").bold().text_color(theme.primary.base))
        .paragraph(
            Paragraph::new()
                .child(Text::new("Combine "))
                .child(Text::new("Text").code_style(&theme))
                .child(Text::new(" and "))
                .child(Text::new("Paragraph").code_style(&theme))
                .child(Text::new(" blocks in one native selection surface.")),
        )
        .paragraph(Paragraph::with_text(
            "Drag from the heading into this paragraph; selection and copy should cross block boundaries as one document-like range.",
        ))
}

fn docs_product_menu(id: &'static str) -> Menu {
    Menu::new()
        .id(id)
        .mode(MenuMode::Vertical)
        .default_active("dashboard")
        .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
        .item("components", "Components", Some(IconName::Component))
        .item("releases", "Releases", Some(IconName::Rocket))
        .item("settings", "Settings", Some(IconName::Settings))
}

fn docs_inspector_menu(id: &'static str) -> Menu {
    Menu::new()
        .id(id)
        .mode(MenuMode::Vertical)
        .default_active("layout")
        .item("layout", "Layout", Some(IconName::PanelRight))
        .item("tokens", "Tokens", Some(IconName::Palette))
        .item("events", "Events", Some(IconName::Activity))
}

fn docs_compact_menu(id: &'static str) -> Menu {
    Menu::new()
        .id(id)
        .mode(MenuMode::Vertical)
        .default_active("overview")
        .item("overview", "Overview", Some(IconName::BookOpen))
        .item("authoring", "Authoring", Some(IconName::PencilLine))
        .item("release", "Release", Some(IconName::Rocket))
}

fn docs_workspace_menu(id: &'static str) -> Menu {
    Menu::new()
        .id(id)
        .mode(MenuMode::Vertical)
        .default_active("dashboard")
        .item("dashboard", "Dashboard", Some(IconName::LayoutDashboard))
        .item("projects", "Projects", Some(IconName::Blocks))
        .item("components", "Components", Some(IconName::Component))
        .item("settings", "Settings", Some(IconName::Settings))
}

fn docs_long_workspace_menu(id: &'static str) -> Menu {
    Menu::new()
        .id(id)
        .mode(MenuMode::Vertical)
        .default_active("overview")
        .item("overview", "Overview", Some(IconName::LayoutDashboard))
        .item("activity", "Activity", Some(IconName::Activity))
        .item("inbox", "Inbox", Some(IconName::Inbox))
        .item("calendar", "Calendar", Some(IconName::CalendarDays))
        .item("files", "Files", Some(IconName::Files))
        .item("components", "Components", Some(IconName::Component))
        .item("packages", "Packages", Some(IconName::Package))
        .item("experiments", "Experiments", Some(IconName::FlaskConical))
        .item("analytics", "Analytics", Some(IconName::ChartNoAxesColumn))
        .item("reports", "Reports", Some(IconName::FileText))
        .item("automations", "Automations", Some(IconName::Bot))
        .item("integrations", "Integrations", Some(IconName::Plug))
        .item("members", "Members", Some(IconName::Users))
        .item("billing", "Billing", Some(IconName::CreditCard))
        .item("support", "Support", Some(IconName::MessagesSquare))
        .item("settings", "Settings", Some(IconName::Settings))
}

fn docs_icon_rail_menu(id: &'static str) -> Menu {
    Menu::new()
        .id(id)
        .mode(MenuMode::Vertical)
        .default_active("home")
        .collapse(true)
        .item("home", "Home", Some(IconName::House))
        .item("search", "Search", Some(IconName::Search))
        .item("inbox", "Inbox", Some(IconName::Inbox))
        .item("settings", "Settings", Some(IconName::Settings))
}

fn docs_shell_minimal_demo(theme: &Theme) -> AnyElement {
    Card::new(
        Flex::new().height_units(300.0).overflow_hidden().child(
            Shell::new(
                Space::new()
                    .vertical()
                    .gap_md()
                    .child(Title::new("Embedded surface").h4())
                    .child(Text::new(
                        "Keep consistent shell background, padding, and overlay policy.",
                    ))
                    .child(Button::new("Run preview").primary()),
            )
            .id("docs-shell-minimal-live")
            .main_padding_units(24.0)
            .main_background(theme.neutral.card)
            .main_rounded_units(18.0)
            .overlay(Text::new("Overlay slot").xs())
            .overlay_position(liora_components::ShellOverlayPosition::BottomRight)
            .overlay_inset_units(18.0)
            .background(theme.neutral.body),
        ),
    )
    .no_shadow()
    .into_any_element()
}

fn docs_sidebar_custom_slots_demo(theme: &Theme) -> AnyElement {
    Card::new(
        Flex::new()
            .height_units(336.0)
            .row()
            .overflow_hidden()
            .child(
                Sidebar::new()
                    .id("docs-sidebar-custom-slots-live")
                    .expanded_width_units(320.0)
                    .header_padding_units(14.0)
                    .content_padding_units(12.0)
                    .footer_padding_units(14.0)
                    .gap_units(10.0)
                    .background(theme.warning.light_9)
                    .border(false)
                    .rounded_units(20.0)
                    .header(
                        Space::new()
                            .gap_sm()
                            .child(Icon::new(IconName::Rocket).size_units(20.0))
                            .child(Text::new("Release cockpit").bold()),
                    )
                    .children([
                        docs_quick_stat("Open PRs", "12"),
                        docs_quick_stat("Queued jobs", "7"),
                        docs_quick_stat("Warnings", "3"),
                    ])
                    .footer(
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .child(Text::new("Custom footer slot").xs().bold())
                            .child(Button::new("Review release").small().primary()),
                    ),
            ),
    )
    .no_shadow()
    .into_any_element()
}

fn docs_quick_stat(label: &'static str, value: &'static str) -> impl IntoElement {
    Space::new()
        .gap_sm()
        .child(Text::new(label).xs())
        .child(Text::new(value).bold())
}

fn docs_accordion_basic_demo() -> liora_components::Accordion {
    liora_components::Accordion::new()
        .id("docs-accordion-basic-live")
        .default_open("account")
        .item_with_description(
            "account",
            "Account",
            "Sign-in, security, and notifications",
            |_, _| Text::new("Only one panel stays open in the default single mode."),
        )
        .item_with_description(
            "billing",
            "Billing",
            "Payment methods and invoices",
            |_, _| Text::new("Use Accordion for FAQ and settings sections."),
        )
}

fn docs_accordion_multiple_demo() -> liora_components::Accordion {
    liora_components::Accordion::new()
        .id("docs-accordion-multiple-live")
        .multiple()
        .default_open("status")
        .default_open("deploy")
        .item("status", "Service status", |_, _| {
            Text::new("Multiple panels can stay expanded.")
        })
        .item("deploy", "Deploy checks", |_, _| {
            Text::new("Good for checklists and audits.")
        })
}

fn docs_accordion_states_demo() -> liora_components::Accordion {
    liora_components::Accordion::new()
        .id("docs-accordion-states-live")
        .large()
        .bordered(false)
        .default_open("enabled")
        .item("enabled", "Enabled item", |_, _| {
            Text::new("Large borderless rows work well in docs.")
        })
        .disabled_item("locked", "Disabled item", |_, _| {
            Text::new("Disabled panels do not toggle.")
        })
}

fn docs_titlebar_surface(theme: &Theme, titlebar: TitleBar) -> impl IntoElement {
    Flex::new()
        .w_full()
        .overflow_hidden()
        .rounded_units(14.0)
        .border()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card)
        .child(titlebar)
}

fn docs_titlebar_controls_right_demo(theme: &Theme) -> AnyElement {
    Card::new(docs_titlebar_surface(
        theme,
        TitleBar::new()
            .id("docs-titlebar-controls-right-live")
            .title("Liora Studio")
            .subtitle("Theme-aware native chrome")
            .icon(Icon::new(IconName::Sparkles).size_units(16.0))
            .height_units(62.0)
            .padding_x_units(20.0)
            .gap_units(12.0)
            .actions_gap_units(8.0)
            .background(theme.neutral.card)
            .border_color(theme.neutral.border)
            .title_color(theme.neutral.text_1)
            .subtitle_color(theme.neutral.text_3)
            .content_align(TitleBarContentAlign::Start)
            .window_controls_position(WindowControlsPosition::Right)
            .window_controls(true)
            .action(Button::new("Share").small())
            .action(Button::new("Deploy").small().primary()),
    ))
    .no_shadow()
    .into_any_element()
}

fn docs_titlebar_controls_left_demo(theme: &Theme) -> AnyElement {
    Card::new(docs_titlebar_surface(
        theme,
        TitleBar::new()
            .id("docs-titlebar-controls-left-live")
            .title("Inspector")
            .subtitle("Left controls + manual drag policy")
            .icon(Icon::new(IconName::SlidersHorizontal).size_units(16.0))
            .compact()
            .draggable(false)
            .background(theme.neutral.popover)
            .border_color(theme.neutral.border)
            .title_color(theme.neutral.text_1)
            .subtitle_color(theme.neutral.text_3)
            .content_align(TitleBarContentAlign::End)
            .window_controls_position(WindowControlsPosition::Left)
            .window_controls(true)
            .action(Button::new("Reset").small()),
    ))
    .no_shadow()
    .into_any_element()
}

fn docs_titlebar_command_demo(theme: &Theme) -> AnyElement {
    Card::new(docs_titlebar_surface(
        theme,
        TitleBar::new()
            .id("docs-titlebar-command-live")
            .title("Command shell")
            .subtitle("Centered slot")
            .leading(
                Space::new()
                    .gap_xs()
                    .child(
                        Icon::new(IconName::Circle)
                            .size_units(10.0)
                            .color(theme.success.base),
                    )
                    .child(Text::new("Online").xs().bold()),
            )
            .center(
                Space::new()
                    .gap_sm()
                    .child(
                        Icon::new(IconName::Search)
                            .size_units(14.0)
                            .color(theme.primary.base),
                    )
                    .child(Text::new("Search commands or files…").sm()),
            )
            .actions([
                Button::new("Inspect").small(),
                Button::new("Publish").small().primary(),
            ])
            .height_units(58.0)
            .padding_x_units(18.0)
            .gap_units(10.0)
            .actions_gap_units(8.0)
            .background(theme.neutral.card)
            .border_color(theme.neutral.border)
            .title_color(theme.neutral.text_1)
            .subtitle_color(theme.neutral.text_3)
            .content_align(TitleBarContentAlign::Center)
            .window_controls(false),
    ))
    .no_shadow()
    .into_any_element()
}

fn docs_titlebar_borderless_demo(theme: &Theme) -> AnyElement {
    Card::new(docs_titlebar_surface(
        theme,
        TitleBar::new()
            .id("docs-titlebar-borderless-live")
            .borderless()
            .border(false)
            .title("Preview canvas")
            .subtitle("Embedded toolbar")
            .height_units(46.0)
            .padding_x_units(16.0)
            .background(theme.primary.light_9)
            .border_color(theme.primary.light_7)
            .title_color(theme.neutral.text_1)
            .subtitle_color(theme.neutral.text_2)
            .window_controls(false)
            .actions([Button::new("Fit").small(), Button::new("Export").small()]),
    ))
    .no_shadow()
    .into_any_element()
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

fn spinner_live_grid(children: Vec<AnyElement>) -> AnyElement {
    div()
        .flex()
        .flex_wrap()
        .gap_3()
        .children(children)
        .into_any_element()
}

fn spinner_live_text(title: &'static str, detail: &'static str) -> impl IntoElement {
    div().flex_1().min_w(px(0.0)).child(
        Space::new()
            .vertical()
            .gap_xs()
            .child(Text::new(title).bold().nowrap())
            .child(Text::new(detail).xs().wrap()),
    )
}

fn spinner_live_card(
    title: &'static str,
    detail: &'static str,
    spinner: liora_components::Spinner,
    bg: Hsla,
    border: Hsla,
) -> AnyElement {
    div()
        .w(px(320.0))
        .min_h(px(84.0))
        .flex()
        .items_center()
        .gap_4()
        .rounded_lg()
        .border_1()
        .border_color(border)
        .bg(bg)
        .p_4()
        .child(spinner_live_text(title, detail))
        .child(div().flex_none().child(spinner))
        .into_any_element()
}

fn spinner_live_status_card(
    title: &'static str,
    detail: &'static str,
    color: u32,
    bg: Hsla,
    border: Hsla,
) -> AnyElement {
    div()
        .w(px(320.0))
        .min_h(px(84.0))
        .flex()
        .items_center()
        .gap_4()
        .rounded_lg()
        .border_1()
        .border_color(border)
        .bg(bg)
        .p_4()
        .child(
            div().flex_none().child(
                liora_components::Spinner::new()
                    .large()
                    .color(rgb(color).into()),
            ),
        )
        .child(spinner_live_text(title, detail))
        .into_any_element()
}

fn dialog_body(message: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_lg()
        .child(Text::new(message))
        .child(
            liora_components::Row::new()
                .justify(liora_components::RowJustify::End)
                .child(
                    Button::new("Close")
                        .primary()
                        .on_click(|_, _, cx| liora_components::Dialog::close(cx)),
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
    theme: &liora_theme::Theme,
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

fn docs_dropdown_button_basic() -> impl IntoElement {
    Space::new().wrap().gap_md().children([
        DropdownButton::new("Actions")
            .id("docs-dropdown-button-actions")
            .primary()
            .item("Create task", |_, _| toast_success!("Create task"))
            .item("Duplicate", |_, _| toast_info!("Duplicate"))
            .item("Archive", |_, _| toast_info!("Archive")),
        DropdownButton::new("Export")
            .id("docs-dropdown-button-export")
            .icon_start(IconName::Download)
            .item("Export CSV", |_, _| toast_info!("Export CSV"))
            .item("Export JSON", |_, _| toast_info!("Export JSON")),
    ])
}

fn docs_dropdown_button_split() -> impl IntoElement {
    Space::new().wrap().gap_md().children([
        DropdownButton::new("Deploy")
            .id("docs-dropdown-button-split-deploy")
            .primary()
            .split(true)
            .icon_start(IconName::Rocket)
            .on_click(|_, _| toast_success!("Deploy clicked"))
            .menu_item(
                DropdownButtonItem::new("Preview deployment", |_, _| {
                    toast_info!("Preview deployment")
                })
                .icon(IconName::Eye),
            )
            .menu_item(
                DropdownButtonItem::new("Rollback", |_, _| toast_info!("Rollback"))
                    .icon(IconName::Undo2),
            )
            .danger_item("Delete release", |_, _| toast_info!("Delete release")),
        DropdownButton::new("Save")
            .id("docs-dropdown-button-split-save")
            .success()
            .split(true)
            .on_click(|_, _| toast_success!("Saved"))
            .item("Save as draft", |_, _| toast_info!("Save as draft"))
            .disabled_item("Locked by reviewer"),
    ])
}

fn docs_dropdown_button_item_states() -> DropdownButton {
    DropdownButton::new("Item states")
        .id("docs-dropdown-button-item-states")
        .menu_item(
            DropdownButtonItem::new("Rename", |_, _| toast_info!("Rename")).icon(IconName::Pencil),
        )
        .menu_item(
            DropdownButtonItem::new("Move", |_, _| toast_info!("Move")).icon(IconName::FolderInput),
        )
        .disabled_item("No permission")
        .danger_item("Delete permanently", |_, _| {
            toast_info!("Delete permanently")
        })
}

fn docs_dropdown_button_sizes() -> impl IntoElement {
    Space::new().wrap().gap_md().children([
        DropdownButton::new("Small")
            .id("docs-dropdown-button-small")
            .small()
            .item("Action", |_, _| toast_info!("Small action")),
        DropdownButton::new("Large top")
            .id("docs-dropdown-button-large-top")
            .large()
            .warning()
            .secondary()
            .placement(Placement::TopEnd)
            .item("Action", |_, _| toast_info!("Large action")),
        DropdownButton::new("Manual close")
            .id("docs-dropdown-button-manual-close")
            .close_on_click_outside(false)
            .close_on_escape(false)
            .item("Item click still closes", |_, _| toast_info!("Item click")),
    ])
}

fn docs_message_box_basic() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Button::new("Open Alert").on_click(|_, _, cx| {
            liora_components::alert("Alert Title", "This is an alert message.", cx);
        }))
        .child(Button::new("Open Confirm").primary().on_click(|_, _, cx| {
            liora_components::confirm(
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
            liora_components::MessageBox::new(
                "Manual Alert",
                "Only the OK button can close this message box.",
            )
            .close_on_click_outside(false)
            .close_on_escape(false)
            .alert(cx);
        }))
        .child(Button::new("Manual Confirm").danger().on_click(|_, _, cx| {
            liora_components::MessageBox::new(
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
        TreeNode::new("src", "src").child(TreeNode::new("components", "liora-components")),
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
    placement: liora_components::DrawerPlacement,
) -> liora_components::Drawer {
    liora_components::Drawer::new()
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
                        .on_click(|_, _, cx| liora_components::Drawer::close(cx)),
                )
        })
}

fn docs_region_options() -> Vec<liora_components::CascaderOption> {
    vec![
        liora_components::CascaderOption::new("zhejiang", "浙江")
            .child(
                liora_components::CascaderOption::new("hangzhou", "杭州")
                    .child(liora_components::CascaderOption::new("xihu", "西湖区"))
                    .child(liora_components::CascaderOption::new("yuhang", "余杭区")),
            )
            .child(
                liora_components::CascaderOption::new("ningbo", "宁波")
                    .child(liora_components::CascaderOption::new("haishu", "海曙区"))
                    .child(liora_components::CascaderOption::new("jiangbei", "江北区")),
            ),
        liora_components::CascaderOption::new("jiangsu", "江苏")
            .child(
                liora_components::CascaderOption::new("nanjing", "南京")
                    .child(liora_components::CascaderOption::new("xuanwu", "玄武区"))
                    .child(liora_components::CascaderOption::new("gulou", "鼓楼区")),
            )
            .child(
                liora_components::CascaderOption::new("suzhou", "苏州")
                    .child(liora_components::CascaderOption::new("gusu", "姑苏区"))
                    .child(
                        liora_components::CascaderOption::new("wuzhong", "吴中区").disabled(true),
                    ),
            ),
    ]
}

fn docs_product_options() -> Vec<liora_components::CascaderOption> {
    vec![
        liora_components::CascaderOption::new("cloud", "云产品")
            .child(
                liora_components::CascaderOption::new("compute", "计算")
                    .child(liora_components::CascaderOption::new("ecs", "云服务器 ECS"))
                    .child(liora_components::CascaderOption::new("fc", "函数计算")),
            )
            .child(
                liora_components::CascaderOption::new("storage", "存储")
                    .child(liora_components::CascaderOption::new("oss", "对象存储 OSS"))
                    .child(liora_components::CascaderOption::new("nas", "文件存储 NAS")),
            ),
        liora_components::CascaderOption::new("data", "数据服务").child(
            liora_components::CascaderOption::new("database", "数据库")
                .child(liora_components::CascaderOption::new(
                    "mysql",
                    "云数据库 MySQL",
                ))
                .child(liora_components::CascaderOption::new("redis", "Redis")),
        ),
    ]
}

fn docs_lazy_options() -> Vec<liora_components::CascaderOption> {
    vec![
        liora_components::CascaderOption::new("remote-a", "远程分组 A"),
        liora_components::CascaderOption::new("remote-b", "远程分组 B"),
        liora_components::CascaderOption::new("ready", "本地叶子").leaf(true),
    ]
}

fn docs_lazy_children_for(path: &[SharedString]) -> Vec<liora_components::CascaderOption> {
    let key = path
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("/");

    match key.as_str() {
        "remote-a" => vec![
            liora_components::CascaderOption::new("team", "团队")
                .child(liora_components::CascaderOption::new("design", "设计组").leaf(true)),
            liora_components::CascaderOption::new("project", "项目")
                .child(liora_components::CascaderOption::new("liora", "Liora UI").leaf(true)),
        ],
        "remote-b" => vec![
            liora_components::CascaderOption::new("north", "华北").leaf(true),
            liora_components::CascaderOption::new("south", "华南").leaf(true),
        ],
        _ => vec![liora_components::CascaderOption::new("loaded", "加载结果").leaf(true)],
    }
}

fn docs_collapse(id: &'static str, accordion: bool) -> liora_components::Collapse {
    let collapse = liora_components::Collapse::new()
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

fn basic_tabs(id: &'static str) -> liora_components::Tabs {
    liora_components::Tabs::new("first")
        .id(id)
        .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
        .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
        .pane("third", "角色管理", |_, _| Text::new("角色管理内容"))
        .pane("fourth", "定时任务", |_, _| Text::new("定时任务内容"))
}

fn short_tabs(id: &'static str) -> liora_components::Tabs {
    liora_components::Tabs::new("first")
        .id(id)
        .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
        .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
}

fn basic_autocomplete_items() -> Vec<AutocompleteItem> {
    vec![
        AutocompleteItem::labeled("rust", "Rust"),
        AutocompleteItem::labeled("gpui", "GPUI"),
        AutocompleteItem::labeled("liora", "Liora UI"),
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
    _theme: &liora_theme::Theme,
) -> AnyElement {
    let rendered_code = if let Some(source) = source {
        load_code_snippet(source.as_ref()).map_or_else(
            || SharedString::from(format!("// Missing external snippet: {}", source.as_ref())),
            |snippet| SharedString::from(snippet.to_string()),
        )
    } else {
        code
    };
    let mut code_block = LioraCodeBlock::new(rendered_code);
    code_block = code_block.selectable(true).on_copy(|_, _, _| {
        toast_success!("Code copied");
    });
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
    theme: &liora_theme::Theme,
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
                || Paragraph::with_text("Missing Liora demo host").into_any_element(),
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
    theme: &liora_theme::Theme,
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

fn render_paragraph(segments: Vec<InlineSegment>, theme: &liora_theme::Theme) -> AnyElement {
    render_paragraph_with_id(segments, theme, liora_core::unique_id("markdown-paragraph"))
}

fn render_paragraph_with_id(
    segments: Vec<InlineSegment>,
    theme: &liora_theme::Theme,
    id: impl Into<SharedString>,
) -> AnyElement {
    Paragraph::new()
        .id(id)
        .children(segments.into_iter().map(|segment| segment.into_text(theme)))
        .into_any_element()
}

pub fn render_docs_shell(
    frame_mode: WindowFrameMode,
    on_frame_mode_change: fn(WindowFrameMode, &mut Window, &mut App),
    on_close: fn(&mut Window, &mut App),
    cx: &mut App,
) -> Entity<DocsShell> {
    let view = cx.new(|cx| {
        let theme_mode = cx.global::<Config>().theme_mode;
        let shell = DocsShell {
            selected: 0,
            nav_menu: None,
            page_views: vec![None; DOC_PAGES.len()],
            update_status: UpdatePanelStatus::Idle,
            install_plan: None,
            theme_mode,
            theme_mode_segmented: cx.new(move |_| theme_mode_segmented(theme_mode)),
            frame_mode,
            frame_mode_switch: cx.new(|cx| Switch::new(frame_mode.is_custom(), cx)),
            on_frame_mode_change,
            on_close,
        };
        shell.wire_shell_controls(cx);
        shell
    });
    let auto_update_view = view.clone();
    cx.defer(move |cx| download_docs_update(auto_update_view.clone(), cx));
    view
}

#[derive(Debug)]
enum UpdatePanelStatus {
    Idle,
    Checking,
    UpToDate(String),
    Available(String),
    Downloading(String),
    Downloaded(String),
    Installing(String),
    Error(String),
}

impl UpdatePanelStatus {
    fn label(&self) -> String {
        match self {
            Self::Idle => "未检查更新 / Not checked".into(),
            Self::Checking => "正在检查 GitHub Release…".into(),
            Self::UpToDate(version) => format!("已是最新版本 {version}"),
            Self::Available(version) => format!("发现新版本 {version}"),
            Self::Downloading(version) => format!("正在下载并校验 {version}…"),
            Self::Downloaded(version) => format!("已下载并通过 SHA-256 校验：{version}"),
            Self::Installing(detail) => format!("安装计划：{detail}"),
            Self::Error(error) => format!("更新失败：{error}"),
        }
    }

    fn status_bar_icon(&self) -> &'static str {
        match self {
            Self::Checking | Self::Downloading(_) | Self::Installing(_) => "syncing",
            Self::Available(_) | Self::Downloaded(_) => "update",
            Self::Error(_) => "error",
            Self::Idle | Self::UpToDate(_) => "ready",
        }
    }
}

pub struct DocsShell {
    selected: usize,
    nav_menu: Option<Entity<Menu>>,
    page_views: Vec<Option<Entity<DocsPageView>>>,
    update_status: UpdatePanelStatus,
    install_plan: Option<InstallPlan>,
    theme_mode: ThemeMode,
    theme_mode_segmented: Entity<Segmented>,
    frame_mode: WindowFrameMode,
    frame_mode_switch: Entity<Switch>,
    on_frame_mode_change: fn(WindowFrameMode, &mut Window, &mut App),
    on_close: fn(&mut Window, &mut App),
}

fn docs_status_icon() -> Arc<RenderImage> {
    static ICON: OnceLock<Arc<RenderImage>> = OnceLock::new();
    ICON.get_or_init(|| {
        let image = ::image::load_from_memory(include_bytes!("../assets/status-icons/status.png"))
            .expect("Docs status icon asset must be a valid PNG")
            .into_rgba8();
        Arc::new(RenderImage::new([::image::Frame::new(image)]))
    })
    .clone()
}

fn docs_status_bar_icon(name: &str) -> Arc<RenderImage> {
    static READY: OnceLock<Arc<RenderImage>> = OnceLock::new();
    static SYNCING: OnceLock<Arc<RenderImage>> = OnceLock::new();
    static UPDATE: OnceLock<Arc<RenderImage>> = OnceLock::new();
    static ERROR: OnceLock<Arc<RenderImage>> = OnceLock::new();

    let (slot, bytes, label) = match name {
        "syncing" => (
            &SYNCING,
            include_bytes!("../assets/status-bar-icons/syncing.png").as_slice(),
            "Docs syncing status-bar icon",
        ),
        "update" => (
            &UPDATE,
            include_bytes!("../assets/status-bar-icons/update.png").as_slice(),
            "Docs update status-bar icon",
        ),
        "error" => (
            &ERROR,
            include_bytes!("../assets/status-bar-icons/error.png").as_slice(),
            "Docs error status-bar icon",
        ),
        _ => (
            &READY,
            include_bytes!("../assets/status-bar-icons/ready.png").as_slice(),
            "Docs ready status-bar icon",
        ),
    };

    slot.get_or_init(|| {
        let image = ::image::load_from_memory(bytes)
            .unwrap_or_else(|error| panic!("{label} asset must be a valid PNG: {error}"))
            .into_rgba8();
        Arc::new(RenderImage::new([::image::Frame::new(image)]))
    })
    .clone()
}

impl Render for DocsShell {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.selected.min(DOC_PAGES.len().saturating_sub(1));
        self.selected = selected;

        let nav_menu = self.nav_menu(selected, cx);
        let page = &DOC_PAGES[selected];
        let page_view = self.page_view(selected, cx);
        let content_body = if page.title == "About" {
            self.render_about_panel(page_view.clone(), cx)
                .into_any_element()
        } else {
            page_view.into_any_element()
        };
        let theme = cx.global::<Config>().theme.clone();
        let shell = Container::new()
            .header(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_4()
                    .child(
                        Space::new()
                            .gap_md()
                            .child(
                                div()
                                    .size(px(42.0))
                                    .rounded(px(13.0))
                                    .overflow_hidden()
                                    .child(img(docs_status_icon()).size(px(42.0))),
                            )
                            .child(
                                Space::new()
                                    .vertical()
                                    .gap_xs()
                                    .child(Title::new("Liora Docs").h2())
                                    .child(Text::new(
                                        "Native Markdown · GPUI elements · Liora components",
                                    )),
                            ),
                    )
                    .child(
                        Space::new()
                            .gap_sm()
                            .child(Text::new("Theme"))
                            .child(self.theme_mode_segmented.clone()),
                    )
                    .child(frame_mode_switch_row(
                        self.frame_mode_switch.clone(),
                        self.frame_mode,
                    )),
            )
            .header_height_lg()
            .aside(
                Sidebar::new()
                    .id("docs-sidebar")
                    .expanded_width(px(280.0))
                    .scrollable()
                    .child(nav_menu),
            )
            .aside_passthrough()
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
                    .child(div().flex_1().min_h_0().child(content_body)),
            )
            .footer(self.render_status_bar(page.title, cx))
            .footer_height(px(38.0))
            .overlay(DocsPortalLayer);

        AppWindowFrame::new("Liora Docs", shell)
            .subtitle("Native documentation")
            .mode(self.frame_mode)
            .on_close(self.on_close)
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
                    let rendered_block = render_persistent_block(
                        block,
                        &theme,
                        &virtual_live_demos,
                        &mut demo_index,
                        &format!("block-{index}"),
                    );
                    div().w_full().child(rendered_block).into_any_element()
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
        liora_components::message::render_messages(cx);
        liora_components::notification::render_notifications(cx);
        liora_components::image::render_image_preview(window, cx);
        liora_core::render_active_tooltip_in_window(window, cx);
        liora_core::render_active_popover_in_window(window, cx);
        liora_core::render_active_modal_in_window(window, cx);
        liora_core::render_active_drawer_in_window(window, cx);

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
                .id("liora-docs-passive-portal-layer")
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
                .id("liora-docs-portal-layer")
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

fn current_platform_label() -> &'static str {
    match Platform::current() {
        Some(Platform::LinuxX64) => "Linux x64",
        Some(Platform::MacosArm64) => "macOS arm64",
        Some(Platform::WindowsX64) => "Windows x64",
        None => "Unsupported platform",
    }
}

fn update_cache_dir(app: LioraApp) -> std::path::PathBuf {
    std::env::var_os("LIORA_UPDATE_CACHE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("liora-updates"))
        .join(app.release_name())
        .join(env!("CARGO_PKG_VERSION"))
}

fn check_docs_update(docs: Entity<DocsShell>, cx: &mut App) {
    let _ = docs.update(cx, |docs, cx| {
        docs.update_status = UpdatePanelStatus::Checking;
        docs.install_plan = None;
        cx.notify();
    });
    let async_cx = cx.to_async();
    let executor = cx.background_executor().clone();
    cx.foreground_executor()
        .spawn(async move {
            let result = executor
                .spawn(async move {
                    Updater::default()
                        .update_available(&format!("v{}", env!("CARGO_PKG_VERSION")), false)
                })
                .await;
            let _ = async_cx.update(move |cx| {
                let _ = docs.update(cx, |docs, cx| {
                    docs.update_status = match result {
                        Ok(None) => UpdatePanelStatus::UpToDate(env!("CARGO_PKG_VERSION").into()),
                        Ok(Some(release)) => UpdatePanelStatus::Available(release.tag),
                        Err(error) => UpdatePanelStatus::Error(error.to_string()),
                    };
                    cx.notify();
                });
            });
        })
        .detach();
}

fn download_docs_update(docs: Entity<DocsShell>, cx: &mut App) {
    let _ = docs.update(cx, |docs, cx| {
        docs.update_status = UpdatePanelStatus::Downloading("latest".into());
        cx.notify();
    });
    let async_cx = cx.to_async();
    let executor = cx.background_executor().clone();
    cx.foreground_executor()
        .spawn(async move {
            let result = executor
                .spawn(async move { download_docs_update_sync() })
                .await;
            let _ = async_cx.update(move |cx| {
                let _ = docs.update(cx, |docs, cx| {
                    match result {
                        Ok(Some((version, plan))) => {
                            docs.install_plan = Some(plan);
                            docs.update_status = UpdatePanelStatus::Downloaded(version);
                        }
                        Ok(None) => {
                            docs.update_status =
                                UpdatePanelStatus::UpToDate(env!("CARGO_PKG_VERSION").into());
                        }
                        Err(error) => {
                            docs.update_status = UpdatePanelStatus::Error(error.to_string())
                        }
                    }
                    cx.notify();
                });
            });
        })
        .detach();
}

fn download_docs_update_sync() -> Result<Option<(String, InstallPlan)>, liora_updater::UpdaterError>
{
    let Some(platform) = Platform::current() else {
        return Ok(None);
    };
    let request = UpdateRequest::new(
        LioraApp::Docs,
        format!("v{}", env!("CARGO_PKG_VERSION")),
        platform,
        update_cache_dir(LioraApp::Docs),
    )
    .selector(liora_asset_selector(
        LioraApp::Docs,
        platform,
        AssetKind::RawExecutable,
    ));
    let Some(update) = Updater::default().prepare_update(&request)? else {
        return Ok(None);
    };
    Ok(Some((update.release.tag, update.install_plan)))
}

fn install_docs_update(docs: Entity<DocsShell>, cx: &mut App) {
    let _ = docs.update(cx, |docs, cx| {
        let Some(plan) = &docs.install_plan else {
            docs.update_status = UpdatePanelStatus::Error("请先下载并校验更新".into());
            cx.notify();
            return;
        };
        let description = install_plan_description(plan);
        match &plan.action {
            InstallAction::RunExecutable { program, args } => {
                match Command::new(program).args(args).spawn() {
                    Ok(_) => docs.update_status = UpdatePanelStatus::Installing(description),
                    Err(error) => docs.update_status = UpdatePanelStatus::Error(error.to_string()),
                }
            }
            InstallAction::OpenWithSystem { program, args } => {
                match Command::new(program).args(args).spawn() {
                    Ok(_) => docs.update_status = UpdatePanelStatus::Installing(description),
                    Err(error) => docs.update_status = UpdatePanelStatus::Error(error.to_string()),
                }
            }
            InstallAction::Manual { .. } => {
                docs.update_status = UpdatePanelStatus::Installing(description);
            }
        }
        cx.notify();
    });
}

fn install_plan_description(plan: &InstallPlan) -> String {
    match &plan.action {
        InstallAction::RunExecutable { program, args } if args.is_empty() => {
            format!("Run {}", program.display())
        }
        InstallAction::RunExecutable { program, args } => {
            format!("Run {} {}", program.display(), args.join(" "))
        }
        InstallAction::OpenWithSystem { program, args } => {
            format!("Run {} {}", program, args.join(" "))
        }
        InstallAction::Manual { description } => description.clone(),
    }
}

fn theme_mode_segmented(mode: ThemeMode) -> Segmented {
    Segmented::new(vec![
        SegmentedOption::new("System", ThemeMode::System.value()),
        SegmentedOption::new("Light", ThemeMode::Light.value()),
        SegmentedOption::new("Dark", ThemeMode::Dark.value()),
    ])
    .id("docs-theme-mode")
    .value(mode.value())
}

impl DocsShell {
    fn render_status_bar(
        &self,
        page_title: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .gap_4()
            .child(
                Space::new()
                    .gap_sm()
                    .child(
                        div()
                            .size(px(22.0))
                            .rounded(px(7.0))
                            .overflow_hidden()
                            .child(
                                img(docs_status_bar_icon(self.update_status.status_bar_icon()))
                                    .size(px(22.0)),
                            ),
                    )
                    .child(Text::new(self.update_status.label()).sm().nowrap()),
            )
            .child(
                Space::new()
                    .gap_sm()
                    .child(
                        Text::new(page_title)
                            .sm()
                            .text_color(theme.neutral.text_3)
                            .nowrap(),
                    )
                    .child(
                        LioraTag::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .small()
                            .round(true),
                    ),
            )
    }

    fn render_about_panel(
        &self,
        page_view: Entity<DocsPageView>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let docs = cx.entity().clone();
        let can_install = self.install_plan.is_some();

        Space::new()
            .vertical()
            .gap_lg()
            .child(page_view)
            .child(
                Card::new(
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Title::new("About / Updates").h3())
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(
                                    LioraTag::new(format!("Docs {}", env!("CARGO_PKG_VERSION")))
                                        .success()
                                        .round(true),
                                )
                                .child(LioraTag::new("GitHub Releases").round(true))
                                .child(LioraTag::new(current_platform_label()).round(true)),
                        )
                        .child(Paragraph::with_text(
                            "Docs 会自动检查 GitHub Releases。Docs 当前按项目发布策略提供跨平台原始可执行程序；下载后会校验 SHA256SUMS.txt，替换正在运行的二进制文件需要由外部启动器或用户手动完成。",
                        ))
                        .child(
                            Text::new(self.update_status.label())
                                .text_color(theme.primary.base)
                                .bold(),
                        )
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(Button::new("检查更新 / Check").primary().on_click({
                                    let docs = docs.clone();
                                    move |_, _window, cx| check_docs_update(docs.clone(), cx)
                                }))
                                .child(Button::new("下载更新 / Download").on_click({
                                    let docs = docs.clone();
                                    move |_, _window, cx| download_docs_update(docs.clone(), cx)
                                }))
                                .child(
                                    Button::new("安装计划 / Install Plan")
                                        .disabled(!can_install)
                                        .on_click({
                                            let docs = docs.clone();
                                            move |_, _window, cx| install_docs_update(docs.clone(), cx)
                                        }),
                                ),
                        ),
                )
                .no_shadow(),
            )
    }

    fn wire_shell_controls(&self, cx: &mut Context<Self>) {
        let docs = cx.entity().clone();
        cx.update_entity(&self.theme_mode_segmented, |segmented, _cx| {
            segmented.set_on_change(move |value, window, cx| {
                let Some(mode) = ThemeMode::from_value(value.as_ref()) else {
                    return;
                };
                apply_theme_mode(window, cx, mode);
                let _ = docs.update(cx, |docs, cx| {
                    docs.theme_mode = mode;
                    cx.notify();
                });
                toast_info!("Docs theme switched to {}", mode.label());
            });
        });

        let docs = cx.entity().clone();
        let on_frame_mode_change = self.on_frame_mode_change;
        cx.update_entity(&self.frame_mode_switch, |switch, _cx| {
            switch.set_on_change(move |enabled, window, cx| {
                let mode = WindowFrameMode::from_custom(enabled);
                on_frame_mode_change(mode, window, cx);
                let _ = docs.update(cx, |docs, cx| {
                    docs.frame_mode = mode;
                    cx.notify();
                });
            });
        });
    }

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
        let active_id = selected.to_string();
        if let Some(nav_menu) = &self.nav_menu {
            cx.update_entity(nav_menu, |menu, cx| {
                menu.set_active_index(active_id, cx);
            });
            return nav_menu.clone();
        }

        let docs = cx.entity().downgrade();
        let nav_menu = cx.new(move |_| build_docs_menu(selected, docs));
        self.nav_menu = Some(nav_menu.clone());
        nav_menu
    }
}

fn build_docs_menu(selected: usize, docs: WeakEntity<DocsShell>) -> Menu {
    Menu::new()
        .id("liora-docs-menu")
        .mode(MenuMode::Vertical)
        .default_active(selected.to_string())
        .with_items(docs_nav_menu_items())
        .on_select(move |id, _, cx| {
            let Ok(index) = id.parse::<usize>() else {
                return;
            };
            let _ = docs.update(cx, |docs, cx| {
                if docs.selected != index {
                    docs.selected = index;
                    cx.notify();
                }
            });
        })
}

fn docs_nav_menu_items() -> Vec<liora_components::MenuNode> {
    let mut indices = (0..DOC_PAGES.len()).collect::<Vec<_>>();
    indices.sort_by(|left, right| {
        let left_page = &DOC_PAGES[*left];
        let right_page = &DOC_PAGES[*right];
        let left_category = docs_nav_category_for(left_page.title);
        let right_category = docs_nav_category_for(right_page.title);
        left_category
            .order()
            .cmp(&right_category.order())
            .then_with(|| {
                docs_nav_sort_rank(left_page.title).cmp(&docs_nav_sort_rank(right_page.title))
            })
            .then_with(|| {
                category::component_key(left_page.title)
                    .cmp(category::component_key(right_page.title))
            })
            .then_with(|| left.cmp(right))
    });

    let mut groups = Vec::new();
    for group_category in category::Category::ALL {
        let children = indices
            .iter()
            .filter_map(|page_index| {
                let page = &DOC_PAGES[*page_index];
                (docs_nav_category_for(page.title) == *group_category).then(|| {
                    liora_components::MenuNode::Item(liora_components::MenuItem {
                        id: page_index.to_string().into(),
                        label: page.title.into(),
                        icon: None,
                    })
                })
            })
            .collect::<Vec<_>>();

        if !children.is_empty() {
            groups.push(liora_components::MenuNode::Group(
                liora_components::MenuItemGroup {
                    title: group_category.name().into(),
                    children,
                },
            ));
        }
    }

    groups
}

fn docs_nav_sort_rank(title: &str) -> usize {
    match category::component_key(title) {
        "About" => 0,
        "Overview" => 10,
        "Quick" | "Quick Start" => 20,
        "Adoption" | "Adoption Guide" => 30,
        "Architecture" => 40,
        "Theme" => 50,
        "Packaging" | "Packaging Workflow" => 60,
        "Release" | "Release Candidate" => 70,
        "Gallery" | "Gallery Dogfooding" => 80,
        "Dashboard" | "Dashboard Patterns" => 90,
        "Dashboard State" => 100,
        "Authoring" => 110,
        "Live" | "Live Demo" => 120,
        _ => 1_000,
    }
}

fn docs_nav_category_for(title: &str) -> category::Category {
    match category::component_key(title) {
        "About" | "Overview" | "Quick" | "Quick Start" | "Architecture" | "Theme" | "Packaging"
        | "Packaging Workflow" | "Release" | "Release Candidate" | "Adoption"
        | "Adoption Guide" | "Gallery" | "Gallery Dogfooding" | "Dashboard"
        | "Dashboard Patterns" | "Dashboard State" | "Live" | "Live Demo" | "Authoring" => {
            category::Category::About
        }
        "Shell" | "TitleBar" | "Sidebar" | "Container" | "Dialog" | "Drawer" | "MessageBox"
        | "Popconfirm" | "Popover" | "Tooltip" | "Tour" | "Tray" | "PageHeader" | "Layout"
        | "Space" | "Affix" | "Anchor" | "Backtop" | "Splitter" | "Scrollbar" => {
            category::Category::WindowLayout
        }
        _ => category::Category::Control,
    }
}

fn render_list(
    ordered: bool,
    start: u64,
    items: Vec<Vec<Block>>,
    theme: &liora_theme::Theme,
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
    fn into_text(self, theme: &liora_theme::Theme) -> Text {
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

    fn docs_menu_group<'a>(
        items: &'a [liora_components::MenuNode],
        title: &str,
    ) -> &'a liora_components::MenuItemGroup {
        items
            .iter()
            .find_map(|node| match node {
                liora_components::MenuNode::Group(group) if group.title.as_ref() == title => {
                    Some(group)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("missing docs nav group {title}"))
    }

    fn docs_menu_group_labels(group: &liora_components::MenuItemGroup) -> Vec<&str> {
        group
            .children
            .iter()
            .map(|node| match node {
                liora_components::MenuNode::Item(item) => item.label.as_ref(),
                _ => panic!("Docs nav groups should contain leaf items"),
            })
            .collect()
    }

    #[test]
    fn quick_start_documents_app_level_font_customization() {
        assert!(QUICK_START_DOC.contains("## 7. 应用级字体自定义"));
        assert!(QUICK_START_DOC.contains(r#"src="quick_start/fonts.rs""#));
        assert!(QUICK_START_DOC.contains("Gallery 和 Docs 当前采用同一策略"));
        assert!(load_code_snippet("quick_start/fonts.rs").is_some());
    }

    #[test]
    fn gallery_and_docs_share_pingfang_font_bootstrap_policy() {
        let gallery = include_str!("../../liora-gallery/src/main.rs");
        let docs = include_str!("main.rs");

        for source in [gallery, docs] {
            assert!(source.contains("FontLoadMode::ExternalThenEmbedded"));
            assert!(source.contains("PingFangSC-Regular.ttf"));
            assert!(source.contains(r#"require_family("PingFang SC")"#));
            assert!(source.contains(r#"with_ui_families(["PingFang SC", "Segoe UI", "Arial"])"#));
            assert!(source.contains(
                r#"with_code_families(["Consolas", "JetBrains Mono", "SF Mono", "Monospace"])"#
            ));
        }
    }

    #[test]
    fn docs_shell_registers_theme_system_page() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        assert!(titles.contains(&"Theme"));
        assert!(THEME_SYSTEM_DOC.contains("ThemeMode::System"));
        assert!(THEME_SYSTEM_DOC.contains("liora::init_liora_with_mode"));
        assert!(THEME_SYSTEM_DOC.contains("attach_system_theme_observer"));
        assert!(THEME_SYSTEM_DOC.contains("observe_window_appearance"));
        assert!(load_code_snippet("theme/system_mode.rs").is_some());
    }

    #[test]
    fn docs_nav_menu_items_group_pages_by_category_then_title() {
        let items = docs_nav_menu_items();

        let first_group_title = match &items[0] {
            liora_components::MenuNode::Group(group) => group.title.as_ref(),
            _ => panic!("Docs nav should start with the About group"),
        };
        assert_eq!(first_group_title, "About");

        let about_group = docs_menu_group(&items, "About");
        let about_labels = docs_menu_group_labels(about_group);
        assert_eq!(
            about_labels,
            vec![
                "About",
                "Overview",
                "Quick Start",
                "Adoption Guide",
                "Architecture",
                "Theme",
                "Packaging Workflow",
                "Release Candidate",
                "Gallery Dogfooding",
                "Dashboard Patterns",
                "Dashboard State",
                "Authoring",
                "Live Demo",
            ],
            "About group should follow a human reading path instead of alphabetical order"
        );

        let window_group = docs_menu_group(&items, "窗体布局");
        let window_labels = docs_menu_group_labels(window_group);
        assert!(window_labels.contains(&"Container"));
        assert!(window_labels.contains(&"Shell"));
        assert!(window_labels.contains(&"Sidebar"));
        assert!(window_labels.contains(&"TitleBar"));
        assert!(!window_labels.contains(&"Architecture"));
        assert!(!window_labels.contains(&"Authoring"));
        assert!(!window_labels.contains(&"Dashboard Patterns"));

        let control_group = docs_menu_group(&items, "控件");
        let control_labels = docs_menu_group_labels(control_group);
        assert!(control_labels.contains(&"Accordion"));
        assert!(control_labels.contains(&"Alert"));
        assert!(control_labels.contains(&"Autocomplete"));
        assert!(control_labels.contains(&"Mention"));
    }

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
    fn core_docs_include_operational_gpui_and_liora_guidance() {
        assert!(INTRO_DOC.contains("纯原生 GPUI"));
        assert!(INTRO_DOC.contains("crates/liora-components"));

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
        assert!(ABOUT_DOC.contains("GitHub Releases"));
    }

    #[test]
    fn adoption_docs_cover_gallery_docs_public_entrypoints() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        assert!(titles.contains(&"Adoption Guide"));
        assert!(titles.contains(&"Release Candidate"));
        assert!(ADOPTION_DOC.contains("cargo run -p liora-gallery"));
        assert!(ADOPTION_DOC.contains("cargo run -p liora-docs"));
        assert!(ADOPTION_DOC.contains("liora::init_liora(cx)"));
        assert!(ADOPTION_DOC.contains("liora::init_liora_with_mode"));
        assert!(ADOPTION_DOC.contains("Entity<T>"));
        assert!(!ADOPTION_DOC.contains("liora-minimal-app"));
        assert!(!ADOPTION_DOC.contains("liora-dashboard-app"));
        assert!(include_str!("../../../README.md").contains("cargo run -p liora-gallery"));
        assert!(
            include_str!("../../../CONTRIBUTING.md").contains("cargo doc --workspace --no-deps")
        );
    }

    #[test]
    fn gallery_dogfooding_is_documented_without_standalone_examples() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        assert!(titles.contains(&"Gallery Dogfooding"));
        assert!(DASHBOARD_APP_DOC.contains("Gallery Dogfooding"));
        assert!(DASHBOARD_APP_DOC.contains("cargo run -p liora-gallery"));
        assert!(DASHBOARD_APP_DOC.contains("cargo run -p liora-docs"));
        assert!(DASHBOARD_APP_DOC.contains(
            "Standalone `minimal-app` and `dashboard-app` binaries are intentionally removed"
        ));
        assert!(!include_str!("../../../Cargo.toml").contains("examples/dashboard-app"));
        assert!(!include_str!("../../../Cargo.toml").contains("examples/minimal-app"));
    }

    #[test]
    fn dashboard_patterns_keep_sample_code_out_of_components() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        assert!(titles.contains(&"Dashboard Patterns"));
        assert!(DASHBOARD_PATTERNS_DOC.contains("app_metric_card"));
        assert!(DASHBOARD_PATTERNS_DOC.contains("Keep business models"));
        assert!(DASHBOARD_PATTERNS_DOC.contains("window.refresh()"));
        assert!(!DASHBOARD_PATTERNS_DOC.contains("DashboardGrid"));
        assert!(!DASHBOARD_PATTERNS_DOC.contains("dashboard_card"));
        assert!(
            !include_str!("../../../crates/liora-components/src/lib.rs")
                .contains("pub mod dashboard")
        );
        assert!(
            include_str!("../../../apps/liora-gallery/src/main.rs")
                .contains("Gallery theme switched")
        );
    }

    #[test]
    fn dashboard_state_docs_keep_state_in_app_layer() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        assert!(titles.contains(&"Dashboard State"));
        assert!(DASHBOARD_STATE_DOC.contains("app layer"));
        assert!(DASHBOARD_STATE_DOC.contains("Entity<T>"));
        assert!(DASHBOARD_STATE_DOC.contains("cx.notify()"));
        assert!(DASHBOARD_STATE_DOC.contains("cargo check -p liora-gallery"));
        assert!(!DASHBOARD_STATE_DOC.contains("liora-dashboard-app"));
    }

    #[test]
    fn packaging_docs_explain_ci_and_release_workflow_boundaries() {
        assert!(PACKAGING_WORKFLOW_DOC.contains(".github/workflows/ci.yml"));
        assert!(PACKAGING_WORKFLOW_DOC.contains(".github/workflows/package.yml"));
        assert!(PACKAGING_WORKFLOW_DOC.contains(".github/workflows/release-sdk.yml"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("Should publish release assets?"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("`rust-quality`"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("`packaging-dry-run`"));
        assert!(
            PACKAGING_WORKFLOW_DOC.contains("Only `v*` tag runs publish GitHub Release assets")
        );
        assert!(PACKAGING_WORKFLOW_DOC.contains("CRATES_IO_TOKEN"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("multiple-location `gpui` dependency"));
        assert!(
            PACKAGING_WORKFLOW_DOC
                .contains("Docs is released as cross-platform raw executables only")
        );
        assert!(
            PACKAGING_WORKFLOW_DOC.contains(
                "Gallery is released as raw executables and as installer/package artifacts"
            )
        );
        assert!(PACKAGING_WORKFLOW_DOC.contains("If a step builds installers, uploads artifacts, or calls `gh release`, it belongs only in `package.yml`."));
        assert!(PACKAGING_WORKFLOW_DOC.contains(
            "If a step publishes crates.io SDK crates, it belongs only in `release-sdk.yml`."
        ));
    }

    #[test]
    fn ci_workflow_splits_workspace_and_packaging_dry_run_jobs() {
        let ci = include_str!("../../../.github/workflows/ci.yml");

        assert!(ci.contains("rust-quality:"));
        assert!(ci.contains("packaging-dry-run:"));
        assert!(ci.contains("cargo check --workspace --all-targets"));
        assert!(ci.contains("cargo run -p xtask -- package validate"));
        assert!(ci.contains("cargo run -p xtask -- package install-smoke --app gallery --format platform-defaults --dry-run"));
        assert!(ci.contains("Install Linux native build dependencies"));
        assert!(ci.contains("Install packaging dry-run tools"));
        assert!(!ci.contains("rpm"));
        assert!(!ci.contains("zsync"));
    }

    #[test]
    fn packaging_docs_and_workflows_include_release_readiness_gate() {
        let ci = include_str!("../../../.github/workflows/ci.yml");
        let package = include_str!("../../../.github/workflows/package.yml");
        let sdk = include_str!("../../../.github/workflows/release-sdk.yml");

        assert!(PACKAGING_WORKFLOW_DOC.contains("package release-readiness"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("packaging/signing-policy.md"));
        assert!(PACKAGING_WORKFLOW_DOC.contains("LIORA_REQUIRE_SIGNING=true"));
        assert!(ci.contains("Release readiness dry-run policy check"));
        assert!(ci.contains("cargo run -p xtask -- package release-readiness"));
        assert!(package.contains("Check release readiness policy"));
        assert!(package.contains("cargo run --release -p xtask -- package validate"));
        assert!(package.contains("cargo run --release -p xtask -- package release-readiness"));
        assert!(package.contains("LIORA_REQUIRE_SIGNING"));
        assert!(package.contains("LIORA_MACOS_CODESIGN_IDENTITY"));
        assert!(package.contains("LIORA_WINDOWS_SIGNTOOL_CERT_PATH"));
        assert!(sdk.contains("CRATES_IO_TOKEN"));
        assert!(sdk.contains("Audit crates.io SDK metadata"));
        assert!(sdk.contains("Package independently publishable crates"));
        assert!(sdk.contains("Verify patched crates.io consumer"));
        assert!(sdk.contains("liora-theme liora-core liora-icons liora-icons-lucide liora-components liora-tray liora-packager liora-updater liora"));
        assert!(sdk.contains("cargo publish -p"));
        assert!(!sdk.contains(concat!("cargo publish -p \"$crate\" ", "--", "token")));
    }

    #[test]
    fn public_readmes_do_not_expose_internal_draft_scaffolding() {
        let readme = include_str!("../../../README.md");
        let readme_zh = include_str!("../../../README.zh-CN.md");

        for forbidden in [
            "GitHub SEO metadata",
            "Current status",
            ".memory",
            ".prompt",
            "source of truth",
            "owner-controlled",
            "protected release",
            "dogfooding",
            "canonical",
            ".omx",
            "sample model",
            "dashboard glue",
            "维护中的",
        ] {
            assert!(
                !readme.contains(forbidden),
                "English README should not expose internal draft phrase: {forbidden}"
            );
            assert!(
                !readme_zh.contains(forbidden),
                "Chinese README should not expose internal draft phrase: {forbidden}"
            );
        }

        for required in [
            "Design principles",
            "Runtime model",
            "liora::init_liora(cx)",
            "liora::init_liora_with_mode",
            r#"liora = "0.1""#,
            "Native packaging",
            "Quality gates",
            "Technical differentiators",
            "GPUI dependency and local patch policy",
        ] {
            assert!(
                readme.contains(required),
                "English README missing {required}"
            );
        }

        for required in [
            "设计原则",
            "运行时模型",
            "liora::init_liora(cx)",
            "liora::init_liora_with_mode",
            r#"liora = "0.1""#,
            "原生打包",
            "质量门禁",
            "技术创新点",
            "GPUI 依赖与本地 patch 策略",
        ] {
            assert!(
                readme_zh.contains(required),
                "Chinese README missing {required}"
            );
        }
    }

    #[test]
    fn gpui_patch_boundary_is_documented_and_not_published() {
        let root_cargo = include_str!("../../../Cargo.toml");
        let readme = include_str!("../../../README.md");
        let readme_zh = include_str!("../../../README.zh-CN.md");
        let patch_readme = include_str!("../../../third_party/zed/README.md");
        let core_source = include_str!("../../../crates/liora-core/src/lib.rs");
        let liora_package = include_str!("../../../crates/liora-core/Cargo.toml");

        assert!(root_cargo.contains(
            r#"gpui = { version = "0.2.2", git = "https://github.com/zed-industries/zed""#
        ));
        assert!(
            root_cargo
                .contains(r#"gpui_platform = { git = "https://github.com/zed-industries/zed""#)
        );
        assert!(!root_cargo.contains(concat!("package = \"open-", "gpui\"")));
        assert!(root_cargo.contains(concat!("gpui", "_platform")));
        assert!(!root_cargo.contains("[patch.crates-io]"));
        assert!(!liora_package.contains("third_party/zed"));
        assert!(!liora_package.contains("[patch"));

        for doc in [
            readme,
            readme_zh,
            ARCHITECTURE_DOC,
            THEME_SYSTEM_DOC,
            patch_readme,
        ] {
            assert!(doc.contains("official") || doc.contains("官方"));
            assert!(doc.contains("gpui"));
            assert!(doc.contains("third_party/zed"));
            assert!(doc.contains("not") || doc.contains("不"));
            assert!(
                doc.contains("[patch")
                    || doc.contains("path override")
                    || doc.contains("local patch")
                    || doc.contains("本地 patch")
            );
            assert!(!doc.contains(concat!("package = \"open-", "gpui\"")));
        }

        assert!(THEME_SYSTEM_DOC.contains("Liora SDK 不耦合"));
        assert!(ARCHITECTURE_DOC.contains("最终应用通过 `[patch.crates-io]`"));
        assert!(patch_readme.contains("official") || patch_readme.contains("upstream"));
        assert!(!THEME_SYSTEM_DOC.contains("Liora 的 patched GPUI 会"));
        assert!(!core_source.contains("Liora uses a patched GPUI build"));
    }

    #[test]
    fn release_candidate_readiness_docs_cover_current_boundaries() {
        let checklist = include_str!("../../../docs/release-candidate-checklist.md");
        let readme = include_str!("../../../README.md");
        let changelog = include_str!("../../../CHANGELOG.md");
        let prompt = include_str!("../../../prompt.md");
        let cargo = include_str!("../../../Cargo.toml");
        let package_workflow = include_str!("../../../.github/workflows/package.yml");
        let sdk_workflow = include_str!("../../../.github/workflows/release-sdk.yml");
        let ci_workflow = include_str!("../../../.github/workflows/ci.yml");

        for command in [
            "cargo fmt --all --check",
            "cargo check --workspace --all-targets",
            "cargo test --workspace",
            "cargo check -p liora-docs --bin check_snippets",
            "cargo doc --workspace --no-deps",
            "cargo run -p xtask -- package validate",
            "cargo run -p xtask -- package release-readiness",
            "cargo run -p xtask -- package ci --app gallery --format platform-defaults --dry-run --skip-build",
            "cargo run -p xtask -- package install-smoke --app gallery --format platform-defaults --dry-run",
            "timeout 10s cargo run -p liora-gallery",
            "timeout 10s cargo run -p liora-docs",
        ] {
            assert!(
                checklist.contains(command),
                "RC checklist missing {command}"
            );
        }

        assert!(checklist.contains("LicenseRef-Liora"));
        assert!(checklist.contains("multiple-location `gpui = 0.2.2` registry fallback"));
        assert!(checklist.contains("third_party/zed"));
        assert!(checklist.contains("must not contain renamed GPUI fork dependencies"));
        assert!(checklist.contains("Downstream applications must use `[patch.crates-io]`"));
        assert!(checklist.contains("pure Rust + GPUI native"));
        assert!(checklist.contains("apps/liora-gallery"));
        assert!(checklist.contains("apps/liora-docs"));
        assert!(checklist.contains("vX.Y.Z"));
        assert!(!checklist.contains("Tauri runtime"));
        assert!(checklist.contains("liora-minimal-app"));
        assert!(checklist.contains("liora-dashboard-app"));
        assert!(checklist.contains("do not re-add"));

        assert!(readme.contains("assets/liora-logo.svg"));
        assert!(readme.contains("README.zh-CN.md"));
        assert!(readme.contains("Design principles"));
        assert!(readme.contains("Runtime model"));
        assert!(readme.contains("liora::init_liora(cx)"));
        assert!(readme.contains("liora::init_liora_with_mode"));
        assert!(readme.contains(r#"liora = "0.1""#));
        assert!(readme.contains("Native packaging"));
        assert!(readme.contains("Quality gates"));
        assert!(readme.contains("Technical differentiators"));
        assert!(readme.contains("GPUI dependency and local patch policy"));
        assert!(readme.contains("official Zed upstream repository"));
        assert!(readme.contains("Do not use renamed or community forks"));
        let readme_zh = include_str!("../../../README.zh-CN.md");
        assert!(readme_zh.contains("纯 Rust + GPUI 原生"));
        assert!(readme_zh.contains("assets/liora-logo.svg"));
        assert!(readme_zh.contains("设计原则"));
        assert!(readme_zh.contains("运行时模型"));
        assert!(readme_zh.contains("技术创新点"));
        assert!(readme_zh.contains("GPUI 依赖与本地 patch 策略"));
        assert!(readme_zh.contains("禁止使用 `open-gpui` 等重命名或社区 fork"));
        let repo_metadata = include_str!("../../../assets/github-repository-metadata.md");
        assert!(repo_metadata.contains("Pure Rust + GPUI native enterprise UI component library"));
        assert!(repo_metadata.contains("rust-desktop"));
        assert!(repo_metadata.contains("no more than 20 topics"));
        let logo = include_str!("../../../assets/liora-logo.svg");
        assert!(logo.contains(r#"<title id="title">Liora modular native UI mark</title>"#));
        assert!(!logo.contains("<text"));
        assert!(!logo.contains("PURE RUST + GPUI NATIVE UI"));
        for wrong_name in ["Liorea", "liorea", "Loirea", "loirea"] {
            assert!(!readme.contains(wrong_name));
            assert!(!readme_zh.contains(wrong_name));
            assert!(!repo_metadata.contains(wrong_name));
            assert!(!logo.contains(wrong_name));
        }
        assert!(changelog.contains("P21 release-candidate readiness"));
        assert!(prompt.contains(".prompt/P21-release-candidate-readiness.md"));
        assert!(!cargo.contains("examples/minimal-app"));
        assert!(!cargo.contains("examples/dashboard-app"));
        assert!(!cargo.contains("[patch.crates-io]"));
        assert!(ci_workflow.contains("cargo run -p xtask -- package release-readiness"));
        assert!(package_workflow.contains("release-assets"));
        assert!(package_workflow.contains("--app gallery --format"));
        assert!(package_workflow.contains("liora-release-gallery-packages-*"));
        assert!(package_workflow.contains("liora-release-binaries-*"));
        assert!(
            !package_workflow
                .contains("package ci --all-apps --format platform-defaults --skip-build")
        );
        assert!(sdk_workflow.contains("Publish Liora crates.io SDK crates"));
        assert!(sdk_workflow.contains("CRATES_IO_TOKEN"));
        assert!(sdk_workflow.contains("liora-theme liora-core liora-icons liora-icons-lucide liora-components liora-tray liora-packager liora-updater liora"));
        assert!(package_workflow.contains("SHA256SUMS.txt"));
        assert!(package_workflow.contains("portable-staging/*|*.md|*.toml|*.json|*/checksums.txt"));
        assert!(!package_workflow.contains("cp release-notes.md release-assets/release-notes.md"));
    }

    #[test]
    fn workspace_package_manifests_have_rc_metadata() {
        let gpui_sdk_manifests = [
            include_str!("../../../crates/liora/Cargo.toml"),
            include_str!("../../../crates/liora-core/Cargo.toml"),
            include_str!("../../../crates/liora-theme/Cargo.toml"),
            include_str!("../../../crates/liora-components/Cargo.toml"),
            include_str!("../../../crates/liora-icons/Cargo.toml"),
            include_str!("../../../crates/liora-icons-lucide/Cargo.toml"),
            include_str!("../../../crates/liora-tray/Cargo.toml"),
        ];

        for manifest in gpui_sdk_manifests {
            assert!(manifest.contains(r#"license-file = "../../LICENSE.md""#));
            assert!(manifest.contains(r#"repository = "https://github.com/yhyzgn/liora""#));
            assert!(manifest.contains("publish = true"));
            assert!(manifest.contains(r#"description = ""#));
        }

        let utility_manifests = [
            include_str!("../../../crates/liora-packager/Cargo.toml"),
            include_str!("../../../crates/liora-updater/Cargo.toml"),
        ];

        for manifest in utility_manifests {
            assert!(manifest.contains(r#"license-file = "../../LICENSE.md""#));
            assert!(manifest.contains(r#"repository = "https://github.com/yhyzgn/liora""#));
            assert!(manifest.contains("publish = true"));
            assert!(manifest.contains(r#"description = ""#));
        }

        let private_manifests = [
            include_str!("../../../apps/liora-gallery/Cargo.toml"),
            include_str!("../../../apps/liora-docs/Cargo.toml"),
            include_str!("../../../xtask/Cargo.toml"),
        ];

        for manifest in private_manifests {
            assert!(manifest.contains(r#"license = "LicenseRef-Liora""#));
            assert!(manifest.contains(r#"repository = "https://github.com/yhyzgn/liora""#));
            assert!(manifest.contains("publish = false"));
            assert!(manifest.contains(r#"description = ""#));
        }

        assert!(
            include_str!("../../../packaging/linux/liora-gallery.metainfo.xml")
                .contains("<project_license>LicenseRef-Liora</project_license>")
        );
        assert!(
            include_str!("../../../packaging/linux/liora-docs.metainfo.xml")
                .contains("<project_license>LicenseRef-Liora</project_license>")
        );
    }

    #[test]
    fn virtualized_docs_explain_performance_and_state_model() {
        assert!(VIRTUALIZED_TABLE_DOC.contains("虚拟化性能表现"));
        assert!(VIRTUALIZED_TABLE_DOC.contains("持久 `ListState`"));
        assert!(VIRTUALIZED_TABLE_DOC.contains("splice"));
        assert!(VIRTUALIZED_LIST_DOC.contains("渲染比例约 1%"));
        assert!(VIRTUALIZED_TREE_DOC.contains("未展开分支不会生成行元素"));
    }

    #[test]
    fn native_menu_docs_cover_preview_and_descriptor_features() {
        assert!(NATIVE_MENU_DOC.contains("原生 GPUI 预览组件"));
        assert!(NATIVE_MENU_DOC.contains("分隔线"));
        assert!(NATIVE_MENU_DOC.contains("嵌套 submenu"));
        assert!(NATIVE_MENU_DOC.contains("Action Catalog"));
        assert!(NATIVE_MENU_DOC.contains("perform_builtin_actions(false)"));
        assert!(NATIVE_MENU_DOC.contains("OpenFile"));
        assert!(NATIVE_MENU_DOC.contains("OpenFolder"));
        assert!(NATIVE_MENU_DOC.contains("prompt_for_paths"));
        assert!(NATIVE_MENU_DOC.contains("prompt_for_new_path"));
        assert!(NATIVE_MENU_DOC.contains("on_paths_selected"));
        assert!(load_code_snippet("native_menu/descriptor.rs").is_some());
        assert!(load_code_snippet("native_menu/actions.rs").is_some());
    }

    #[test]
    fn component_docs_cover_gallery_registry_coverage() {
        let docs_titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        let gallery_keys = liora_gallery::demos::registry()
            .into_iter()
            .map(|entry| entry.name.split_whitespace().next().unwrap())
            .collect::<Vec<_>>();

        let gallery_source = include_str!("../../liora-gallery/src/demos/mod.rs");
        for key in &gallery_keys {
            assert!(docs_titles.contains(key), "missing docs page for {key}");
            assert!(
                gallery_source.contains(&format!("\"{key}\" => Some(")),
                "missing reusable gallery demo for {key}"
            );
        }
    }

    #[test]
    fn docs_cover_recent_gallery_shell_sidebar_titlebar_tray_mention_changes() {
        let titles = DOC_PAGES.iter().map(|page| page.title).collect::<Vec<_>>();
        for title in ["Shell", "Sidebar", "TitleBar", "Tray", "Mention"] {
            assert!(
                titles.contains(&title),
                "missing docs page for recent Gallery page {title}"
            );
        }

        assert!(SHELL_DOC.contains("ShellFullProduct"));
        assert!(SHELL_DOC.contains("ShellContentFirst"));
        assert!(SHELL_DOC.contains("ShellMinimal"));
        for snippet in [
            "shell/full_product.rs",
            "shell/content_first.rs",
            "shell/minimal.rs",
        ] {
            assert!(SHELL_DOC.contains(&format!("src=\"{snippet}\"")));
            assert!(load_code_snippet(snippet).is_some());
        }

        assert!(SIDEBAR_DOC.contains("SidebarBrand"));
        assert!(SIDEBAR_DOC.contains("SidebarScrollable"));
        assert!(SIDEBAR_DOC.contains("SidebarInspector"));
        assert!(SIDEBAR_DOC.contains("SidebarIconRail"));
        assert!(SIDEBAR_DOC.contains("SidebarCustomSlots"));
        for snippet in [
            "sidebar/brand.rs",
            "sidebar/scrollable.rs",
            "sidebar/inspector.rs",
            "sidebar/icon_rail.rs",
            "sidebar/custom_slots.rs",
        ] {
            assert!(SIDEBAR_DOC.contains(&format!("src=\"{snippet}\"")));
            assert!(load_code_snippet(snippet).is_some());
        }

        assert!(TITLEBAR_DOC.contains("TitleBarControlsRight"));
        assert!(TITLEBAR_DOC.contains("TitleBarControlsLeft"));
        assert!(TITLEBAR_DOC.contains("TitleBarCommandCenter"));
        assert!(TITLEBAR_DOC.contains("TitleBarBorderless"));
        for snippet in [
            "titlebar/window_controls_right.rs",
            "titlebar/window_controls_left.rs",
            "titlebar/command_center.rs",
            "titlebar/borderless.rs",
        ] {
            assert!(TITLEBAR_DOC.contains(&format!("src=\"{snippet}\"")));
            assert!(load_code_snippet(snippet).is_some());
        }

        assert!(TRAY_DOC.contains("TrayControlCenter"));
        assert!(TRAY_DOC.contains("状态栏驻留"));
        assert!(MENTION_DOC.contains("Up"));
        assert!(MENTION_DOC.contains("Down"));
        assert!(MENTION_DOC.contains("Enter"));
        assert!(MENTION_DOC.contains("回填"));
    }

    #[test]
    fn component_effect_sections_keep_code_next_to_effect() {
        let gallery_keys = liora_gallery::demos::registry()
            .into_iter()
            .map(|entry| entry.name.split_whitespace().next().unwrap())
            .collect::<Vec<_>>();

        for page in DOC_PAGES {
            if !gallery_keys.contains(&page.title) || !page.markdown.contains("::LioraDemo") {
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
            while let Some(effect_start) = remaining.find("LioraDemo") {
                let after_effect = &remaining[effect_start..];
                let next_section = after_effect.find("\n## ").unwrap_or(after_effect.len());
                let current_example = &after_effect[..next_section];
                assert_eq!(
                    current_example.matches("LioraDemo").count(),
                    1,
                    "{} should not batch multiple effects before one code block; keep one example followed by its code",
                    page.title
                );
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
                let code_heading = current_example.find("\n### 代码\n").unwrap();
                let code_block = current_example[code_heading..]
                    .find("```rust src=")
                    .map(|offset| code_heading + offset)
                    .expect("checked above");
                assert!(
                    code_heading < code_block,
                    "{} should show the effect first and the code immediately after it",
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
        let _ = render_markdown("# Liora\n\nNative docs");
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
            MarkdownDocument::parse("# Liora\n\nHello **bold** and *italic* with `code`.");
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
    fn quick_start_uses_unified_liora_application_init() {
        let quick_start = include_str!("../content/snippets/quick_start/main_window.rs");
        let gallery = include_str!("../../liora-gallery/src/main.rs");
        let docs = include_str!("main.rs");
        let components = include_str!("../../../crates/liora-components/src/lib.rs");

        for source in [quick_start, gallery, docs] {
            assert!(source.contains("init_liora(cx)"));
            assert!(!source.contains("MessageManager::init(cx)"));
            assert!(!source.contains("Input::register_key_bindings(cx)"));
        }
        assert!(quick_start.contains("init_liora_with_mode"));
        assert!(quick_start.contains("ThemeMode::Dark"));

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
                components.contains(registration),
                "unified component init missing {registration}"
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
                !page.contains("::::LioraDemo"),
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
    fn docs_reference_code_blocks_keep_authored_source_without_generated_comments() {
        let source = include_str!("markdown.rs");
        let render_code_block = &source[source
            .find("fn render_code_block(")
            .expect("render_code_block should exist")
            ..source
                .find("fn collect_live_demo_components(")
                .expect("collect_live_demo_components should follow")];

        assert!(render_code_block.contains("LioraCodeBlock::new(rendered_code)"));
        assert!(!render_code_block.contains("reference_annotated_code"));
        assert!(!render_code_block.contains("explain_reference_statement"));
    }

    #[test]
    fn docs_virtualized_markdown_rows_provide_full_available_width() {
        let source = include_str!("markdown.rs");
        let docs_page_view = &source[source
            .find("struct DocsPageView")
            .expect("DocsPageView should exist")
            ..source
                .find("impl Render for DocsPageView")
                .expect("DocsPageView render impl should follow")];

        assert!(docs_page_view.contains("let rendered_block = render_persistent_block("));
        assert!(docs_page_view.contains("div().w_full().child(rendered_block).into_any_element()"));
    }

    #[test]
    fn docs_live_demo_hosts_expand_to_available_content_width() {
        let source = include_str!("markdown.rs");
        let live_demo_host_render = &source[source
            .find("impl Render for LiveDemoHost")
            .expect("LiveDemoHost render impl should exist")
            ..source
                .find("struct LiveDemoContent")
                .expect("LiveDemoContent should follow")];

        assert!(live_demo_host_render.contains("Flex::new()"));
        assert!(live_demo_host_render.contains(".w_full()"));
        assert!(live_demo_host_render.contains(".child(self.demo.clone())"));
    }

    #[test]
    fn docs_markdown_code_block_copy_shows_success_toast() {
        let source = include_str!("markdown.rs");
        let render_code_block = &source[source
            .find("fn render_code_block(")
            .expect("render_code_block should exist")
            ..source
                .find("fn collect_live_demo_components(")
                .expect("collect_live_demo_components should follow")];

        assert!(render_code_block.contains(".on_copy("));
        assert!(render_code_block.contains("toast_success!"));
        assert!(render_code_block.contains("Code copied"));
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
        let docs_shell_render = source
            .split("let shell = Container::new()")
            .nth(1)
            .and_then(|part| part.split("AppWindowFrame::new").next())
            .expect("Docs shell should build a Container before AppWindowFrame");

        assert!(source.contains("Container::new()"));
        assert!(docs_shell_render.contains("Sidebar::new()"));
        assert!(docs_shell_render.contains(".aside_passthrough()"));
        assert!(source.contains("AppWindowFrame::new"));
        assert!(docs_shell_render.contains(r#".id("docs-sidebar")"#));
        assert!(!docs_shell_render.contains(".aside_width_lg()"));
        assert!(source.contains("theme_mode_segmented"));
        assert!(source.contains("ThemeMode::System"));
        let docs_main = include_str!("main.rs");
        let attach_call = format!("{}(window, cx);", "attach_system_theme_observer");
        let open_window = &docs_main[docs_main
            .find("match cx.open_window")
            .expect("Docs should open a GPUI window")..];
        assert!(docs_main.contains("show: false,"));
        assert!(docs_main.contains("startup_maximized_window_bounds"));
        let attach_index = open_window
            .find(&attach_call)
            .expect("Docs should attach System theme before first render");
        let view_index = open_window
            .find("let view = markdown::render_docs_shell")
            .expect("Docs should build shell after theme attach");
        assert!(
            attach_index < view_index,
            "System theme must sync from the real window before docs shell creation to avoid first-frame theme flash"
        );
        let ok_branch = &docs_main[docs_main
            .find("Ok(handle) =>")
            .expect("Docs should handle opened window")..];
        assert!(ok_branch.contains("window.activate_window()"));
        assert!(source.contains("frame_mode_switch"));
        assert!(source.contains("Sidebar::new()"));
        assert!(source.contains("Menu::new()"));
        assert!(source.contains("nav_menu: Option<Entity<Menu>>"));
        assert!(source.contains("menu.set_active_index(active_id, cx);"));
        assert!(source.contains("if docs.selected != index"));
        assert!(source.contains("shell.wire_shell_controls(cx);"));
        assert!(source.contains("check_docs_update"));
        assert!(source.contains("About / Updates"));
        assert!(source.contains("docs_status_bar_icon"));
        assert!(source.contains("../assets/status-bar-icons/ready.png"));
        assert!(source.contains("VirtualizedList::new"));
        assert!(source.contains("virtual_list: Entity<VirtualizedList>"));
        assert!(source.contains("measure_all_items_for_scrollbar"));
        assert!(source.contains(".flex_1().min_h_0().child(page_view)"));
        let docs_shell_bootstrap = &source[source
            .find("pub fn render_docs_shell")
            .expect("DocsShell bootstrap should exist")
            ..source
                .find("pub struct DocsShell")
                .expect("DocsShell struct should follow bootstrap")];
        assert!(docs_shell_bootstrap.contains("shell.wire_shell_controls(cx);"));
        assert!(!docs_shell_bootstrap.contains("nav_scroll: ScrollHandle"));
        assert!(!docs_shell_bootstrap.contains("ScrollHandle::new()"));
        let docs_shell_render = &source[source
            .find("impl Render for DocsShell")
            .expect("DocsShell render should exist")
            ..source
                .find("struct DocsPageView")
                .expect("DocsPageView should follow DocsShell")];
        assert!(!docs_shell_render.contains("self.wire_shell_controls(cx);"));
        assert!(!docs_shell_render.contains(r#".id("liora-docs-nav-scroll")"#));
        assert!(!docs_shell_render.contains(".overflow_y_scroll()"));
        assert!(!docs_shell_render.contains(".track_scroll(&self.nav_scroll)"));
        assert!(docs_shell_render.contains(".child(nav_menu)"));
        assert!(!docs_shell_render.contains(".aside_scroll()"));
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
            "- `crates/liora-components`：所有可复用组件，例如 `Button`、`Input`。",
        );

        let [Block::List { items, .. }] = document.blocks() else {
            panic!("expected list");
        };
        let [Block::Paragraph(segments)] = &items[0][..] else {
            panic!("expected list paragraph");
        };

        assert_eq!(segments[0].text.as_ref(), "crates/liora-components");
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
            MarkdownDocument::parse("Before\n\n::LioraDemo{component=\"Button\"}::\n\nAfter");
        let blocks = document.blocks();

        assert_eq!(blocks.len(), 3);
        assert!(matches!(
            &blocks[1],
            Block::LiveDemo { component } if component.as_ref() == "Button"
        ));
        assert!(
            !blocks.iter().any(|block| {
                matches!(block, Block::Paragraph(segments) if segments.iter().any(|segment| segment.text.as_ref().contains("::LioraDemo")))
            }),
            "live demo marker should not remain as literal paragraph text"
        );
    }

    #[test]
    fn splits_live_demo_markers_from_surrounding_text() {
        let parts = split_live_demo_parts("A ::LioraDemo{component=\"Button\"}:: B");

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
    fn titlebar_docs_split_window_control_variants_into_separate_examples() {
        assert!(TITLEBAR_DOC.contains("TitleBarControlsRight"));
        assert!(TITLEBAR_DOC.contains("TitleBarControlsLeft"));
        assert!(TITLEBAR_DOC.contains("titlebar/window_controls_right.rs"));
        assert!(TITLEBAR_DOC.contains("titlebar/window_controls_left.rs"));
        assert!(load_code_snippet("titlebar/window_controls_right.rs").is_some());
        assert!(load_code_snippet("titlebar/window_controls_left.rs").is_some());

        let source = include_str!("markdown.rs")
            .split("mod tests")
            .next()
            .unwrap();
        assert!(source.contains("docs_titlebar_controls_right_demo"));
        assert!(source.contains("docs_titlebar_controls_left_demo"));
        assert!(!source.contains("fn docs_titlebar_controls_demo"));
    }

    #[test]
    fn split_sidebar_live_demos_use_component_local_menu_indices() {
        let source = include_str!("markdown.rs")
            .split("mod tests")
            .next()
            .unwrap();

        for stale_index in [
            "self.menus[2]",
            "self.menus[3]",
            "self.menus[4]",
            "self.menus[5]",
            "self.menus[6]",
        ] {
            assert!(
                !source.contains(stale_index),
                "split sidebar demos must not use stale whole-page menu index {stale_index}"
            );
        }
    }

    #[test]
    fn component_pages_do_not_repeat_same_live_demo_key_for_distinct_examples() {
        let pages = [
            ("calendar", CALENDAR_DOC),
            ("carousel", CAROUSEL_DOC),
            ("input_tag", INPUT_TAG_DOC),
            ("mention", MENTION_DOC),
            ("tree_select", TREE_SELECT_DOC),
            ("watermark", WATERMARK_DOC),
        ];

        for (name, doc) in pages {
            let mut seen = std::collections::HashSet::new();
            let mut remaining = doc;
            while let Some(index) = remaining.find("::LioraDemo{component=\"") {
                let after = &remaining[index + "::LioraDemo{component=\"".len()..];
                let end = after
                    .find('\"')
                    .expect("live demo marker should close component");
                let key = &after[..end];
                assert!(seen.insert(key), "{name}.md repeats live demo key {key}");
                remaining = &after[end + 1..];
            }
        }
    }

    #[test]
    fn split_component_pages_have_precise_live_demo_renderers() {
        let source = include_str!("markdown.rs")
            .split("mod tests")
            .next()
            .unwrap();
        for key in [
            "CalendarEvents",
            "CalendarRange",
            "CarouselBasic",
            "CarouselAutoplay",
            "CarouselCustom",
            "CodeBlockBasic",
            "CodeBlockLanguage",
            "CodeBlockTheme",
            "CodeBlockInline",
            "QrCodeDecode",
            "InputTagBasic",
            "InputTagLimited",
            "InputTagDuplicates",
            "MentionPeople",
            "MentionIssues",
            "MentionDisabled",
            "TreeSelectSingle",
            "TreeSelectMultiple",
            "TreeSelectFilterable",
            "WatermarkCover",
            "WatermarkHeader",
            "WatermarkCustom",
            "TimerResult",
            "TourMiddle",
            "TourNoMask",
            "TourClosePolicy",
            "TrayResidency",
            "TrayInstall",
            "TrayDynamicIcon",
            "TrayCheckbox",
            "TrayCloseConfirm",
            "TrayNestedMenu",
            "TypographyParagraph",
        ] {
            assert!(source.contains(key), "missing precise renderer for {key}");
        }
    }

    #[test]
    fn component_doc_examples_are_strict_effect_code_pairs_per_section() {
        let gallery_keys = liora_gallery::demos::registry()
            .into_iter()
            .map(|entry| entry.name.split_whitespace().next().unwrap())
            .collect::<Vec<_>>();

        for page in DOC_PAGES {
            if !gallery_keys.contains(&page.title)
                || (!page.markdown.contains("::LioraDemo")
                    && !page.markdown.contains("```rust src="))
            {
                continue;
            }

            assert!(
                !page.markdown.contains(":::LioraDemo"),
                "{} must use the exact ::LioraDemo marker, not a malformed fenced marker",
                page.title
            );

            let mut sections = page.markdown.split("\n## ");
            sections.next();
            for section in sections {
                let section_title = section.lines().next().unwrap_or(page.title);
                let live_count = section.matches("\n::LioraDemo{component=\"").count()
                    + usize::from(section.starts_with("::LioraDemo{component=\""));
                let code_count = section.matches("```rust src=\"").count();

                if live_count == 0 && code_count == 0 {
                    continue;
                }

                assert_eq!(
                    live_count, 1,
                    "{} / {section_title} must contain exactly one effect demo",
                    page.title
                );
                assert_eq!(
                    code_count, 1,
                    "{} / {section_title} must contain exactly one code block",
                    page.title
                );

                let live_index = section
                    .find("::LioraDemo{component=\"")
                    .expect("checked live_count");
                let code_index = section.find("```rust src=\"").expect("checked code_count");
                assert!(
                    live_index < code_index,
                    "{} / {section_title} must render effect first, then its exact code",
                    page.title
                );
                assert!(
                    section[live_index..code_index].contains("\n### 代码\n"),
                    "{} / {section_title} must put the code heading immediately after the effect block",
                    page.title
                );
            }
        }
    }

    #[test]
    fn docs_do_not_render_whole_gallery_pages_for_split_component_examples() {
        let source = include_str!("markdown.rs")
            .split("mod tests")
            .next()
            .unwrap();

        for key in [
            "ShellFullProduct",
            "ShellContentFirst",
            "ShellMinimal",
            "SidebarBrand",
            "SidebarScrollable",
            "SidebarInspector",
            "SidebarIconRail",
            "SidebarCustomSlots",
            "AccordionBasic",
            "AccordionMultiple",
            "AccordionStates",
        ] {
            assert!(
                source.contains(key),
                "{key} should have an explicit docs renderer"
            );
        }

        assert!(source.contains("docs_shell_full_product_demo"));
        assert!(source.contains("docs_shell_content_first_demo"));
        assert!(source.contains("docs_shell_minimal_demo"));
        assert!(source.contains("docs_sidebar_brand_demo"));
        assert!(source.contains("docs_sidebar_scrollable_demo"));
        assert!(source.contains("docs_accordion_basic_demo"));
        assert!(ACCORDION_DOC.contains("AccordionBasic"));
        assert!(ACCORDION_DOC.contains("AccordionMultiple"));
        assert!(ACCORDION_DOC.contains("AccordionStates"));
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
                &[
                    "select/basic.rs",
                    "select/searchable.rs",
                    "select/grouped.rs",
                    "select/multiple.rs",
                    "select/footer.rs",
                ][..],
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
                include_str!("../content/pages/shell.md"),
                "ShellFullProduct",
                &[
                    "shell/full_product.rs",
                    "shell/content_first.rs",
                    "shell/minimal.rs",
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
                include_str!("../content/pages/accordion.md"),
                "AccordionBasic",
                &[
                    "accordion/basic.rs",
                    "accordion/multiple.rs",
                    "accordion/states.rs",
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
                include_str!("../content/pages/dock_layout.md"),
                "DockLayoutWorkbench",
                &["dock_layout/workbench.rs", "dock_layout/inspector.rs"][..],
            ),
            (
                include_str!("../content/pages/drawer.md"),
                "DrawerPlacements",
                &[
                    "drawer/placements.rs",
                    "drawer/sizes.rs",
                    "drawer/manual_close.rs",
                    "drawer/sheet_placements.rs",
                    "drawer/sheet_controlled.rs",
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
                include_str!("../content/pages/dropdown_button.md"),
                "DropdownButtonBasic",
                &[
                    "dropdown_button/basic.rs",
                    "dropdown_button/split.rs",
                    "dropdown_button/item_states.rs",
                    "dropdown_button/sizes.rs",
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
            assert!(page.contains(&format!("::LioraDemo{{component=\"{first_demo}\"}}::")));

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
    fn spinner_docs_live_and_snippets_keep_fixed_card_layout() {
        let renderer = include_str!("markdown.rs");
        let sizes = include_str!("../content/snippets/spinner/sizes.rs");
        let colors = include_str!("../content/snippets/spinner/colors.rs");
        let composition = include_str!("../content/snippets/spinner/composition.rs");

        for source in [renderer, sizes, colors, composition] {
            assert!(source.contains(".w(px(320.0))"));
            assert!(source.contains(".flex_1().min_w(px(0.0))"));
            assert!(source.contains(".flex_none()"));
        }

        assert!(renderer.contains("fn spinner_live_grid"));
        assert!(renderer.contains("fn spinner_live_text"));
        assert!(sizes.contains("fn spinner_snippet_grid"));
        assert!(colors.contains("fn spinner_snippet_grid"));
    }

    #[test]
    fn live_demo_renderer_maps_button_to_native_liora_component() {
        let source = include_str!("../content/snippets/live_demo/button.rs");

        assert!(source.contains("Button::new(\"Native Button\")"));
        assert!(source.contains("toast_success!"));
        assert!(source.contains(".on_click(|_, _, _|"));
    }
}
