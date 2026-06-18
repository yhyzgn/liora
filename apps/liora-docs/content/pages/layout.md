# Layout

`Layout` 页面集中展示 Liora 的基础布局原语：分割线、间距和 24 栅格。

## Divider 分割线

`Divider` 支持水平、带文字和垂直分割线，用于分隔内容块或行内区域。

### 效果

::LioraDemo{component="LayoutDivider"}::

### 代码

```rust src="layout/divider.rs"
```

## Space 间距

`Space` 用于为一组子节点提供一致的横向或纵向间距。

### 效果

::LioraDemo{component="LayoutSpace"}::

### 代码

```rust src="layout/space.rs"
```

## Grid 栅格

`Row` 和 `Col` 组合实现 24 格栅格布局，适合业务表单和数据卡片排版。

### 效果

::LioraDemo{component="LayoutGrid"}::

### 代码

```rust src="layout/grid.rs"
```
