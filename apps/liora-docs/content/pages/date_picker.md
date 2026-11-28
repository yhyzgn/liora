# DatePicker

`DatePicker` 用于选择日期、日期范围、月份/月范围和年份/年范围，支持自定义展示格式与禁用状态。

## 基础用法

选择单个日期，并可从组件状态读取当前值。

未设置初始值时，面板默认显示创建组件时的本地当前月份，选中值仍为空；显式调用 `value` 或 `range` 会改为显示所选日期或区间起点的月份。组件创建后不会随跨月自动切换面板月份。

### 效果

::Demo{component="DatePickerBasic"}::

### 代码

```rust src="date_picker/basic.rs"
```

## 单日期键盘操作

以下键盘交互仅适用于 `DatePickerType::Date`，不会改变日期范围、月份或年份模式的选择语义。

- 启用状态下，触发器可通过 Tab 聚焦；按 Enter 或 Space 打开日历，并将焦点移到活动日期。活动日期与已选日期分开：当前显示月份内有已选日期时，活动日期从该日期开始；否则从当月第一天开始。移动活动日期不会预先修改选中值。
- 在日历格中，← / → 按一天移动，↑ / ↓ 按七天移动；键盘导航不会移出当前显示月份。Home / End 移到当前周的第一天 / 最后一天，并限制在当前显示月份内。
- 四个上一年、上个月、下个月、下一年按钮均可通过键盘到达。日历格中的 Shift+Tab 可逆向到达这些按钮；从按钮按 Tab 可继续到日历格。日历格按 Tab 离开时继续到触发器之后的下一个焦点项，不会在弹层内循环；从首个导航按钮按 Shift+Tab 会继续到触发器之前的焦点项。
- 在活动日期上按 Enter 或 Space 会提交该日期、关闭弹层，并将焦点还给触发器。Escape 遵循 `close_on_escape`：允许关闭时会取消尚未提交的键盘移动并还原焦点；关闭策略关闭时，弹层保持打开。点击弹层外部关闭不会主动抢回焦点。
- 弹层打开时禁用组件会关闭弹层，不提交活动日期，也不会把焦点移到已禁用的触发器。带修饰键的按键交由宿主处理；Tab 和 Shift+Tab 仍按上述焦点遍历规则工作。

相邻月份日期仍可通过鼠标选择。上述键盘行为尚未完成原生辅助技术验证，不代表原生屏幕阅读器支持认证。

## 自定义展示格式

通过 `format` 将日期展示为业务需要的字符串格式；同时可以配置 `close_on_click_outside(false)` / `close_on_escape(false)` 控制弹层关闭策略。

### 效果

::Demo{component="DatePickerFormatted"}::

### 代码

```rust src="date_picker/formatted.rs"
```

## 日期范围

调用 `date_range()` 后可以选择开始日期和结束日期。

### 效果

::Demo{component="DatePickerRange"}::

### 代码

```rust src="date_picker/range.rs"
```

## 月份选择

调用 `month()` 按月粒度选择。

### 效果

::Demo{component="DatePickerMonth"}::

### 代码

```rust src="date_picker/month.rs"
```

## 月份范围

调用 `month_range()` 选择月份区间。

### 效果

::Demo{component="DatePickerMonthRange"}::

### 代码

```rust src="date_picker/month_range.rs"
```

## 年份选择

调用 `year()` 按年粒度选择。

### 效果

::Demo{component="DatePickerYear"}::

### 代码

```rust src="date_picker/year.rs"
```

## 年份范围

调用 `year_range()` 选择年份区间，并可配合年份格式化。

### 效果

::Demo{component="DatePickerYearRange"}::

### 代码

```rust src="date_picker/year_range.rs"
```

## 禁用状态

禁用后不可打开选择面板。

### 效果

::Demo{component="DatePickerDisabled"}::

### 代码

```rust src="date_picker/disabled.rs"
```
