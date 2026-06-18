# VirtualizedTable

`VirtualizedTable` 是面向大数据行的原生虚拟表格：表头固定在容器顶部，表体由 GPUI `ListState` 只渲染可见行，并通过 Liora `VirtualScrollbar` 显示滚动位置。

## 能力

- 固定表头：列定义沿用 `TableColumn`，支持宽度、对齐、sortable。
- 行虚拟化：通过 `row_count + render_cell(row, key, ...)` 生成单元格，不缓存跨帧 `AnyElement`。
- 可配置高度、行高、overdraw、斑马纹、边框、加载和空状态。
- 排序状态外置：`on_sort_change` 把三态排序交回业务层，适合本地 index 映射或后端分页查询。

## 万行固定表头

### 效果

::LioraDemo{component="VirtualizedTableBasic"}::

### 代码

```rust src="virtualized_table/basic.rs"
```

## 排序状态

### 效果

::LioraDemo{component="VirtualizedTableSortable"}::

### 代码

```rust src="virtualized_table/sortable.rs"
```

## 设计说明

`VirtualizedTable` 和普通 `Table` 的差异在于数据流：普通 `Table` 接收完整 `TableRow` 列表；虚拟表格只接收总行数和单元格渲染闭包。这样可以避免万级行一次性创建所有 GPUI 元素，也避免把 frame-local 元素缓存到组件状态中。
