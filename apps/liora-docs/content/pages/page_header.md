# PageHeader

`PageHeader` 用于页面顶部，集中展示标题、返回入口、辅助说明和页面级操作。

## 基础用法

最小配置只需要标题和返回事件。

### 效果

::Demo{component="PageHeaderBasic"}::

### 代码

```rust src="page_header/basic.rs"
```

## 副标题与操作区

通过 `sub_title` 和 `extra` 可以在标题区补充说明和右侧操作按钮。

### 效果

::Demo{component="PageHeaderExtra"}::

### 代码

```rust src="page_header/extra.rs"
```

## 完整案例

`content` 与 `footer` 可以让页头承载更完整的上下文信息和页脚区域。

### 效果

::Demo{component="PageHeaderFull"}::

### 代码

```rust src="page_header/full.rs"
```
