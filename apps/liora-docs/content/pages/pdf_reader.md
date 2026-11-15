# PdfReader PDF 阅读器

`PdfReader` 是纯 Rust + 原生 GPUI PDF 阅读器。`PdfDocument::from_bytes(title, bytes)` 使用 Hayro 在 CPU 上解析并栅格化真实 PDF；无需 MuPDF、PDFium、WebView 或 JavaScript 运行时。

## 基础用法

### 效果

::Demo{component="PdfReader"}::

Gallery/Docs 使用同一个原生 demo：启动时加载随应用捆绑的 PDF，也可通过 **Open PDF** 原生文件对话框选择本地 PDF。文件选择和读取是 demo 宿主行为，不是 SDK 隐式 I/O。

### 代码

```rust src="pdf_reader/basic.rs"
```

`decode_pdf` 返回后台任务。宿主保留并等待该任务，处理 `Result`，成功后在 UI 线程调用 `create_reader`，或调用已有阅读器的 `set_document(document, cx)`。失败时显示错误并保留原文档；不要在 GPUI 渲染或输入回调中同步解码。

## 解码 API 与资源边界

公开签名是 `PdfDocument::from_bytes(title: impl Into<SharedString>, bytes: Vec<u8>) -> Result<PdfDocument, PdfDecodeError>`。它消费宿主提供的字节，预检所有页面后一次性生成页面图像及缩略图；它本身是同步 CPU 工作，需由宿主放到后台 worker。

- 输入最多 **64 MiB**；空输入、无页面、无效或无法解密的 PDF 返回显式错误，不支持密码输入。
- 全部页面和缩略图的保留 RGBA 栅格总预算为 **128 MiB**，超限返回 `RasterBudgetExceeded`，不会返回悄悄截断的文档。该预算不是解析器、临时工作内存或 GPU 内存的总上限。
- 页面栅格最长边最多 **2048 像素**，小页面不放大；无效页面几何和不可渲染尺寸返回错误。放大阅读不会重新以更高分辨率解码。
- 不执行 PDF 脚本、不下载外部资源、不写回 PDF。

宿主也可用 `PdfDocument::new(title, pages)` 提供自己的 `PdfPage` / `Arc<gpui::RenderImage>`，或用 `with_page_count` 建立未加载页。内置解码器是有界、预先渲染的路径，不是大文档按需分页缓存。

## 阅读器交互

- 单页和连续页面布局，页码前进/后退、缩略图跳转。
- Fit page、Fit width、25%–400% 手动缩放和顺时针旋转。
- 嵌入 `FindBar`，通过 `PdfReaderEvent::SearchRequested`、`SearchPrevious`、`SearchNext`、`SearchClosed` 通知宿主。宿主负责搜索索引、匹配定位及高亮；通过 `find_bar()` 访问查询栏，或用 `set_find_match_status()` 回写匹配状态。
- Select、Hand、Highlight、Underline、StrikeThrough、Note、TextBox、Pen、Rectangle、Ellipse、Arrow 是工具选择状态，通过事件交给宿主；不是已实现的文本选择、手形拖动或批注绘制/保存引擎。
- 未加载页面显示原生占位状态。

文件 I/O、下载权限、搜索索引、文本选择和批注持久化始终由宿主负责。

## FindBar

`FindBar` 是独立的原生查询输入实体。订阅 `FindBarEvent::{QueryChanged, Previous, Next, Closed}` 处理查询和导航；用 `set_query` 设置查询、`set_match_status` 显示宿主计算的匹配数。查询值变化会清除旧匹配状态；仅移动光标不会发出查询变化或清除匹配数。

标准 `init_liora(cx)` 会注册 FindBar/CandidateBar 的作用域 Escape 绑定，避免被全局 overlay 关闭动作抢先消费。使用低层初始化时须调用各组件的 `register_key_bindings(cx)`。

调用 `focus` 或通过 `Focusable` 聚焦输入框后，Enter 请求下一项、Shift-Enter 请求上一项、Escape 请求关闭。原生 IME 标记组合态和带 Control/Alt/平台修饰键的快捷键留给输入框，不被查找导航截获。组件不计算匹配、不滚动宿主内容；独立使用时由宿主响应 `Closed` 隐藏它。

## CandidateBar

`CandidateBar` 展示宿主提供的组合文本和 `CandidateItem`，不包含 fcitx、词典、语言模型或系统 IME 集成。`set_items` 替换候选并重置到第一页；默认每页 8 项，`page_size` 限制为 1–9 项。页索引、可见槽位和 `Selected.index` 都是零基；其中 `Selected.index` 是完整候选列表索引，而非当前页槽位。

显式聚焦候选栏后，方向键在当前页内移动，PageUp/PageDown 翻页，1–9 提交可见槽位，Space 提交所选候选，Enter 提交原始组合文本，Escape 取消。带修饰键或焦点在后代输入框中的按键不会被候选栏截获。

订阅 `CandidateBarEvent::{Selected, CommittedLiteral, Cancelled, PageChanged}` 由宿主完成提交、清空组合态或其他业务操作。提交/取消事件不会自动改写宿主文本或清空候选数据。

## 参考范围

- Kitab：仅参考页面/工具栏/侧栏公开交互；其 AGPL 实现未复制，Liora 不绑定 MuPDF。
- Pake：仅参考桌面查找栏语义，不迁移 GPL 的 Tauri/WebView 注入层。
- WowType：仅参考候选数字选择、翻页和提交/取消语义，不迁移输入法引擎、词典或模型。
