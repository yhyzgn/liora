# Empty Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Empty component for no-data states.

**Architecture:** Empty is a `RenderOnce` component that centers an image, a description, and an extra action area vertically.

**Tech Stack:** Rust, GPUI 0.2.2, liora-theme, liora-icons.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/empty.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define Empty structure and builders**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, IntoElement, RenderOnce, Window,
    div, SharedString, AnyElement,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct Empty {
    image: Option<AnyElement>,
    description: SharedString,
    extra: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl Empty {
    pub fn new() -> Self {
        Self {
            image: None,
            description: "暂无数据".into(),
            extra: None,
        }
    }

    pub fn image(mut self, image: impl IntoElement) -> Self {
        self.image = Some(image.into_any_element());
        self
    }

    pub fn description(mut self, d: impl Into<SharedString>) -> Self {
        self.description = d.into();
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
- Modify: `crates/liora-components/src/empty.rs`

- [ ] **Step 1: Implement RenderOnce for Empty**
- [ ] **Step 2: Implement default SVG placeholder image if none provided**
- [ ] **Step 3: Layout vertically with gap**
- [ ] **Step 4: Apply theme colors for description**
- [ ] **Step 5: Verify compilation**

---

### Task 3: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/empty_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with multiple cases (Basic, Custom Image, With Action)**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
