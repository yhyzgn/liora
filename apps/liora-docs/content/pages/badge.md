# Badge

`Badge` 用于在按钮、头像或文本上叠加状态和数量提示。徽章不改变子控件的布局，也不接管其点击或键盘行为。Gallery 与本页直接执行下面展示的同一组构造函数。

## 基础用法

示例覆盖 Danger（默认）、Primary、Warning、Success、Info 五种语义类型。`value` 接受文本或整数计数字符串；零值保持显示，不会自动隐藏。

### 效果

::Demo{component="BadgeBasic"}::

### 代码

```rust src="badge/basic.rs"
```

## 最大值

有效整数文本超过 `max` 时显示 `max+`，包括超出 `i32` 范围的大计数；未超出时保留原始文本。非整数文本（例如 `New`）不参与封顶，也不会截断看似数字开头的普通文本。未设置 `max` 时不解析计数。

同一视图中重复出现的同名、同样式按钮需要不同的稳定 `id`。本例为 Messages / Updates 指定最大值示例专用的 ID，避免与基础示例共享交互状态或无障碍节点。

### 效果

::Demo{component="BadgeMax"}::

### 代码

```rust src="badge/max.rs"
```

## 小红点

`is_dot(true)` 用圆点替代数量文本；圆点模式不解析 `value` 或 `max`。

### 效果

::Demo{component="BadgeDot"}::

### 代码

```rust src="badge/dot.rs"
```

## 显示与隐藏

`hidden(true)` 只隐藏数字或圆点徽章，子控件仍然存在并保持原来的尺寸和交互。隐藏分支跳过主题读取、计数解析和徽章元素构造；显示时借用主题，不深复制完整主题。

### 效果

::Demo{component="BadgeVisibility"}::

点击 Toggle badges，或用 Tab 聚焦后按 Enter / Space。宿主视图保留 `hidden`，在 `on_toggle` 中切换它并调用 `cx.notify()`；下面函数不创建全局状态或持续动画任务。按钮标签保持不变，切换时不会因标签变化替换焦点身份或移动后面的控件。

### 代码

```rust src="badge/visibility.rs"
```
