# Descriptions Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Descriptions component for structured field display.

**Architecture:** Descriptions is a `RenderOnce` component that organizes `DescriptionItem` nodes into a simulated grid using flexbox and percentage widths.

**Tech Stack:** Rust, GPUI 0.2.2, aura-theme.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/aura-components/src/descriptions.rs`
- Modify: `crates/aura-components/src/lib.rs`

- [ ] **Step 1: Define Descriptions structure and enums**

```rust
use aura_core::Config;
use gpui::{
    prelude::*, px, App, IntoElement, RenderOnce, Window,
    div, SharedString, AnyElement,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DescriptionsDirection {
    #[default]
    Horizontal,
    Vertical,
}

pub struct DescriptionItem {
    pub label: SharedString,
    pub value: AnyElement,
    pub span: u32,
}

pub struct Descriptions {
    title: Option<SharedString>,
    extra: Option<AnyElement>,
    column: u32,
    direction: DescriptionsDirection,
    border: bool,
    items: Vec<DescriptionItem>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl Descriptions {
    pub fn new() -> Self {
        Self {
            title: None,
            extra: None,
            column: 3,
            direction: DescriptionsDirection::Horizontal,
            border: false,
            items: vec![],
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn extra(mut self, extra: impl IntoElement) -> Self {
        self.extra = Some(extra.into_any_element());
        self
    }

    pub fn column(mut self, c: u32) -> Self {
        self.column = c.max(1);
        self
    }

    pub fn direction(mut self, d: DescriptionsDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn border(mut self, b: bool) -> Self {
        self.border = b;
        self
    }

    pub fn item(mut self, label: impl Into<SharedString>, value: impl IntoElement, span: u32) -> Self {
        self.items.push(DescriptionItem {
            label: label.into(),
            value: value.into_any_element(),
            span,
        });
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Layout & Row Wrapping Logic

**Files:**
- Modify: `crates/aura-components/src/descriptions.rs`

- [ ] **Step 1: Implement RenderOnce for Descriptions**
- [ ] **Step 2: Implement logic to group `items` into rows based on `column` and `span`**
- [ ] **Step 3: Ensure each row's spans sum up to `column` (padding last item if necessary)**

---

### Task 3: Rendering Styles (Bordered & Non-Bordered)

**Files:**
- Modify: `crates/aura-components/src/descriptions.rs`

- [ ] **Step 1: Implement `Horizontal` layout (Label next to Value)**
- [ ] **Step 2: Implement `Vertical` layout (Label above Value)**
- [ ] **Step 3: Implement `border` mode (cells with borders and background colors)**
- [ ] **Step 4: Support `extra` and `title` header area**
- [ ] **Step 5: Apply theme tokens for cell padding and text colors**

---

### Task 4: Gallery Demo

**Files:**
- Create: `apps/aura-gallery/src/demos/descriptions_demo.rs`
- Modify: `apps/aura-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with Basic, Bordered, and Vertical examples**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
