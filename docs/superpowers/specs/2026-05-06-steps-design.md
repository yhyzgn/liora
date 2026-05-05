# Design Spec - Steps 步骤条

**日期**: 2026-05-06
**阶段**: P4 Navigation
**状态**: 待评审

## 1. 概述

Steps 步骤条组件引导用户按照流程完成任务。支持水平/垂直方向，以及多种状态展示。

## 2. 核心功能

- **基础流程**: 展示步骤序列，反映当前进度。
- **状态管理**: 每个步骤支持 `Wait` (等待), `Process` (进行中), `Finish` (已完成), `Error` (错误) 四种状态。
- **方向选择**: `Horizontal` (水平, 默认) 和 `Vertical` (垂直)。
- **自定义图标**: 允许为每个步骤设置特定的图标。
- **详细描述**: 除了主标题外，还支持辅助描述文字。

## 3. 架构设计

### 3.1 组件结构

- `Steps`: 容器组件，管理 `active` (当前激活索引) 和 `direction`。
- `Step`: 节点数据模型，包含 `title`, `description`, `icon`, `status`。

### 3.2 渲染策略

- **水平模式**: 使用 `flex-row`。步骤之间通过连接线贯穿。
- **垂直模式**: 使用 `flex-col`。连接线位于图标下方。
- **状态颜色**:
    - `Finish`: `theme.primary.base`
    - `Process`: `theme.neutral.text_1`
    - `Wait`: `theme.neutral.text_3`
    - `Error`: `theme.danger.base`

### 3.3 关键状态

```rust
pub struct Steps {
    active: usize,
    direction: StepsDirection,
    items: Vec<StepItem>,
}

pub enum StepsDirection {
    Horizontal,
    Vertical,
}

pub enum StepStatus {
    Wait,
    Process,
    Finish,
    Error,
}

pub struct StepItem {
    title: SharedString,
    description: Option<SharedString>,
    icon: Option<IconName>,
    status: Option<StepStatus>, // 如果为 None，则根据 active 自动计算
}
```

## 4. API 设计

```rust
Steps::new()
    .active(1)
    .direction(StepsDirection::Horizontal)
    .step(StepItem::new("步骤 1").description("这是一段描述性文字"))
    .step(StepItem::new("步骤 2").icon(IconName::User))
    .step(StepItem::new("步骤 3"))
```

## 5. UI 规范 (Tokens)

- **尺寸**: 图标容器通常为 24x24px 或 32x32px。
- **连接线**: 1px 实线或虚线。
- **字体**: 标题 14px (Bold), 描述 12px。

## 6. 待办事项 (Todos)

- [ ] 实现 `Steps` 基础结构。
- [ ] 实现步骤状态自动计算逻辑。
- [ ] 实现水平模式渲染。
- [ ] 实现垂直模式渲染。
- [ ] 注册到 Gallery Demo。
