# TodoList

`TodoList` / `TodoItem` 面向待办任务场景设计：它复用 Checkbox 的完成状态和键盘交互语义，同时把勾选图标、内容区域和尾部信息开放为可组合的 renderer。

## 基础待办列表

### 效果

::Demo{component="TodoListBasic"}::

### 代码

```rust src="todo/basic.rs"
```

`TodoItemData` 支持稳定 `id`、标题、描述、初始完成状态和禁用状态。列表项本身是 GPUI entity，可以在父视图中保留并更新；运行中可通过 `set_checked`、`set_title` 和 `set_description` 修改状态与内容。

## 自定义视觉插槽

### 效果

::Demo{component="TodoListCustom"}::

### 代码

```rust src="todo/custom.rs"
```

- `leading_renderer`：在勾选区域之前放置项目图标或拖拽把手。
- `check_renderer`：替换默认勾选框，适合自定义图标、颜色和动画。
- `content_renderer`：自定义标题、描述、富文本和完成态删除线。
- `trailing_renderer`：放置优先级、标签、截止时间或状态徽标。

renderer 接收 `TodoItemRenderContext`，其中包含稳定 ID、标题、描述、完成态和禁用态。

## 监听完成状态

### 效果

::Demo{component="TodoListChanges"}::

### 代码

```rust src="todo/changes.rs"
```

通过 `TodoList::on_change` 统一监听任意子项的完成状态变化。回调适合连接异步保存、错误提示或业务状态同步；控件本身不绑定具体的数据层。

## 与 Checkbox 的关系

简单同质选项继续使用 `CheckboxGroup`；待办项需要复杂视觉、描述、标签、优先级或后续拖拽/分组能力时，使用专用 `TodoList` 可以避免把任务语义继续堆进基础 Checkbox。
