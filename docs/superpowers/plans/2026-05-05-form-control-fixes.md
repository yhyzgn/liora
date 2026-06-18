# Form Control Stability and Ergonomics Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix multi-row Textarea display, Select click-outside-to-close behavior, and Input password toggle crash, while restoring Form ergonomics.

**Architecture:**
1. Refactor `Input` to natively support `min_rows` in its layout logic.
2. Fix `Input` coordinate mapping using safe character-based offsets to prevent multi-byte string panics.
3. Update `Select` with `on_mouse_down_out` for standard dropdown closing.
4. Restore `Form` and `FormItem` to the `RenderOnce` pattern for easier builder-style usage.

**Tech Stack:** Rust, GPUI 0.2

---

### Task 1: Fix Input Password Panic and Coordinate Safety

**Files:**
- Modify: `crates/liora-components/src/input.rs`

- [ ] **Step 1: Refactor `original_to_display_offset_in_line` to be robust.**
Ensure it counts characters correctly and handles potential out-of-bounds line offsets.

- [ ] **Step 2: Update `InputElement::prepaint` and `InputElement::paint`.**
Ensure all indexing into the display text uses safe character-boundary offsets.

- [ ] **Step 3: Test password toggle.**
Run `cargo run -p liora-gallery`, go to Form demo, and toggle password visibility. Verify no crash.

### Task 2: Implement Multi-row Textarea Height Fix

**Files:**
- Modify: `crates/liora-components/src/input.rs`
- Modify: `crates/liora-components/src/textarea.rs`

- [ ] **Step 1: Enhance `Input` with `min_rows` and `h_auto`.**
Update `InputElement::request_layout` to use `actual_lines.max(min_rows)`.
Update `Input::render` to use `.h_auto().min_h(px(34.0))` when `min_rows > 1`.

- [ ] **Step 2: Update `Textarea` to sync `min_rows`.**
Ensure `Textarea::new` and `Textarea::rows` correctly update the inner `Input` entity.

- [ ] **Step 3: Verify Textarea height.**
Check the Form demo to ensure 3 rows are visible.

### Task 3: Implement Select Click-Outside-to-Close

**Files:**
- Modify: `crates/liora-components/src/select.rs`
- Modify: `apps/liora-gallery/src/main.rs`

- [ ] **Step 1: Add `on_mouse_down_out` to `Select`.**
Set `is_open = false` and notify on click outside.

- [ ] **Step 2: Ensure Gallery background is focusable.**
Confirm `main.rs` allows the root container to take focus on click.

### Task 4: Restore Form Component Ergonomics

**Files:**
- Modify: `crates/liora-components/src/form.rs`
- Modify: `apps/liora-gallery/src/demos/form_demo.rs`

- [ ] **Step 1: Revert `Form` and `FormItem` to `RenderOnce`.**
Use `children: Vec<AnyElement>` for `Form`.
Remove `Entity<Form>` and `Entity<FormItem>` wrapping in the demo.

- [ ] **Step 2: Verify demo layout.**
Ensure all components are visible and correctly aligned.
