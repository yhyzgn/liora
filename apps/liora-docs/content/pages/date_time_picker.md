# DateTimePicker

`DateTimePicker` 用于选择日期时间和日期时间范围，支持展示格式、时间步进、隐藏秒和禁用状态。

## 基础用法

选择日期时间，并可从组件状态读取当前值。

### 效果

::Demo{component="DateTimePickerBasic"}::

### 代码

```rust src="date_time_picker/basic.rs"
```

## 自定义展示格式

通过 `format` 使用中文日期时间格式展示已选值。

### 效果

::Demo{component="DateTimePickerFormatted"}::

### 代码

```rust src="date_time_picker/formatted.rs"
```

## 固定步进

使用 `minute_step` 和 `second_step` 控制时间列可选项间隔。

### 效果

::Demo{component="DateTimePickerStepped"}::

### 代码

```rust src="date_time_picker/stepped.rs"
```

## 隐藏秒

调用 `without_seconds()` 仅选择日期、小时和分钟。

### 效果

::Demo{component="DateTimePickerNoSeconds"}::

### 代码

```rust src="date_time_picker/no_seconds.rs"
```

## 日期时间范围

调用 `date_time_range()` 选择开始与结束日期时间。

### 效果

::Demo{component="DateTimePickerRange"}::

### 代码

```rust src="date_time_picker/range.rs"
```

## 禁用状态

禁用后不可打开日期时间面板。

### 效果

::Demo{component="DateTimePickerDisabled"}::

### 代码

```rust src="date_time_picker/disabled.rs"
```
