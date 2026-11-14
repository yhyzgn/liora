# Transcript

`Transcript` 是按稳定 ID 渲染的原生可变高度会话视口。它使用官方 GPUI `ListState` 的 tail-follow、局部重测量和滚动锚点；只调用可见/overdraw 行的渲染闭包，不存储消息业务或运行 Agent。

## 用法

### 效果

::Demo{component="Transcript"}::

### 代码

```rust src="transcript/basic.rs"
```

- 将 `Entity<Transcript>` 保存在宿主视图，不在每帧重新创建。
- `set_items` 接受有序、唯一的 ID；追加/历史前插只使变化区间失效。重复 ID 返回错误，原视口不变。
- 行内容变高时调用 `notify_item_changed(id)`，然后 `cx.notify()`。更新内容的业务实体、Markdown 更新事件和布局失效由宿主连接，SDK 不依赖 Gallery/Docs。
- 初始跟随末尾。滚轮向上或滚动条定位离开底部后停止跟随，回到底部自动恢复；`follow_tail` 显式跳到最新。需要暂时冻结时使用 `pause_following`。
- `position` 返回可序列化的 `TranscriptPosition`。恢复时先装载该会话 ID，再调用 `restore_position`；缺失的锚点、负数和非有限偏移返回错误，宿主决定如何处理已被删除的历史。
- GPUI 视口需要外部有界高度。行中有状态组件同样由宿主持有；行 ID 保持稳定，不能使用每帧随机 ID。

## 边界

控件不拥有会话数据库、分页请求、文本选择模型、Markdown 解析器或执行进程。行中的 `SelectableText`、`MarkdownView` 等原生组件保留自己的选择状态；跨行选择属于宿主内容层，不由虚拟列表伪造。`list_state` 用于原生滚动观察和尺寸查询。
