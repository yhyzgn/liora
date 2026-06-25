# OtpInput

`OtpInput` 用固定长度格子承载一次性验证码、PIN 或设备配对码。它不是静态展示：点击格子会定位光标，键盘输入、退格、粘贴和选择行为复用 Liora `Input` 编辑管线。

## 可输入验证码

### 效果

::LioraDemo{component="OtpInputInteractive"}::

### 代码

```rust src="otp_input/interactive.rs"
```

## Masked PIN

### 效果

::LioraDemo{component="OtpInputMasked"}::

### 代码

```rust src="otp_input/masked.rs"
```

## 状态和尺寸

### 效果

::LioraDemo{component="OtpInputStates"}::

### 代码

```rust src="otp_input/states.rs"
```
