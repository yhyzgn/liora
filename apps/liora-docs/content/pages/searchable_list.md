# SearchableList

`SearchableList` 是可复用的过滤列表底座，适合被 `Select::searchable`、命令面板、设置项搜索和资源选择器复用。它统一处理 value/label/description/group/disabled/selected/empty state，避免每个复合控件重复实现过滤和分组渲染。

## 基础分组列表

### 效果

::Demo{component="SearchableListBasic"}::

### 代码

```rust src="searchable_list/basic.rs"
```

## 查询过滤

### 效果

::Demo{component="SearchableListFiltered"}::

### 代码

```rust src="searchable_list/filtered.rs"
```

## 空态与数量限制

### 效果

::Demo{component="SearchableListEmpty"}::

### 代码

```rust src="searchable_list/empty.rs"
```
