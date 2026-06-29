# VirtualizedTree

`VirtualizedTree` 是大型层级数据的原生虚拟树控件。它把当前展开状态下的节点 flatten 成轻量 `VirtualTreeItem`，再交给 GPUI `ListState` 只渲染可见行。

## 虚拟化性能表现

`VirtualizedTree` 先按展开状态 flatten 当前可见节点，再交给 `ListState` 虚拟渲染。未展开分支不会生成行元素；大组织树、文件树、权限树可以保持低布局压力。

## 能力

- 大树可见区渲染：展开/折叠时只维护可见节点元数据，不缓存跨帧 GPUI 元素。
- 展开/折叠：支持默认展开 key、手动点击展开图标、`expand_all`。
- 选择状态：支持单选、多选、checkbox 风格与 `on_node_click` 回调。
- 可配置高度、行高、缩进、overdraw，并内置 Liora `VirtualScrollbar`。

## 大型组织树

### 效果

::Demo{component="VirtualizedTreeBasic"}::

### 代码

```rust src="virtualized_tree/basic.rs"
```

## 多选与回调

### 效果

::Demo{component="VirtualizedTreeCheckable"}::

### 代码

```rust src="virtualized_tree/checkable.rs"
```

## 设计说明

普通 `Tree` 适合中小规模节点；`VirtualizedTree` 则面向组织架构、权限树、文件树等大规模层级数据。它的状态只保存原始 `TreeNode`、展开 key、选择 key 和可见节点元数据，每一行在进入可见区时才生成元素，避免 stale `ArenaRef` 与一次性布局压力。
