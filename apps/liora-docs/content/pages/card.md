# Card

`Card` 用于把标题、正文和底部操作聚合到一个边界清晰的内容容器中。

## 基础卡片

常规卡片适合承载简短信息；`hoverable` 会在鼠标悬停时增强阴影和边框反馈。

### 效果

::Demo{component="CardBasic"}::

### 代码

```rust src="card/basic.rs"
```

## 底部操作

通过 `footer` 可以把按钮、链接或其他原生节点放到卡片底部，形成常见的确认/取消操作区。

### 效果

::Demo{component="CardFooter"}::

### 代码

```rust src="card/footer.rs"
```
