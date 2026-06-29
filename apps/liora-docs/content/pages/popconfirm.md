# Popconfirm

轻量确认框，适合二次确认删除、发布、归档等风险操作。它复用 Popover 的定位能力，同时内置确认/取消按钮。

## 基础确认

`on_confirm` 与 `on_cancel` 直接接收 GPUI `Window/App` 上下文，可触发 toast、状态更新或业务命令。

### 效果

::Demo{component="PopconfirmBasic"}::

### 代码

```rust src="popconfirm/basic.rs"
```

## 位置

Popconfirm 支持与 Popover 一致的方向配置，常用于贴近触发按钮展示确认操作。

### 效果

::Demo{component="PopconfirmPlacements"}::

### 代码

```rust src="popconfirm/placements.rs"
```

## 自定义文案与 ESC 策略

危险操作可以自定义按钮文案，并通过 `close_on_escape(false)` 与 `close_on_click_outside(false)` 禁止快捷关闭，避免误触丢失决策上下文。

### 效果

::Demo{component="PopconfirmCustomText"}::

### 代码

```rust src="popconfirm/custom_text.rs"
```
