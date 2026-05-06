# Design Spec - Timeline 时间线

**日期**: 2026-05-06
**阶段**: P4 Data
**状态**: 待评审

## 1. 概述

Timeline 时间线组件垂直展示一系列信息。常用于日志、操作历史、物流信息等。

## 2. 核心功能

- **基础展示**: 信息点 + 描述 + 时间戳。
- **自定义节点**: 支持自定义圆点颜色、类型（实心/空心）或自定义 Icon。
- **布局模式**:
    - `Left`: 节点在左，内容在右（默认）。
    - `Right`: 节点在右，内容在左。
    - `Alternate`: 左右交替展示。
- **倒序排列**: 支持数据倒序展示。

## 3. 架构设计

### 3.1 组件结构

- `Timeline`: 容器组件。
- `TimelineItem`: 节点组件，包含 `timestamp`, `content`, `color`, `icon`, `hollow`。

### 3.2 渲染策略

- **纵向轴线**: 所有的 `TimelineItem` 共享一条垂直线（通过第一个到最后一个节点的中心连接）。
- **节点定位**: 节点圆点相对于内容区绝对定位或通过 flex 布局。
- **轴线断开**: 最后一个节点下方不显示轴线。

### 3.3 关键状态

```rust
pub struct Timeline {
    items: Vec<TimelineItem>,
    reverse: bool,
    mode: TimelineMode,
}

pub struct TimelineItem {
    timestamp: Option<SharedString>,
    content: AnyElement,
    color: Option<Hsla>,
    icon: Option<IconName>,
    hollow: bool,
}

pub enum TimelineMode { Left, Right, Alternate }
```

## 4. API 设计

```rust
Timeline::new()
    .item(TimelineItem::new("2026-05-06").content("创建成功").color(theme.success.base))
    .item(TimelineItem::new("2026-05-07").content("通过审核"))
    .item(TimelineItem::new("2026-05-08").content("项目发布").hollow(true))
```

## 5. UI 规范 (Tokens)

- **圆点尺寸**: 默认直径 12px。
- **线宽**: 2px。
- **颜色**:
    - 线条: `theme.neutral.border`
    - 默认圆点: `theme.neutral.text_3`
- **间距**: 节点垂直间距约 20-30px。

## 6. 待办事项 (Todos)

- [ ] 实现 `TimelineItem` 渲染。
- [ ] 实现 `Timeline` 容器及轴线逻辑。
- [ ] 实现不同状态与颜色的节点。
- [ ] 实现 `Alternate` 模式（可选）。
- [ ] 注册到 Gallery Demo。
