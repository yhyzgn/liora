# Menu Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Menu component supporting Horizontal/Vertical modes, submenus, and groups.

**Architecture:** Menu is a `Render` (View) that maintains `active_index` and `opened_submenus`. Submenus use inline expansion in Vertical mode and `Popper` in Horizontal/Collapsed modes.

**Tech Stack:** Rust, GPUI 0.2.2, liora-core (Popper), liora-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/menu.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Create menu.rs with basic types**

```rust
use crate::Alert; // Temporary to satisfy imports or just use standard gpui
use liora_core::Config;
use gpui::{
    prelude::*, px, App, Context, IntoElement, Render, Window,
    div, SharedString, AnyElement, Component,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuMode {
    #[default]
    Vertical,
    Horizontal,
}

pub enum MenuNode {
    Item(MenuItem),
    SubMenu(SubMenu),
    Group(MenuItemGroup),
}

pub struct MenuItem {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<IconName>,
}

pub struct SubMenu {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<IconName>,
    pub children: Vec<MenuNode>,
}

pub struct MenuItemGroup {
    pub title: SharedString,
    pub children: Vec<MenuNode>,
}

pub struct Menu {
    mode: MenuMode,
    is_collapsed: bool,
    active_index: Option<SharedString>,
    opened_submenus: HashSet<SharedString>,
    items: Vec<MenuNode>,
    on_select: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}
```

- [ ] **Step 2: Implement Menu constructors and builders**

```rust
impl Menu {
    pub fn new() -> Self {
        Self {
            mode: MenuMode::Vertical,
            is_collapsed: false,
            active_index: None,
            opened_submenus: HashSet::new(),
            items: vec![],
            on_select: None,
        }
    }

    pub fn mode(mut self, mode: MenuMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn collapse(mut self, collapsed: bool) -> Self {
        self.is_collapsed = collapsed;
        self
    }

    pub fn default_active(mut self, index: impl Into<SharedString>) -> Self {
        self.active_index = Some(index.into());
        self
    }

    pub fn on_select(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }

    pub fn item(mut self, id: impl Into<SharedString>, label: impl Into<SharedString>, icon: Option<IconName>) -> Self {
        self.items.push(MenuNode::Item(MenuItem { id: id.into(), label: label.into(), icon }));
        self
    }

    // Submenu and Group will be added in next tasks
}
```

- [ ] **Step 3: Register in lib.rs**

```rust
// crates/liora-components/src/lib.rs
pub mod menu;
pub use menu::*;
```

- [ ] **Step 4: Verify compilation**
Run: `cargo check`
Expected: PASS

- [ ] **Step 5: Commit**
`git add crates/liora-components/src/menu.rs crates/liora-components/src/lib.rs && git commit -m "✨ menu: add base types and builders"`

---

### Task 2: SubMenu & Group Builders

**Files:**
- Modify: `crates/liora-components/src/menu.rs`

- [ ] **Step 1: Implement SubMenu builder with closure**

```rust
impl Menu {
    pub fn submenu<F>(mut self, id: impl Into<SharedString>, label: impl Into<SharedString>, icon: Option<IconName>, f: F) -> Self
    where F: FnOnce(SubMenuBuilder) -> SubMenuBuilder
    {
        let builder = SubMenuBuilder {
            id: id.into(),
            label: label.into(),
            icon,
            children: vec![],
        };
        let result = f(builder);
        self.items.push(MenuNode::SubMenu(SubMenu {
            id: result.id,
            label: result.label,
            icon: result.icon,
            children: result.children,
        }));
        self
    }
}

pub struct SubMenuBuilder {
    id: SharedString,
    label: SharedString,
    icon: Option<IconName>,
    children: Vec<MenuNode>,
}

impl SubMenuBuilder {
    pub fn item(mut self, id: impl Into<SharedString>, label: impl Into<SharedString>, icon: Option<IconName>) -> Self {
        self.children.push(MenuNode::Item(MenuItem { id: id.into(), label: label.into(), icon }));
        self
    }
    // Allow nested submenus
    pub fn submenu<F>(mut self, id: impl Into<SharedString>, label: impl Into<SharedString>, icon: Option<IconName>, f: F) -> Self
    where F: FnOnce(SubMenuBuilder) -> SubMenuBuilder
    {
        let builder = SubMenuBuilder { id: id.into(), label: label.into(), icon, children: vec![] };
        let result = f(builder);
        self.children.push(MenuNode::SubMenu(SubMenu {
            id: result.id,
            label: result.label,
            icon: result.icon,
            children: result.children,
        }));
        self
    }
}
```

- [ ] **Step 2: Implement Group builder**

- [ ] **Step 3: Verify and Commit**

---

### Task 3: Vertical Rendering (Expanded)

**Files:**
- Modify: `crates/liora-components/src/menu.rs`

- [ ] **Step 1: Implement Render for Menu**
- [ ] **Step 2: Recursive rendering of MenuNodes for Vertical mode**
- [ ] **Step 3: Handle click to select and toggle submenus**
- [ ] **Step 4: Verify and Commit**

---

### Task 4: Horizontal Rendering & Popover Submenus

**Files:**
- Modify: `crates/liora-components/src/menu.rs`

- [ ] **Step 1: Implement Horizontal mode layout**
- [ ] **Step 2: Use Popover for SubMenus in Horizontal mode**
- [ ] **Step 3: Handle Hover for Popover submenus**
- [ ] **Step 4: Verify and Commit**

---

### Task 5: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/menu_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with multiple cases (Horizontal, Vertical, Collapsed, Nested)**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verify with cargo check**
- [ ] **Step 4: Commit and Push**
