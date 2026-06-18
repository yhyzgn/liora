# Form

组织输入项、标签、必填标记和校验信息的原生表单布局。Form 不持有业务数据，字段状态由 Input、Select、Switch 等组件自身或上层 view 管理。

## 基础表单

使用 `FormItem` 包裹每个字段，设置 `label`、`required` 和具体控件。字段控件通常作为 `Entity` 保存，保证输入状态在重绘间持久。

### 效果

::LioraDemo{component="FormBasic"}::

### 代码

```rust src="form/basic.rs"
```

## 校验信息

`required(true)` 显示必填标记，`error(...)` 显示错误信息。真实校验逻辑应由业务层决定何时设置错误文案。

### 效果

::LioraDemo{component="FormValidation"}::

### 代码

```rust src="form/validation.rs"
```

## 行内表单

`inline(true)` 适合筛选条、工具栏和窄表单，字段会在一行内排列并自动换行。

### 效果

::LioraDemo{component="FormInline"}::

### 代码

```rust src="form/inline.rs"
```
