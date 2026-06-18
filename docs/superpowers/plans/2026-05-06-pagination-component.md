# Pagination Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Pagination component with customizable layout, background mode, and jump-to-page functionality.

**Architecture:** Pagination is a `View` component that manages the `current_page` state. It renders different sub-components (total, prev, pager, next, jumper) based on a layout string.

**Tech Stack:** Rust, GPUI 0.2.2, liora-theme, liora-components (Input/Button).

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/pagination.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define Pagination structure and state**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, Context, IntoElement, Render, Window,
    div, SharedString, AnyElement,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct Pagination {
    total: usize,
    page_size: usize,
    current_page: usize,
    background: bool,
    layout: SharedString,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl Pagination {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            page_size: 10,
            current_page: 1,
            background: false,
            layout: "prev, pager, next".into(),
            on_change: None,
        }
    }

    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = size.max(1);
        self
    }

    pub fn current_page(mut self, page: usize) -> Self {
        self.current_page = page.max(1);
        self
    }

    pub fn background(mut self, bg: bool) -> Self {
        self.background = bg;
        self
    }

    pub fn layout(mut self, l: impl Into<SharedString>) -> Self {
        self.layout = l.into();
        self
    }

    pub fn on_change(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    fn change_page(&mut self, page: usize, window: &mut Window, cx: &mut Context<Self>) {
        let page = page.clamp(1, self.page_count());
        if page != self.current_page {
            self.current_page = page;
            if let Some(ref on_change) = self.on_change {
                (on_change)(page, window, cx.app_mut());
            }
            cx.notify();
        }
    }

    fn page_count(&self) -> usize {
        (self.total as f32 / self.page_size as f32).ceil() as usize
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Pager Algorithm & Basic Navigation

**Files:**
- Modify: `crates/liora-components/src/pagination.rs`

- [ ] **Step 1: Implement `Render` trait for Pagination**
- [ ] **Step 2: Implement Pager algorithm**
    - Show 1, ..., N, ..., Max based on current_page position.
    - Max pager count = 7 (1 + prev_more + 3 + next_more + 1).
- [ ] **Step 3: Render Prev / Next buttons**
- [ ] **Step 4: Render Pager buttons**
- [ ] **Step 5: Add hover and click states (triggering `change_page`)**

---

### Task 3: Layout Support (Total, Jumper) & Background Mode

**Files:**
- Modify: `crates/liora-components/src/pagination.rs`

- [ ] **Step 1: Implement `layout` parsing**
    - Split layout string by `,` and iterate to render parts sequentially.
- [ ] **Step 2: Implement `total` rendering**
- [ ] **Step 3: Implement `jumper` rendering**
    - Needs an `Input` component or a custom small input box. Since `Input` requires a model, we can use a simpler approach or integrate `Input`.
- [ ] **Step 4: Apply `background` mode styles**
    - When `background=true`, add grey background to all buttons, active button gets primary background and white text.

---

### Task 4: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/pagination_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with Basic, Background, Layout variations**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
