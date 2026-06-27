/// Navigation categories shared by Gallery and Docs.
///
/// The menu intentionally stays compact:
/// - `About` is a standalone first group for project status, update entry
///   points, contribution rules, and release notes.
/// - `窗体布局` groups application shell, window/frame, overlay, layout, and
///   guide pages.
/// - `控件` groups every reusable interactive or data-display component.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    About,
    WindowLayout,
    Control,
}

impl Category {
    pub const ALL: &'static [Category] =
        &[Category::About, Category::WindowLayout, Category::Control];

    /// Returns the navigation group label shown in Gallery and Docs.
    pub fn name(self) -> &'static str {
        match self {
            Category::About => "About",
            Category::WindowLayout => "窗体布局",
            Category::Control => "控件",
        }
    }

    /// Returns a compact icon-like marker for optional category displays.
    #[allow(dead_code)]
    pub fn icon(self) -> &'static str {
        match self {
            Category::About => "ℹ",
            Category::WindowLayout => "▣",
            Category::Control => "⊞",
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

/// Maps a Gallery label or Docs title to its compact navigation category.
pub fn category_for(label_or_title: &str) -> Category {
    match component_key(label_or_title) {
        "About" => Category::About,
        "Overview" | "Quick" | "Quick Start" | "Architecture" | "Packaging"
        | "Packaging Workflow" | "Release" | "Release Candidate" | "Adoption"
        | "Adoption Guide" | "Gallery" | "Gallery Dogfooding" | "Dashboard"
        | "Dashboard Patterns" | "Dashboard State" | "Live" | "Live Demo" | "Authoring"
        | "Shell" | "TitleBar" | "Sidebar" | "Container" | "Dialog" | "Drawer" | "MessageBox"
        | "Popconfirm" | "Popover" | "Tooltip" | "Tour" | "Tray" | "PageHeader" | "Layout"
        | "Space" | "Affix" | "Anchor" | "Backtop" | "Splitter" | "Scrollbar" => {
            Category::WindowLayout
        }
        _ => Category::Control,
    }
}
