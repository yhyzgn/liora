# Dialog

`Dialog` 用于展示需要用户关注或确认的模态内容。它通过 Liora 原生 modal portal 渲染，不离开当前 GPUI 窗口。

对话框无障碍根节点以标题命名；标题为空时使用本地化的“对话框”作为名称。嵌套对话框各自暴露根节点。当前固定版本的 GPUI 未提供 `aria-modal`/modal 状态设置接口，因此组件不声明原生模态语义。

## 基础用法

点击按钮打开对话框，内容区域可以放置任意 Liora 组件。

### 效果

::Demo{component="DialogBasic"}::

### 代码

```rust src="dialog/basic.rs"
```

## 手动关闭

禁用遮罩点击与 ESC 关闭不会禁用标题栏关闭按钮；标题栏关闭按钮和内容中的业务按钮仍可关闭对话框。

遮罩总会消费鼠标左键按下事件：禁用遮罩关闭时不会关闭当前对话框，也不会穿透关闭下层对话框；允许遮罩关闭时，同次点击只关闭最上层对话框。

`Dialog::close(cx)`（包括 `MessageBox::close(cx)`）只关闭当前活动窗口最上层 modal，保留下层及其他窗口的对话框；无窗口归属的 modal 仍参与当前窗口的关闭顺序，没有活动窗口时按全局栈顶关闭。当前窗口无可关闭的 modal 或空栈时无操作。手动关闭不会调用 `on_close`，业务状态清理由调用方负责。需要清空全部 modal 时显式使用 `liora_core::clear_active_modal(cx)`。

标题栏关闭按钮的原生名称为 `Dismiss dialog`，可用 Tab/Shift+Tab 聚焦，并用 Enter 或 Space 关闭。标题栏按钮、允许的 Escape 和遮罩关闭属于 built-in close，调用 `on_close`；这与业务调用 `Dialog::close(cx)` 不触发回调的约定不同。

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

示例中的子对话框禁用遮罩关闭，点击遮罩应保留父、子两层且不增加次数；Escape 仍可逐层关闭。`Child built-in closes: N` 由父视图持有；业务 `Close` 不增加次数，标题栏按钮或 Escape 关闭子对话框时各增加一次。

### 效果

::Demo{component="Dialog"}::

### 代码

```rust src="gallery/dialog_demo.rs"
```
