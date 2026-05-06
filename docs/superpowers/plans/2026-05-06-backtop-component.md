# Backtop Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a Backtop component that appears after scrolling and allows returning to the top.

**Architecture:** Backtop is a `View` that takes a `ScrollHandle`. It monitors `handle.offset().y` and toggles its own visibility.

**Tech Stack:** Rust, GPUI 0.2.2, aura-theme, aura-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/aura-components/src/backtop.rs`
- Modify: `crates/aura-components/src/lib.rs`

- [ ] **Step 1: Define Backtop structure and builders**

```rust
use aura_core::Config;
use gpui::{
    prelude::*, px, App, Context, IntoElement, Render, Window,
    div, SharedString, AnyElement, Pixels, ScrollHandle, point,
};
use aura_icons::Icon;
use aura_icons_lucide::IconName;

pub struct Backtop {
    scroll_handle: ScrollHandle,
    visibility_height: Pixels,
    right: Pixels,
    bottom: Pixels,
    content: Option<Box<dyn Fn(&mut Window, &mut Context<Backtop>) -> AnyElement + 'static>>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl Backtop {
    pub fn new(scroll_handle: ScrollHandle) -> Self {
        Self {
            scroll_handle,
            visibility_height: px(200.0),
            right: px(40.0),
            bottom: px(40.0),
            content: None,
        }
    }

    pub fn visibility_height(mut self, h: impl Into<Pixels>) -> Self {
        self.visibility_height = h.into();
        self
    }

    pub fn right(mut self, r: impl Into<Pixels>) -> Self {
        self.right = r.into();
        self
    }

    pub fn bottom(mut self, b: impl Into<Pixels>) -> Self {
        self.bottom = b.into();
        self
    }

    pub fn content<F>(mut self, f: F) -> Self 
    where F: Fn(&mut Window, &mut Context<Backtop>) -> AnyElement + 'static {
        self.content = Some(Box::new(f));
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Rendering & Scroll Monitoring

**Files:**
- Modify: `crates/aura-components/src/backtop.rs`

- [ ] **Step 1: Implement Render for Backtop**
- [ ] **Step 2: Check `scroll_handle.offset().y` during render**
- [ ] **Step 3: Render a floating button if threshold is met**
- [ ] **Step 4: Use absolute positioning relative to window or parent**
- [ ] **Step 5: Apply theme shadow and circular styling**

---

### Task 3: Scroll Logic

**Files:**
- Modify: `crates/aura-components/src/backtop.rs`

- [ ] **Step 1: Implement `on_click` handler**
- [ ] **Step 2: Use `scroll_handle.scroll_to(point(px(0.0), px(0.0)))`**
- [ ] **Step 3: Trigger re-render to hide the button**

---

### Task 4: Gallery Demo

**Files:**
- Create: `apps/aura-gallery/src/demos/backtop_demo.rs`
- Modify: `apps/aura-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with a scrollable list and Backtop**
- [ ] **Step 2: Showcase custom position and content**
- [ ] **Step 3: Final verification**
