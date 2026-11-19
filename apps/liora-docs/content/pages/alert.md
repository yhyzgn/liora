# Alert

`Alert` 用于在页面中展示重要提示。每一种效果都紧跟对应代码，避免把效果和代码拆散。

## 类型

### 效果

::Demo{component="AlertTypes"}::

### 代码

```rust src="alert/types.rs"
```

## 辅助性文字

### 效果

::Demo{component="AlertDescription"}::

### 代码

```rust src="alert/description.rs"
```

## 可关闭提示

`closable(true)` 显示关闭按钮，`on_close` 通知父视图更新显示状态；组件不会自行隐藏。同一父级中的可关闭提示需使用不同且稳定的 `.id(...)`，不要使用标题内容或每次渲染递增的 ID。下面两个同内容提示分别拥有独立 ID，父视图独立控制关闭和恢复。使用 Tab / Shift+Tab 切换关闭按钮，键盘聚焦时显示焦点环，Enter 或 Space 释放后触发关闭，再点击对应的“Restore alert”按钮恢复。

Gallery / Docs 的窗口壳为未被子控件处理的 Tab / Shift+Tab 提供焦点遍历；禁用的 Input 不参与。Docs 为虚拟滚动条保留独立的交互区域，关闭按钮不会被滚动条轨道遮挡。

### 效果

::Demo{component="AlertClosable"}::

### 代码

```rust src="alert/closable.rs"
```
