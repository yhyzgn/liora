# Radio

`Radio` 用于单选项和单选组，适合互斥选择场景。

## 基础状态

### 效果

::AuraDemo{component="RadioBasic"}::

### 代码

```rust src="radio/basic.rs"
```

## 单选组

### 效果

::AuraDemo{component="RadioGroup"}::

### 代码

```rust src="radio/group.rs"
```

## 按钮样式

### 效果

::AuraDemo{component="RadioButtons"}::

### 代码

```rust src="radio/buttons.rs"
```


## 自定义选项样式与布局

### 效果

::AuraDemo{component="RadioCustom"}::

### 代码

`option_style` 负责容器、颜色、间距和选中态样式；`option_renderer` 可以按 `selected` / `index` 渲染完全自定义的选项内容，例如图标、说明文本和选中态徽标。

```rust src="radio/custom.rs"
```
