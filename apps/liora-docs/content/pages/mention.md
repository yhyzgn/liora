# Mention

`Mention` 复用 Liora `Input` 作为输入内核，在触发符后显示候选项。适合 @成员、#事项、命令引用等场景。

候选弹层支持鼠标和键盘两种选择方式：移动鼠标会同步高亮当前候选，点击候选会把当前触发符查询（例如 `@al`）替换为候选 `value`（例如 `@alice `）并触发 `on_select`；键盘可以用 `Up` / `Down` 在候选中循环移动，用 `Enter` 选择当前高亮项并完成同样的回填。

## @ 成员提及

### 效果

::LioraDemo{component="MentionPeople"}::

### 代码

```rust src="mention/people.rs"
```

## 自定义触发符

### 效果

::LioraDemo{component="MentionIssues"}::

### 代码

```rust src="mention/issues.rs"
```

## 禁用状态

### 效果

::LioraDemo{component="MentionDisabled"}::

### 代码

```rust src="mention/disabled.rs"
```
