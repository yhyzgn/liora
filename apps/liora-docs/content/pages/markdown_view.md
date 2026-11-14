# MarkdownView

`MarkdownDocument` / `MarkdownView` 是通用原生 Markdown SDK；Docs 应用私有的文档 renderer 另见 Markdown Renderer。两者均不执行 HTML、不引入 WebView。

## 动态文档

把 `Entity<MarkdownView>` 保存在宿主视图中，通过 `cx.new(|cx| MarkdownView::new(source, cx))` 创建，使用 `set_source` / `append` 更新。`MarkdownDocument` 也可以独立保存、批量更新后调用 `flush()`，再通过 `MarkdownView::from_document` 交给原生视图。

### 效果

::Demo{component="MarkdownView"}::

### 代码

```rust src="markdown/streaming.rs"
```

SDK 支持段落、标题、强调/粗体/删除线、行内和围栏代码、嵌套列表、引用、表格（含左/中/右列对齐）、链接与图片。正文使用可选择的原生文本；代码立即呈现等宽可选文本与复制按钮，不使用异步高亮 skeleton，避免未通知宿主的二次高度增长。未闭合强调、链接和围栏保留可见内容，不等待完整文档才显示。HTML 作为文字显示，绝不执行；SDK 不解释 `src=` snippet、LioraDemo 等 Docs 业务语法。

## 连续更新与稳定身份

追加只更新源字符串并通知 GPUI，下次 render 才解析；同一帧的多次追加合并为一次全文解析。无更新的 render 复用解析缓存。解析必须覆盖全文，因为后加的引用定义可能改变前文链接。`parse_count()` 可观察实际解析次数。

同一父块下、同一起始字节偏移、同类块和同一兄弟位置复用 ID，包括增长中的段落/代码块；合成的空表格单元格也按列区分身份。插入导致偏移或兄弟位置移动、块类型变化时会产生新 ID。不要把块 ID 当作跨任意编辑的持久文档键。解析时最多保留 64 层块嵌套，更深部分展平为可见文字，限制渲染、遍历及销毁的递归深度。

## 选择与链接

富布局模式的拖选、Ctrl/Cmd+A/C 在当前段落或代码块内生效；不会声称跨越表格、图片的异构元素能自动组成同一选区。点击 **Select across blocks**，或调用 `set_continuous_selection(true, cx)`，切换为整个文档的连续原生纯文本范围，可跨块拖选和复制；再次切换返回富布局。连续模式不展示图片或可点击链接，不修改源文档。

安全链接保留行内样式，另以原生 **Open URL** 按钮提供明确的键盘/指针操作，避免破坏段落拖选。只允许显式 http、https 和 mailto；file、data、javascript、相对地址、控制字符、空白、反斜线及带凭据的 web 地址不进入平台 opener。

`cargo run -p liora-gallery --example selection_native_smoke --locked` 需要真实显示后端，断言跨块 Select All/Copy 的实际系统剪贴板内容，以及中/右对齐文本的鼠标选择。它保存并恢复 GPUI 能表示的剪贴板格式；存在其他格式时应在一次性剪贴板或隔离显示会话运行。headless 的键盘派发不能替代这项验证。

## 图片与宿主权限

默认拒绝所有图片加载，仍显示 alt 文本和阻止提示。宿主可通过 `set_image_resolver` 返回 `MarkdownImage::File(PathBuf)` 批准本地文件，或返回 `MarkdownImage::Render(Arc<gpui::RenderImage>)` 提供已经解码的图像；返回 None 拒绝。SDK 不接受 URL 作为图片源、不执行网络请求，避免图片加载器自动重定向绕过宿主 allowlist。远程图片仍受支持：宿主网络层负责每跳重定向权限、下载/解码大小限制，再把解码结果传给 SDK。不要原样放行模型提供的文件路径。授权结果按 destination 缓存，但每个图片 occurrence 分别渲染、保留自己的 alt（包括空 alt）；`clear_image_resolver` 撤销授权。SDK 不管理下载权限或文件权限。

## 高度变化与宿主组合

订阅 `MarkdownChanged` 可得知一批内容解析完成、选择模式或图片策略改变。上方 snippet 的宿主实体保留 `Subscription`，在延迟 effect 中调用 Transcript 的 `notify_item_changed` 并通知重绘；丢弃订阅会停止通知。SDK 不暗中耦合任何会话列表。代码从第一帧就占最终文本高度；图片区域保留固定高度，加载完成不增加文档高度。

初始化沿用 `init_liora(cx)` 的 SelectableText、Button 键绑定，不需要额外全局服务。
