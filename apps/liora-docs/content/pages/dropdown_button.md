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
