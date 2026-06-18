# TimePicker

`TimePicker` 用于选择时分秒，支持自定义展示格式、固定步进、隐藏秒和禁用状态。

## 基础用法

选择时分秒，并可从组件状态读取当前值。

### 效果

::AuraDemo{component="TimePickerBasic"}::

### 代码

```rust src="time_picker/basic.rs"
```

## 自定义展示格式

使用 `format` 设置输入框中的时间展示格式；弹层关闭策略也可通过 `close_on_click_outside(false)` / `close_on_escape(false)` 控制。

### 效果

::AuraDemo{component="TimePickerFormatted"}::

### 代码

```rust src="time_picker/formatted.rs"
```

## 固定步进

通过 `minute_step` 和 `second_step` 控制列表中的可选值间隔。

### 效果

::AuraDemo{component="TimePickerStepped"}::

### 代码

```rust src="time_picker/stepped.rs"
```

## 隐藏秒

`without_seconds` 仅展示小时和分钟。

### 效果

::AuraDemo{component="TimePickerNoSeconds"}::

### 代码

```rust src="time_picker/no_seconds.rs"
```

## 禁用状态

禁用后不可打开时间面板。

### 效果

::AuraDemo{component="TimePickerDisabled"}::

### 代码

```rust src="time_picker/disabled.rs"
```
