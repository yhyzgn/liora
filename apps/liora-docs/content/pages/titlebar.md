# TitleBar

`TitleBar` 是 Liora 的原生自定义标题栏组件。它负责标题、副标题、左侧/中间/右侧插槽、拖动区域、双击标题栏行为和窗口控制按钮。真实应用通常把它交给 `AppWindowFrame::titlebar(...)` 使用；嵌入文档或组件卡片时可以关闭 `window_controls(false)`。

## 独立标题栏

### 效果

:::LioraDemo{component="TitleBarBasic"}::

### 代码

```rust src="titlebar/basic.rs"
```

## 使用建议

- 真实窗口中搭配 `AppWindowFrame::new(...).mode(WindowFrameMode::Custom).titlebar(...)`。
- 常用样式可通过 `.height(...)`、`.padding_x(...)`、`.gap(...)`、`.actions_gap(...)`、`.background(...)`、`.border_color(...)`、`.border(...)`、`.title_color(...)`、`.subtitle_color(...)`、`.content_align(...)` 调整。
- 窗口控制位置可通过 `.window_controls_position(WindowControlsPosition::Left | Right)` 控制。
- 嵌入 demo、设置面板或普通卡片时使用 `.window_controls(false)`，避免误操作宿主窗口。
- 按钮等可点击控件应放在 `action(...)` 或 `actions(...)` 中，不要放进拖动区域。
