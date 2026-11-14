# CodeEditor

`CodeEditor` 是 Liora 的原生代码编辑控件 v2 基座，面向配置面板、低代码脚本、小型规则编辑器和企业应用中的内嵌代码编辑场景。它保持纯 Rust + GPUI native：编辑状态由 `CodeBuffer` / selection / viewport 分层管理，可见行通过 GPUI 官方 `list` 虚拟渲染；语法高亮采用 Zed 风格路线，由 tree-sitter 解析 buffer、Liora query 产出兼容 Zed 的 capture name、`CodeEditorSyntaxTheme` 再把 capture 映射为 GPUI text run。诊断信息通过数据结构或 provider 注入，不绑定 WebView、Monaco 或浏览器运行时。`CodeBlock` 与 `CodeEditor` 是两个独立控件，后续可以共享高亮引擎，但不会把展示型 CodeBlock 强行耦合成编辑器。

## 基础编辑器

### 效果

::Demo{component="CodeEditorBasic"}::

### 代码

```rust src="code_editor/basic.rs"
```


## 默认语言矩阵

### 效果

::Demo{component="CodeEditorLanguages"}::

### 代码

```rust src="code_editor/languages.rs"
```

## Diagnostics 扩展点

### 效果

::Demo{component="CodeEditorDiagnostics"}::

### 代码

```rust src="code_editor/diagnostics.rs"
```

## 高级扩展点：搜索、补全和 hover

### 效果

::Demo{component="CodeEditorAdvanced"}::

### 代码

```rust src="code_editor/advanced.rs"
```
补全不是展示列表：聚焦编辑器后用 `↑/↓` 选择，`Tab/Enter` 或鼠标按下候选接受，`Escape` 取消。旧 `CodeCompletionItem` 与 provider 契约保持不变：label 替换选区，未选择时替换光标所在的完整代码词（包括光标右侧的词后缀）。需要 display label 与插入文本不同或明确替换范围时，用 `CodeCompletionEdit::new(item, utf8_range, new_text)`。

异步宿主先捕获 `request_token()`，再用 `set_completions_for_request` 或 `set_completion_edits_for_request` 送回结果。版本或光标/选区不一致时返回 `false`；显式范围倒置、越界或落在 UTF-8 字符内部时整批拒绝。只读不能接受补全，IME composition 期间不能装入或接受候选。SDK 不启动 LSP 进程或网络。

`set_search_query` 更新区分大小写的字面查询；`search_matches` 返回 UTF-8 非重叠命中并缓存到文本版本。所有可见命中高亮，`find_next` / `find_previous`（`F3` / `Shift+F3`）循环定位、选中并展开所在折叠。`replace_current_match` 只替换当前精确命中并定位下一处；`replace_all_matches` 对已有命中逆序编辑，不重新匹配插入文本，整批可一步撤销。空查询不产生匹配；只读或 IME composition 期间不替换。


## 高级布局：行高自定义

### 效果

::Demo{component="CodeEditorLineHeight"}::

### 代码

```rust src="code_editor/line_height.rs"
```

## 高级布局：缩进指示线

### 效果

::Demo{component="CodeEditorIndentGuides"}::

### 代码

```rust src="code_editor/indent_guides.rs"
```

## 高级布局：自动代码块折叠

### 效果

::Demo{component="CodeEditorFolding"}::

### 代码

```rust src="code_editor/folding.rs"
```

## 配置矩阵：只读、Chrome 和扩展面板

### 效果

::Demo{component="CodeEditorConfiguration"}::

### 代码

```rust src="code_editor/configuration.rs"
```

## 配置文件驱动：TOML/JSON 与显式应用

### 效果

::Demo{component="CodeEditorConfigFile"}::

### 代码

```rust src="code_editor/config_file.rs"
```

`CodeEditorConfig::load_from_path(...)` 用于启动时加载一次外部 TOML/JSON 配置；运行时修改配置时，推荐由业务触发动作显式重新加载，例如用户保存配置、点击“应用/重新加载”，或应用重启后重新读取文件。SDK 不在后台监视文件，避免把配置生命周期和业务状态耦合到组件内部。解析失败时业务侧可以提示错误，并保留当前编辑器配置不变。`theme_file` 会按配置文件所在目录解析相对路径。

## 自定义高亮主题和高级编辑行为

### 效果

::Demo{component="CodeEditorTheme"}::

### 代码

```rust src="code_editor/theme.rs"
```

## 架构说明

- v2 不再把 `Input` 当作编辑内核，而是由独立 `CodeBuffer` 管理文本、行索引、offset 与 point 映射。
- selection、viewport 与行级 shaped layout 独立于渲染层；`CodeDisplayMap` 统一维护行高、gutter 和回退定位，真实可见行通过 GPUI `shape_line` 产出的 glyph layout 负责 pointer-to-offset 映射，后续可继续扩展软换行、折叠与 inlay hints。
- 可见行通过 GPUI 官方 `list` 渲染，配合 Liora `VirtualScrollbar`，避免大文件时把所有行一次性塞进普通元素树。每一行直接渲染 tree-sitter syntax-highlighted text，编辑区本身就是实时效果区，不再额外展示割裂的预览面板。
- 设计参考 Zed 官方源码的分层思想和 SyntaxTheme/capture 命名模型，但不复制或依赖 Zed GPL editor/language/multi_buffer/text/rope crate 代码；Liora 仅维护自己的精简 query 文件和公开 API。

## 能力边界

- 支持行号、语言、主题、缩进配置、编辑回调，`Tab` / `Shift+Tab` 可按当前缩进配置缩进或反缩进。
- 支持基础导航、键盘选择、鼠标点击定位、双击选词、三击选行、列级光标、拖拽选择、选区左右键折叠、词级跳转/选择/删除、智能 Home、PageUp/PageDown、文档首尾跳转/选择、复制、粘贴、剪切、回车换行、缩进/反缩进、`Ctrl/Cmd+Z` undo 与 `Ctrl/Cmd+Shift+Z` redo。
- 支持 `CodeEditorOptions` 统一配置：`read_only`、`header`、`status_bar`、`line_numbers`、`diagnostics_panel`、`completions_panel`、`hover_panel`、`current_line_highlight`、`indent_guides`、`code_folding`、`rulers`、`ruler_column`、`inline_diagnostics`、`whitespace`、`selection`、`copy`、`clipboard_editing`、`cursor_blink`、`drag_selection`、`word_selection`、`line_selection`、`indentation`、`history`、`reveal_cursor`、`scrollbar`、`completion_limit` 与 `diagnostics_limit`；也可用 builder 快速配置常用单项。
- 支持 `CodeEditorHighlightTheme` 与 `CodeEditorSyntaxTheme` 自定义编辑器主题：可覆盖 base `CodeTheme`、surface、chrome/gutter、border、text/muted text、caret、selection、current line、ruler、whitespace、diagnostic semantic colors，也可用 Zed-style syntax capture 配置每类 token 的颜色、字重和 italic。
- 支持搜索命中高亮、循环定位、当前/全部替换、可键盘/鼠标接受的 completion candidates，以及 hover/help provider；异步结果通过版本与光标 token 防止过期编辑。
- 支持 `CodeDiagnostic` 静态注入，也支持 `diagnostics_provider` 根据最新文本动态生成诊断结果，用于展示语法检查、lint、业务规则检查等结果。
- 语法高亮已经进入编辑行内部，输入变化会驱动同一区域重新渲染；光标、选区和 IME marked text 下划线都在行级 GPUI shaped-line paint 阶段绘制，不参与文本 flex 布局，避免把后续文本挤开；点击定位、IME bounds 和选区矩形使用同一个 glyph layout 反查 offset/坐标并保持 UTF-8 边界安全。
- 快捷键按 GPUI/Zed 常见习惯注册：Linux/Windows 使用 `Ctrl+←/→` 做词级移动，macOS 可使用 `Alt+←/→`；`Ctrl/Alt+Backspace` 与 `Ctrl/Alt+Delete` 做词级删除；`Shift` 组合扩展选区；`Home` 在缩进首字符和行首之间切换；`Ctrl+Home/End` 与 `Cmd+↑/↓` 跳到文档首尾。
- 渲染捕获共享不可变行快照，不深复制全文；单次编辑只重建受影响行文本并复用未变行的 `Arc<str>`。多处缩进、全部替换及其 undo/redo 按事务应用 delta，只在批次结束时完整重建一次行索引与行文本，不为每个子编辑重复扫描。历史保存插入/删除 delta 与前后选区，不保存每次按键前后全文。底层连续 `String` 编辑仍会移动后缀，行索引更新仍为 O(行数)。
- 高亮按语言和文本版本缓存，光标/闪烁 render 复用 capture runs。文本编辑后可靠重新解析；当前不宣称增量 tree-sitter，后续可保留 parser/tree。事务分组合并、rope 后端、软换行与横向滚动仍不在本次范围。


## 配置文件和 Zed JSON 主题

CodeEditor 支持用 `CodeEditorConfig` 从 TOML/JSON 文件加载配置。基础编辑体验配置覆盖语言、主题、字体族、字号、字重、显式高度、行高、缩进、header/status/panel 开关以及 gutter/content/row/header/panel 间距。编辑器真实内容行数来自 buffer；配置文件只用 `height_px` 表达显式视口高度；不提供 `rows` 配置，真实内容行数始终来自 buffer，父级布局仍可决定最终尺寸。`theme_file` 指向 Zed JSON theme family，`theme_name` 指定其中一个主题；`appearance.syntax.*` 可以继续覆盖单个 capture。配置文件既可以通过 `.config(config)` 启动时应用，也可以在业务触发 reload 时通过 `.set_config(config, cx)` 显式应用。

```toml
language = "rust"
theme_file = "themes/one.json"
theme_name = "One Dark"
font_family = "Monospace"
font_size_px = 16
font_weight = 500
line_height_px = 26
height_px = 420
tab_size = 4
soft_tabs = true

[options]
line_numbers = true
rulers = true
ruler_column = 100
whitespace = "boundary"

[layout]
gutter_width_px = 72
content_padding_x_px = 18
row_padding_y_px = 2
viewport_padding_y_px = 18
header_padding_x_px = 18
header_padding_y_px = 10
header_gap_px = 16
panel_padding_x_px = 18
panel_padding_y_px = 10
panel_gap_px = 6

[appearance.syntax.keyword]
color = "#ff79c6ff"
font_style = "italic"
```

```rust
use liora_components::{CodeEditor, CodeEditorConfig};

let config = CodeEditorConfig::load_from_path("assets/editor.toml")?;
let editor = CodeEditor::new(source, cx).config(config);
```

当前内置 tree-sitter grammar 覆盖 Rust、TypeScript/JavaScript、SQL、Shell、XML、TOML、YAML、INI/conf、JSON 和 Markdown；不认识的语言会安全回退为纯文本。
