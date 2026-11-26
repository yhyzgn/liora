# Message

`Message` 是 Liora 的全局提示层，适合用来展示操作反馈、成功提示、告警和错误。

调用这些宏前，宿主应用必须先调用 `liora::init_liora(cx)`，在根视图渲染时调用 `liora_components::message::render_messages(cx)`，并挂载 `liora_components::PortalLayer`。这些提示共享应用级 Message 管理器和提示层，不属于示例按钮自身的局部状态。

每条消息从加入时起独立计时，3 秒后无条件自动移除；新消息不会重置旧消息的到期时间。与 Notification 不同，Message 不会因悬停而暂停，也没有关闭按钮。

## 快捷宏

下面是四种最常用的 toast 宏，它们都会复用同一个全局 Message 层。

点击 **Stack messages** 会一次加入三条独立消息。也可以间隔点击其他按钮，观察每条消息按各自的 3 秒期限移除。

### 效果

::Demo{component="MessageTypes"}::

### 代码

```rust src="message/types.rs"
```

## 模板格式化

下面的两个按钮分别使用 `format!` 风格的位置参数和命名参数。

### 效果

::Demo{component="MessageFormatting"}::

### 代码

```rust src="message/formatting.rs"
```

## 交互演示

Gallery 和 Docs 都直接执行上述 `.rs` snippet 中的 `message_types()` 与 `message_formatting()` 工厂，展示同一套真实按钮和回调；页面展示的代码就是两处演示共用的可执行源码。这两个 snippet 也由现有 `check_snippets` 编译检查入口导入。
