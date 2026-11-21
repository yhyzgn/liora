# DropdownButton

组合式下拉按钮，用于把一个主操作和一组相关命令放在同一个入口里。它复用 Liora 的 `Button` 语义样式和 Popover 定位/关闭策略，保持纯 Rust + GPUI native。

## 基础菜单

整颗按钮作为触发器，点击后展开菜单；菜单项点击后会自动关闭。

### 效果

::Demo{component="DropdownButtonBasic"}::

### 代码

```rust src="dropdown_button/basic.rs"
```

## Split button

Split 模式下，左侧执行主操作，右侧箭头展开更多操作；即使未设置 `on_click`，左侧也不会打开菜单。点击和 Tab 聚焦后的 Enter/Space 遵循同一分工；非 Split 模式仍由整颗按钮展开菜单。

`Menu only` 示例未设置主操作回调：主按钮没有主操作，只有右侧箭头展开菜单；菜单中的 Preview deployment 仍执行真实提示操作。

### 效果

::Demo{component="DropdownButtonSplit"}::

### 代码

```rust src="dropdown_button/split.rs"
```

## 菜单项状态

菜单项支持图标、禁用项和危险项，便于表达命令层级和破坏性操作。

打开后焦点进入首个可用菜单项；上下方向键环绕并跳过禁用项，Home/End 移到首/末可用项。Enter/Space 完整按下并释放后执行当前项一次并关闭本菜单；指针操作同样在完整点击后执行，按下后移出或跨行释放不会执行。禁用项不提供 Click 操作；菜单已打开时禁用整个按钮会关闭该菜单。Tab/Shift+Tab 使用宿主焦点遍历并关闭本菜单，保留移动后的宿主焦点；Escape 和外部点击仍遵循各自关闭策略。

### 效果

::Demo{component="DropdownButtonItemStates"}::

### 代码

```rust src="dropdown_button/item_states.rs"
```

## 尺寸、位置与关闭策略

`small()` / `large()` 复用 Button 尺寸；`placement` 复用 Popover 定位；`close_on_click_outside(false)` 与 `close_on_escape(false)` 可用于强约束流程。

### 效果

::Demo{component="DropdownButtonSizes"}::

### 代码

```rust src="dropdown_button/sizes.rs"
```

## 长菜单滚动

长菜单保持行高并提供滚轮滚动。方向键与 Home/End 会自动滚动到当前可用项；末项仍可正常执行，禁用项仍被跳过。

### 效果

::Demo{component="DropdownButtonScrollable"}::

### 代码

```rust src="dropdown_button/scrollable.rs"
```
