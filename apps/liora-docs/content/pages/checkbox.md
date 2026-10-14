# Checkbox

`Checkbox` 用于独立多选和多选组。每个示例都把效果与代码放在一起。

## 基础状态

### 效果

::Demo{component="CheckboxBasic"}::

### 代码

```rust src="checkbox/basic.rs"
```

## 多选组

### 效果

::Demo{component="CheckboxGroup"}::

### 代码

```rust src="checkbox/group.rs"
```

## 按钮样式

### 效果

::Demo{component="CheckboxButtons"}::

### 代码

```rust src="checkbox/buttons.rs"
```


## 自定义选项样式与布局

### 效果

::Demo{component="CheckboxCustom"}::

### 代码

`option_style` 负责容器、颜色、间距和选中态样式；`option_renderer` 可以按 `selected` / `index` 渲染完全自定义的选项内容，例如多行说明、图标和状态徽标。

```rust src="checkbox/custom.rs"
```

## 待办任务列表的选型建议

`Checkbox` 适合单个完成状态，`CheckboxGroup` 适合一组同质选项。临时实现简单待办列表时，可以使用 `CheckboxGroup::option_renderer` 自定义多行文本、图标、标签和状态徽标。

如果待办项需要复杂视觉和交互，建议提供专门的 `TodoList` / `TodoItem` 控件，而不是继续扩展 `Checkbox`：

- 每项使用稳定 `id`，支持标题、描述、标签、优先级、截止时间和禁用状态。
- 勾选图标、未勾选图标、完成态图标和动画都允许替换。
- 文本区域使用自定义渲染器，可实现删除线、渐变、富文本、折叠说明和状态徽标。
- 保留键盘焦点、无障碍语义和 `on_change`，同时支持异步保存、加载中和错误态。
- 后续可扩展拖拽排序、分组、批量完成和虚拟化，而不会污染基础 `Checkbox` API。

因此建议：**短期用 `CheckboxGroup::option_renderer` 验证场景；正式的待办产品直接设计 `TodoList`，内部复用 `Checkbox` 的交互语义，但把视觉和内容区域开放为 slot/renderer。**
