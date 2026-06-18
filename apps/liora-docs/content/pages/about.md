# About Liora

Liora UI 是一个探索 Rust 原生桌面 UI 的组件库项目。它的目标不是复刻网页组件库，而是在 GPUI 的原生窗口模型上沉淀一套适合企业级工具、后台系统、数据密集应用和开发者工具的 UI 基础设施。

## 项目愿景

Liora 希望证明：复杂文档、组件展示、表单、弹层、消息反馈、代码阅读和图片预览都可以在纯 Rust 原生应用里完成，而不必退回 WebView。

## 当前重点

- 建立稳定的组件 API。
- 完善 Docs，每个效果都配套可检查代码。
- 优化交互性能，例如代码块选择、图片远程加载、Preview 关闭行为。
- 补齐动画基础设施，使 Button、Switch、Loading、Popover、Dialog、Drawer 等组件拥有一致动效。
- 保持 Gallery 和 Docs 双主程序互相验证。

## 贡献文档的规则

新增或修改组件文档时，请遵守：

1. Markdown 页面放在 `apps/liora-docs/content/pages`。
2. Rust 代码片段放在 `apps/liora-docs/content/snippets/<component>`。
3. 每个效果章节必须紧跟对应代码。
4. `.rs` snippet 应包含必要 imports 和注释，并通过 `check_snippets` 编译检查。
5. Live Demo key 需要在 Docs renderer 中映射到真实组件。

## 非目标

Liora 不计划提供 HTML 输出、不计划运行在浏览器里，也不把 CSS 作为样式系统。样式、布局和交互都应通过 Rust API、GPUI 元素和 Liora theme token 表达。
