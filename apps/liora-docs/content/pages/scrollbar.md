# Scrollbar

`Scrollbar` 是 Liora 自举滚动容器，用于在原生 GPUI 视图中展示可滚动内容和可拖拽滚动条。

## 基础用法

通过 `Scrollbar::new` 提供内容渲染闭包，并用 `height` 限制可视区域高度。

### 效果

::LioraDemo{component="ScrollbarBasic"}::

### 代码

```rust src="scrollbar/basic.rs"
```
