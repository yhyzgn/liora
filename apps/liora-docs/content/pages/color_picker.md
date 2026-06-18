# ColorPicker

`ColorPicker` 用于选择颜色值，支持取色弹层、预设色、透明度和禁用状态。

## 基础用法

默认展示颜色方块和当前颜色文本，点击后打开原生取色弹层。

### 效果

::LioraDemo{component="ColorPickerBasic"}::

### 代码

```rust src="color_picker/basic.rs"
```

## 自定义预设色

通过 `presets` 替换底部快捷色板。

### 效果

::LioraDemo{component="ColorPickerPresets"}::

### 代码

```rust src="color_picker/presets.rs"
```

## 隐藏文本标签

`show_label(false)` 仅展示颜色方块触发器，并可禁用点击外部/ESC 自动关闭以保留取色上下文。

### 效果

::LioraDemo{component="ColorPickerCompact"}::

### 代码

```rust src="color_picker/compact.rs"
```

## 禁用状态

禁用后不可打开弹层。

### 效果

::LioraDemo{component="ColorPickerDisabled"}::

### 代码

```rust src="color_picker/disabled.rs"
```
