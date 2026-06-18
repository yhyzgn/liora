# Pagination

`Pagination` 用于把大量数据拆分为多个页面，降低单页信息密度，并提供页码跳转能力。

## 基础用法

默认布局包含上一页、页码和下一页，并可通过 `on_change` 监听页码变化。

### 效果

::LioraDemo{component="PaginationBasic"}::

### 代码

```rust src="pagination/basic.rs"
```

## 背景色分页

开启 `background(true)` 后，页码按钮会带有背景色，更适合强调分页控制区域。

### 效果

::LioraDemo{component="PaginationBackground"}::

### 代码

```rust src="pagination/background.rs"
```

## 附加功能

通过 `layout` 组合总数、页尺寸选择器、页码和跳转区域。

### 效果

::LioraDemo{component="PaginationAdvanced"}::

### 代码

```rust src="pagination/advanced.rs"
```
