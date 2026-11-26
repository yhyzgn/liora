# Loading

`Loading` 用于展示局部或全屏加载状态，内置旋转和淡入动效。

## 基础用法

### 效果

::Demo{component="LoadingBasic"}::

### 代码

```rust src="loading/basic.rs"
```

## 全屏配置

`Loading::full_screen()` 绘制覆盖定位容器的遮罩，本身不拦截输入或管理焦点。将 `PortalLayer` 挂在窗口根节点最后，再在加载期间调用 `push_portal`，可覆盖窗口内容区并拦截背景指针点击。

点击 **Preview full-screen loading** 查看真实遮罩，点击右上角 **Cancel preview** 立即关闭，或等待 3 秒自动结束。预览保留在当前页面实例中；取消、超时或移除实例会停止该任务。此示例不是键盘模态对话框，不限制背景键盘焦点；需要模态交互时使用 `Dialog`。

### 效果

::Demo{component="LoadingFullscreen"}::

### 代码

```rust src="loading/fullscreen.rs"
```
