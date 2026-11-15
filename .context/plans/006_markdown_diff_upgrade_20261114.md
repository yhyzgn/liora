# 006 Markdown and Diff Upgrade - 2026-11-14

- 状态: 已完成(已交付 bounded Rich 段落编辑；完整富文本 composition/undo 映射明确保留为边界)
- 负责人: omp
- 当前任务: `.context/tasks/006_markdown_diff_upgrade_20261114.md`

## 目标

在纯 Rust + 原生 GPUI 约束下增强 Markdown 与 Diff 控件：Markdown 支持常规 CommonMark/GFM 语法、宿主授权图片的稳定布局和同区段落编辑即时渲染/源码模式；Diff 复用 Liora 字体、主题和代码高亮体系，增加可导航、可复制、可审阅的能力。

## 已确认契约

- Markdown 编辑模式为真正 WYSIWYG：同一编辑面直接输入，渲染样式即时更新；提供显式源码模式切换，源码是唯一持久化值。
- 保留原生输入法、选区、撤销/重做和键盘操作；不使用 WebView、HTML/DOM 或浏览器编辑器。
- HTML 不执行；外链与图片仍由宿主策略控制。远程图片由宿主抓取/解码，组件只布局批准资源。
- Markdown 语法基线为 CommonMark + GFM；脚注、定义列表、上下标和数学作为 pulldown-cmark 已有事件的原生渲染，不引入公式引擎。
- Diff 文本只读，不隐式修改文件；代码主题切换使用既有 `CodeTheme`/`CodeEditor` 主题体系，默认跟随 Liora 主题。

## 实施边界

- 先修共享文档模型和渲染映射，再接编辑器；不把 Docs 专属 directive 放入 SDK。
- 只使用现有 `pulldown-cmark`、`CodeEditor`、字体/主题 token 和 GPUI 原生布局能力。
- 图片布局必须有最大尺寸、宽度约束、比例保持、缺失/失败状态和 alt 文本，不能把原图强制裁剪成固定高度。
- 代码高亮和 Diff 主题必须共享 `CodeTheme`，不复制一套颜色常量。

## 阶段

1. Markdown 模型/语法事件和图片布局契约。
2. Markdown WYSIWYG 编辑器及源码模式，保持 source offset 映射。
3. Diff 主题/字体、导航/复制/审阅增强。
4. Gallery/Docs、原生 smoke、截图和文档同步。

## 当前证据边界

阶段 1 与阶段 3 已有代码和回归测试；阶段 2 当前支持可编辑段落、inline marker 的 source/display offset 保持和源码切换，但跨块结构编辑、完整富文本选区及 composition/undo 分组仍未完成。

## 风险

- GPUI 自定义富文本输入的 source/display offset、IME marked range 和命中测试容易失步；先做单块、单光标、选区和撤销回归，再扩大到多块。
- 图片 intrinsic size 或异步宿主 resolver 可能造成测量抖动；每次资源策略变化必须发 `MarkdownChanged` 并重新测量。
- 代码主题的高亮只覆盖已有语言解析能力；未知语言必须安全回退纯文本。

## 完成标准

- 针对常规 Markdown/GFM、追加/编辑、脚注/任务列表/表格、图片比例和失败状态有测试。
- WYSIWYG 模式可以输入、删除、选择、撤销/重做、处理 marked composition，并切换源码模式且 source 不丢失。
- Diff 默认使用 Liora code font/weight 和 token，支持主题切换、hunk 上下导航、复制当前/全部 diff、接受/拒绝状态和 locate 回调。
- Gallery/Docs 有可见示例和 compile-checked snippets；原生 smoke 覆盖编辑、图片布局和 Diff 主题切换。
- `cargo fmt --all --check`、相关组件测试、定向 smoke、`cargo check --workspace --all-targets --locked`、CTX 校验和 `git diff --check` 通过；跨平台/真实 IME 限制明确记录。
