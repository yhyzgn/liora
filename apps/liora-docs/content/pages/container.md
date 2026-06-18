# Container

`Container` 是页面级布局容器，可组合 Header、Aside、Main 和 Footer 区域；本页也展示常用基础布局原语。

## Space 间距

横向与纵向 `Space` 可以快速统一按钮、表单项或信息块之间的间距。

### 效果

::LioraDemo{component="ContainerSpace"}::

### 代码

```rust src="container/space.rs"
```

## Divider 分割线

在容器内使用 `Divider` 分隔信息层级，垂直分割线可用于行内区域。

### 效果

::LioraDemo{component="ContainerDivider"}::

### 代码

```rust src="container/divider.rs"
```

## 页面布局容器

`Container::header / aside / footer / child` 可组合典型后台页面骨架。

### 效果

::LioraDemo{component="ContainerLayout"}::

### 代码

```rust src="container/layout.rs"
```
