# Message

`Message` 是 Liora 的全局提示层，适合用来展示操作反馈、成功提示、告警和错误。

## 快捷宏

下面是四种最常用的 toast 宏，它们都会复用同一个全局 Message 层。

### 效果

::Demo{component="MessageTypes"}::

### 代码

```rust src="message/types.rs"
```

## 模板格式化

### 效果

::Demo{component="MessageFormatting"}::

### 代码

```rust src="message/formatting.rs"
```

## 交互演示

在 Gallery 里可以点击按钮直接触发这些提示。Docs 里则重点展示用法、配置和对应代码。
