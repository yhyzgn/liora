# Select

`Select` 用于从有限选项中选择一个值，弹层和选择状态都保持在原生 GPUI 视图中。

## 基础选择

默认点击外部或 ESC 会关闭下拉层；需要固定决策上下文时可配置 `close_on_click_outside(false)` / `close_on_escape(false)`。

### 效果

::Demo{component="SelectBasic"}::

### 代码

```rust src="select/basic.rs"
```


## 可搜索选择

`Select::searchable(...)` 覆盖原先 Combobox 的基础搜索能力：输入过滤和空态都收敛在同一个 `Select` 控件里。固定少量选项继续使用 `Select::new(...)`。

### 效果

::Demo{component="SelectSearchable"}::

### 代码

```rust src="select/searchable.rs"
```

## 分组选项

搜索模式支持 `SearchableListItem::group(...)` 和 disabled item，适合组件、命令或资源选择器。

### 效果

::Demo{component="SelectGrouped"}::

### 代码

```rust src="select/grouped.rs"
```

## 多选

调用 `.multiple()` 后，Select 会保留多个 selected values，并在输入区域展示选中摘要。

### 效果

::Demo{component="SelectMultiple"}::

### 代码

```rust src="select/multiple.rs"
```

## 清空与应用状态

单选和多选的 × 清空按钮仅在原先有选中值时，调用一次 `on_values_change` 并传入空 `Vec`。输入或删除搜索词只更新过滤条件，不触发选择变更；没有选中值时清空搜索词也不会发送空选择事件。

下面复用 Gallery 示例；单选卡片显示父视图保存的 `Single selected: [...]`，多选卡片显示应用保存的 selected values 和回调次数。点击 × 后，单选应显示 `Single selected: []`；多选应显示 `Selected: []`，次数仅增加一次。

::Demo{component="SelectClear"}::

```rust src="select/clear.rs"
```

## Footer 扩展

需要创建入口或高级操作时，可以在搜索下拉面板底部添加 footer。

### 效果

::Demo{component="SelectFooter"}::

### 代码

```rust src="select/footer.rs"
```
