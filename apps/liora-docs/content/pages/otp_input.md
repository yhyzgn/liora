# OtpInput

`OtpInput` 用固定长度格子展示一次性验证码、PIN 或设备配对码。第一阶段保持受控展示模型，父级应用负责真实输入和状态同步，组件负责视觉、状态和布局。

## 验证码格子

### 效果

::LioraDemo{component="OtpInputBasic"}::

### 代码

```rust src="otp_input/basic.rs"
```
