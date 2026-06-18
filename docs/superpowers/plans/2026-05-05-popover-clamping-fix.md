# Popover Boundary Clamping and TopCenter Fix Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ensure Popover stays within viewport when centering and fix the `TopCenter` disappearance issue.

**Architecture:**
1. `PopoverView` will determine the "ideal center" for the pivot container.
2. It will apply clamping to this center point based on a `reference_size` (e.g., 200x150) to ensure the content edges don't exceed viewport boundaries [0, viewport_width/height].
3. For `Top` placement, use `bottom(viewport_height - anchor_top + offset)` but ensure it doesn't result in negative positioning or being pushed off-screen.
4. Simplified centering: Use `left(clamped_center_x - container_width/2)` to ensure the 2000px container is shifted correctly to keep the content inside the window.

**Tech Stack:** Rust, GPUI 0.2.2

---

### Task 1: Implement Boundary Clamping and Fix TopCenter

**Files:**
- Modify: `crates/liora-components/src/popover.rs`

- [ ] **Step 1: Update PopoverView::render with clamping logic**

```rust
// In crates/liora-components/src/popover.rs

impl Render for PopoverView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let anchor_bounds = self.anchor_bounds;
        let placement = self.placement;
        let offset = self.offset;
        let on_close = self.on_close.clone();

        let content = (self.content)(_window, cx);
        let viewport_size = _window.viewport_size();
        let viewport = Bounds { origin: gpui::Point::default(), size: viewport_size };

        // Use Popper for flip logic
        let popper = liora_core::Popper { anchor_bounds, placement, offset };
        let reference_size = gpui::Size { width: px(200.0), height: px(150.0) };
        let (_pos, final_placement) = popper.calculate_position_with_flip(reference_size, viewport);

        let mut pivot_container = div().absolute().flex();

        match final_placement {
            Placement::Top | Placement::Bottom | Placement::TopStart | Placement::BottomStart | Placement::TopEnd | Placement::BottomEnd => {
                let container_width = px(2000.0);
                let ideal_center_x = anchor_bounds.left() + anchor_bounds.size.width / 2.0;

                // Clamping for horizontal centering
                let half_content_width = reference_size.width / 2.0;
                let clamped_center_x = ideal_center_x
                    .max(half_content_width)
                    .min(viewport_size.width - half_content_width);

                pivot_container = pivot_container
                    .w(container_width)
                    .left(clamped_center_x - container_width / 2.0);

                if final_placement == Placement::Top || final_placement == Placement::TopStart || final_placement == Placement::TopEnd {
                    // Use Top coordinate if possible to avoid bottom calculation confusion
                    // But our V3 used bottom. Let's make it robust.
                    let bottom_offset = viewport_size.height - anchor_bounds.top() + offset;
                    pivot_container = pivot_container.bottom(bottom_offset).flex_col_reverse();
                } else {
                    pivot_container = pivot_container.top(anchor_bounds.bottom() + offset).flex_col();
                }

                match final_placement {
                    Placement::Top | Placement::Bottom => {
                        pivot_container = pivot_container.justify_center().items_center();
                    }
                    Placement::TopStart | Placement::BottomStart => {
                        pivot_container = pivot_container.items_start();
                        pivot_container = pivot_container.pl(container_width / 2.0 - (clamped_center_x - anchor_bounds.left()));
                    }
                    Placement::TopEnd | Placement::BottomEnd => {
                        pivot_container = pivot_container.items_end();
                        pivot_container = pivot_container.pr(container_width / 2.0 - (anchor_bounds.right() - clamped_center_x));
                    }
                    _ => unreachable!()
                }
            }
            Placement::Left | Placement::Right | Placement::LeftStart | Placement::RightStart | Placement::LeftEnd | Placement::RightEnd => {
                let container_height = px(2000.0);
                let ideal_center_y = anchor_bounds.top() + anchor_bounds.size.height / 2.0;

                // Clamping for vertical centering
                let half_content_height = reference_size.height / 2.0;
                let clamped_center_y = ideal_center_y
                    .max(half_content_height)
                    .min(viewport_size.height - half_content_height);

                pivot_container = pivot_container
                    .h(container_height)
                    .top(clamped_center_y - container_height / 2.0);

                if final_placement == Placement::Left || final_placement == Placement::LeftStart || final_placement == Placement::LeftEnd {
                    let dist_from_right = viewport_size.width - anchor_bounds.left() + offset;
                    pivot_container = pivot_container.right(dist_from_right).flex_row_reverse();
                } else {
                    pivot_container = pivot_container.left(anchor_bounds.right() + offset).flex_row();
                }

                match final_placement {
                    Placement::Left | Placement::Right => {
                        pivot_container = pivot_container.justify_center().items_center();
                    }
                    Placement::LeftStart | Placement::RightStart => {
                        pivot_container = pivot_container.items_start();
                        pivot_container = pivot_container.pt(container_height / 2.0 - (clamped_center_y - anchor_bounds.top()));
                    }
                    Placement::LeftEnd | Placement::RightEnd => {
                        pivot_container = pivot_container.items_end();
                        pivot_container = pivot_container.pb(container_height / 2.0 - (anchor_bounds.bottom() - clamped_center_y));
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
                pivot_container
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
git add crates/liora-components/src/popover.rs
git commit -m "fix(popover): implement boundary clamping and fix TopCenter positioning"
```

---

### Task 2: Verify in Gallery

- [ ] **Step 1: Check TopCenter / BottomCenter at window edges**
Run: `cargo run -p liora-gallery`
(Verify that if the button is near the edge, the popover stays inside the window).

- [ ] **Step 2: Verify TopCenter visibility**
Confirm TopCenter popovers are now visible.
