# Form

组织输入项、标签、必填标记和校验信息的原生表单布局。Form 不持有业务数据，字段状态由 Input、Select、Switch 等组件自身或上层 view 管理。

## 基础表单

使用 `FormItem` 包裹每个字段，设置 `label`、`required` 和具体控件。示例中的 Role Select 设为 disabled，禁用时不会打开下拉层或改变当前选项。字段控件通常作为 `Entity` 保存，保证输入状态在重绘间持久。

在示例的 `Form::new()` 后加上 `.label_width(gpui::px(96.0))`，即可为通过 `.item(...)` 添加的字段设置默认标签宽度。单项可用 `FormItem::label_width(...)` 覆盖；通过 `.child(...)` 添加的子元素不继承这个默认值。

### 效果

::Demo{component="FormBasic"}::

### 代码

```rust src="form/basic.rs"
```

## 校验信息

`required(true)` 显示必填标记，`error(...)` 显示错误信息。真实校验逻辑应由业务层决定何时设置错误文案。

### 效果

::Demo{component="FormValidation"}::

### 代码

```rust src="form/validation.rs"
```

## 行内表单

`inline(true)` 适合筛选条、工具栏和窄表单，字段会在一行内排列并自动换行。

### 效果

::Demo{component="FormInline"}::

### 代码

```rust src="form/inline.rs"
```
