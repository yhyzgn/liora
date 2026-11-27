# TreeSelect

`TreeSelect` 将树形层级数据放入选择器面板中，适合权限、目录、组织架构和组件分类选择。

## 单选

### 效果

::Demo{component="TreeSelectSingle"}::

### 代码

```rust src="tree_select/single.rs"
```

## 多选与禁用节点

### 效果

::Demo{component="TreeSelectMultiple"}::

### 代码

```rust src="tree_select/multiple.rs"
```

## 搜索过滤

移除 `TreeSelect` 的最后一个持有者后，内部搜索框的回调不会继续持有该选择器。

### 效果

::Demo{component="TreeSelectFilterable"}::

### 代码

```rust src="tree_select/filterable.rs"
```
