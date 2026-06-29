# Steps

`Steps` 用于把任务拆成清晰的阶段，帮助用户理解当前进度和后续步骤。

## 基础用法

通过 `active` 标记当前步骤，之前的步骤会自动显示为完成状态。

### 效果

::Demo{component="StepsBasic"}::

### 代码

```rust src="steps/basic.rs"
```

## 描述与图标

每个 `StepItem` 可以追加说明文字和图标，让步骤语义更清晰。

### 效果

::Demo{component="StepsDescription"}::

### 代码

```rust src="steps/description.rs"
```

## 步骤状态

使用 `status` 可以显式设置完成、错误或等待等状态。

### 效果

::Demo{component="StepsStatus"}::

### 代码

```rust src="steps/status.rs"
```

## 垂直方向

切换到 `StepsDirection::Vertical` 可以展示纵向流程，适合较长说明内容。

### 效果

::Demo{component="StepsVertical"}::

### 代码

```rust src="steps/vertical.rs"
```
