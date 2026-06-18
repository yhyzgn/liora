# Timer

`Timer` 用于展示正向计时、倒计时和单位换算结果。默认仍可作为受控展示组件使用；调用 `.start()` / `.running(true)` 后会自动刷新并动态计时，同时保留 `snapshot`、`elapsed_as`、`remaining_as` 等结果读取 API。

## 正向计时

### 效果

::LioraDemo{component="TimerCountUp"}::

### 代码

```rust src="timer/count_up.rs"
```

## 倒计时

### 效果

::LioraDemo{component="TimerCountDown"}::

### 代码

```rust src="timer/count_down.rs"
```

## 单位与紧凑模式

### 效果

::LioraDemo{component="TimerUnits"}::

### 代码

```rust src="timer/units.rs"
```

## 读取计时结果

### 代码

```rust src="timer/result.rs"
```

## 时钟格式

### 效果

::LioraDemo{component="TimerClock"}::

### 代码

```rust src="timer/clock.rs"
```
