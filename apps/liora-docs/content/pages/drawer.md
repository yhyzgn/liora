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
