# Switch

`Switch` 用于在两个互斥状态之间切换。它支持键盘操作、禁用态和弹性滑动动画。

保持 `Entity<Switch>` 跨帧存活，并用 `aria_label` 为开关提供与当前界面语言一致的功能名称，例如 `Switch::new(true, cx).aria_label("Auto save")`。名称独立于 `.id(...)`，不会改变布局，也不会自动继承外层设置行的文字；`aria_label` 可接受动态解析的 `LocalizedText`。

启用时可用 `Tab` / `Shift+Tab` 进入和离开，保留 `Space` / `Enter` 切换合同。一次有效指针点击只回调一次；在外部按下后移入释放不会切换。`set_disabled` 可跨帧禁用/恢复同一个实体；禁用时不参与 Tab 顺序、不响应指针或键盘。

现有原生根节点暴露 `Switch` role、checked/toggled、disabled 与名称，不额外添加伪语义节点。Linux AT 的实际状态映射取决于固定版本的 AccessKit adapter，不能把 SDK 节点属性等同于所有平台已验证。

## 基础状态

### 效果

::Demo{component="SwitchBasic"}::

### 代码

```rust src="switch/basic.rs"
```

## 禁用状态

### 效果

::Demo{component="SwitchDisabled"}::

### 代码

```rust src="switch/disabled.rs"
```

## 变化回调

### 效果

::Demo{component="SwitchCallback"}::

### 代码

```rust src="switch/callback.rs"
```
