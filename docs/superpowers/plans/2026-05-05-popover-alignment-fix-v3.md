# Robust Popover Centering and Positioning Plan V2

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix Popover centering and disappearance by using wide/tall pivot containers that enable robust flexbox centering without knowing content size.

**Architecture:**
1. `PopoverView` will calculate the target `final_placement` using `Popper`'s flip logic.
2. It will create a "Wide Pivot Container" (2000px wide for vertical placements) or a "Tall Pivot Container" (2000px tall for horizontal placements).
3. The pivot container's center will be aligned with the anchor's center point.
4. By using `flex` and `justify_center` on this large container, the popover content will be perfectly centered on the pivot, regardless of its actual size.
5. All calculations will use `final_placement` to ensure correct anchoring even after flipping.

**Tech Stack:** Rust, GPUI 0.2.2

---

### Task 1: Implement Wide/Tall Pivot Container Positioning

**Files:**
- Modify: `crates/liora-components/src/popover.rs`

- [ ] **Step 1: Rewrite PopoverView::render with robust centering**

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
        let viewport = Bounds {
            origin: gpui::Point::default(),
            size: viewport_size,
        };

        // Use Popper just for flip logic
        let popper = liora_core::Popper {
            anchor_bounds,
            placement,
            offset,
        };
        // Reference size for flipping
        let reference_size = gpui::Size { width: px(200.0), height: px(150.0) };
        let (_pos, final_placement) = popper.calculate_position_with_flip(reference_size, viewport);

        let mut pivot_container = div().absolute().flex();

        match final_placement {
            Placement::Top | Placement::Bottom | Placement::TopStart | Placement::BottomStart | Placement::TopEnd | Placement::BottomEnd => {
                // Vertical placements use a WIDE container for horizontal centering
                let container_width = px(2000.0);
                let center_x = anchor_bounds.left() + anchor_bounds.size.width / 2.0;

                pivot_container = pivot_container
                    .w(container_width)
                    .left(center_x - container_width / 2.0);

                if final_placement == Placement::Top || final_placement == Placement::TopStart || final_placement == Placement::TopEnd {
                    let dist_from_bottom = viewport_size.height - anchor_bounds.top() + offset;
                    pivot_container = pivot_container.bottom(dist_from_bottom).flex_col_reverse();
                } else {
                    pivot_container = pivot_container.top(anchor_bounds.bottom() + offset).flex_col();
                }

                match final_placement {
                    Placement::Top | Placement::Bottom => {
                        pivot_container = pivot_container.justify_center().items_center();
                    }
                    Placement::TopStart | Placement::BottomStart => {
                        pivot_container = pivot_container.items_start();
                        // Adjust left to match anchor left (container is centered on anchor center)
                        pivot_container = pivot_container.pl(container_width / 2.0 - anchor_bounds.size.width / 2.0);
                    }
                    Placement::TopEnd | Placement::BottomEnd => {
                        pivot_container = pivot_container.items_end();
                        // Adjust right to match anchor right
                        pivot_container = pivot_container.pr(container_width / 2.0 - anchor_bounds.size.width / 2.0);
                    }
                    _ => unreachable!()
                }
            }
            Placement::Left | Placement::Right | Placement::LeftStart | Placement::RightStart | Placement::LeftEnd | Placement::RightEnd => {
                // Horizontal placements use a TALL container for vertical centering
                let container_height = px(2000.0);
                let center_y = anchor_bounds.top() + anchor_bounds.size.height / 2.0;

                pivot_container = pivot_container
                    .h(container_height)
                    .top(center_y - container_height / 2.0);

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
                        pivot_container = pivot_container.pt(container_height / 2.0 - anchor_bounds.size.height / 2.0);
                    }
                    Placement::LeftEnd | Placement::RightEnd => {
                        pivot_container = pivot_container.items_end();
                        pivot_container = pivot_container.pb(container_height / 2.0 - anchor_bounds.size.height / 2.0);
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
git commit -m "fix(popover): ultra-robust centering using wide/tall pivot containers"
```

---

### Task 2: Verify in Gallery

- [ ] **Step 1: Check TopCenter / BottomCenter / LeftCenter / RightCenter**
Run: `cargo run -p liora-gallery`
(Verify all centering variants are perfectly aligned and TopCenter appears correctly).

- [ ] **Step 2: Check Start/End variants**
Verify TopStart, TopEnd, etc., still align correctly with anchor edges.
