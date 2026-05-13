//! Editable Tabs can add and remove panes.

use aura_components::{Tabs, Text, toast_info};

pub fn editable_tabs() -> Tabs {
    Tabs::new("1")
        .id("docs-tabs-editable")
        .editable(true)
        .pane("1", "Tab 1", |_, _| Text::new("Content of Tab 1"))
        .pane("2", "Tab 2", |_, _| Text::new("Content of Tab 2"))
        .on_tab_add(|_, _| toast_info!("Add Tab Clicked"))
        .on_tab_remove(|name, _, _| toast_info!("Remove Tab: {}", name))
}
