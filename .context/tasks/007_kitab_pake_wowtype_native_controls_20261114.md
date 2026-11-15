# 007 Kitab/Pake/WowType Native Controls Execution - 2026-11-14

- 状态: 已完成
- 计划: `.context/plans/007_kitab_pake_wowtype_native_controls_20261114.md`
- 目标: 参考三个开源仓库的交互模式，在 Liora 封装原生 GPUI PDF Reader、FindBar 和 CandidateBar

## 目标范围

- Kitab：真实 PDF 字节解码、原生页面阅读、页码、缩放、视口适配、实际旋转、缩略图、搜索请求和工具状态；Gallery/Docs 宿主后台读取本地文件。
- Pake：不依赖 WebView 的查找栏、上一项/下一项/关闭事件；已有 tray/titlebar/command-palette 不重复实现。
- WowType：不依赖 fcitx 的候选项栏、数字快捷键提示、候选翻页、提交/取消事件。

## 非目标

- 不复制 AGPL Kitab 或 GPL Pake 源码。
- 不引入 MuPDF/PDFium、WebView、HTML/DOM、JavaScript、fcitx、词典、语言模型或 PDF 写回依赖。
- 不在 SDK 读取文件、下载资源、执行搜索索引、保存批注或修改宿主输入法状态。

## 验收

- [x] `PdfReader`、`FindBar`、`CandidateBar` 公开 API 和事件可编译。
- [x] 页面、缩放、旋转、工具、查询和候选翻页边界有回归测试。
- [x] Gallery 有真实 PDF demo；Docs 有 compile-checked 页面/snippet。
- [x] workspace all-targets check、727 个 workspace 单元/集成测试、3 个 doctest、格式与 CTX 检查通过；1 个既有 doctest ignored。未运行 Git diff 校验。
- [x] 原生 smoke 覆盖真实 PDF、连续滚动、Find Enter/Shift-Enter/Escape、组合态保持和候选分页/提交/取消；Linux Xwayland 实际窗口已确认页面文字、RGB 图形、缩略图与 90° 旋转。

## 风险与回滚

- 由于 Liora 当前许可证尚未确定，所有实现保持独立原创，只吸收公开行为和架构模式。
- Hayro 0.9.0 为 Apache-2.0 OR MIT；保留栅格预算不覆盖解析器/临时工作/GPU 内存，不宣称恶意 PDF 沙箱安全。工具状态不代表完整搜索、文本选择或批注引擎。
- 回滚新增模块、导出、demo、docs 及本任务文件即可，不重置无关工作区。

## 验证记录

- `cargo check --workspace --all-targets --locked`: PASS（初次字体/图像类型及 smoke API 编译问题已修复）。
- `cargo test -p liora-components pdf_reader --locked`: PASS，9 个 PDF 测试，包括真实矢量/文字/颜色解码、近零页面拒绝、像素旋转、视口尺寸和页序号规范化。
- 首次 `cargo test --workspace --locked`：727 passed、1 failed；失败为 SDK README 引用了 app 路径，已移除该引用，`cargo test -p xtask sdk_crates_do_not_reference_repository_demo_apps --locked` 随后 PASS。
- `cargo check -p liora-docs --bin check_snippets --locked`: PASS。
- `cargo test --workspace --doc --locked`: PASS，3 passed、1 既有 ignored。
- `pdf_reader_smoke`: PASS，真实 PDF 解码/绘制、25%–400% setter、页零拒绝、原生键盘翻页、连续定位/手动滚动、旋转绘制和 Find 焦点/Enter 转发。
- `find_bar_smoke`: 最终 PASS；实际 Enter/Shift-Enter 不插入换行，Escape 恰好发一次 Closed；修饰键不导航，组合文本/标记范围保持，普通 Input 仍换行。附加 CandidateBar 原生 PageDown/数字提交/Enter literal/Escape cancel 事件断言全部通过。
- 最终 workspace 单元/集成测试：28 suites、727 passed、0 failed。该轮 doctest 因并发重建导致旧 rlib 不存在而中断；停止并发构建后单独 `cargo test --workspace --doc --locked` PASS，16 suites、3 passed、1 既有 ignored。
- 最终 `cargo fmt --all --check` 与 `python scripts/context_bootstrap.py validate --root .`: PASS。
- 原生可视：隔离 Xwayland `:24` 启动实际 Gallery，经搜索进入 PdfReader；后台加载原创两页 PDF，确认可读文字、红蓝绿矢量、两页缩略图、顺时针 90° 图像旋转。截图：`/tmp/liora-pdf-loaded.png`。已停止该 Gallery 进程。
- 原生 Open PDF 按钮已触发选择请求；隔离 Xwayland 未呈现 portal 文件对话框，未完成系统文件选择往返验证。真实系统 IME、屏幕阅读器和 Windows/macOS GUI 未验证，不据此声称支持。
- Rust/backend 与原生交互审查发现的近零页面 fallback、组合态换行问题均已修复并有实际回归验证。未提交、未推送、未发布。

### WebView 前置原生控件复验（2026-11-14）

- 按用户要求，在添加任何 WebView 依赖前复验现有原生控件；本轮未修改生产代码或依赖。
- `cargo test -p liora-components --locked`：PASS，447 个单元测试、38 个集成测试、2 个 doctest，共 487 passed、0 failed、0 ignored。
- `cargo check -p liora-docs --bin check_snippets --locked` 与 `cargo build -p liora-gallery --examples --locked`：PASS。
- 独立执行 13 个 headless smoke：`gpui_upgrade_smoke`、`hover_watermark_smoke`、`todo_smoke`、`markdown_smoke`、`transcript_smoke`、`composer_smoke`、`diff_smoke`、`terminal_smoke`、`layout_smoke`、`command_palette_smoke`、`editor_interaction_smoke`、`pdf_reader_smoke`、`find_bar_smoke`，全部退出 0，原生交互断言通过。
- `selection_native_smoke` 在隔离 Xwayland `:27` 退出 0，实际验证 Markdown 跨块剪贴板复制和居中/右对齐指针选区；不触碰桌面会话剪贴板。总计 14 个 smoke、0 failed。
- `cargo fmt --all --check` 与 CTX validate：PASS。源码未发现 `todo!()`、`unimplemented!()` 或 TODO/FIXME/XXX 标记；`liora-components` 无 WebView 后端依赖或运行时引用。源码扫描不代替行为覆盖证明。
- 结论仅为既定范围的原生实现复验通过；006 记录的完整结构化富文本编辑缺口、PDF 宿主搜索/文本选择/批注职责，以及真实 IME、文件选择器往返和 Windows/macOS 验证边界不变。未复跑整个 workspace、严格 Clippy 或发布门禁，未提交、推送或发布。
