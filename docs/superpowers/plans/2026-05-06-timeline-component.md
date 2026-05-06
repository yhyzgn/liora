# Timeline Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Timeline component for event sequencing.

**Architecture:** Timeline is a `RenderOnce` component that renders a list of `TimelineItem` nodes. It manages a vertical axis line that connects all items.

**Tech Stack:** Rust, GPUI 0.2.2, aura-theme, aura-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/aura-components/src/timeline.rs`
- Modify: `crates/aura-components/src/lib.rs`

- [ ] **Step 1: Define Timeline structure and enums**

```rust
use aura_core::Config;
use gpui::{
    prelude::*, px, App, IntoElement, RenderOnce, Window,
    div, SharedString, AnyElement, Hsla,
};
use aura_icons::Icon;
use aura_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimelineMode {
    #[default]
    Left,
    Right,
    Alternate,
}

pub struct TimelineItem {
    pub timestamp: Option<SharedString>,
    pub content: AnyElement,
    pub color: Option<Hsla>,
    pub icon: Option<IconName>,
    pub hollow: bool,
}

pub struct Timeline {
    items: Vec<TimelineItem>,
    reverse: bool,
    mode: TimelineMode,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl TimelineItem {
    pub fn new() -> Self {
        Self {
            timestamp: None,
            content: div().into_any_element(),
            color: None,
            icon: None,
            hollow: false,
        }
    }

    pub fn timestamp(mut self, t: impl Into<SharedString>) -> Self {
        self.timestamp = Some(t.into());
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = content.into_any_element();
        self
    }

    pub fn color(mut self, c: Hsla) -> Self {
        self.color = Some(c);
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn hollow(mut self, h: bool) -> Self {
        self.hollow = h;
        self
    }
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            items: vec![],
            reverse: false,
            mode: TimelineMode::Left,
        }
    }

    pub fn reverse(mut self, r: bool) -> Self {
        self.reverse = r;
        self
    }

    pub fn mode(mut self, m: TimelineMode) -> Self {
        self.mode = m;
        self
    }

    pub fn item(mut self, item: TimelineItem) -> Self {
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
- Modify: `crates/aura-components/src/timeline.rs`

- [ ] **Step 1: Implement `RenderOnce` for Timeline**
- [ ] **Step 2: Handle `reverse` logic for items**
- [ ] **Step 3: Implement item rendering: Vertical Line + Node + Content + Timestamp**
- [ ] **Step 4: Hide line for the last item**
- [ ] **Step 5: Apply theme colors for nodes and lines**
- [ ] **Step 6: Support different node variants (hollow, icon)**

---

### Task 3: Gallery Demo

**Files:**
- Create: `apps/aura-gallery/src/demos/timeline_demo.rs`
- Modify: `apps/aura-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with Basic, Custom Node, and Timestamp examples**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
