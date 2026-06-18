# Result Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Result component for feedback after operations.

**Architecture:** Result is a `RenderOnce` component that displays a status icon, title, subtitle, and an extra action area.

**Tech Stack:** Rust, GPUI 0.2.2, liora-theme, liora-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/result.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define Result structure and enums**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, IntoElement, RenderOnce, Window,
    div, SharedString, AnyElement,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResultStatus {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

pub struct Result {
    status: ResultStatus,
    title: SharedString,
    sub_title: Option<SharedString>,
    icon: Option<AnyElement>,
    extra: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl Result {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            status: ResultStatus::Info,
            title: title.into(),
            sub_title: None,
            icon: None,
            extra: None,
        }
    }

    pub fn status(mut self, s: ResultStatus) -> Self {
        self.status = s;
        self
    }

    pub fn sub_title(mut self, sub: impl Into<SharedString>) -> Self {
        self.sub_title = Some(sub.into());
        self
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn extra<F>(mut self, f: F) -> Self
    where F: Fn(&mut Window, &mut App) -> AnyElement + 'static {
        self.extra = Some(Box::new(f));
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Rendering Logic

**Files:**
- Modify: `crates/liora-components/src/result.rs`

- [ ] **Step 1: Implement RenderOnce for Result**
- [ ] **Step 2: Map ResultStatus to default Lucide icons and colors**
- [ ] **Step 3: Layout vertically (Icon -> Title -> Subtitle -> Extra)**
- [ ] **Step 4: Apply large typography for title**
- [ ] **Step 5: Verify compilation**

---

### Task 3: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/result_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with Success, Warning, Error, and Info examples**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
---
