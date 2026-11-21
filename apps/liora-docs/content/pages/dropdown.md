# Dropdown

点击触发的原生下拉菜单，适合收纳一组低频操作。菜单项点击后会自动关闭对应 Popover。

## 基础菜单

使用 `item` 逐项注册回调，使用 `disabled_item(label)` 添加真正不可执行的禁用项；禁用项不需要空回调。回调中可以发送 toast、更新状态或派发业务命令。

打开后焦点进入首个可用菜单项；上下方向键环绕并跳过禁用项，Home/End 移到首/末可用项。Enter/Space 完整按下并释放后执行当前项一次并关闭本菜单。Tab/Shift+Tab 使用宿主焦点遍历并关闭本菜单，保留移动后的宿主焦点；空菜单/全禁用菜单仍保留 Escape 关闭路径。

### 效果

::Demo{component="DropdownBasic"}::

### 代码

```rust src="dropdown/basic.rs"
```

## 位置

通过 `placement` 控制菜单展开方向。顶部、底部、左右方向都使用同一个 Portal 定位管线。

### 效果

::Demo{component="DropdownPlacements"}::

### 代码

```rust src="dropdown/placements.rs"
```

## 关闭策略

默认点击菜单外部或按 ESC 会关闭 Dropdown。需要强约束流程时，可以通过 `close_on_click_outside(false)` 和 `close_on_escape(false)` 禁用这些自动关闭路径；激活菜单项时先关闭原菜单，再执行回调，因此回调可以用相同 ID 打开替代 Popover。

### 效果

::Demo{component="DropdownCloseStrategy"}::

### 代码

```rust src="dropdown/close_strategy.rs"
```

## 长菜单滚动

菜单保持有界高度，不压缩菜单行。可用滚轮访问末项；上下方向键与 Home/End 自动滚动到当前可用项，Enter/Space 仍只执行一次并关闭菜单。

### 效果

::Demo{component="DropdownScrollable"}::

### 代码

```rust src="dropdown/scrollable.rs"
```
