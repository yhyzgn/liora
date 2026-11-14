# Terminal 终端

`Terminal` 是保存为 `Entity<Terminal>` 的原生 GPUI VT/ANSI 屏幕。它使用成熟的 `vt100 0.18.1` 增量解析器，不是将 ANSI 转义序列删除后显示普通文本。

## 终端屏幕

### 效果

::Demo{component="Terminal"}::

### 代码

```rust src="terminal/screen.rs"
```

## 宿主边界

SDK **不启动 PTY、shell 或进程**，不创建线程读取 I/O。宿主持有进程、输出流、输入流及退出生命周期：

1. 在 GPUI 上下文中 `cx.new(|cx| Terminal::new(size, scrollback_capacity, cx))`，长期保存这个 entity。
2. 收到输出后在 GPUI entity update 内调用 `process_output(&bytes, cx)`。字节可以任意分片，包括 UTF-8 和 ANSI 序列中间。
3. 通过 `cx.subscribe(&terminal, ...)` 订阅 `TerminalEvent::InputBytes(Vec<u8>)` 并写入自己的输入流。不要将输入本地回显成输出；终端 echo 应来自宿主进程。
4. 宿主调用 `resize(size, cx)` 更新屏幕。`TerminalEvent::Resized(size)` 只在尺寸变化时发出，宿主据此更新自己的 PTY。容器变化不会猜测或自动改写宿主尺寸。


保持订阅返回的 `Subscription` 存活。完整事件订阅见 Gallery `terminal_demo.rs`，它只记录真实输入字节，不模拟 shell 或伪造进程输出。

## 交互与屏幕

- VT 状态包含 CR 覆盖、clear/move、SGR 前景/背景色、粗体/斜体/下划线/反色及 256 色和 RGB。
- 解析器保存 alternate screen 与主屏幕；屏幕切换来自真实输出控制序列。
- 采用固定 8.4 × 18 逻辑像素的原生单元格；双宽字符占两列，后续列不会偏移。
- 鼠标左键拖动选择，滚轮浏览保留的 scrollback。滚动、resize 或新输出会清空选择，防止复制过期坐标。
- `selected_text()` 返回选区对应的真实屏幕文本；`copy_selection(cx)` 请求写入系统剪贴板并返回同一 payload，无选区返回 `None`。
- Cmd-C / Ctrl-Shift-C 复制，Cmd-V / Ctrl-Shift-V 粘贴；Ctrl-C 保留为进程字节 `0x03`。粘贴遵循输出设置的 bracketed paste 模式。
- 方向键遵循 application cursor 模式；Ctrl/Shift/Alt + 方向键编码修饰符。Enter、Tab、Backspace、Escape、Home/End、Insert/Delete、PageUp/PageDown 发出终端控制字节。
- 普通输入通过 GPUI 原生文本输入边界提交；IME marked text 在光标处显示，只有提交文本才发给宿主。

## 范围与资源

`TerminalSize::new(rows, cols)` 对每个维度要求 `1..=1024`，不合法时返回 `None`。`rows()`、`cols()` 读取通过校验的尺寸。scrollback 容量由宿主传入；宿主应按自己的内存预算设置合理值。

`size()`、`visible_text()`、`scrollback_len()`、`scrollback_offset()` 和 `is_alternate_screen()` 可用于宿主状态展示及行为断言。不需要额外全局 keybinding 初始化；按常规调用 `init_liora(cx)` 即可。

## 行为验证

- `terminal_model` 单元测试覆盖输出、CR、clear/move/SGR、分片 UTF-8、备用屏幕、历史选区和 resize。
- `terminal_smoke` 通过官方 GPUI headless 窗口派发真实鼠标、滚轮和键盘事件，断言选区/复制 payload、Ctrl/方向键字节、alternate screen、scrollback 和 resize 去重。
- 官方 headless 后端的系统剪贴板为 no-op，因此 smoke **不声称验证操作系统剪贴板读回**；真实桌面剪贴板与 IME 候选窗需要桌面手测。

解析器：[官方源码与 MIT 许可证](https://github.com/doy/vt100-rust)，[固定版本 API](https://docs.rs/vt100/0.18.1/vt100/)。GPUI 使用 workspace 固定的官方 Zed 源码。
