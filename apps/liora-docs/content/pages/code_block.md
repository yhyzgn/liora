# CodeBlock

`CodeBlock` 是 Liora 的原生代码显示控件，用于展示代码片段、语言标签和复制按钮。

## 能力

- 块级代码显示。
- 行内代码显示。
- 语言标识：Rust、TOML、JSON、Markdown、Shell、TypeScript、JavaScript。
- `syntect` + `two-face` 语法高亮与扩展语法/主题资源。
- 高亮后端抽象：当前默认 `CodeHighlighter::Syntect`，后续可接 Tree-sitter。
- 主题切换：默认跟随 Liora 全局主题，也支持显式 Liora / GitHub / One Dark / Nord / Dracula。
- 鼠标拖拽选中代码并复制：支持 `cmd/ctrl-a` 与 `cmd/ctrl-c`。
- 复制按钮：使用 GPUI clipboard API。
- 高亮结果缓存：避免菜单切换和右侧面板滚动时重复解析高亮。
- 横向滚动：长代码不会撑破布局。

## 基础用法

### 效果

::Demo{component="CodeBlockBasic"}::

### 代码

```rust src="code_block/basic.rs"
```

## 指定语言

### 效果

::Demo{component="CodeBlockLanguage"}::

### 代码

```rust src="code_block/language.rs"
```

## 主题切换

### 效果

::Demo{component="CodeBlockTheme"}::

### 代码

```rust src="code_block/theme.rs"
```

## 行内格式

### 效果

::Demo{component="CodeBlockInline"}::

### 代码

```rust src="code_block/inline.rs"
```

## 设计说明

CodeBlock 使用 Rust 原生 `syntect` 解析 Sublime 语法定义和主题，并通过 `two-face` 引入 bat 生态的扩展语法与主题集合，再转换为 GPUI `StyledText` / `TextRun`。高亮能力更完整，但渲染结果仍然是原生 Liora/GPUI 节点。

`CodeHighlighter` 先保留后端边界，当前仅启用 `Syntect`；如果后续需要代码编辑器、AST 交互或更强语义分析，可在不改调用侧主题 API 的前提下新增 Tree-sitter 后端。
