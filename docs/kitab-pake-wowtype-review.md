# Kitab / Pake / WowType 调研与迁移边界

日期：2026-11-14

## Kitab

来源：<https://github.com/t21dev/kitab/commit/c8eb989b9e32866366e226334657ca88a6a24c67>
公开项目说明显示 Kitab 是 Rust + GPUI 的桌面阅读器，使用 MuPDF 渲染 PDF，不依赖浏览器引擎。公开功能包括 PDF/EPUB 等阅读、缩放、旋转、适配页面/宽度、目录/缩略图、文本搜索、文本选择、批注工具和批注导出。
审计快照：`c8eb989b9e32866366e226334657ca88a6a24c67`；Kitab 与 MuPDF 均标示 AGPL-3.0-or-later。

迁移到 Liora 的价值模式：

- 页面是宿主后端提供的渲染结果，UI 只负责页面排列、滚动和当前页状态。
- 工具栏把页码、缩放、旋转、搜索和批注工具集中管理。
- 缩略图/目录作为可折叠侧栏，不污染主阅读区。
- 批注工具通过状态和事件交给宿主，不在 SDK 中隐式写回 PDF。

许可证边界：Kitab 标示 AGPL-3.0-or-later。本仓库不复制 Kitab 源码、MuPDF 绑定、资源或实现细节；只依据公开行为设计独立 API。

## Pake

来源：<https://github.com/tw93/Pake/commit/6498b4d72d16049ad5bb929d32d6349e55f215ba>
公开文档说明 Pake 是基于 Tauri/WebView 的网站桌面包装器，提供窗口尺寸/最小尺寸、全屏、置顶、标题栏、系统托盘、关闭行为、快捷键和可选的 Find in Page。Pake 的 WebView 注入脚本、HTML/CSS 和浏览器容器不符合 Liora 的纯 Rust + 原生 GPUI 约束。
审计快照：`6498b4d72d16049ad5bb929d32d6349e55f215ba`；Pake 源码为 GPL-3.0，`LICENSE-EXCEPTION` 仅针对生成产物，不覆盖复制或修改源码。

迁移到 Liora 的价值模式：

- 通用查找栏：查询、上一项、下一项、关闭。
- 快捷键语义应由宿主 Action/窗口层接管；Liora 已有 titlebar、tray、command palette，因此不重复造窗口壳。

许可证边界：Pake 源码标示 GPL-3.0-or-later，另有标准构建输出例外。本仓库不复制 Pake 源码或注入脚本。

## WowType

来源：<https://github.com/Gloridust/WowType/commit/d145a93b1aac9fb1d464de0646f62374cd4ebb4d>
公开项目说明 WowType 是 fcitx5 输入法，使用 Rust 引擎和 C/C++ 集成层；设置界面使用 GTK/libadwaita，不使用 GPUI。交互价值包括候选项面板、数字候选快捷选择、左右移动、上下翻页、提交字面输入、取消组合态和工具栏入口。
审计快照：`d145a93b1aac9fb1d464de0646f62374cd4ebb4d`；应用代码标示 MIT 或 Apache-2.0，但词典、语言模型、CLDR 和语料数据各有独立许可证。

迁移到 Liora 的价值模式：

- `CandidateBar` 只展示宿主提供的候选项和组合文本。
- 候选项提供 1-9 快捷键提示、选中态、分页和提交/取消事件。
- 输入法引擎、学习数据、词典、语音和翻译模型不进入 Liora SDK。

许可证边界：WowType 应用代码公开标示 MIT OR Apache-2.0，但词典、语言模型、emoji/CLDR 数据有独立许可证。本仓库不复制其模型、词典、数据或 fcitx 集成。

## Liora 交付决定

新增原创原生 GPUI 控件：

- `PdfReader`：使用 `hayro = 0.9.0` 从宿主字节解码真实 PDF，或接收宿主注入的 `Arc<gpui::RenderImage>`；提供页码、布局、缩放、实际图像旋转、缩略图侧栏、搜索请求和批注工具状态。
- `FindBar`：通用查询栏和事件控制。
- `CandidateBar`：通用候选项面板和提交/取消事件。

文件 I/O、文本索引、批注保存、窗口/WebView、输入法和模型能力保持宿主边界。Gallery/Docs 宿主用原生文件选择器读取 PDF，后台解码失败保留原文档并显示错误。

## Hayro 许可证与运行边界

- 来源：<https://github.com/LaurenzV/hayro>；固定 crate `hayro = 0.9.0`，发布清单声明 `Apache-2.0 OR MIT`，不复制 Kitab/MuPDF 源码。
- 使用默认内嵌字体与 CMap；分发时仍须遵守 Hayro 及其字体、CMap、解码依赖各自的许可证，Liora 自身的 `LicenseRef-Liora` 不覆盖第三方许可义务。
- `PdfDocument::from_bytes` 同步执行纯 Rust CPU 解码，宿主必须安排后台 worker；限制 64 MiB 输入、128 MiB 保留页面/缩略图栅格、最长边 2048 像素。该预算不限制解析器、临时渲染分配或 GPU 内存，也不是恶意 PDF 的沙箱隔离。
- 放大不会重新解码更高分辨率；大文档按需渲染、密码 UI、搜索索引和批注引擎未引入。`assets/samples/reader.pdf` 是本仓库原创的两页文字/非对称彩色矢量样本。

## WebView 决策

本阶段不封装 WebView。Pake 的行为审计确认其查找功能依赖浏览器 DOM，不能迁移成通用原生 GPUI 控件。若后续产品必须在应用内承载网页，应新建独立的可选 `liora-webview` 集成 crate，隔离平台运行时、导航白名单、脚本桥接、存储、下载、权限和打包依赖；不得把它放入 `liora-components` 默认路径。
