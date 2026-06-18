# Steps Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Steps component with horizontal/vertical directions and status tracking.

**Architecture:** Steps is a `RenderOnce` component that displays a sequence of steps with connecting lines. Status is automatically derived from the `active` index if not explicitly set.

**Tech Stack:** Rust, GPUI 0.2.2, liora-theme, liora-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/steps.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define Steps models and enums**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, IntoElement, RenderOnce, Window,
    div, SharedString,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepsDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    Wait,
    Process,
    Finish,
    Error,
}

pub struct StepItem {
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<IconName>,
    pub status: Option<StepStatus>,
}

pub struct Steps {
    active: usize,
    direction: StepsDirection,
    items: Vec<StepItem>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl StepItem {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            icon: None,
            status: None,
        }
    }

    pub fn description(mut self, d: impl Into<SharedString>) -> Self {
        self.description = Some(d.into());
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn status(mut self, s: StepStatus) -> Self {
        self.status = Some(s);
        self
    }
}

impl Steps {
    pub fn new() -> Self {
        Self {
            active: 0,
            direction: StepsDirection::Horizontal,
            items: vec![],
        }
    }

    pub fn active(mut self, active: usize) -> Self {
        self.active = active;
        self
    }

    pub fn direction(mut self, d: StepsDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn step(mut self, item: StepItem) -> Self {
        self.items.push(item);
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Rendering Logic (Horizontal)

**Files:**
- Modify: `crates/liora-components/src/steps.rs`

- [ ] **Step 1: Implement RenderOnce for Steps**
- [ ] **Step 2: Implement Step Status calculation logic**
- [ ] **Step 3: Render horizontal step nodes with connecting lines**
- [ ] **Step 4: Apply theme colors for different statuses**
- [ ] **Step 5: Support custom icons and description text**
- [ ] **Step 6: Verify compilation**

---

### Task 3: Vertical Mode Support

**Files:**
- Modify: `crates/liora-components/src/steps.rs`

- [ ] **Step 1: Adjust layout for Vertical direction**
- [ ] **Step 2: Position connecting lines vertically under icons**
- [ ] **Step 3: Verify alignment and spacing**

---

### Task 4: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/steps_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with multiple use cases (Horizontal, Vertical, with Icons, Status override)**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
