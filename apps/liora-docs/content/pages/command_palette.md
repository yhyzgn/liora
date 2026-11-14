# CommandPalette 命令面板

`CommandPalette` 是原生 GPUI 命令执行界面，不是过滤列表或额外事件总线。每个 `CommandPaletteCommand` 具有稳定 `CommandPaletteId`、可本地化标签/描述、快捷键提示、显式禁用状态以及可选宿主可用性谓词。查询按 Unicode 字符、不区分大小写进行 fuzzy 子序列匹配；连续字符、词边界和标签优先，多个查询词必须全部匹配元数据。

## 交互演示

### 效果

::Demo{component="CommandPalette"}::

### 代码

```rust src="command_palette/basic.rs"
```

Gallery 展示 File 菜单和 palette 共享 `document.save` 身份并派发相同 `SaveDocument` GPUI action，以及 250 个真实宿主 callback 命令。禁用项保留显示，但上下键会跳过，鼠标及 Enter 都不能执行。候选行由固定版本 GPUI `uniform_list` 按可视范围生成，选中项移动时滚动跟随。

## 宿主接入

1. 正常初始化 `liora_components::init_liora(cx)`。如果使用尚未集成该绑定的低层初始化入口，启动时调用一次 `CommandPalette::register_key_bindings(cx)`；它只注册 `CommandPalette` 上下文的 Escape。
2. 把 `Entity<CommandPalette>` 保存为宿主字段并添加到渲染树。应用根层在内容之后渲染 `PortalLayer`（Gallery 和 Docs 已有 portal host，不要重复添加）。
3. 打开按钮或宿主快捷键调用 `palette.update(cx, |palette, cx| palette.open(window, cx))`。组件记住当前焦点并把焦点移入查询输入，关闭或执行后恢复。
4. `.with_action(SaveDocument)` 使用 GPUI 原生 action dispatch；宿主用 `.on_action(...)` 处理同一个 action。`from_menu_item(&item)` 复制菜单的稳定 ID、标签、快捷键和禁用状态，不猜测如何执行 `MenuAction`。在菜单的 `.on_select(...)` 里把该 ID 映射到相同 GPUI action 即可。分隔符和子菜单父项会映射为禁用项。
5. 无需 GPUI action 的命令使用 `.on_execute(|id, window, cx| ...)` 调用已有宿主业务逻辑。它与 `.with_action(...)` 互斥，后设置的执行入口生效；未绑定执行入口的元数据命令显示为不可用，查询不会 panic。快捷键文本只是提示，不自动注册全局快捷键。

`available_when(|cx| ...)` 在搜索、导航以及实际执行时读取宿主状态。宿主状态变化后调用 `refresh(cx)`，使活动项和画面立即更新；即使尚未刷新，执行时也会重新校验，不能绕过可用性。谓词必须只读取状态、不要产生副作用。

`set_query(query, cx)` 同步输入文本和候选，重新选择排名最高的可执行结果；无结果或所有结果禁用时没有活动项。`set_commands(commands, cx)` 替换命令集并重算选择。每个 palette 内的命令 ID 应唯一且不随标签/本地化变化。↑ / ↓ 循环移动、Enter 执行、Escape 或遮罩点击关闭。

## 验证入口

- `cargo test -p liora-components command_palette::tests --locked`：fuzzy 排序、多词匹配、菜单身份映射。
- `cargo run -p liora-gallery --example command_palette_smoke --locked`：官方 Linux/FreeBSD headless GPUI 真实键盘事件，覆盖 action/callback、禁用和动态可用性、上下键、Escape、焦点恢复、空结果及 2,000 候选滚动导航。

原生实现依据仓库固定 GPUI 源码：`third_party/zed/crates/gpui/src/window.rs` 的 `dispatch_action`、`focus` 和 `defer`，以及 `elements/uniform_list.rs` 的虚拟列表与 `UniformListScrollHandle`。没有 WebView/DOM 或自建命令总线。
