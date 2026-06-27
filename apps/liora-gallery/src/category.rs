/// Component categories used by Gallery and Docs navigation.
///
/// The order intentionally follows the way users scan a component library: app
/// frame primitives first, layout foundations next, then interactive controls,
/// data display, feedback, media, charts, and finally miscellaneous pages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    Guide,
    Window,
    Layout,
    Basic,
    Form,
    Navigation,
    Feedback,
    List,
    Table,
    Media,
    Chart,
    Metric,
    Advanced,
    Other,
}

impl Category {
    pub const ALL: &'static [Category] = &[
        Category::Guide,
        Category::Window,
        Category::Layout,
        Category::Basic,
        Category::Form,
        Category::Navigation,
        Category::Feedback,
        Category::List,
        Category::Table,
        Category::Media,
        Category::Chart,
        Category::Metric,
        Category::Advanced,
        Category::Other,
    ];

    /// Returns the Chinese navigation group label shown in Gallery and Docs.
    pub fn name(self) -> &'static str {
        match self {
            Category::Guide => "指南",
            Category::Window => "窗体控件",
            Category::Layout => "布局控件",
            Category::Basic => "基础控件",
            Category::Form => "表单控件",
            Category::Navigation => "导航控件",
            Category::Feedback => "反馈控件",
            Category::List => "列表控件",
            Category::Table => "表格控件",
            Category::Media => "媒体控件",
            Category::Chart => "图表控件",
            Category::Metric => "统计图",
            Category::Advanced => "高级控件",
            Category::Other => "其他",
        }
    }

    /// Returns a compact icon-like marker for optional category displays.
    #[allow(dead_code)]
    pub fn icon(self) -> &'static str {
        match self {
            Category::Guide => "?",
            Category::Window => "▣",
            Category::Layout => "▤",
            Category::Basic => "⊞",
            Category::Form => "☰",
            Category::Navigation => "☈",
            Category::Feedback => "⚡",
            Category::List => "☷",
            Category::Table => "▦",
            Category::Media => "◈",
            Category::Chart => "◒",
            Category::Metric => "#",
            Category::Advanced => "✦",
            Category::Other => "⋯",
        }
    }

    /// Returns the stable group ordering rank.
    pub fn order(self) -> usize {
        self as usize
    }
}

/// Extracts the English component key from Gallery labels such as
/// `"Button 按钮"` while leaving Docs titles such as `"Button"` unchanged.
pub fn component_key(label_or_title: &str) -> &str {
    label_or_title
        .split_whitespace()
        .next()
        .unwrap_or(label_or_title)
}

/// Maps a Gallery label or Docs title to its navigation category.
pub fn category_for(label_or_title: &str) -> Category {
    match component_key(label_or_title) {
        "Overview" | "Quick" | "Quick Start" | "Architecture" | "Packaging"
        | "Packaging Workflow" | "Release" | "Release Candidate" | "Adoption"
        | "Adoption Guide" | "Gallery" | "Gallery Dogfooding" | "Dashboard"
        | "Dashboard Patterns" | "Dashboard State" | "Live" | "Live Demo" | "Authoring" => {
            Category::Guide
        }
        "Shell" | "TitleBar" | "Sidebar" | "Container" | "Dialog" | "Drawer" | "MessageBox"
        | "Popconfirm" | "Popover" | "Tooltip" | "Tour" | "Tray" | "PageHeader" => Category::Window,
        "Layout" | "Space" | "Affix" | "Anchor" | "Backtop" | "Splitter" | "Scrollbar" => {
            Category::Layout
        }
        "Button" | "Icon" | "Link" | "Typography" | "Text" | "Title" | "Paragraph" | "Kbd"
        | "Tag" | "Badge" | "Avatar" | "Card" | "Label" | "Operation" => Category::Basic,
        "Input" | "InputNumber" | "Textarea" | "Checkbox" | "Radio" | "Switch" | "Select"
        | "Slider" | "Rate" | "Autocomplete" | "Cascader" | "DatePicker" | "DateTimePicker"
        | "TimePicker" | "ColorPicker" | "Upload" | "Form" | "InputTag" | "Mention"
        | "OtpInput" | "TreeSelect" | "Transfer" => Category::Form,
        "Breadcrumb" | "Dropdown" | "DropdownButton" | "Menu" | "Pagination" | "Segmented"
        | "Steps" | "Tabs" => Category::Navigation,
        "Alert" | "Empty" | "Loading" | "Message" | "Notification" | "Result" | "Skeleton"
        | "Progress" | "Spinner" => Category::Feedback,
        "Tree" | "HorizontalList" | "VirtualizedList" | "VirtualizedTree" | "Carousel"
        | "Timeline" | "Calendar" => Category::List,
        "Table" | "VirtualizedTable" | "Descriptions" => Category::Table,
        "Image" | "Preview" | "QrCode" | "CodeBlock" | "CodeEditor" | "Markdown" => Category::Media,
        "AreaChart" | "BarChart" | "LineChart" | "PieChart" | "RingChart" => Category::Chart,
        "Statistic" | "SignalMeter" | "Timer" | "Sparkline" | "HeatBar" | "SegmentRatioBar" => {
            Category::Metric
        }
        "Accordion" | "Collapse" | "Watermark" | "Theme" => Category::Advanced,
        _ => Category::Other,
    }
}
