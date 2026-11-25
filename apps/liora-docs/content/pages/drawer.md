# Drawer

`Drawer` 是从窗口边缘滑出的浮层面板，适合承载配置、详情、过滤器等不中断主流程的内容。
关闭回调只会移除创建它的抽屉实例；旧抽屉被同 ID 的新抽屉替换后，迟到的关闭回调不会关闭新实例。
插入、替换或移除抽屉时会自动重绘宿主窗口，无需额外输入或手动刷新。

打开抽屉不会强制移动焦点。焦点留在触发控件或底层抽屉内容时，Escape 仍只遵循栈顶抽屉的关闭策略；`close_on_escape(false)` 不会把 Escape 漏给底层抽屉。此行为不是模态 Tab 焦点陷阱，也不承诺从任意抽屉内容关闭后的焦点恢复。

## 四方向

通过 `placement` 指定从右、左、上或下方滑出。

### 效果

::Demo{component="DrawerPlacements"}::

### 代码

```rust src="drawer/placements.rs"
```

## 尺寸

横向抽屉使用宽度，纵向抽屉使用高度；也可以使用语义化尺寸快捷方法。

### 效果

::Demo{component="DrawerSizes"}::

### 代码

```rust src="drawer/sizes.rs"
```

## 手动关闭

关闭遮罩点击和 ESC 关闭，改由内容区按钮手动关闭。

先打开父抽屉，再点击 “Open Child Drawer”。“Close Child Drawer” 调用 `Drawer::close(cx)`，只关闭栈顶子抽屉，父抽屉仍可操作。需要关闭指定抽屉时，使用 `Drawer::close_id(id, cx)`。

### 效果

::Demo{component="DrawerManualClose"}::

### 代码

```rust src="drawer/manual_close.rs"
```


## 轻量面板模式

`Drawer::sheet()` 覆盖原先 Sheet 的语义：它仍然是 Drawer overlay runtime，只是默认尺寸更轻，适合筛选、检查器、快速创建和局部设置等短流程。

### 效果

::Demo{component="DrawerSheetPlacements"}::

### 代码

```rust src="drawer/sheet_placements.rs"
```

## 受控轻量面板

关闭遮罩点击和 ESC 关闭。Cancel 和 Apply 都通过 `Drawer::close_id(id, cx)` 关闭当前面板，不影响其他抽屉；示例不保存数据。

### 效果

::Demo{component="DrawerSheetControlled"}::

### 代码

```rust src="drawer/sheet_controlled.rs"
```
