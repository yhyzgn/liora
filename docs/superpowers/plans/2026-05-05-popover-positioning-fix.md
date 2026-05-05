# Popover Positioning and Event Handling Fix Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix Popover/Popconfirm display issues: restore `on_click` behavior and fix the positioning gap/alignment when placed at Top/Left.

**Architecture:** 
1. Use `#[track_caller]` to generate a stable `ElementId` for the Popover trigger, allowing the use of `.id(id).on_click(...)` on a `div` wrapper.
2. Implement "Opposite Side Positioning" in `PopoverView`: for `Top` placement, position the `bottom` edge relative to the viewport bottom; for `Left` placement, position the `right` edge. This allows elements of unknown height/width to "grow" away from the anchor correctly without needing measurement.
3. Improve centering by assuming a default size for alignment but allowing the container to shrink/wrap content.

**Tech Stack:** Rust, GPUI 0.2.2

---

### Task 1: Update Popover with Stable ID and on_click

**Files:**
- Modify: `crates/aura-components/src/popover.rs`

- [ ] **Step 1: Add auto_id and update render to use on_click**

```rust
// In crates/aura-components/src/popover.rs

impl Popover {
    #[track_caller]
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            content: Arc::new(|_, _| div().child("Popover Content").into_any_element()),
            placement: Placement::Bottom,
            offset: px(8.0),
            // Add this field to Popover struct if not present, 
            // but we can just use the caller location directly in render.
        }
    }
}

// Update RenderOnce impl
impl RenderOnce for Popover {
    #[track_caller]
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let placement = self.placement;
        let offset = self.offset;
        let content = self.content.clone();
        
        let bounds_cell = Rc::new(Cell::new(None));
        let bounds_cell_clone = bounds_cell.clone();

        // Generate a stable ID based on caller location
        let caller = std::panic::Location::caller();
        let id = ElementId::from(SharedString::from(format!("popover-trigger-{}", caller)));

        div()
            .id(id) // Now we can use on_click!
            .child(
                BoundsTracker {
                    trigger: self.trigger,
                    bounds: bounds_cell,
                }
            )
            .on_click(move |_event, _window, cx| {
                if let Some(anchor_bounds) = bounds_cell_clone.get() {
                    let content = content.clone();
                    let view = cx.new(|_cx| {
                        PopoverView::new(
                            content,
                            anchor_bounds,
                            placement,
                            offset,
                            |_window, _cx| {
                                clear_active_popover(_cx);
                            }
                        )
                    });
                    set_active_popover(view.into(), cx);
                }
            })
    }
}
```

- [ ] **Step 2: Verify compilation**
Run: `cargo check`

- [ ] **Step 3: Commit**
```bash
git add crates/aura-components/src/popover.rs
git commit -m "fix(popover): use stable id and on_click for trigger"
```

---

### Task 2: Implement "Opposite Side Positioning" in PopoverView

**Files:**
- Modify: `crates/aura-components/src/popover.rs`

- [ ] **Step 1: Update PopoverView::render to handle Top/Left via bottom/right**

```rust
// In crates/aura-components/src/popover.rs

impl Render for PopoverView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let anchor_bounds = self.anchor_bounds;
        let placement = self.placement;
        let offset = self.offset;
        let on_close = self.on_close.clone();
        
        let content = (self.content)(_window, cx);
        let viewport_size = _window.viewport_size();
        let viewport = Bounds {
            origin: gpui::Point::default(),
            size: viewport_size,
        };

        let popper = Popper {
            anchor_bounds,
            placement,
            offset,
        };

        // We still use a reference size for horizontal/vertical centering if needed,
        // but the "gap" will be fixed by side-anchoring.
        let reference_size = gpui::Size {
            width: px(200.0),
            height: px(150.0),
        };

        let (pos, final_placement) = popper.calculate_position_with_flip(reference_size, viewport);

        let mut popover_div = div()
            .absolute()
            .on_mouse_down(MouseButton::Left, |_, _, _| {}) // Consume click
            .bg(theme.neutral.card)
            .border_1().border_color(theme.neutral.border)
            .rounded(px(theme.radius.md))
            .shadow_lg()
            .child(content);

        // Positioning logic based on final_placement
        match final_placement {
            Placement::Top | Placement::TopStart | Placement::TopEnd => {
                // Position bottom edge
                let dist_from_bottom = viewport_size.height - anchor_bounds.top() + offset;
                popover_div = popover_div.bottom(dist_from_bottom);
            }
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                popover_div = popover_div.top(anchor_bounds.bottom() + offset);
            }
            Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                // Position right edge
                let dist_from_right = viewport_size.width - anchor_bounds.left() + offset;
                popover_div = popover_div.right(dist_from_right);
            }
            Placement::Right | Placement::RightStart | Placement::RightEnd => {
                popover_div = popover_div.left(anchor_bounds.right() + offset);
            }
        }

        // Horizontal/Vertical alignment for center placements
        match final_placement {
            Placement::Top | Placement::Bottom => {
                // Try to center horizontally
                // Since we don't know the width, we'll use left positioning 
                // but we might need a translate or flex wrapper.
                // For now, let's use the calculated pos.x as a baseline.
                popover_div = popover_div.left(pos.x);
            }
            Placement::TopStart | Placement::BottomStart => {
                popover_div = popover_div.left(anchor_bounds.left());
            }
            Placement::TopEnd | Placement::BottomEnd => {
                // Position right edge relative to anchor right
                let dist_from_right = viewport_size.width - anchor_bounds.right();
                popover_div = popover_div.right(dist_from_right);
            }
            Placement::Left | Placement::Right => {
                popover_div = popover_div.top(pos.y);
            }
            Placement::LeftStart | Placement::RightStart => {
                popover_div = popover_div.top(anchor_bounds.top());
            }
            Placement::LeftEnd | Placement::RightEnd => {
                let dist_from_bottom = viewport_size.height - anchor_bounds.bottom();
                popover_div = popover_div.bottom(dist_from_bottom);
            }
        }

        div()
            .absolute()
            .size_full()
            .on_mouse_down(MouseButton::Left, cx.listener(move |_, _, window, cx| {
                on_close(window, cx);
            }))
            .child(popover_div)
    }
}
```

- [ ] **Step 2: Verify compilation**
Run: `cargo check`

- [ ] **Step 3: Commit**
```bash
git add crates/aura-components/src/popover.rs
git commit -m "fix(popover): improve positioning using opposite side anchoring"
```

---

### Task 3: Verify in Gallery

- [ ] **Step 1: Check Popover and Popconfirm in the Gallery**
Run: `cargo run -p aura-gallery`
(Manually verify that Top placement has no gap and horizontal alignment is better).

- [ ] **Step 2: Verify other components (Dropdown)**
Ensure Dropdown still works and closes correctly.

- [ ] **Step 3: Update documentation/memory if needed**
Update `.memory/sessions.md` with the new findings.
