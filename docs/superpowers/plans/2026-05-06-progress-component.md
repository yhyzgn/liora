# Progress Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Progress component with line and circular types.

**Architecture:** Progress is a `RenderOnce` component that renders a progress bar using nested divs for the line type.

**Tech Stack:** Rust, GPUI 0.2.2, liora-theme, liora-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/progress.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define Progress structure and enums**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, IntoElement, RenderOnce, Window,
    div, SharedString, Pixels, Hsla,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressType {
    #[default]
    Line,
    Circle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressStatus {
    Success,
    Warning,
    Exception,
}

pub struct Progress {
    percentage: f32,
    type_: ProgressType,
    stroke_width: Pixels,
    status: Option<ProgressStatus>,
    color: Option<Hsla>,
    show_text: bool,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl Progress {
    pub fn new(percentage: f32) -> Self {
        Self {
            percentage: percentage.clamp(0.0, 100.0),
            type_: ProgressType::Line,
            stroke_width: px(6.0),
            status: None,
            color: None,
            show_text: true,
        }
    }

    pub fn type_(mut self, t: ProgressType) -> Self {
        self.type_ = t;
        self
    }

    pub fn stroke_width(mut self, w: impl Into<Pixels>) -> Self {
        self.stroke_width = w.into();
        self
    }

    pub fn status(mut self, s: ProgressStatus) -> Self {
        self.status = Some(s);
        self
    }

    pub fn color(mut self, c: Hsla) -> Self {
        self.color = Some(c);
        self
    }

    pub fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Rendering Logic (Line)

**Files:**
- Modify: `crates/liora-components/src/progress.rs`

- [ ] **Step 1: Implement RenderOnce for Progress**
- [ ] **Step 2: Implement Line mode layout (track + bar)**
- [ ] **Step 3: Calculate bar width based on `percentage`**
- [ ] **Step 4: Apply status colors (Success, Warning, Exception)**
- [ ] **Step 5: Render text/percentage label**

---

### Task 3: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/progress_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with multiple use cases (Different percentages, statuses, stroke widths)**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
