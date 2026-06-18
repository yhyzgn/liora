# Breadcrumb Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Breadcrumb component with custom separators and icons.

**Architecture:** Breadcrumb is a `RenderOnce` component that renders a list of `BreadcrumbItem` nodes with separators in between.

**Tech Stack:** Rust, GPUI 0.2.2, liora-theme, liora-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/breadcrumb.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define Breadcrumb models**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, IntoElement, RenderOnce, Window,
    div, SharedString,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreadcrumbSeparator {
    String(SharedString),
    Icon(IconName),
}

pub struct BreadcrumbItem {
    pub label: SharedString,
    pub icon: Option<IconName>,
    pub on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

pub struct Breadcrumb {
    separator: BreadcrumbSeparator,
    items: Vec<BreadcrumbItem>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl BreadcrumbItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn on_click(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {
            separator: BreadcrumbSeparator::String("/".into()),
            items: vec![],
        }
    }

    pub fn separator(mut self, s: impl Into<SharedString>) -> Self {
        self.separator = BreadcrumbSeparator::String(s.into());
        self
    }

    pub fn separator_icon(mut self, icon: IconName) -> Self {
        self.separator = BreadcrumbSeparator::Icon(icon);
        self
    }

    pub fn item(mut self, item: BreadcrumbItem) -> Self {
        self.items.push(item);
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Rendering Logic

**Files:**
- Modify: `crates/liora-components/src/breadcrumb.rs`

- [ ] **Step 1: Implement RenderOnce for Breadcrumb**
- [ ] **Step 2: Loop through items and insert separators**
- [ ] **Step 3: Handle hover and click styles for items**
- [ ] **Step 4: Apply bold style to the last item**
- [ ] **Step 5: Support Icon separators**
- [ ] **Step 6: Verify compilation**

---

### Task 3: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/breadcrumb_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with multiple use cases (String separator, Icon separator, with Icons)**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
