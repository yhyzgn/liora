# Composer

`Entity<Composer>` 组合原生 `Input`，提供多行草稿、结构化引用和附件。宿主订阅 `ComposerEvent`，负责请求、停止、上传与文件处理；组件不会执行业务或打开文件。

## 消息编辑

### 效果

::Demo{component="Composer"}::

### 代码

```rust src="composer/basic.rs"
```

## 编辑与候选

`input()` 返回原始 `Entity<Input>`，可使用 UTF-8 字节选区 API `selected_range` / `set_selection`，以及 GPUI 原生 UTF-16 IME 接口。没有第二套文本引擎。

`set_candidates` 提供 `Reference` 和 `Command`。在折叠光标前的空白分隔 token 以 `@` 或 `/` 开头时显示匹配候选。Up/Down 切换，Enter 或点击插入，Escape 关闭。只替换触发符到光标之间的文本，保留光标后的文本；非折叠选区不启动候选。引用同时加入结构化引用区，按稳定 ID 去重。命令只是插入草稿，不执行命令。

引用/附件是独立于文本的结构化上下文。删除 chip 只移除对应上下文，不会隐式删除用户文本。每个 chip 使用原生 Button，可以鼠标点击或键盘激活。

## 发送与忙碌态

Enter 接受打开的候选，否则发送；Shift-Enter 在选区插入换行。marked composition 存在时不会发送或插入换行。宿主可调用 `send(window, cx)`，它同样检查 marked composition、空草稿和 busy 状态。

成功发送发出 `Send(ComposerDraft)` 快照并清空当前草稿。`DraftChanged` 包含文本和结构化上下文。宿主通过 `set_busy` 管理执行状态；busy 只把发送按钮换成停止按钮并阻止重复发送，**不禁用编辑**，下一条草稿保留。`Stop` 只是宿主事件，不自动更改 busy 或清空草稿。宿主可用 `draft`/`set_draft` 保存和恢复草稿。

## 粘贴

原生 Ctrl-V / Cmd-V 读取完整 `ClipboardItem` 并发出 `Paste`。字符串条目按顺序插入当前选区一次；图片和文件路径保持原始 payload 交给宿主处理，不自动上传、不把文件路径伪装成已添加附件。宿主处理完成后可调用 `add_attachment`。多个类型混合的剪贴板条目仍完整保留在事件中。也可通过 `paste` 注入平台提供的剪贴板项。

## 集成与验证

只需 `init_liora` 已有的 Input 初始化；Composer 不注册全局按键。原生行为覆盖见 `apps/liora-gallery/examples/composer_smoke.rs`，包括选区、候选、通过公开 `paste` API 注入完整文本/图片/文件 payload 后的插入和事件、发送/停止、busy 草稿和 marked composition。

官方 headless backend 的系统剪贴板写入是 no-op，读取返回 `None`，因此该 smoke 不使用 Ctrl-V 验证系统剪贴板，也不声称验证了操作系统剪贴板。真实 Ctrl-V / Cmd-V 读取与平台图片/文件剪贴板格式须在真实后端验证。headless 的 marked composition 接口断言也不代替真实平台 IME 候选窗口验证。
