# Skeleton Component Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement an Element-Plus style Skeleton component for content placeholders.

**Architecture:** Skeleton is a `RenderOnce` component that either renders its children or a collection of `SkeletonItem` placeholders based on the `loading` status.

**Tech Stack:** Rust, GPUI 0.2.2, liora-theme.

---

### Task 1: Foundation & Types

**Files:**
- Create: `crates/liora-components/src/skeleton.rs`
- Modify: `crates/liora-components/src/lib.rs`

- [ ] **Step 1: Define Skeleton structure and variants**

```rust
use liora_core::Config;
use gpui::{
    prelude::*, px, App, IntoElement, RenderOnce, Window,
    div, SharedString, AnyElement,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonVariant {
    #[default]
    Paragraph,
    Circle,
    Square,
    Image,
}

pub struct SkeletonItem {
    variant: SkeletonVariant,
}

pub struct Skeleton {
    loading: bool,
    rows: u32,
    animated: bool,
    template: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    child: Option<AnyElement>,
}
```

- [ ] **Step 2: Implement constructors and builders**

```rust
impl SkeletonItem {
    pub fn new(variant: SkeletonVariant) -> Self {
        Self { variant }
    }
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            loading: true,
            rows: 3,
            animated: true,
            template: None,
            child: None,
        }
    }

    pub fn loading(mut self, l: bool) -> Self {
        self.loading = l;
        self
    }

    pub fn rows(mut self, r: u32) -> Self {
        self.rows = r;
        self
    }

    pub fn animated(mut self, a: bool) -> Self {
        self.animated = a;
        self
    }

    pub fn template<F>(mut self, f: F) -> Self
    where F: Fn(&mut Window, &mut App) -> AnyElement + 'static {
        self.template = Some(Box::new(f));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }
}
```

- [ ] **Step 3: Register in lib.rs**
- [ ] **Step 4: Verify compilation**

---

### Task 2: Rendering Logic

**Files:**
- Modify: `crates/liora-components/src/skeleton.rs`

- [ ] **Step 1: Implement `RenderOnce` for `SkeletonItem`**
- [ ] **Step 2: Implement styles for variants (Circle, Square, etc.)**
- [ ] **Step 3: Implement `RenderOnce` for `Skeleton`**
- [ ] **Step 4: Implement logical switch: `loading` ? `template` / `rows` : `child`**
- [ ] **Step 5: Implement row generation (width variation for paragraph)**

---

### Task 3: Gallery Demo

**Files:**
- Create: `apps/liora-gallery/src/demos/skeleton_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Create demo with multiple use cases (Basic rows, with Circle, Complex template)**
- [ ] **Step 2: Register in mod.rs**
- [ ] **Step 3: Final verification**
