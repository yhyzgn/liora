# DatePicker

`DatePicker` 用于选择日期、日期范围、月份/月范围和年份/年范围，支持自定义展示格式与禁用状态。

## 基础用法

选择单个日期，并可从组件状态读取当前值。

### 效果

::LioraDemo{component="DatePickerBasic"}::

### 代码

```rust src="date_picker/basic.rs"
```

## 自定义展示格式

通过 `format` 将日期展示为业务需要的字符串格式；同时可以配置 `close_on_click_outside(false)` / `close_on_escape(false)` 控制弹层关闭策略。

### 效果

::LioraDemo{component="DatePickerFormatted"}::

### 代码

```rust src="date_picker/formatted.rs"
```

## 日期范围

调用 `date_range()` 后可以选择开始日期和结束日期。

### 效果

::LioraDemo{component="DatePickerRange"}::

### 代码

```rust src="date_picker/range.rs"
```

## 月份选择

调用 `month()` 按月粒度选择。

### 效果

::LioraDemo{component="DatePickerMonth"}::

### 代码

```rust src="date_picker/month.rs"
```

## 月份范围

调用 `month_range()` 选择月份区间。

### 效果

::LioraDemo{component="DatePickerMonthRange"}::

### 代码

```rust src="date_picker/month_range.rs"
```

## 年份选择

调用 `year()` 按年粒度选择。

### 效果

::LioraDemo{component="DatePickerYear"}::

### 代码

```rust src="date_picker/year.rs"
```

## 年份范围

调用 `year_range()` 选择年份区间，并可配合年份格式化。

### 效果

::LioraDemo{component="DatePickerYearRange"}::

### 代码

```rust src="date_picker/year_range.rs"
```

## 禁用状态

禁用后不可打开选择面板。

### 效果

::LioraDemo{component="DatePickerDisabled"}::

### 代码

```rust src="date_picker/disabled.rs"
```
