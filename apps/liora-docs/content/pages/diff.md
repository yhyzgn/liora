# DiffView

`DiffView` 是纯 Rust/native GPUI 的 UTF-8 文本审阅组件。输入为 `DiffDocument::new(before, after)`，可用 `.paths(Some(before_path), Some(after_path))` 添加仅供显示的路径。它不读取路径、不写盘、不执行 patch，也不启动进程；不接受二进制字节。

## 基础审阅

### 效果

::Demo{component="DiffView"}::

### 代码

```rust src="diff/basic.rs"
```

## 原生审阅交互

- **Unified / Side by side** 按钮切换 inline/双栏布局。两种布局都显示旧/新行号，新增与删除有显式标记或独立列。
- 每个 hunk 的 **Accept / Reject** 按钮记录独立、不可再次覆盖的决策，发出 `DiffEvent::HunkAccepted` / `HunkRejected`。已决策按钮禁用。事件不改变 before/after 文本。
- `.on_locate(|DiffLineNumbers, window, cx| ...)` 注册宿主定位回调，并显示可键盘访问的 **Locate change** 按钮，报告 hunk 首个变化行的旧/新位置。宿主决定如何打开/滚动自己的编辑器。
- `cx.subscribe(&diff, ...)` 订阅审阅事件。`decide_hunk(id, decision, cx)` 提供同样的程序化入口；非法、跨文档或过期 ID 返回 `DiffActionError::UnknownHunk`，重复决策返回 `AlreadyDecided`，失败不会伪装成成功事件。
- `set_document(document, cx)` 先完整计算，成功后原子替换快照并重置决策。失败保留原文档；每个新 hunk 都有不复用的进程内 ID。

## 行映射与边界

`hunks()` 暴露真实 Myers shortest-edit-script 计算的 hunks，含三行上下文；距离过近的上下文自动合并。`DiffHunk::{before_start, after_start}` 为 **零基**索引/插入边界，`*_count` 是源文本行数。`DiffLine::{before_line, after_line}` 和 `DiffLineNumbers` 为 **一基**行号，空侧为 `None`。双栏中替换保留删除/新增独立行，不声称两行具有语义配对关系。

空字符串为零行；两侧都为空时显示明确的 **Empty file — no differences** 状态，空文件不会生成可审阅 hunk。`"\n"` 是一个有终止符的空行。LF、CRLF、缺少末尾换行分别保留并参与比较，CRLF/无末尾换行显示显式提示；孤立 CR 仍是文本内容。Unicode 不以字节拆行。输入只能是 UTF-8 文本；二进制或宿主判定为不适用的资源可通过 `DiffView::with_status(document, DiffStatus::Binary/NotApplicable, cx)` 显示明确状态，组件不会读取、改写或替换文件。

`Ready` / `Empty` 始终从两侧文本是否都为空推导，不能用 `Empty` 隐藏非空文本差异。`set_status(status, cx)` 在规范化后的状态未变化时不重建 hunk，保留 ID 和审阅决策；从 `Binary` / `NotApplicable` 回到文本状态时才重新计算。

## 错误与性能边界

使用 Myers frontier + traceback，而不是 O(n×m) 全表 LCS；无需新依赖。每侧最多 100,000 行，frontier 工作预算 2,000,000 个候选节点。超限分别返回 `TooManyLines` / `WorkLimitExceeded`，不返回截断、猜测或假 diff。计算同步进行，宿主应避免在输入逐字更新路径反复比较大文档。当前不虚拟化 hunk 行。

## Demo 与行为验证

Gallery/Docs demo key：`DiffView`（`diff_demo::render`）。Gallery 展示原生切换、决策事件与定位回调（复制旧/新坐标到系统剪贴板）。独立 snippet：`diff/basic.rs`。

行为测试位于 `diff_view::tests`，覆盖空文件、全新增/删除、Unicode、CRLF/LF/末尾换行、hunk 分组与映射、重复行 Myers traceback、显式资源上限。`cargo run -p liora-gallery --example diff_smoke --locked` 使用官方 Linux/FreeBSD headless 后端，断言原生布局按钮、事件、决策、失效 ID 与源文不变；其他平台明确退出而不假报通过。
