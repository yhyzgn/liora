# Design Spec - Tabs 标签页

**日期**: 2026-05-06
**阶段**: P4 Navigation
**状态**: 待评审

## 1. 概述

Tabs 组件用于在同一区域展示多个面板，通过点击标签进行切换。支持多种风格和位置，以及可编辑状态。

## 2. 核心功能

- **基础切换**: 响应点击事件，同步更新选中项和面板内容。
- **多种风格**:
    - `Standard`: 极简风格，带下划线指示器。
    - `Card`: 卡片风格，标签有边框和背景。
    - `BorderCard`: 带边框的卡片风格，内容区也有边框。
- **标签位置**: `Top` (默认), `Bottom`, `Left`, `Right`。
- **可编辑**: 支持动态新增和关闭标签。
- **延迟渲染**: 默认仅渲染当前激活的面板内容。

## 3. 架构设计

### 3.1 组件结构

- `Tabs`: 主视图 (View)，维护 `active_name` (SharedString) 状态。
- `TabPane`: 面板项，定义 `label` (标签文案), `name` (唯一标识), `content` (渲染闭包), `closable` (是否可关闭)。

### 3.2 渲染策略

- **Header**: 使用 `flex` 容器展示所有 `TabPane` 的标签。
- **Indicator**: 针对 `Standard` 模式，实现一个跟随选中项移动的下划线（首期先实现静态切换）。
- **Panels**: 使用 `div` 包裹，根据 `active_name` 匹配显示对应的内容。

### 3.3 关键状态

```rust
pub struct Tabs {
    active_name: SharedString,
    position: TabPosition,
    tab_type: TabType,
    panes: Vec<TabPane>,
    editable: bool,
    on_tab_click: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    on_tab_remove: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    on_tab_add: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}
```

## 4. API 设计

```rust
cx.new(|_| {
    Tabs::new("first")
        .type_(TabType::Card)
        .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
        .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
        .closable(true)
        .on_tab_click(|name, _, _| println!("Clicked {}", name))
})
```

## 5. UI 规范 (Tokens)

- **颜色**:
    - 激活文字: `theme.primary.base`
    - 激活下划线: `theme.primary.base`
    - Hover 背景: `theme.neutral.hover`
    - 边框: `theme.neutral.border`
- **间距**: 标签间距通常为 20-30px，高度约 40px。

## 6. 待办事项 (Todos)

- [ ] 实现 `Tabs` View 基础结构。
- [ ] 实现 Header 渲染 (支持四种位置)。
- [ ] 实现 `Standard`, `Card`, `BorderCard` 三种样式。
- [ ] 实现标签关闭/新增交互。
- [ ] 注册到 Gallery Demo。
