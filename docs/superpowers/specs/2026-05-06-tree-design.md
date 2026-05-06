# Design Spec - Tree 树形控件

**日期**: 2026-05-06
**阶段**: P4 Data
**状态**: 待评审

## 1. 概述

Tree 树形控件用于展示分层数据，支持展开、折叠、选中等交互。

## 2. 核心功能

- **基础展示**: 层级化展示节点，通过缩进体现父子关系。
- **展开/折叠**: 点击展开图标切换子节点可见性。
- **选中功能**: 支持单选或多选（带 Checkbox）。
- **节点定制**: 支持自定义图标及节点内容。
- **懒加载**: 支持按需加载子节点。

## 3. 架构设计

### 3.1 组件结构

- `Tree`: 主视图 (View)，管理全局状态 (`expanded_keys`, `selected_keys`)。
- `TreeNode`: 数据模型，包含 `id`, `label`, `children`, `is_leaf`。

### 3.2 实现原理

- **递归渲染**: 使用递归函数渲染树结构。
- **状态维护**: 使用 `HashSet<SharedString>` 存储已展开的节点 ID。
- **缩进控制**: 根据节点深度 (`depth`) 动态设置左侧 padding。

### 3.3 关键状态

```rust
pub struct Tree {
    data: Vec<TreeNode>,
    expanded_keys: HashSet<SharedString>,
    selected_keys: HashSet<SharedString>,
    show_checkbox: bool,
    indent: Pixels,
}

pub struct TreeNode {
    id: SharedString,
    label: SharedString,
    children: Vec<TreeNode>,
}
```

## 4. API 设计

```rust
cx.new(|_| {
    Tree::new(vec![
        TreeNode::new("1", "一级 1")
            .child(TreeNode::new("1-1", "二级 1-1"))
            .child(TreeNode::new("1-2", "二级 1-2")),
        TreeNode::new("2", "一级 2")
    ])
    .show_checkbox(true)
})
```

## 5. UI 规范 (Tokens)

- **缩进**: 默认 18px。
- **高度**: 节点高度通常为 26-32px。
- **颜色**:
    - 悬浮: `theme.neutral.hover`
    - 选中: `theme.primary.opacity(0.1)`
- **图标**: `ChevronRight` / `ChevronDown` 切换。

## 6. 待办事项 (Todos)

- [ ] 实现 `Tree` View 结构及数据模型。
- [ ] 实现递归渲染逻辑。
- [ ] 实现展开/折叠交互。
- [ ] 实现选中状态展示。
- [ ] 注册到 Gallery Demo。
