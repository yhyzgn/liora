# Layout

`Layout` 页面集中展示 Liora 的基础布局原语：分割线、间距和 24 栅格。

## Divider 分割线

`Divider` 支持水平、带文字和垂直分割线，用于分隔内容块或行内区域。

### 效果

::Demo{component="LayoutDivider"}::

### 代码

```rust src="layout/divider.rs"
```

## Space 间距

`Space` 用于为一组子节点提供一致的横向或纵向间距。

### 效果

::Demo{component="LayoutSpace"}::

### 代码

```rust src="layout/space.rs"
```

## Grid 栅格

`Row` 和 `Col` 组合实现 24 格栅格布局，适合业务表单和数据卡片排版。

### 效果

::Demo{component="LayoutGrid"}::

### 代码

```rust src="layout/grid.rs"
```

## Accessible 元数据

`Accessible` 为组合节点附加稳定标识、角色、名称和说明；它不替代子控件的焦点与键盘实现。此例为原生 Input 的外层组设置元数据，输入内容显示在 Project value。组名称不等于给内部文本输入单独设置名称，也不代表已经完成屏幕阅读器验证。

### 效果

::Demo{component="LayoutAccessible"}::

### 代码

```rust src="layout/accessible.rs"
```
