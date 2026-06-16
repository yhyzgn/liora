# CodeEditor

`CodeEditor` 是 Aura 的原生代码编辑控件基础版，面向配置面板、低代码脚本、小型规则编辑器和文档示例编辑场景。它保持纯 Rust + GPUI native：编辑核心复用 Aura `Input`，高亮预览复用 `CodeBlock` 的 `syntect` / `two-face` 资源，诊断信息通过数据结构注入，不绑定 WebView、Monaco 或浏览器运行时。

## 基础编辑器

### 效果

:::AuraDemo{component="CodeEditorBasic"}::

### 代码

```rust src="code_editor/basic.rs"
```

## Diagnostics 扩展点

### 效果

:::AuraDemo{component="CodeEditorDiagnostics"}::

### 代码

```rust src="code_editor/diagnostics.rs"
```

## 能力边界

- 支持行号、语言、主题、缩进配置、编辑回调，`Tab` / `Shift+Tab` 可按当前缩进配置缩进或反缩进。
- 支持 `CodeDiagnostic` 静态注入，也支持 `diagnostics_provider` 根据最新文本动态生成诊断结果，用于展示语法检查、lint、业务规则检查等结果。
- 当前阶段语法高亮以预览区呈现；后续可继续把高亮 run 与编辑布局合并，形成完整编辑态高亮。
- 语法检查设计为外部 provider 注入结果，避免 P13 MVP 硬绑定 Rust Analyzer 或 LSP 进程。
