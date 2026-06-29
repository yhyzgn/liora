# DockLayout

`DockLayout` 是面向 IDE、数据工作台和后台工具的原生工作区布局组件。它提供 side/bottom/top panel 与中心 document tabs 的结构化 API，适合先搭出稳定的信息架构；复杂拖拽重排可以在同一 API 上继续演进，而不需要在应用里手写一套平行布局系统。

## Workbench 工作台

### 效果

::Demo{component="DockLayoutWorkbench"}::

### 代码

```rust src="dock_layout/workbench.rs"
```

## 左右检查器 + 底部日志

### 效果

::Demo{component="DockLayoutInspector"}::

### 代码

```rust src="dock_layout/inspector.rs"
```
