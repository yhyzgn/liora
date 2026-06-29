# Dialog

`Dialog` 用于展示需要用户关注或确认的模态内容。它通过 Liora 原生 modal portal 渲染，不离开当前 GPUI 窗口。

## 基础用法

点击按钮打开对话框，内容区域可以放置任意 Liora 组件。

### 效果

::Demo{component="DialogBasic"}::

### 代码

```rust src="dialog/basic.rs"
```

## 手动关闭

关闭遮罩点击与 ESC 关闭，只允许业务按钮主动关闭。

### 效果

::Demo{component="DialogManualClose"}::

### 代码

```rust src="dialog/manual_close.rs"
```

## 自定义内容

内容区可以组合文本、行布局、按钮等任意原生组件。

### 效果

::Demo{component="DialogCustomContent"}::

### 代码

```rust src="dialog/custom_content.rs"
```
