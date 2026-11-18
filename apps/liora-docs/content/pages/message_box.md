# MessageBox

用于全局阻塞式确认或提示。MessageBox 是原生模态层，适合必须等待用户决策的流程；普通非阻塞反馈优先使用 Message/Toast。

## Alert 与 Confirm

`alert` / `confirm` 是常用快捷入口，适合简单提示与确认流程。

点击 Confirm 时会先关闭当前确认框，再执行回调。下面的 Open Confirm 示例在回调中打开后续提示框，该提示框会保留，直到用户将其关闭。

### 效果

::Demo{component="MessageBoxBasic"}::

### 代码

```rust src="message_box/basic.rs"
```

## 手动关闭策略

对于不可误关的对话，可以禁用点击遮罩关闭与 ESC 关闭，只允许按钮路径结束流程。

### 效果

::Demo{component="MessageBoxManualClose"}::

### 代码

```rust src="message_box/manual_close.rs"
```
