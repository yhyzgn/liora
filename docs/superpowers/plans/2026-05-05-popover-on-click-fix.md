# Popover Stable ID and on_click Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Restore `on_click` behavior for the Popover trigger by using a stable `ElementId` generated via `#[track_caller]`.

**Architecture:** Use `#[track_caller]` on `Popover::new` and `RenderOnce::render` to capture the caller's location. Use this location to generate a stable `ElementId` for the trigger `div`, which allows using `.on_click()` instead of `.on_mouse_up()`.

**Tech Stack:** Rust, GPUI 0.2.x

---

### Task 1: Update Popover Implementation

**Files:**
- Modify: `crates/aura-components/src/popover.rs`

- [ ] **Step 1: Update `Popover::new` with `#[track_caller]`**
    ```rust
    impl Popover {
        #[track_caller]
        pub fn new(trigger: impl IntoElement) -> Self {
            // ...
        }
    }
    ```

- [ ] **Step 2: Update `RenderOnce` for `Popover` with `#[track_caller]` and stable ID**
    ```rust
    impl RenderOnce for Popover {
        #[track_caller]
        fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
            // ... generate ID using std::panic::Location::caller()
            // ... use .id(id) and .on_click(...)
        }
    }
    ```

- [ ] **Step 3: Run `cargo check` to verify compilation**
    Run: `cargo check -p aura-components`
    Expected: Success

- [ ] **Step 4: Commit changes**
    ```bash
    git add crates/aura-components/src/popover.rs
    git commit -m "fix(popover): use stable id and on_click for trigger"
    ```
