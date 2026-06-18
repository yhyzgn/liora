# Checkbox

`Checkbox` 用于独立多选和多选组。每个示例都把效果与代码放在一起。

## 基础状态

### 效果

::LioraDemo{component="CheckboxBasic"}::

### 代码

```rust src="checkbox/basic.rs"
```

## 多选组

### 效果

::LioraDemo{component="CheckboxGroup"}::

### 代码

```rust src="checkbox/group.rs"
```

## 按钮样式

### 效果

::LioraDemo{component="CheckboxButtons"}::

### 代码

```rust src="checkbox/buttons.rs"
```


## 自定义选项样式与布局

### 效果

::LioraDemo{component="CheckboxCustom"}::

### 代码

`option_style` 负责容器、颜色、间距和选中态样式；`option_renderer` 可以按 `selected` / `index` 渲染完全自定义的选项内容，例如多行说明、图标和状态徽标。

```rust src="checkbox/custom.rs"
```
