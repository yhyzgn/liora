# Design Spec - Backtop 回到顶部

**日期**: 2026-05-06
**阶段**: P4 Navigation
**状态**: 待评审

## 1. 概述

Backtop 组件允许用户快速回到页面顶部。通常在滚动到一定距离后显示。

## 2. 核心功能

- **可见阈值**: 当滚动高度达到 `visibility_height` 时显示。
- **自定义样式**: 支持自定义内容及位置 (`right`, `bottom`)。
- **点击回顶**: 点击后平滑（或即时）滚动到容器顶部。

## 3. 架构设计

### 3.1 组件结构

- `Backtop`: `View` 组件，因为需要根据滚动状态切换自身可见性。
- 绑定 `ScrollHandle` 以监听滚动偏移量。

### 3.2 实现原理

- 在 `render` 中，监听 `ScrollHandle` 的状态变化。
- 比较 `scroll_handle.offset().y` 与 `visibility_height`。
- 如果需要显示，则渲染一个绝对定位在右下角的按钮。

### 3.3 关键状态

```rust
pub struct Backtop {
    scroll_handle: ScrollHandle,
    visibility_height: Pixels,
    right: Pixels,
    bottom: Pixels,
    content: Option<Box<dyn Fn(&mut Window, &mut Context<Backtop>) -> AnyElement + 'static>>,
}
```

## 4. API 设计

```rust
let scroll_handle = ScrollHandle::new();
// ... 在渲染列表时使用该 handle
cx.new(|_| {
    Backtop::new(scroll_handle)
        .visibility_height(px(200.0))
        .right(px(40.0))
        .bottom(px(40.0))
})
```

## 5. UI 规范 (Tokens)

- **尺寸**: 默认 40x40px 圆形。
- **阴影**: 通常需要 `shadow_lg` 以突出悬浮感。
- **图标**: 默认 `CaretUp` 或 `ArrowUp`。

## 6. 待办事项 (Todos)

- [ ] 实现 `Backtop` View 结构。
- [ ] 实现滚动状态监听与可见性切换。
- [ ] 实现点击回顶逻辑。
- [ ] 注册到 Gallery Demo。
