# Tabs Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Tabs component supporting multiple styles, positions, and editable tabs.

**Architecture:** Tabs is a `Render` (View) managing `active_name`. It contains `TabPane` items which hold label and content rendering closures.

**Tech Stack:** Rust, GPUI 0.2.2, aura-theme, aura-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/aura-components/src/tabs.rs`
- Modify: `crates/aura-components/src/lib.rs`

- [ ] **Step 1: Define basic types and models**

```rust
use aura_core::Config;
use gpui::{
    prelude::*, px, App, Context, IntoElement, Render, Window,
    div, SharedString, AnyElement,
};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabType {
    #[default]
    Standard,
    Card,
    BorderCard,
}

pub struct TabPane {
    pub name: SharedString,
    pub label: SharedString,
    pub content: Arc<dyn Fn(&mut Window, &mut Context<Tabs>) -> AnyElement + 'static>,
    pub closable: bool,
    pub icon: Option<IconName>,
}

pub struct Tabs {
    active_name: SharedString,
    position: TabPosition,
    tab_type: TabType,
    panes: Vec<TabPane>,
    editable: bool,
    on_tab_click: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    on_tab_remove: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    on_tab_add: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}
```

- [ ] **Step 2: Implement Tabs constructors and builders**

```rust
impl Tabs {
    pub fn new(active_name: impl Into<SharedString>) -> Self {
        Self {
            active_name: active_name.into(),
            position: TabPosition::Top,
            tab_type: TabType::Standard,
            panes: vec![],
            editable: false,
            on_tab_click: None,
            on_tab_remove: None,
            on_tab_add: None,
        }
    }

    pub fn position(mut self, pos: TabPosition) -> Self {
        self.position = pos;
        self
    }

    pub fn type_(mut self, t: TabType) -> Self {
        self.tab_type = t;
        self
    }

    pub fn editable(mut self, e: bool) -> Self {
        self.editable = e;
        self
    }

    pub fn pane<F, E>(mut self, name: impl Into<SharedString>, label: impl Into<SharedString>, f: F) -> Self 
    where 
        F: Fn(&mut Window, &mut Context<Self>) -> E + 'static,
        E: IntoElement,
    {
        self.panes.push(TabPane {
            name: name.into(),
            label: label.into(),
            content: Arc::new(move |window, cx| f(window, cx).into_any_element()),
            closable: true,
            icon: None,
        });
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**
Run: `cargo check`

---

### Task 2: Standard Style Rendering (Top)

**Files:**
- Modify: `crates/aura-components/src/tabs.rs`

- [ ] **Step 1: Implement Render for Tabs**
- [ ] **Step 2: Implement Header rendering with active indicator**
- [ ] **Step 3: Implement Content rendering (lazy)**
- [ ] **Step 4: Handle tab click interaction**
- [ ] **Step 5: Verify compilation**

---

### Task 3: Support for Card & BorderCard Styles

**Files:**
- Modify: `crates/aura-components/src/tabs.rs`

- [ ] **Step 1: Add styling for Card mode (header background and borders)**
- [ ] **Step 2: Add styling for BorderCard mode (outer border and content background)**
- [ ] **Step 3: Verify style transitions**

---

### Task 4: Tab Positions (Bottom, Left, Right)

**Files:**
- Modify: `crates/aura-components/src/tabs.rs`

- [ ] **Step 1: Implement layout switching for different positions**
- [ ] **Step 2: Adjust indicator orientation for vertical positions**
- [ ] **Step 3: Verify all 4 positions**

---

### Task 5: Editable Tabs (Add/Remove)

**Files:**
- Modify: `crates/aura-components/src/tabs.rs`

- [ ] **Step 1: Add "Add" button to the header**
- [ ] **Step 2: Add "Close" icon to each tab pane if closable**
- [ ] **Step 3: Implement callback logic for add/remove**
- [ ] **Step 4: Verify interactions**

---

### Task 6: Gallery Demo

**Files:**
- Create: `apps/aura-gallery/src/demos/tabs_demo.rs`
- Modify: `apps/aura-gallery/src/demos/mod.rs`

- [ ] **Step 1: Implement demo with multiple cases (Styles, Positions, Editable)**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verify and Commit**
