# Anchor Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Anchor component that tracks scroll position and allows jumping to sections.

**Architecture:** Anchor is a `View` that maintains a registry of `AnchorTarget` bounds. `AnchorTarget` elements report their window bounds during `paint`. Anchor uses these bounds to highlight the active link.

**Tech Stack:** Rust, GPUI 0.2.2, liora-theme, liora-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/anchor.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define Anchor models and builders**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, Context, IntoElement, Render, Window,
    div, SharedString, AnyElement, Pixels, ScrollHandle, point, Bounds, Entity,
};
use std::collections::HashMap;

pub struct AnchorLink {
    pub title: SharedString,
    pub href: SharedString,
    pub children: Vec<AnchorLink>,
}

pub struct Anchor {
    scroll_handle: ScrollHandle,
    active_link: Option<SharedString>,
    links: Vec<AnchorLink>,
    offset: Pixels,
    targets_bounds: HashMap<SharedString, Bounds<Pixels>>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl AnchorLink {
    pub fn new(title: impl Into<SharedString>, href: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            href: href.into(),
            children: vec![],
        }
    }

    pub fn child(mut self, link: AnchorLink) -> Self {
        self.children.push(link);
        self
    }
}

impl Anchor {
    pub fn new(scroll_handle: ScrollHandle) -> Self {
        Self {
            scroll_handle,
            active_link: None,
            links: vec![],
            offset: px(0.0),
            targets_bounds: HashMap::new(),
        }
    }

    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn link(mut self, link: AnchorLink) -> Self {
        self.links.push(link);
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: AnchorTarget & Bounds Reporting

**Files:**
- Modify: `crates/liora-components/src/anchor.rs`

- [ ] **Step 1: Implement `AnchorTarget` helper component**

```rust
pub struct AnchorTarget {
    id: SharedString,
    anchor: Entity<Anchor>,
    child: AnyElement,
}

impl AnchorTarget {
    pub fn new(id: impl Into<SharedString Riley>, anchor: Entity<Anchor>, child: impl IntoElement) -> Self {
        Self { id: id.into(), anchor, child: child.into_any_element() }
    }
}
```

- [ ] **Step 2: Implement `paint` for `AnchorTarget` to update registry**
- [ ] **Step 3: Add `update_target_bounds` method to `Anchor` view**

---

### Task 3: Anchor Rendering & Active Detection

**Files:**
- Modify: `crates/liora-components/src/anchor.rs`

- [ ] **Step 1: Implement `Render` for `Anchor`**
- [ ] **Step 2: Implement recursive link rendering**
- [ ] **Step 3: Implement active link detection based on `targets_bounds` and `offset`**
- [ ] **Step 4: Apply active styles (left indicator line)**

---

### Task 4: Jump Logic

**Files:**
- Modify: `crates/liora-components/src/anchor.rs`

- [ ] **Step 1: Handle link click**
- [ ] **Step 2: Calculate target scroll offset from bounds**
- [ ] **Step 3: Call `scroll_handle.set_offset()` to jump**

---

### Task 5: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/anchor_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with long content and multiple `AnchorTarget`s**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
