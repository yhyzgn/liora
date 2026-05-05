# Robust Popover Centering and Positioning Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix Popover centering and disappearance (TopCenter) by using zero-size anchor points and flexbox centering in the overlay.

**Architecture:** 
1. `PopoverView` will calculate a "pivot point" based on the anchor and placement.
2. For `Center` placements (Top/Bottom/Left/Right), a zero-size `div` will be positioned at the pivot.
3. This pivot `div` will use `flex`, `justify_center`, and `items_center` with `overflow_visible` to ensure the child (popover content) grows outward from the pivot point, achieving perfect centering without knowing the content size.
4. Fix `Top` positioning calculation: Ensure `bottom` coordinate is relative to the viewport height correctly.

**Tech Stack:** Rust, GPUI 0.2.2

---

### Task 1: Refactor PopoverView for Pivot-based Positioning

**Files:**
- Modify: `crates/aura-components/src/popover.rs`

- [ ] **Step 1: Rewrite PopoverView::render with pivot logic**

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

        // Calculate Pivot Point and Pivot Alignment
        // A pivot point is where the zero-size container is placed.
        // We use flexbox to grow the content FROM this pivot point.
        
        let mut pivot_div = div().absolute().overflow_visible().flex();

        match placement {
            Placement::Top | Placement::Bottom | Placement::TopStart | Placement::BottomStart | Placement::TopEnd | Placement::BottomEnd => {
                // Vertical placements
                if placement == Placement::Top || placement == Placement::TopStart || placement == Placement::TopEnd {
                    let dist_from_bottom = viewport_size.height - anchor_bounds.top() + offset;
                    pivot_div = pivot_div.bottom(dist_from_bottom).flex_col_reverse();
                } else {
                    pivot_div = pivot_div.top(anchor_bounds.bottom() + offset).flex_col();
                }

                // Horizontal alignment within vertical placement
                match placement {
                    Placement::Top | Placement::Bottom => {
                        pivot_div = pivot_div.left(anchor_bounds.left() + anchor_bounds.size.width / 2.0).items_center();
                    }
                    Placement::TopStart | Placement::BottomStart => {
                        pivot_div = pivot_div.left(anchor_bounds.left()).items_start();
                    }
                    Placement::TopEnd | Placement::BottomEnd => {
                        pivot_div = pivot_div.left(anchor_bounds.right()).items_end();
                    }
                    _ => unreachable!()
                }
            }
            Placement::Left | Placement::Right | Placement::LeftStart | Placement::RightStart | Placement::LeftEnd | Placement::RightEnd => {
                // Horizontal placements
                if placement == Placement::Left || placement == Placement::LeftStart || placement == Placement::LeftEnd {
                    let dist_from_right = viewport_size.width - anchor_bounds.left() + offset;
                    pivot_div = pivot_div.right(dist_from_right).flex_row_reverse();
                } else {
                    pivot_div = pivot_div.left(anchor_bounds.right() + offset).flex_row();
                }

                // Vertical alignment within horizontal placement
                match placement {
                    Placement::Left | Placement::Right => {
                        pivot_div = pivot_div.top(anchor_bounds.top() + anchor_bounds.size.height / 2.0).items_center();
                    }
                    Placement::LeftStart | Placement::RightStart => {
                        pivot_div = pivot_div.top(anchor_bounds.top()).items_start();
                    }
                    Placement::LeftEnd | Placement::RightEnd => {
                        pivot_div = pivot_div.top(anchor_bounds.bottom()).items_end();
                    }
                    _ => unreachable!()
                }
            }
        }

        div()
            .absolute()
            .size_full()
            .on_mouse_down(MouseButton::Left, cx.listener(move |_, _, window, cx| {
                on_close(window, cx);
            }))
            .child(
                pivot_div
                    .child(
                        div()
                            .on_mouse_down(MouseButton::Left, |_, _, _| {}) // Consume click
                            .bg(theme.neutral.card)
                            .border_1().border_color(theme.neutral.border)
                            .rounded(px(theme.radius.md))
                            .shadow_lg()
                            .child(content)
                    )
            )
    }
}
```

- [ ] **Step 2: Verify compilation**
Run: `cargo check`

- [ ] **Step 3: Commit**
```bash
git add crates/aura-components/src/popover.rs
git commit -m "fix(popover): robust centering using pivot and flexbox alignment"
```

---

### Task 2: Verify in Gallery

- [ ] **Step 1: Run Gallery and check TopCenter / BottomCenter**
Run: `cargo run -p aura-gallery`
(Verify that the popover is perfectly centered horizontally relative to the button).

- [ ] **Step 2: Verify Left / Right Centering**
Check if Left/Right also center correctly vertically.

- [ ] **Step 3: Update documentation/memory**
Update `.memory/sessions.md` with the new alignment strategy.
