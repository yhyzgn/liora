# Affix Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Affix component that pins content to a fixed window position when a scroll threshold is met.

**Architecture:** Affix is a `View` that renders a placeholder in the layout. It tracks its window bounds during `paint`. If it crosses the threshold, it renders its content in a `Portal` at a fixed position.

**Tech Stack:** Rust, GPUI 0.2.2, liora-core (Portal/Popper logic).

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/affix.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define Affix structure and enums**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, Context, IntoElement, Render, Window,
    div, SharedString, AnyElement, Pixels, Bounds,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AffixPosition {
    #[default]
    Top,
    Bottom,
}

pub struct Affix {
    offset: Pixels,
    position: AffixPosition,
    is_fixed: bool,
    placeholder_bounds: Option<Bounds<Pixels>>,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    content: Option<Box<dyn Fn(&mut Window, &mut Context<Affix>) -> AnyElement + 'static>>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl Affix {
    pub fn new() -> Self {
        Self {
            offset: px(0.0),
            position: AffixPosition::Top,
            is_fixed: false,
            placeholder_bounds: None,
            on_change: None,
            content: None,
        }
    }

    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn position(mut self, pos: AffixPosition) -> Self {
        self.position = pos;
        self
    }

    pub fn on_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn content<F>(mut self, f: F) -> Self
    where F: Fn(&mut Window, &mut Context<Affix>) -> AnyElement + 'static {
        self.content = Some(Box::new(f));
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Implementation of Affix Element & Paint Logic

**Files:**
- Modify: `crates/liora-components/src/affix.rs`

- [ ] **Step 1: Create `AffixElement` to handle custom layout and paint**
- [ ] **Step 2: In `paint`, get the current window bounds of the element**
- [ ] **Step 3: Calculate if `is_fixed` should change based on bounds and `offset`**
- [ ] **Step 4: Trigger `cx.notify()` if state changes**
- [ ] **Step 5: Use `Portal` to render content when `is_fixed` is true**
- [ ] **Step 6: Handle `placeholder` size to prevent layout jump**

---

### Task 3: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/affix_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with a scrollable container and multiple Affix examples**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
---

**Note on GPUI 0.2.x:** Since GPUI doesn't have a built-in Portal component that is easily accessible globally, I might need to implement a simple one or use `window.add_view` if appropriate. However, for `Affix`, absolute positioning within the window is key.
