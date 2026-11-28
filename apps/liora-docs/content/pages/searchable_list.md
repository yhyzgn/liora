# SearchableList

`SearchableList` 是可复用的过滤列表底座，适合被 `Select::searchable`、命令面板、设置项搜索和资源选择器复用。它统一处理 value/label/description/group/disabled/selected/empty state，避免每个复合控件重复实现过滤和分组渲染。

匹配行超过 `max_height`（默认 260px）时，列表在限定高度内纵向滚动；`max_items` 仍限制匹配结果总数，而不是滚动视口内的行数。

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
