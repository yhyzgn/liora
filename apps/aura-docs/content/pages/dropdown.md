# Dropdown

点击触发的原生下拉菜单，适合收纳一组低频操作。菜单项点击后会自动关闭对应 Popover。

## 基础菜单

使用 `item` 逐项注册点击回调。回调中可以发送 toast、更新状态或派发业务命令。

### 效果

::AuraDemo{component="DropdownBasic"}::

### 代码

```rust src="dropdown/basic.rs"
```

## 位置

通过 `placement` 控制菜单展开方向。顶部、底部、左右方向都使用同一个 Portal 定位管线。

### 效果

::AuraDemo{component="DropdownPlacements"}::

### 代码

```rust src="dropdown/placements.rs"
```

## 关闭策略

默认点击菜单外部或按 ESC 会关闭 Dropdown。需要强约束流程时，可以通过 `close_on_click_outside(false)` 和 `close_on_escape(false)` 禁用这些自动关闭路径；菜单项点击仍会在回调执行后关闭菜单。

### 效果

::AuraDemo{component="DropdownCloseStrategy"}::

### 代码

```rust src="dropdown/close_strategy.rs"
```
