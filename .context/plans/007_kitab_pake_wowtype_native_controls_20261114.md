# 007 Kitab/Pake/WowType Native Controls - 2026-11-14

- 状态: 已完成
- 负责人: omp
- 当前任务: `.context/tasks/007_kitab_pake_wowtype_native_controls_20261114.md`

## 目标

在 Liora 的纯 Rust + 原生 GPUI 边界内，参考 Kitab、Pake 和 WowType 的可验证交互模式，交付可复用的 PDF 阅读器、查找栏和候选项面板控件。

## 外部调研结论

- Kitab (`https://github.com/t21dev/kitab`, audit commit `c8eb989b9e32866366e226334657ca88a6a24c67`) 使用 GPUI + MuPDF，价值模式包括页面渲染缓存、页码导航、缩放/适配、旋转、缩略图/目录侧栏、搜索和批注工具。Kitab 为 AGPL 项目，不能复制其实现到当前未定许可证的 Liora；只参考公开行为和架构边界。
- Pake (`https://github.com/tw93/Pake`, audit commit `6498b4d72d16049ad5bb929d32d6349e55f215ba`) 是 Tauri/WebView 桌面包装器，价值模式包括原生窗口命令、系统托盘、查找页、快捷键和窗口生命周期。Liora 已有 tray/titlebar/command-palette；本阶段只新增不依赖 WebView 的通用 `FindBar`，不迁移注入 JS/CSS 或浏览器容器。Pake 源码为 GPL-3.0，生成产物例外不覆盖源码复制。
- WowType (`https://github.com/Gloridust/WowType`, audit commit `d145a93b1aac9fb1d464de0646f62374cd4ebb4d`) 是 fcitx5 输入法，价值模式包括候选项面板、数字快捷选择、前后翻页、组合态和显式提交/取消。其应用代码为 MIT/Apache-2.0，但词典、语言模型和 emoji 数据有独立许可证；本阶段只实现不依赖 fcitx 的通用 `CandidateBar` 数据/交互控件，不复制模型或词典。

## 实施契约

- SDK 提供状态、布局、事件和纯 Rust Hayro PDF 字节解码；文件 I/O、搜索文本索引、文本选择和批注持久化由宿主提供。
- `PdfDocument::from_bytes` 消费字节并在宿主后台 worker 中生成真实页面/缩略图；也接受宿主 `Arc<gpui::RenderImage>`。`PdfReader` 支持页码、连续/单页模式、缩放、视口适配、实际图像旋转、缩略图、搜索请求和工具状态。
- `FindBar` 只管理查询文本和上一项/下一项/关闭事件；匹配计算由宿主提供。
- `CandidateBar` 只管理候选展示、键盘序号提示、翻页和提交/取消事件；输入法引擎和学习数据由宿主提供。
- 不引入 WebView、HTML/DOM、MuPDF 源码、PDF 文件写入、fcitx、词典或语言模型依赖。

## 预期文件

- `crates/liora-components/src/pdf_reader.rs`
- `crates/liora-components/src/find_bar.rs`
- `crates/liora-components/src/candidate_bar.rs`
- `crates/liora-components/src/lib.rs`
- `apps/liora-gallery/src/demos/pdf_reader_demo.rs`
- `apps/liora-gallery/src/demos/mod.rs`
- `apps/liora-docs/content/pages/pdf_reader.md`
- `apps/liora-docs/src/markdown.rs`
- `docs/kitab-pake-wowtype-review.md`
- `.context/system/overview.md` 或 `.context/system/conventions.md`（仅在稳定事实变化时）

## 阶段

1. 调研、许可证边界和 API 契约。
2. PDF Reader vertical slice：页面模型、工具栏、页码/缩放/旋转/侧栏、事件。
3. Pake/WowType 通用控件：FindBar、CandidateBar。
4. Gallery/Docs、测试、原生 smoke 和上下文同步。

## 验收

- PDF Reader 能显示原创两页 PDF 的真实文字/彩色矢量内容；Gallery/Docs 可使用原生文件选择器加载本地 PDF，失败保留旧文档并显示错误。
- PDF Reader 不读取路径、不下载资源、不执行脚本、不写回 PDF。
- FindBar 能输入查询并发出 previous/next/close/query 事件。
- CandidateBar 支持候选页、1-9 快捷选择提示和提交/取消事件，候选内容只来自宿主。
- 组件测试覆盖页码边界、缩放边界、旋转、候选翻页和空状态。
- Docs snippets、Gallery demo、workspace check/test、fmt、CTX validation、diff whitespace 通过。

## 风险与回滚

- 解码限制 64 MiB 输入、128 MiB 保留栅格、2048 像素最长边；解析器/临时渲染/GPU 内存不受该预算约束。大文档按需渲染和恶意 PDF 沙箱不在本阶段。
- 只回滚新增控件、导出、demo、docs 和本计划/任务，不恢复或重置其他用户改动。
