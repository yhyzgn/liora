# Checkbox

`Checkbox` 用于独立多选和多选组。每个示例都把效果与代码放在一起。

启用的单个控件可用 `Tab` / `Shift+Tab` 进入和离开，`Space` 或 `Enter` 切换一次；指针必须在同一控件内按下并释放。禁用控件不参与 Tab 顺序，也不响应激活。使用 `set_disabled` 动态更新时保留原 Entity 和焦点 identity。

原生根节点暴露 `CheckBox`、checked 和 disabled 状态。可见 `label` 默认提供无障碍名称；无文本示例用 `aria_label` 命名，覆盖名称不会改变可见标签。外层 `FormItem` 文本不会自动成为控件名称。`aria_label` 同样接受随当前语言解析的 `LocalizedText`。

## 基础状态

### 效果

::Demo{component="CheckboxBasic"}::

### 代码

```rust src="checkbox/basic.rs"
```

## 多选组

### 效果

::Demo{component="CheckboxGroup"}::

`on_change` 在启用选项的实际选择集合变化时调用一次，参数为当前选中索引；禁用或未发生变化时不调用。

按钮样式、带自定义样式或自定义渲染内容的选项，须在同一选项内按下并释放鼠标左键才会切换；从选项外拖入、拖出，或在另一选项上释放，都不会改变选择。

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
