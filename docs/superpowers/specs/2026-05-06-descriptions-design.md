# Design Spec - Descriptions 描述列表

**日期**: 2026-05-06
**阶段**: P4 Data
**状态**: 待评审

## 1. 概述

Descriptions 描述列表组件用于展示多个字段。支持多种布局方向及边框样式。

## 2. 核心功能

- **基础展示**: 以“标签: 内容”的形式排列。
- **列数控制**: 支持自定义每行显示的列数 (`column`)。
- **布局方向**:
    - `Horizontal`: 标签与内容在同一行。
    - `Vertical`: 标签在内容上方。
- **边框样式**: 支持带边框 (`border`) 模式。
- **插槽支持**: 标题 (`title`)、操作区 (`extra`)。

## 3. 架构设计

### 3.1 组件结构

- `Descriptions`: 容器组件，管理全局布局 (column, direction, border)。
- `DescriptionItem`: 子项组件，包含 `label`, `value`, `span` (占据列数)。

### 3.2 渲染策略

- **Grid 模拟**: GPUI 暂无原生 Grid 布局，通过 `flex-row` 结合百分比宽度模拟。
- **Border**: 带边框模式下，通过 `border_1` 配合各单元格的边框处理实现表格感。
- **自动换行**: 根据 `column` 和项的 `span` 自动计算换行。

### 3.3 关键状态

```rust
pub struct Descriptions {
    title: Option<SharedString>,
    extra: Option<AnyElement>,
    column: u32,
    direction: DescriptionsDirection,
    border: bool,
    items: Vec<DescriptionItem>,
}

pub struct DescriptionItem {
    label: SharedString,
    value: AnyElement,
    span: u32,
}

pub enum DescriptionsDirection { Horizontal, Vertical }
```

## 4. API 设计

```rust
Descriptions::new()
    .title("用户信息")
    .column(3)
    .border(true)
    .item("用户名", "kooriookami")
    .item("手机号", "18100000000")
    .item("居住地", "苏州市")
    .item("联系地址", "江苏省苏州市吴中区越溪街道月苑路", 2)
```

## 5. UI 规范 (Tokens)

- **背景**: 标签背景通常为 `theme.neutral.hover` (带边框模式)。
- **文字**: 标签文字加粗或使用 `text_2`。内容文字使用 `text_1`。
- **边距**: 单元格内边距约 12-16px。

## 6. 待办事项 (Todos)

- [ ] 实现 `Descriptions` 基础结构。
- [ ] 实现动态列宽计算逻辑。
- [ ] 实现带边框布局渲染。
- [ ] 注册到 Gallery Demo。
