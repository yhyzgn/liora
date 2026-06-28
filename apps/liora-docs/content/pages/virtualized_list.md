# VirtualizedList

`VirtualizedList` 是 Liora 的原生虚拟列表控件，用于长文档、大量行项目或复杂组件流。它封装 GPUI `ListState`，只渲染当前可见区域和少量 overdraw 内容，并内置 Liora 自举滚动条。

## 虚拟化性能表现

`VirtualizedList` 只在 viewport 与 overdraw 范围内创建行元素。对于 1,000+ 行列表，典型可见行通常只有 8-12 行，渲染比例约 1%；拖动排序也只作用于当前虚拟窗口附近的行。

## 能力

- 可见区渲染：避免长列表一次性布局所有元素。
- 动态 item 渲染：通过闭包按 index 创建原生 GPUI / Liora 节点。
- 可配置高度、overdraw 与 item 间距。
- 可选垂直拖动排序：开启后每行前端显示 Grip 拖拽把手，拖动项会沿垂直方向跟随鼠标，并通过 `on_reorder` 获取即时排序结果。
- 内置 `VirtualScrollbar`，滚动条坐标直接来自 `ListState`。
- 适合 Liora Docs 这类 Markdown block 流，也适合业务数据长列表。

## 基础用法

::LioraDemo{component="VirtualizedListBasic"}::

### 代码

```rust src="virtualized_list/basic.rs"
```

## 垂直拖动排序

::LioraDemo{component="VirtualizedListDraggable"}::

### 代码

```rust src="virtualized_list/draggable.rs"
```

## 设计说明

`VirtualizedList` 不是 Web 虚拟滚动。它仍然运行在 GPUI 原生窗口中：列表状态由 `ListState` 管理，滚轮事件、布局测量、可见范围和滚动条定位都发生在 Rust / GPUI 元素树内。

当列表项高度会变化时，应更新 item count、render closure 或调用组件提供的重测量配置入口，确保 `ListState` 重新测量可见项。
