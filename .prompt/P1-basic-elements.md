# P1 Basic Elements — 基础物料层

> 上游: `.prompt/P0-foundation.md` | 主文档: `architecture-design.md#四`

## 目标

完成 13 个基础组件的开发和 Gallery Demo 注册。

## 组件清单 (共 13 个，按优先级排列)

### 第一优先级 (完善现有)
1. **Button 增强** — 添加 `.icon_start()` / `.icon_end()` 支持、`ButtonGroup`、幽灵按钮 (text variant)
   - 文件: `crates/liora-components/src/button.rs` (修改)
   - 文件: `crates/liora-components/src/button_group.rs` (新建)
   - Demo: 更新 `apps/liora-gallery/src/demos/button_demo.rs`

### 第二优先级 (核心布局与排版)
2. **Link** — 链接按钮 (underline, hover 变色)
3. **Text** — 单行文本组件 (截断 ellipsis, 行数限制)
4. **Title** — 标题组件 (h1-h6 级别, 字重)
5. **Paragraph** — 段落组件 (行高, 首行缩进)
6. **Space** — 间距包裹组件 (横向/纵向自动 gap)
7. **Divider** — 分割线 (横向/纵向, 带文字, 虚线样式)

### 第三优先级 (布局系统)
8. **Row** — 栅格行 (gutter, justify, align)
9. **Col** — 栅格列 (span, offset, push, pull)
10. **Container** — 布局容器 (header/aside/main/footer)
11. **Scrollbar** — 自定义滚动条
12. **Splitter** — 分隔面板 (拖拽调整宽度)

### 第四优先级 (图标升级)
13. **Icon** — SVG 图标集成 (选择 Lucide 或 Element Icons 作为图标集)

## 开发流程

```
每个组件:
  1. 创建/修改 crates/liora-components/src/<name>.rs
  2. 在 crates/liora-components/src/lib.rs 中 pub mod + pub use
  3. 创建 apps/liora-gallery/src/demos/<name>_demo.rs (render() -> AnyElement)
  4. 在 apps/liora-gallery/src/demos/mod.rs 注册表添加 DemoEntry
  5. cargo check 通过
  6. cargo run -p liora-gallery 验证窗口效果
  7. git add + commit + push
  8. 更新 .memory/inventory.md
```

## Demo 编写规范

```rust
use gpui::{div, prelude::*, px, AnyElement, App, Component, RenderOnce, Window};

pub fn render() -> AnyElement {
    Component::new(NameDemo).into_any_element()
}

struct NameDemo;

impl RenderOnce for NameDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<liora_core::Config>().theme;
        div().flex().flex_col().gap_4()
            .child(section(theme, "Variants 变体"))
            .child(demo_row(vec![...]))
            .child(section(theme, "Sizes 尺寸"))
            .child(demo_row(vec![...]))
            .child(section(theme, "States 状态"))
            .child(demo_row(vec![...]))
    }
}
```

## 布局组件特殊说明

Row/Col 栅格系统参照 Element-Plus 24 栅格:
```rust
LioraRow::new()
    .gutter(px(20.0))
    .child(LioraCol::new().span(12).child(...))
    .child(LioraCol::new().span(6).offset(6).child(...))
```

## 完成标准

- [ ] 全部 13 个组件编译通过 (cargo check 0 errors)
- [ ] 每个组件在 Gallery 中有 Demo 卡片
- [ ] `cargo run -p liora-gallery` 可滚动查看全部组件
- [ ] Git commit 已推送
- [ ] .memory/ 已更新 (state.md, inventory.md)
- [ ] .prompt/P2-form-controls.md 已就绪
