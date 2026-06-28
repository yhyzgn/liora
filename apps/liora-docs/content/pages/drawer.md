# Drawer

`Drawer` 是从窗口边缘滑出的浮层面板，适合承载配置、详情、过滤器等不中断主流程的内容。

## 四方向

通过 `placement` 指定从右、左、上或下方滑出。

### 效果

::LioraDemo{component="DrawerPlacements"}::

### 代码

```rust src="drawer/placements.rs"
```

## 尺寸

横向抽屉使用宽度，纵向抽屉使用高度；也可以使用语义化尺寸快捷方法。

### 效果

::LioraDemo{component="DrawerSizes"}::

### 代码

```rust src="drawer/sizes.rs"
```

## 手动关闭

关闭遮罩点击和 ESC 关闭，改由内容区按钮手动关闭。

### 效果

::LioraDemo{component="DrawerManualClose"}::

### 代码

```rust src="drawer/manual_close.rs"
```


## 轻量面板模式

`Drawer::sheet()` 覆盖原先 Sheet 的语义：它仍然是 Drawer overlay runtime，只是默认尺寸更轻，适合筛选、检查器、快速创建和局部设置等短流程。

### 效果

::LioraDemo{component="DrawerSheetPlacements"}::

### 代码

```rust src="drawer/sheet_placements.rs"
```

## 受控轻量面板

关闭遮罩点击和 ESC 关闭，改由内容区按钮显式关闭。

### 效果

::LioraDemo{component="DrawerSheetControlled"}::

### 代码

```rust src="drawer/sheet_controlled.rs"
```
