# Tree Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Tree component with nested data, expansion, and selection.

**Architecture:** Tree is a `View` that maintains `expanded_keys` and `selected_keys`. It renders a tree structure recursively.

**Tech Stack:** Rust, GPUI 0.2.2, liora-theme, liora-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/tree.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define Tree and TreeNode structure**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, Context, IntoElement, Render, Window,
    div, SharedString, AnyElement, Pixels,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::collections::HashSet;

pub struct TreeNode {
    pub id: SharedString,
    pub label: SharedString,
    pub children: Vec<TreeNode>,
}

pub struct Tree {
    data: Vec<TreeNode>,
    expanded_keys: HashSet<SharedString>,
    selected_keys: HashSet<SharedString>,
    indent: Pixels,
    show_checkbox: bool,
    on_node_click: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl TreeNode {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: vec![],
        }
    }

    pub fn child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }
}

impl Tree {
    pub fn new(data: Vec<TreeNode>) -> Self {
        Self {
            data,
            expanded_keys: HashSet::new(),
            selected_keys: HashSet::new(),
            indent: px(18.0),
            show_checkbox: false,
            on_node_click: None,
        }
    }

    pub fn indent(mut self, indent: impl Into<Pixels>) -> Self {
        self.indent = indent.into();
        self
    }

    pub fn show_checkbox(mut self, show: bool) -> Self {
        self.show_checkbox = show;
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Rendering & Expansion Logic

**Files:**
- Modify: `crates/liora-components/src/tree.rs`

- [ ] **Step 1: Implement `Render` trait for Tree**
- [ ] **Step 2: Implement recursive `render_node` method**
- [ ] **Step 3: Handle click to toggle expansion**
- [ ] **Step 4: Calculate indentation based on depth**
- [ ] **Step 5: Apply hover and active styles**

---

### Task 3: Selection Logic (Optional/Basic)

**Files:**
- Modify: `crates/liora-components/src/tree.rs`

- [ ] **Step 1: Handle click to select node**
- [ ] **Step 2: Support `show_checkbox` (visual only for now or simple set toggle)**

---

### Task 4: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/tree_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with complex hierarchical data**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
