# Sheet

`Sheet` 是轻量边缘面板，适合筛选、检查器、快速创建、局部设置等短流程。它复用 Liora 的 `Drawer` overlay runtime，但提供更轻的默认尺寸和 sheet 语义 API。

## 边缘位置

### 效果

::LioraDemo{component="SheetPlacements"}::

### 代码

```rust src="sheet/placements.rs"
```

## 受控关闭

### 效果

::LioraDemo{component="SheetControlled"}::

### 代码

```rust src="sheet/controlled.rs"
```
