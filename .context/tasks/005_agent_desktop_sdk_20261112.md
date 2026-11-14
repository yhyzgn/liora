# 005 Agent Desktop SDK Execution Checklist

- 状态: 已完成
- 计划: `.context/plans/005_agent_desktop_sdk_20261112.md`
- 规模: 多工作项增量交付；每项单独验证、审查、回滚
- 依赖: 官方 GPUI 固定版本、现有组件与 Gallery/Docs
- 是否改变生产行为: 是（SDK-01 至 SDK-09 实现、集成、根因修复及本地验收已完成；真实 IME 与跨平台实机边界另记）

## 任务目标

维护唯一工作清单并依次交付用户选择的通用 SDK 能力。只有实现、行为验证、独立审查及修复均完成，才把对应项标为已完成。

## 范围

- [x] SDK-01 动态 Markdown：原生富文本、表格/代码/链接/图片、不完整输入和持续更新。
- [x] SDK-02 会话滚动：可变高度、跟随/脱离、历史插入锚点、位置恢复。
- [x] SDK-03 Composer：引用/附件/粘贴、发送与换行/IME、草稿和忙碌状态。
- [x] SDK-04 diff：真实行差异与 hunk、定位、接受/拒绝事件。
- [x] SDK-05 终端：真实 VT/ANSI、光标/选择/scrollback、输入/resize 接口。
- [x] SDK-06 Splitter：真实拖动与键盘尺寸调整、方向/边界/回调。
- [x] SDK-07 Dock：切换/关闭/重排、分组分屏、序列化恢复和 center 内容。
- [x] SDK-08 命令面板：命令身份与可用性、排序、键盘执行和菜单 Action 接入。
- [x] SDK-09 编辑器：接受补全、查找导航与替换、历史/渲染规模边界。
- [x] 集成验收：Gallery/Docs、compile-checked snippets、双语 README 和 CHANGELOG。

## 非目标

用户已选择只补 SDK。Agent 连接与执行生命周期、权限执行、事件/会话数据库、账户、OAuth、deep link、单实例、后台系统通知、进程 supervisor 属于独立产品清单，不在组件库实现；没有把这些项算作已完成。任意 HTML/JS artifact 不在原生运行时执行。

## 预期文件

`crates/liora-components/src/` 的独立组件模块、共享导出/初始化与 manifest；必要的 `apps/liora-gallery` 演示与原生 smoke、`apps/liora-docs` 页面/snippets；README.md、README.zh-CN.md、CHANGELOG.md；本计划/任务和稳定系统事实。

## 验收标准

每项遵循所属计划的行为契约。工作项记录包含改动文件、实际命令及结果、GUI/场景证据、审查者与结论、修复情况和已知限制；没有验证或审查的项保持未完成。禁止只检查源码字符串来代替行为证明。

## 验证

- 每项运行最窄的组件行为测试及原生场景，独立审查后处理发现，再按需要重验修改路径。
- 最终运行 `cargo check --workspace --all-targets --locked`、相关组件测试与 `cargo check -p liora-docs --bin check_snippets --locked`。
- `python scripts/context_bootstrap.py validate --root .` 校验规范上下文。
- 不把 headless 布局、编译或进程存活描述成真实 GPU/IME/三平台成功。

## 风险与回滚

每项独立记录 API/依赖/平台风险；回滚该项相关文件，不执行全仓 restore/reset。公开符号修改前使用 LSP 引用查询，保留错误传播和宿主所有权边界。新依赖须明确许可与来源。

## 完成记录

### 清单建立

- 用户通过表单确认实施范围为 `sdk`；不创建独立 Agent 产品。
- 初始 `git status --short` 输出为空。
- CTX init 演练识别为 existing；未应用迁移或覆盖已有资产。
- 所有九项仍为未完成；尚无实现、测试或审查完成声明。

### 逐项交付记录

仅在实际获得证据后追加；不得预填 PASS。

### 内容与输入实现（尚未验收）

- SDK-01：新增 `markdown.rs` 与 `markdown/{model,render}.rs`，使用既有 `pulldown-cmark = 0.13.3`；新增原生富布局、缓存批量解析、显式连续文本选择、默认拒绝图片及宿主授权、`MarkdownChanged` 事件、Gallery/Docs/snippet 与 `markdown_smoke`。实现者 MarkdownSdk；独立审查 MarkdownReview 已派发，尚无测试 PASS。
- SDK-02：新增 `transcript.rs`，复用官方 GPUI `FollowMode::Tail`、`splice` 与 `remeasure_items`；稳定消息 ID、前插/重排锚点及可序列化位置恢复。新增 Gallery/Docs/snippet 与 `transcript_smoke`。独立审查 TranscriptReview 已派发，尚无测试 PASS。
- SDK-03：新增 `composer.rs`，组合现有 Input 的原生选区/IME 与 action capture；结构化引用/附件、候选、宿主粘贴 payload、busy 草稿和 Send/Stop 事件。新增 Gallery/Docs/snippet 与 `composer_smoke`。实现者 ComposerSdk；独立审查 ComposerReview 已派发，尚无测试 PASS。
- 已观察：官方 Linux headless 剪贴板写入为 no-op、读取为 None；Composer smoke 使用公开粘贴 API 注入 payload，不把它描述成真实系统 Ctrl-V 验证。当前命令环境可见 DISPLAY/Wayland，但尚未完成真实 GPU/系统剪贴板验证。
- 新终端引擎依赖计划为 `vt100 = 0.18.1`。已读取 crates.io 版本元数据：MIT、Rust 1.85、未撤回；来源 `https://crates.io/api/v1/crates/vt100/0.18.1`。锁文件待集中解析。

### 首次窄验证与审查修复

- `cargo test -p liora-components --lib transcript::tests -- --nocapture`：退出 101；组件测试尚未运行，因并行实施中的 Dock/Splitter/diff/editor 共 22 个编译错误中止。完整输出为本次运行 artifact 332；各模块实现者已收到准确诊断继续修复。不得将其记录为 Transcript 行为 PASS。
- TranscriptReview 指出旧 intra-row 偏移恢复未钳制、未测量行 reveal 依赖未知累计高度；已分别用原生 pending-offset 重测和按 index 定位修复，绘制 smoke 加入对应回归断言。审查者定向源码复核已关闭两条，运行结果仍待获得。
- ComposerReview 的 headless 系统剪贴板依赖问题已改为公开 paste payload 注入；定向复核已关闭。系统剪贴板仍未验证。
- MarkdownReview 的代码异步骨架高度、synthetic table ID、列对齐、重复图片、重定向权限及宿主订阅问题已修复；深层结构改为有界树并展平，后续修复了闭合事件造成的空行放大，尚待执行回归。
- `MarkdownImage::{File, Render}` 只接受宿主批准本地路径/已解码图像；远程请求、每跳重定向权限及解码资源限制属于宿主，不由 SDK 自动请求。
- 终端实现和新模块已接入 Gallery/Docs/snippets；TerminalReview、RustContentReview、RustMarkdownReview 在做独立源码审查，当前没有完成声明。

### 2026-11-13 恢复与根因修复（运行验收中）

- 恢复 005 的九项 SDK 范围；不扩展为 Agent 产品，不提交、推送或发布。
- 初步修复 Markdown Event/range 移动后的源偏移、稳定 Rust hunk ID 分配、编辑器 Escape/F3 的 GPUI action 优先级、无 blink 时重绘、完整代码词补全和无执行目标命令的可用性查询。
- 独立审查 `ContentLayoutReview`（code-reviewer）发现九项内容/布局缺陷；`EditorCommandReview`（rust-reviewer）发现十项编辑/执行/集成缺陷。均按根因修复，新增场景断言；两位审查者定向复核已在源码层关闭全部十九项，不代表运行 PASS。
- 内容/布局修复覆盖：冷会话与变高行的逐像素锚点恢复；共享 Input 在换草稿时清除 IME marked/reversed 状态；Markdown 持久焦点、Tab、Enter/Space 和安全链接；多实例 Splitter/Dock 拖拽身份；容器重新测量与受控回调重置；Dock 身份耗尽不部分修改；全新宿主的实际布局恢复。
- 编辑/执行修复覆盖：每次 viewport prepaint 清除旧行坐标；未记录文本编辑清空 delta 历史；批量事务只重建一次行索引；反缩进端点映射；初始 diagnostics provider；相同文本操作不清 redo、不改文本版本/回调；逐视觉行对齐的指针/选区几何；DiffStatus 的文本状态规范化与幂等决策；命令面板 Docs 与 snippet 编译注册。
- `selection_native_smoke` 从实际剪贴板内容验证跨 Markdown 块复制与中/右对齐文本指针选择；保存/恢复 GPUI 可表示的剪贴板格式。运行安排在独立 Xwayland 显示会话，避免触碰桌面剪贴板。该场景尚待执行。
- 第一轮集成检查发现 `layout_smoke` 的临时 `&str` 不符合固定 GPUI `Keystroke::parse` 的静态所有权要求。LSP 确认两个调用均为字符串字面量，已改为 `&'static str`，不增加分配。修复后的集成检查/组件测试/示例构建正在执行；未预填成功结果。
- 编辑器文档已明确：单编辑复用未变行；多编辑批次完整重建一次行索引/行文本；未声称已测得性能数据。

### 命令面板点击根因与定向复核

- 原生事件日志定位：`uniform_list` 对每行独立 `layout_as_root`，行宽仅包住文字，x=400 的空白点击未命中行；公共行增加 `.w_full()`，保留 panel 遮挡、hover 和 shadow，删除临时诊断与行级遮挡实验。
- `cargo run -p liora-gallery --example command_palette_smoke --locked`：PASS；验证空白区点击、禁用命令、执行一次、外部点击关闭与焦点恢复，并覆盖键盘、动态可用性和大列表导航。
- `PaletteRustReview`（rust-reviewer）定向复核：通过，未发现两个目标文件中的消费者可见阻塞；未用源码审查冒充运行或视觉验证。
- `SdkIntegrationReview`（code-reviewer）发现 Docs snippet 加载缺口、八个相关 snippet 编译注册缺口和 Gallery 默认顺序回归；已按原约定修复，编译与既有断言验证进行中。
- `cargo build -p liora-gallery --examples --locked`：超时（600 秒），无成功声明；后续改为明确列举 SDK smoke 的定向构建，限制并发链接。

### 集成注册定向修复

- Docs 加载器已接入 `command_palette/basic.rs`；`check_snippets` 已注册两份 DockLayout 和六份 CodeEditor 示例；Gallery 保持 Button 为默认首项，未修改既有默认顺序断言。
- 新注册示例的实际编译暴露 `code_editor/configuration.rs` 的 E0063：遗漏新增选项字段。用既有 `..CodeEditorOptions::default()` 初始化惯例补齐，不改变显式选项、不抑制诊断。
- `SdkIntegrationReview` 对上述四处修正定向复核通过，四条发现均在源码层关闭；运行结果另记。
- `cargo fmt --all --check`：修正输出指出的五个本任务文件格式后 PASS（退出码 0）；未以格式检查代替行为验收。

### 集成检查与原生场景实测

- `cargo check --workspace --all-targets --locked -j 2`：PASS；`cargo test -p liora-components --lib --locked -j 2`：424 passed、0 failed。
- SDK 定向构建（八个交互 smoke、`selection_native_smoke` 和临时视觉 probe）：PASS。未将先前超时的全 examples 构建记为成功。
- 直接运行已构建 `markdown_smoke`、`transcript_smoke`、`composer_smoke`、`diff_smoke`、`terminal_smoke`、`command_palette_smoke`、`editor_interaction_smoke`：全部 PASS。Terminal 仅验证复制 payload；Composer 使用公开粘贴 API 注入，不是 OS Ctrl-V 或真实 IME 候选窗验证。
- `layout_smoke` 暴露分屏后 group 内容/放置区域未填满叶节点尺寸；`render_group` 增加 `.size_full()` 后，跨组移动及空组折叠断言已通过，执行继续到新宿主恢复场景。随后发现 fixture 的 typed `WindowHandle::update` 内 draw 导致根实体二次借用；已统一改用 `App::update_window`，保留全部交互与几何断言，复验进行中。
- 首轮 Gallery/Docs app tests 分别为 67 passed/1 failed、67 passed/4 failed；未标记整套 PASS。修复真实的文档示例配对/命令身份缺口；删除过期发布文案和源码拼写断言，不重钉旧计划或 GPUI 函数名。默认 Gallery 顺序、Docs 页面覆盖、snippet 加载与严格效果/代码配对契约保留。
- Gallery 和 Docs 的公共示例身份统一为 `DiffView`、`MarkdownView`；SDK Markdown 独立成 `markdown_view.md` 并注册导航，旧 `markdown.md` 保留 Docs 私有 renderer 说明，不复制两套规范内容。
- 独立 Xwayland 首轮 Markdown/Transcript 真实截图成功；Composer 在 render 阶段过早报告 ready，截图空白。临时 probe 已改用官方 `Window::on_next_frame` 在当前帧完成后报告，并补齐既有 `PortalLayer` 宿主；真实剪贴板和剩余截图尚待复验。

### Linux 原生显示与集成复验

- Gallery `cargo test -p liora-gallery --lib --locked -j 2`：67 passed、0 failed；Docs `cargo test -p liora-docs --bin liora-docs --locked -j 2`：69 passed、0 failed。`cargo check -p liora-docs --bin check_snippets --locked -j 2`：PASS。
- 独立 Xwayland 显示会话完成九项 SDK 场景的真实绘制截图（含打开的命令面板），已检查 `/tmp/liora-sdk-visual/contact.png`；临时 probe 用帧完成回调报告 ready，退出后关闭隔离显示。
- `selection_native_smoke` 在该显示会话 PASS：真实 GPUI 剪贴板跨 Markdown 块复制，以及中/右对齐文本的指针选区；恢复此前 GPUI 可表示的剪贴板内容。未据此宣称 Composer 系统 Ctrl-V、真实输入法候选窗或 macOS/Windows 已通过。
- Markdown 与 Composer 的组件测试、原生 smoke、定向源码审查及修复闭环完成。Composer 的 marked composition 通过公开原生输入 handler 验证，不是人工输入法实机认证。
- `PaletteRustReview` 对 Dock 叶节点 `.size_full()` 和 fixture `App::update_window` 借用修正定向源码批准；`SdkIntegrationReview` 对文档身份、MarkdownView 页面、效果/代码配对及过期断言移除定向批准；两者均未代跑命令。

### 恢复场景的测量 fixture 根因

- 复验通过原始 Dock 分组移动/空组折叠、全新宿主内容与恢复几何，随后在“restored native cross-group drop emits layout”失败；未将整套 `layout_smoke` 标记 PASS。
- 有界诊断显示：`measured("a")` 的 absolute canvas 未设置 inset，原点记录为 `(14,80)`；推导的起拖点 `(34,52)` 落在标签栏下方，两次原生 move 的 active-drag 均为 false。官方 GPUI 每次鼠标事件都会重算 hit test，未采用额外 MouseMove 或组件兼容分支。
- fixture 测量 canvas 增加 `.top_0().left_0()`；移除诊断日志，保留全部几何、恢复交互、多实例隔离和受控尺寸断言。修复后的原生场景、workspace check 和 fmt 验证进行中。
- `PaletteRustReview` 定向复核批准测量修正：canvas 保持脱离普通流和空绘制，显式 inset 使记录原点对应真实内容槽，不改变生产 Dock 行为。运行结论仍以本轮命令输出为准。

### 集成收尾

- Gallery/Docs 九项能力导航与示例身份同步完成，Docs 页面覆盖、效果/代码配对、snippet 加载测试和实际 snippet 编译均已通过；中英文 README、CHANGELOG 和稳定风险记录已同步真实验证边界。
- 真实显示验收后已删除临时 `sdk_visual_probe.rs`、隔离显示脚本及截图采集脚本，保留永久交互 smoke 和 `/tmp/liora-sdk-visual/` 截图证据。
- 本轮 `python scripts/context_bootstrap.py validate --root .` 输出 `context validation OK`。最终布局/集成命令尚未返回，不预填 Splitter、Dock 或整项完成。
- 测量修正后的 `cargo run -p liora-gallery --example layout_smoke --locked -j 2` 编译完成，但原生驱动未退出，1200 秒超时；同串行命令中的最终 workspace/fmt 未执行。保留失败结论，正在定位 deferred 场景调度，后续原生运行使用独立 30 秒上限。
- gdb 实测：程序已越过全部 Dock 原始/恢复/隔离断言；在 `run_split_cases` 首次排入 `cx.defer` 的断点，`cx.pending_updates == 0`，调用栈直接来自 `Application::run` 启动回调。官方 `App::defer` 只入 effect 队列，`finish_update` 才刷新；无后续显示事件，故驱动永久等待而未执行第一组 deferred 断言。
- 唯一场景调用改为 `cx.spawn(async move |cx| cx.update(run_split_cases)).detach()`，让真实断言进入正式更新周期，不添加 no-op 刷新或生产分支。调试器会话已结束；构建与 `timeout 30s target/debug/examples/layout_smoke` 分离后重新验证。

### 最终验收完成（2026-11-13）

- `cargo build -p liora-gallery --example layout_smoke --locked -j 2`：PASS；独立 `timeout 30s target/debug/examples/layout_smoke`：退出 0，输出 `layout_smoke: native resize isolation, measured bounds, controlled resets, dock isolation and fresh rendered restore completed`。全部 Splitter/Dock 交互、测量和恢复断言已执行，不再依赖进程存活判断。
- 最终 `cargo check --workspace --all-targets --locked -j 2`：PASS；`cargo fmt --all --check`：PASS。先前组件 424、Gallery 67、Docs 69 项测试及 `check_snippets` 的成功结果保留；未声称本轮运行了 workspace 全测试。
- `PaletteRustReview` 已批准 `spawn → AsyncApp::update` 入口修正机制（审查时为拟议改动）；实际应用由 edit 回执证明，实际行为由上述完整 smoke 证明。
- SDK-01 至 SDK-09、Gallery/Docs、双语 README、CHANGELOG 及 CTX 验收闭环完成。未新增 Agent 产品、PTY/LSP 进程管理、发布、提交或推送。
- 已知边界：Composer 系统 Ctrl-V、实际输入法候选窗、Windows/macOS 和硬件实机覆盖未验证；严格 Clippy 沿用已记录基线债务，未新增抑制。截图证据位于 `/tmp/liora-sdk-visual/`，属于临时文件而非永久验收资产。
- 完成状态落盘后再次运行 `python scripts/context_bootstrap.py validate --root .`：退出 0，输出 `context validation OK`。

### Terminal 响应性修复

- 将终端每单元格一个 GPUI `div` 改为每行一个 canvas，使用官方 GPUI 终端相同的 `shape_line(..., Some(cell_width))` 保持固定字格；为每个宽字符 continuation 保留 render-only 空格，使后续字形、选区和光标继续对齐 VT 列。原生元素数量从按单元格增长改为按行增长。原生文本输入层显式锚定在终端视口左上角并铺满视口。
- `terminal_smoke` 新增点击焦点与 native caret 原点断言，并实际绘制调整后的 24×80 终端及宽字符/组合字符；验证两次 resize 事件。定宽 canvas 版本构建及独立 30 秒 smoke 均 PASS。
- `cargo test -p liora-components --lib terminal:: --locked -j 2`：PASS，12 passed、0 failed；新增 `wide_cell_render_placeholder_preserves_following_vt_column` 覆盖生产 cell 转换路径。
- `cargo build -p liora-gallery --example terminal_smoke --locked -j 2` 和 `timeout 30s target/debug/examples/terminal_smoke`：PASS；覆盖 VT 状态、指针选择/复制 payload、键盘字节、scrollback 与 resize。未测试系统剪贴板。
- 独立 Xwayland OS XTest 点击和 `a`、`b`、Enter 实测，原生 input handler 发出 `InputBytes` 61/62/0d。`TerminalReview-2` 和 `TerminalRustRelease` 复核宽字符修复通过；实际 IME 候选窗和 GPU 性能基准未验证。

### v0.6.0 发布准备

- 用户授权：最终测试通过后提交并推送全部项目改动，创建新 tag 触发 GitHub Actions 构建与发布。本轮九项 SDK 能力按次版本发布为 `v0.6.0`；18 个工作区 package、内部版本依赖、lockfile、双语 README 和发布清单同步。
- 已确认 GitHub CLI 账号有 repo/workflow 权限；仓库存在 `CRATES_IO_TOKEN`、`LIORA_RPM_SIGNING_PRIVATE_KEY` secret 和独立 RPM fingerprint variable。仅确认名称/公开 fingerprint，未读取或导出私钥。
- tag 将触发现有 `package.yml` 的三平台应用构建及 GitHub Release，以及 `release-sdk.yml` 的 crates.io SDK 发布；不改变保护、签名策略或发布行为，仅修正生成 notes 中过时的 git-only SDK 文案并增加本轮能力说明。

- 宽字符修复及 Docs 版本文案更新后，串行复跑 `cargo fmt --all --check`、CTX 校验、`cargo check --workspace --all-targets --locked -j 2`、`cargo test --workspace --locked -j 2` 和 `git diff --check`：全部 PASS；44 组测试，708 passed、0 failed、1 ignored（含 doctest）。
- 串行 `cargo check -p liora-docs --bin check_snippets --locked -j 2`、`cargo doc --workspace --no-deps --locked -j 2`、release profile 的 `xtask package validate`、`release-readiness --dry-run`、`ci --app gallery --format platform-defaults --dry-run --skip-build`、`install-smoke --app gallery --format platform-defaults --dry-run`：全部 PASS。本地 dry-run 不替代保护环境 RPM 签名或真实安装/卸载。
- 当前版本 Gallery/Docs 在独立 Xwayland 各保持运行 10 秒并完成截图；已检查 `/tmp/liora-release-app-smoke/` 两份真实原生界面。临时 Terminal probe 与 OS 输入脚本已删除，永久回归 smoke 保留。
- 本机 crates.io 查询经 reader 和 curl 均遭 connection reset；不把 registry 可达性或 SDK 发布记为成功，实际受保护 CI 结果为准。
