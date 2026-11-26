# Notification

悬浮在窗口角落的全局通知，用于展示较长的系统反馈、异步任务结果或需要保留几秒的信息。

## 通知类型

内置 `Info`、`Success`、`Warning`、`Error` 四种类型。标题适合表达结论，描述适合补充下一步行动。

## 生命周期与堆叠

- 每条通知默认累计显示 4 秒未悬停时间后自动消失，各自独立计时。
- 悬停在一条通知上会暂停该条通知，并保留剩余时间；移开后从剩余时间继续，不会重新计满 4 秒。
- 点击该条通知的 **Close notification** 按钮可手动关闭它；其他通知的显示、剩余时间和关闭行为不受影响。
- 点击 **Stack notifications** 一次显示 `Queue alpha`、`Queue beta`、`Queue gamma` 三条不同通知。悬停其中一条，观察其他通知独立消失；移开后观察该条通知继续计时。再次堆叠并单独关闭一条，确认其余通知不受影响。

Gallery 与本页的交互示例直接编译使用下方同一份 Rust 代码，不另写一份演示逻辑。

### 效果

::Demo{component="NotificationTypes"}::

### 代码

```rust src="notification/types.rs"
```
