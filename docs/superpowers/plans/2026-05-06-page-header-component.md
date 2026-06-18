# PageHeader Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style PageHeader component with back button, titles, and slots for extra content.

**Architecture:** PageHeader is a `RenderOnce` component that organizes content using flex layouts. It uses closures for its "slots" (Extra, Content, Footer).

**Tech Stack:** Rust, GPUI 0.2.2, liora-theme, liora-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/page_header.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define PageHeader structure and builders**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, IntoElement, RenderOnce, Window,
    div, SharedString, AnyElement,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct PageHeader {
    title: SharedString,
    sub_title: Option<SharedString>,
    back_icon: Option<IconName>,
    on_back: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    extra: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    content: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    footer: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl PageHeader {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            sub_title: None,
            back_icon: Some(IconName::ArrowLeft),
            on_back: None,
            extra: None,
            content: None,
            footer: None,
        }
    }

    pub fn sub_title(mut self, sub_title: impl Into<SharedString>) -> Self {
        self.sub_title = Some(sub_title.into());
        self
    }

    pub fn back_icon(mut self, icon: IconName) -> Self {
        self.back_icon = Some(icon);
        self
    }

    pub fn on_back(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_back = Some(Box::new(f));
        self
    }

    pub fn extra<F>(mut self, f: F) -> Self
    where F: Fn(&mut Window, &mut App) -> AnyElement + 'static {
        self.extra = Some(Box::new(f));
        self
    }

    pub fn content<F>(mut self, f: F) -> Self
    where F: Fn(&mut Window, &mut App) -> AnyElement + 'static {
        self.content = Some(Box::new(f));
        self
    }

    pub fn footer<F>(mut self, f: F) -> Self
    where F: Fn(&mut Window, &mut App) -> AnyElement + 'static {
        self.footer = Some(Box::new(f));
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Rendering Logic

**Files:**
- Modify: `crates/liora-components/src/page_header.rs`

- [ ] **Step 1: Implement RenderOnce for PageHeader**
- [ ] **Step 2: Layout Row 1 (Back, Title, Subtitle, Divider, Extra)**
- [ ] **Step 3: Implement Back button click logic**
- [ ] **Step 4: Render Content and Footer sections if present**
- [ ] **Step 5: Apply theme typography and spacing**
- [ ] **Step 6: Verify compilation**

---

### Task 3: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/page_header_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with multiple cases (Basic, with Breadcrumb, with Content, with Footer)**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
