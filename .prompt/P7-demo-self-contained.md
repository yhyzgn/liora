# P7 Demo Self-Contained — Demo 完全自举

> 上游: `.prompt/P6-builtin-id.md`

## 目标

Gallery Demo 完全使用 Liora 组件库自身控件构建，避免在 Demo 中直接使用 GPUI 原生组件。若发现需要的控件缺失，应自行新增到组件库中。

## 动机

当前 Gallery Demo 中存在大量直接使用 `div()`、`div().flex()` 等 GPUI 原生 API 的情况，这导致：
- **Demo 无法展示组件库真实能力** — 用户看到的是 GPUI 用法，不是 Liora 用法
- **组件库缺失信号** — Demo 中手写的布局/样式模式没有沉淀为可复用组件
- **风格不一致** — 混用原生 GPUI 和 Liora 控件导致 Demo 外观不统一
- **"吃自己的狗粮"缺失** — 组件库自身不用自己的组件，难以发现 API 问题

## 要求

### 1. Demo 中禁止直接使用 GPUI 原生布局/样式 API

```rust
// ❌ 禁止 — 直接使用 div() 构建 demo 布局
fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
    div().flex().flex_col().gap_4()
        .child(div().text_xl().font_weight(FontWeight::BOLD).child("Title"))
        .child(div().child("Content"))
}

// ✅ 正确 — 使用 Liora 组件构建完整 demo
fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
    Container::new()
        .child(Title::new("Title").level(2))
        .child(Paragraph::new().with_text("Content"))
}
```

### 2. 缺失控件应新增到组件库

如果 Demo 需要某种布局/样式模式但组件库中没有对应控件，应按以下优先级处理：

| 优先级 | 处理方式 | 适用场景 |
|--------|---------|---------|
| 1 | 使用现有 Liora 控件组合 | 能用 Space + Container + Text 等已有控件拼出来的效果 |
| 2 | 新增通用控件到组件库 | 确实缺失且具有复用价值的控件 |
| 3 | 扩展现有控件能力 | 现有控件已存在但缺少某个 builder 方法 |

### 3. Demo 辅助函数规范

Demo 中允许使用纯函数 helper，但这些 helper 必须：
- 返回 Liora 控件（`Container`, `Space`, `Text`, `Divider` 等）
- 不直接调用 `div()`, `px()`, `rgb()` 等 GPUI 原语

```rust
// ✅ 正确的 demo helper — 使用 Liora 控件
fn section_header(title: &str) -> Container {
    Container::new()
        .child(Title::new(title).level(3))
        .child(Divider::new())
}

// ❌ 错误的 demo helper — 直接使用 GPUI 原语
fn section_header(title: &str) -> Div {
    div().flex().flex_col()
        .child(div().text_lg().font_weight(FontWeight::BOLD).child(title))
        .child(div().h_1px().bg(rgb(0xdcdfe6)))
}
```

### 4. 现有 Demo 改造范围

逐文件审查 `apps/liora-gallery/src/demos/*_demo.rs`：

- [ ] `button_demo.rs`
- [ ] `link_demo.rs`
- [ ] `text_demo.rs`
- [ ] `title_demo.rs`
- [ ] `paragraph_demo.rs`
- [ ] `space_demo.rs`
- [ ] `divider_demo.rs`
- [ ] `row_demo.rs`
- [ ] `col_demo.rs`
- [ ] `container_demo.rs`
- [ ] `scrollbar_demo.rs`
- [ ] `splitter_demo.rs`
- [ ] `button_group_demo.rs`
- [ ] `input_demo.rs`
- [ ] `input_number_demo.rs`
- [ ] `textarea_demo.rs`
- [ ] `checkbox_demo.rs`
- [ ] `radio_demo.rs`
- [ ] `switch_demo.rs`
- [ ] `select_demo.rs`
- [ ] `slider_demo.rs`
- [ ] `form_demo.rs`
- [ ] `rate_demo.rs`
- [ ] `tooltip_demo.rs`
- [ ] `popover_demo.rs`
- [ ] `popconfirm_demo.rs`
- [ ] `dialog_demo.rs`
- [ ] `drawer_demo.rs`
- [ ] `message_demo.rs`
- [ ] `notification_demo.rs`
- [ ] `alert_demo.rs`
- [ ] `loading_demo.rs`
- [ ] `message_box_demo.rs`
- [ ] `dropdown_demo.rs`
- [ ] `card_demo.rs`
- [ ] `collapse_demo.rs`
- [ ] `menu_demo.rs`
- [ ] `tabs_demo.rs`
- [ ] `breadcrumb_demo.rs`
- [ ] `steps_demo.rs`
- [ ] `page_header_demo.rs`
- [ ] `affix_demo.rs`
- [ ] `backtop_demo.rs`
- [ ] `anchor_demo.rs`
- [ ] `progress_demo.rs`
- [ ] `skeleton_demo.rs`
- [ ] `empty_demo.rs`
- [ ] `result_demo.rs`
- [ ] `descriptions_demo.rs`
- [ ] `timeline_demo.rs`
- [ ] `tree_demo.rs`
- [ ] `pagination_demo.rs`
- [ ] `statistic_demo.rs`
- [ ] `segmented_demo.rs`
- [ ] `tag_demo.rs`
- [ ] `avatar_demo.rs`
- [ ] `badge_demo.rs`

以及核心框架文件：
- [ ] `apps/liora-gallery/src/main.rs`
- [ ] `apps/liora-gallery/src/category.rs`

### 5. 可能需要新增的控件

以下控件可能在 Demo 改造中发现缺失，需要新增：

| 潜在缺失控件 | 用途 | 优先级 |
|-------------|------|--------|
| `PageLayout` / `PageContainer` | Demo 页面级容器（header + body + footer 模式） | 高 |
| `FlexRow` / `FlexCol` | 语义化的 flex 布局容器（如果 Space 不足以覆盖） | 中 |
| `DemoBlock` | 代码演示用的卡片容器（title + description + preview） | 高 |
| `ColorSwatch` | 主题色板展示 | 低 |

### 6. The Order Of Components in demo

Must be Ordered by Component's name with dictionary, ASC

## 验证标准

1. `cargo check` 0 errors, 0 warnings
2. 搜索 `apps/liora-gallery/src/demos/` 下所有文件，无直接 `div().flex().flex_col()` 等 GPUI 布局原语（仅 `liora-components` 内部实现可保留）
3. Gallery 运行正常，所有 Demo 页面视觉一致
4. Gallery 的 `category.rs` 和 `main.rs` 也使用 Liora 控件
