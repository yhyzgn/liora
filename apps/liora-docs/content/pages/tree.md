# Tree

树形控件用于展示层级数据，支持展开/折叠、节点选择、多选和勾选样式。

## 基础层级

用 `TreeNode::child` 构建嵌套节点，点击有子节点的行可展开或折叠。

### 效果

::LioraDemo{component="TreeBasic"}::

### 代码

```rust src="tree/basic.rs"
```

## 勾选与多选

`show_checkbox(true)` 展示复选框，`multiple(true)` 允许多个节点同时选中，节点点击回调可同步业务状态。

### 效果

::LioraDemo{component="TreeCheckable"}::

### 代码

```rust src="tree/checkable.rs"
```
