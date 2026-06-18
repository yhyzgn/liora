# P3 Popper + Feedback Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement 13 feedback and popper-based components leveraging the Liora Popper foundation.

**Architecture:** Use `RenderOnce + IntoElement` (codex) paradigm. Popper components will use `Popper::calculate_position_with_flip` and `push_portal`. Global feedback (Message/Notification) will use specialized singleton managers.

**Tech Stack:** Rust (Edition 2024), GPUI 0.2.2, Liora-Core (Popper Foundation).

---

### Task 1: Tooltip (文字提示)

**Files:**
- Create: `crates/liora-components/src/tooltip.rs`
- Modify: `crates/liora-components/src/lib.rs`
- Create: `apps/liora-gallery/src/demos/tooltip_demo.rs`
- Modify: `apps/liora-gallery/src/demos/mod.rs`

- [ ] **Step 1: Implement Tooltip Component**
```rust
pub struct Tooltip {
    content: SharedString,
    placement: Placement,
    // ...
}
impl RenderOnce for Tooltip { ... }
```
- [ ] **Step 2: Add to components lib.rs**
- [ ] **Step 3: Create Tooltip Demo**
- [ ] **Step 4: Register in Gallery**
- [ ] **Step 5: Verify & Commit**

### Task 2: Popover (气泡卡片)

**Files:**
- Create: `crates/liora-components/src/popover.rs`
- Modify: `crates/liora-components/src/lib.rs`
- Create: `apps/liora-gallery/src/demos/popover_demo.rs`

- [ ] **Step 1: Implement Popover (supports arbitrary child content)**
- [ ] **Step 2: Add to lib.rs**
- [ ] **Step 3: Create Demo & Register**
- [ ] **Step 4: Verify & Commit**

### Task 3: Popconfirm (气泡确认框)

**Files:**
- Create: `crates/liora-components/src/popconfirm.rs`
- Create: `apps/liora-gallery/src/demos/popconfirm_demo.rs`

- [ ] **Step 1: Implement Popconfirm (Title + Confirm/Cancel buttons)**
- [ ] **Step 2: Add to lib.rs and Demo**
- [ ] **Step 3: Verify & Commit**

### Task 4: Dialog (模态对话框)

**Files:**
- Create: `crates/liora-components/src/dialog.rs`
- Create: `apps/liora-gallery/src/demos/dialog_demo.rs`

- [ ] **Step 1: Implement Dialog (Overlay + Centered content + ESC close)**
- [ ] **Step 2: Implement FocusTrap logic**
- [ ] **Step 3: Add to lib.rs and Demo**
- [ ] **Step 4: Verify & Commit**

### Task 5: Drawer (抽屉)

**Files:**
- Create: `crates/liora-components/src/drawer.rs`
- Create: `apps/liora-gallery/src/demos/drawer_demo.rs`

- [ ] **Step 1: Implement Drawer (Left/Right/Top/Bottom slide in)**
- [ ] **Step 2: Add to lib.rs and Demo**
- [ ] **Step 3: Verify & Commit**

### Task 6: Message (全局消息)

**Files:**
- Create: `crates/liora-components/src/message.rs`
- Create: `apps/liora-gallery/src/demos/message_demo.rs`

- [ ] **Step 1: Implement Message Manager (Global stack for top-center toasts)**
- [ ] **Step 2: Implement Message entry with auto-close**
- [ ] **Step 3: Add to lib.rs and Demo**
- [ ] **Step 4: Verify & Commit**

### Task 7: Notification (通知)

**Files:**
- Create: `crates/liora-components/src/notification.rs`
- Create: `apps/liora-gallery/src/demos/notification_demo.rs`

- [ ] **Step 1: Implement Notification Manager (Top-right corner stack)**
- [ ] **Step 2: Implement Notification entry with Title/Desc/Icon**
- [ ] **Step 3: Add to lib.rs and Demo**
- [ ] **Step 4: Verify & Commit**

### Task 8: Alert (警告)

**Files:**
- Create: `crates/liora-components/src/alert.rs`
- Create: `apps/liora-gallery/src/demos/alert_demo.rs`

- [ ] **Step 1: Implement Alert (Success/Info/Warning/Error themes)**
- [ ] **Step 2: Add to lib.rs and Demo**
- [ ] **Step 3: Verify & Commit**

### Task 9: Loading (加载)

**Files:**
- Create: `crates/liora-components/src/loading.rs`
- Create: `apps/liora-gallery/src/demos/loading_demo.rs`

- [ ] **Step 1: Implement Loading Spinner & Overlay**
- [ ] **Step 2: Add to lib.rs and Demo**
- [ ] **Step 3: Verify & Commit**

### Task 10: Card (卡片)

**Files:**
- Create: `crates/liora-components/src/card.rs`
- Create: `apps/liora-gallery/src/demos/card_demo.rs`

- [ ] **Step 1: Implement Card (Shadow/Border variants, Header/Footer slots)**
- [ ] **Step 2: Add to lib.rs and Demo**
- [ ] **Step 3: Verify & Commit**

### Task 11: Collapse (折叠面板)

**Files:**
- Create: `crates/liora-components/src/collapse.rs`
- Create: `apps/liora-gallery/src/demos/collapse_demo.rs`

- [ ] **Step 1: Implement Collapse (Accordion mode, custom headers)**
- [ ] **Step 2: Add to lib.rs and Demo**
- [ ] **Step 3: Verify & Commit**

### Task 12: Dropdown (下拉菜单)

**Files:**
- Create: `crates/liora-components/src/dropdown.rs`
- Create: `apps/liora-gallery/src/demos/dropdown_demo.rs`

- [ ] **Step 1: Implement Dropdown (Trigger + Menu items + Nested submenus)**
- [ ] **Step 2: Add to lib.rs and Demo**
- [ ] **Step 3: Verify & Commit**

### Task 13: MessageBox (弹窗消息)

**Files:**
- Create: `crates/liora-components/src/message_box.rs`
- Create: `apps/liora-gallery/src/demos/message_box_demo.rs`

- [ ] **Step 1: Implement MessageBox helper (Alert/Confirm/Prompt dialogs)**
- [ ] **Step 2: Add to lib.rs and Demo**
- [ ] **Step 3: Verify & Commit**
