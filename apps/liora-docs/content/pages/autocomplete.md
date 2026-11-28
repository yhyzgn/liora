# Autocomplete

`Autocomplete` 在输入时展示建议列表，适合搜索、路由跳转和命令面板入口。

选择建议会先回填输入值并关闭弹层，再延后调用 `on_select`。回调可以读取同一 `Autocomplete` 的 `value(cx)`，或通过保留的 Entity 更新其建议列表。

默认 `trigger_on_focus(true)`：启用的输入框获得焦点时立即展示当前匹配建议，包括空查询。设置 `trigger_on_focus(false)` 仅关闭焦点触发，输入非空查询仍会展开建议；禁用状态不会因焦点触发展开。

输入框保持焦点时，使用 Up/Down 高亮并滚动到建议，Enter 选择高亮项；Escape 关闭建议并保留输入框焦点，Tab/Shift+Tab 离开且不提交。组合键和输入法组合输入仍由输入框处理。

## 基础用法

### 效果

::Demo{component="AutocompleteBasic"}::

### 代码

```rust src="autocomplete/basic.rs"
```

## 自定义建议

自定义建议也可以配置关闭策略，例如禁用点击外部与 ESC 自动关闭。
即使禁用点击外部自动关闭，建议弹层也不会阻挡其他控件的点击。

### 效果

::Demo{component="AutocompleteCustom"}::

### 代码

```rust src="autocomplete/custom.rs"
```

## 无右侧图标

### 效果

::Demo{component="AutocompleteNoSuffix"}::

### 代码

```rust src="autocomplete/no_suffix.rs"
```

## 禁用状态

### 效果

::Demo{component="AutocompleteDisabled"}::

### 代码

```rust src="autocomplete/disabled.rs"
```
