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

`Dialog::close(cx)`（包括 `MessageBox::close(cx)`）只关闭最上层 modal，保留下层对话框；空栈时无操作。手动关闭不会调用 `on_close`，业务状态清理由调用方负责。需要清空全部 modal 时显式使用 `liora_core::clear_active_modal(cx)`。

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

## 嵌套对话框

点击 `Open Parent Dialog` 打开父对话框，再点击 `Open Child Dialog` 打开子对话框。在子对话框中点击 `Close` 后，父对话框应保持打开，并可继续使用 `Open Child Dialog` 再次打开子对话框。

### 效果

::Demo{component="Dialog"}::

### 代码

```rust src="gallery/dialog_demo.rs"
```
