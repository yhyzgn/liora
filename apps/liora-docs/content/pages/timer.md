# Timer

`Timer` 用于展示正向计时、倒计时和单位换算结果。默认仍可作为受控展示组件使用；调用 `.start()` / `.running(true)` 后会自动刷新并动态计时，同时保留 `snapshot`、`elapsed_as`、`remaining_as` 等结果读取 API。

正向计时和倒计时示例提供 Start / Pause / Resume / Reset：Pause 由宿主读取 `snapshot().elapsed` 并重建静止显示；Resume 以保存的已用时创建新 Timer，保留倒计时总时长；Reset 清零已用时并停止。已结束的倒计时只能 Reset 后再次 Start。

宿主必须保存返回的 `AnyView`，在渲染时克隆该视图，而不是每次创建。每次运行保留同一个 Timer 并克隆渲染；恢复时使用新 Timer 的唯一身份，避免复用全局起始时间。仅调用 `.paused()` 不会自动保存运行时长。

运行中的计时器可用 `.tick_interval(Duration::from_millis(50))` 设置刷新调度间隔；默认值为 250ms，最小值为 16ms。同一窗口共享最早的待刷新时刻，因此较快的计时器也可能让较慢的计时器提前刷新。仍在渲染的运行中计时器会安排下一次刷新；暂停、结束或移除的计时器不再续约，已关闭窗口的注册在已有待刷新任务执行后回收。同一窗口内其他运行中的计时器仍可继续调度刷新。

超出平台单调时钟可表示范围的间隔不会安排新刷新，也不会取消已有的其他计时器刷新。保留的 Timer 首次渲染沿用原始起点，不会把已经完成的倒计时重新启动。稳定 ID 的起点仍保留至应用退出；本次仅约束待刷新任务，不代表起点存储已回收。

## 正向计时

### 效果

::Demo{component="TimerCountUp"}::

### 代码

```rust src="timer/count_up.rs"
```

## 倒计时

### 效果

::Demo{component="TimerCountDown"}::

### 代码

```rust src="timer/count_down.rs"
```

### 宿主状态与按钮（两个示例共用）

以下是上述示例实际使用的原生 GPUI 宿主；按钮回调通过 `cx.listener` 弱引用宿主，页面关闭后不保留宿主。

```rust src="timer/controls.rs"
```

## 单位与紧凑模式

### 效果

::Demo{component="TimerUnits"}::

### 代码

```rust src="timer/units.rs"
```

## 读取计时结果

### 效果

::Demo{component="TimerResult"}::

### 代码

```rust src="timer/result.rs"
```

## 时钟格式

### 效果

::Demo{component="TimerClock"}::

### 代码

```rust src="timer/clock.rs"
```
