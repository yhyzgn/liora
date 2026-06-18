# Empty

`Empty` 用于在列表、表格或搜索结果没有数据时给出明确占位提示，并可追加引导操作。

## 基础用法

默认空状态会展示内置图标和“暂无数据”文案。

### 效果

::LioraDemo{component="EmptyBasic"}::

### 代码

```rust src="empty/basic.rs"
```

## 自定义描述

使用 `description` 覆盖默认文案，让空状态更贴近当前业务场景。

### 效果

::LioraDemo{component="EmptyDescription"}::

### 代码

```rust src="empty/description.rs"
```

## 自定义图片

使用 `image` 注入任意原生节点，例如搜索、归档或网络状态图标。

### 效果

::LioraDemo{component="EmptyImage"}::

### 代码

```rust src="empty/image.rs"
```

## 底部操作

使用 `extra` 在空状态底部追加按钮，引导用户创建第一条数据或调整筛选条件。

### 效果

::LioraDemo{component="EmptyExtra"}::

### 代码

```rust src="empty/extra.rs"
```
