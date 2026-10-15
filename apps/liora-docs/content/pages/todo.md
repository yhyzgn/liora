# TodoList

`TodoList` / `TodoItem` 面向待办任务场景设计：它复用 Checkbox 的完成状态和键盘交互语义，同时把勾选图标、内容区域和尾部信息开放为可组合的 renderer。

## 基础待办列表

### 效果

::Demo{component="TodoListBasic"}::

### 代码

```rust src="todo/basic.rs"
```

`TodoItemData` 支持稳定 `id`、标题、描述、初始完成状态和禁用状态。列表项本身是 GPUI entity，可以在父视图中保留并更新；运行中可通过 `set_checked`、`set_title` 和 `set_description` 修改状态与内容。

## 动态增删改

`TodoList` 提供面向宿主状态的 CRUD API，不要求销毁并重建整个列表：

- `push_data`：运行中新增任务，并返回可继续操作的 `Entity<TodoItem>`。
- `find` / `item`：按稳定 ID 查找任务。
- `set_title`、`set_description`、`set_checked`、`set_disabled`：按 ID 编辑任务。
- `remove`：按 ID 删除任务；`clear`：清空并返回被移除的实体。
- `on_mutation`：监听 added / removed / updated / checked 变更，适合持久化或同步远端状态。

列表也支持原地编辑，不需要再放置一个固定的外部表单：

- `TodoItem::editor_renderer`：只在当前项进入编辑态时渲染输入框、保存按钮或取消按钮。
- `TodoList::set_editing`：按稳定 ID 切换展示态和编辑态，并保留原有 entity 身份。
- `TodoList::footer_renderer`：在列表底部放置“+ 新增”按钮或其他列表级操作。

新增任务时可以先插入一个空标题的临时项，再立即设为编辑态；保存时写回 `set_title` / `set_description`，取消时删除该临时项。已有任务取消编辑只需恢复展示态。

这些操作只改变列表状态和对应 entity，不会丢失其他任务的焦点或自定义 renderer。

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

renderer 接收 `TodoItemRenderContext`，其中包含稳定 ID、标题、描述、完成态、禁用态和 `editing` 状态。

renderer 可以返回任意原生 GPUI `AnyElement`，因此图标、头像、图片、emoji、标签、按钮和自定义组合都可以直接嵌入：

```rust
TodoItem::new("design", "Polish the hero", cx)
    .leading_renderer(|_| {
        Image::new("https://images.unsplash.com/photo-1558655146-d09347e92766")
            .thumbnail_sm()
            .round()
            .into_any_element()
    })
    .check_renderer(|context| {
        Text::new(if context.checked { "✅" } else { "🟡" }).into_any_element()
    })
```

建议将图片地址、emoji、业务图标名称保存在宿主数据模型中，再由 renderer 闭包捕获；不要把一次性 `AnyElement` 存进 `TodoItemData`。

原地编辑 renderer 可以直接捕获每个任务自己的 `Entity<Input>`：

```rust
TodoItem::new("design", "Polish the hero", cx)
    .editor_renderer(move |_| {
        Flex::new()
            .column()
            .child(title_input.clone())
            .child(description_input.clone())
            .child(Button::new("Save").on_click(save_handler))
            .into_any_element()
    })
```

将编辑、删除动作放到 `trailing_renderer`，将新增动作放到 `footer_renderer`，即可得到“每行操作 + 列表底部新增”的紧凑交互。

## 项目交付看板

待办项也可以组合进度条、优先级徽标、截止状态和锁定态，形成更完整的项目任务看板：

- 进度条放在 `content_renderer` 中，和标题/描述保持同一列布局。
- `TagType::Warning` / `TagType::Danger` 可表达 P1、逾期等风险状态。
- `disabled(true)` 配合锁图标可以展示暂不可操作的任务。
- 完成态可替换为星形、圆形或渐变勾选，并同步标题删除线和透明度。

Gallery 中的“项目交付看板”示例覆盖已完成、进行中、逾期和锁定四类任务。

## 专注任务视觉

如果待办列表属于专注模式，也可以使用深色背景、圆角图标、时间胶囊和自定义勾选态；这些效果仍然只需要四个 renderer slot，不需要把业务逻辑塞进基础 Checkbox。Gallery 的“专注任务”示例演示了这一套独立视觉语言。

## 监听完成状态

### 效果

::Demo{component="TodoListChanges"}::

### 代码

```rust src="todo/changes.rs"
```

通过 `TodoList::on_change` 统一监听任意子项的完成状态变化。回调适合连接异步保存、错误提示或业务状态同步；控件本身不绑定具体的数据层。

## 实现建议

这些效果都只由 renderer 返回原生 GPUI 元素组成，不需要 WebView、CSS 或额外状态层。推荐把业务状态（优先级、截止日期、负责人、进度百分比）保留在宿主数据模型中，再通过闭包捕获只读展示值；勾选状态仍由 `TodoItem` 统一管理。

### 常见自定义组合

| 需求 | 推荐组合 |
| --- | --- |
| 优先级任务 | `leading_renderer` + `Tag::warning/danger` |
| 完成动画或特殊勾选符号 | `check_renderer` + `Text::strikethrough` |
| 截止日期/状态 | `trailing_renderer` + `Tag::round` |
| 负责人和头像 | `leading_renderer` + `Avatar` |
| 进度百分比 | `content_renderer` + `Progress` 或自定义进度条 |
| 禁用/锁定任务 | `TodoItemData::disabled(true)` + 灰度 renderer |

## 与 Checkbox 的关系

简单同质选项继续使用 `CheckboxGroup`；待办项需要复杂视觉、描述、标签、优先级或后续拖拽/分组能力时，使用专用 `TodoList` 可以避免把任务语义继续堆进基础 Checkbox。
