# VirtualizedTable

`VirtualizedTable` 是面向大数据行的原生虚拟表格：表头固定在容器顶部，表体由 GPUI `ListState` 只渲染可见行，并通过 Liora `VirtualScrollbar` 显示滚动位置。

## 虚拟化性能表现

`VirtualizedTable` 的性能收益来自 **持久 `ListState` + 可见区 cell 生成**：10,000 行数据不会一次性构建 10,000 个行元素，滚动时只请求视口附近的 row/cell。行数变化时通过 `splice` 增量同步，避免加载更多时重建滚动状态。
程序化滚动：完成 builder 配置后、首次挂载前调用 `list_state()` 并保留句柄；调用其 `scroll_to(...)` 后，宿主需请求重绘。该句柄只用于滚动与视口/行位置观察；总行数和单元格由 builder 的 `row_count` 与 `render_cell` 提供。同一 ID 的普通 inline 重建保留已挂载状态，不会绑定新 builder 的句柄；改变 `overdraw` 会重建该 ID 的挂载状态并使旧句柄失效，改变 `row_height` 则重置已请求的句柄。

## 能力

- 固定表头：列定义沿用 `TableColumn`，支持宽度、对齐、sortable。
- 行虚拟化：通过 `row_count + render_cell(row, key, ...)` 生成单元格，不缓存跨帧 `AnyElement`。
- 可配置高度、行高、overdraw、斑马纹、边框、加载和空状态。
- 排序状态外置：`on_sort_change` 把三态排序交回业务层，适合本地 index 映射或后端分页查询。

## 万行固定表头

### 效果

::Demo{component="VirtualizedTableBasic"}::

### 代码

```rust src="virtualized_table/basic.rs"
```

## 排序状态

### 效果

::Demo{component="VirtualizedTableSortable"}::

### 代码

```rust src="virtualized_table/sortable.rs"
```

## DataTable 增强：选择、固定列和 load more

示例由持久的父视图保存已加载行数：初始 500 行，点击现有「加载更多数据」页脚每次追加 500 行，最多 10,000 行。追加数据会更新页脚计数并保留表格滚动位置；无需重新创建父视图或添加额外控制按钮。达到上限后再次点击不再通知父视图刷新。

`load_more` 页脚使用原生 GPUI Button 语义：现有标签同时作为语义名称，Tab 可聚焦，Enter 或 Space 可激活。按钮覆盖整行宽度；标签更新后保留焦点与表格滚动位置。表格处于 `loading(true)` 时，内置页脚按钮禁用，键盘或指针均不会再次发起加载请求；`loading` 与 `load_more` 的 builder 调用顺序不影响该行为，自定义 `footer` 内容不变。这里描述组件语义，不代表已验证平台辅助技术实际暴露的角色或名称，后者需经原生 AT 观测确认。


### 效果

::Demo{component="VirtualizedTableDataTable"}::

### 代码

```rust src="virtualized_table/data_table.rs"
```

## 设计说明

`VirtualizedTable` 和普通 `Table` 的差异在于数据流：普通 `Table` 接收完整 `TableRow` 列表；虚拟表格只接收总行数和单元格渲染闭包。这样可以避免万级行一次性创建所有 GPUI 元素，也避免把 frame-local 元素缓存到组件状态中。
