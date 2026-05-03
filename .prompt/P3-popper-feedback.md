# P3 Popper + Feedback — 弹出层与反馈

> 上游: `.prompt/P2-form-controls.md`

## 目标

攻克原生 GUI 最大的工程难题：弹出层基建 (Popper/Portal) + 全部反馈组件。

## 弹出层基建 (先于任何 Popup 组件)

| 模块 | 说明 |
|------|------|
| **AnchorPosition** | 锚点定位引擎 (top/bottom/left/right + 12 种偏移对齐) |
| **Portal** | 渲染元素到窗口根节点 (脱离布局流) |
| **ViewportBoundary** | 边缘溢出检测 + 自动翻转方向 |
| **ZIndexStack** | 全局 Z-Index 栈 (popup=+100, modal=+200, notification=+300, tooltip=+400) |
| **ClickOutside** | 点击外部检测关闭 |
| **FocusTrap** | Tab 键焦点锁定 (弹窗内循环) |

## 组件清单 (13)

1. **Tooltip** — 文字提示 ⚠️ Popper
2. **Popover** — 气泡卡片 ⚠️ Popper
3. **Popconfirm** — 气泡确认 ⚠️ Popper
4. **Dialog** — 模态对话框 (遮罩、FocusTrap、ESC 关闭)
5. **Drawer** — 抽屉面板 (左/右/上/下)
6. **Message** — 全局消息提示 (顶部居中)
7. **Notification** — 通知 (右上角弹出)
8. **Alert** — 警示提示 (4 种主题)
9. **Loading** — 加载状态 (全屏/局部指令)
10. **MessageBox** — 消息弹窗 (confirm/prompt)
11. **Dropdown** — 下拉菜单 ⚠️ Popper
12. **Card** — 卡片 (header/body/footer)
13. **Collapse** — 折叠面板 (手风琴模式)

## 推荐开发顺序

Popper基建(Anchor+Portal+ZIndex) → Tooltip → Popover → Dialog → Drawer → Message → Notification → Alert → Loading → Card → Collapse → Dropdown → Popconfirm → MessageBox
