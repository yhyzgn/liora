# Live Demo

Live Demo 是 Liora Docs 区别于静态 Markdown 文档的核心能力。

当 renderer 识别到特殊语法时，不渲染为普通文字，而是创建真实 Liora 组件节点。

```text src="live_demo/button.rs"
```

下面的按钮不是截图或文本，而是真实的 Liora `Button` 节点：

::LioraDemo{component="Button"}::

## 为什么这样设计

- 文档示例和组件实现不会分叉。
- Hover、click、focus 等交互保留真实行为。
- 文档本身成为组件库的集成测试面。

## 后续扩展方向

- 支持更多组件：`CodeBlock`、`Input`、`Switch`、`Table`、`Message`。
- 支持 demo 参数：variant、size、disabled、loading。
- 支持 demo 容器：示例区、源码区、说明区。
