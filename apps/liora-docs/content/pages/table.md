# Table

`Table` 用于展示结构化数据，支持基础行列、斑马纹、边框、固定表头、加载、空状态和三态排序。

## 自定义表头与排序

可排序列通过 `sortable` 开启，`on_sort_change` 负责把排序状态交回业务层。

### 效果

::LioraDemo{component="TableSortable"}::

### 代码

```rust src="table/sortable.rs"
```

## 基础用法

通过 `TableColumn` 定义列，通过 `TableRow` 写入单元格。

### 效果

::LioraDemo{component="TableBasic"}::

### 代码

```rust src="table/basic.rs"
```

## 斑马纹与边框

`stripe(true)` 和 `border(true)` 可以增强数据行的层次感。

### 效果

::LioraDemo{component="TableStripeBorder"}::

### 代码

```rust src="table/stripe_border.rs"
```

## 固定表头

长列表可以在固定高度区域内滚动，同时保持表头可见。

### 效果

::LioraDemo{component="TableFixedHeader"}::

### 代码

```rust src="table/fixed_header.rs"
```

## 加载状态

`loading(true)` 会覆盖表格内容并展示加载反馈。

### 效果

::LioraDemo{component="TableLoading"}::

### 代码

```rust src="table/loading.rs"
```

## 空数据

没有 rows 时可以通过 `empty_text` 设置空状态文案。

### 效果

::LioraDemo{component="TableEmpty"}::

### 代码

```rust src="table/empty.rs"
```
