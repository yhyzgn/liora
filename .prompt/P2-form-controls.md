# P2 Form Controls — 表单数据录入

> 上游: `.prompt/P1-basic-elements.md`

## 目标

完成 10 个表单核心组件的开发。

## 组件清单

1. **Input** — 文本输入框 (prefix/suffix icon, clearable, password toggle, maxlength)
2. **InputNumber** — 数字输入 (步进按钮 ±, min/max/precision)
3. **Textarea** — 多行文本 (auto-resize, maxlength 计数)
4. **Checkbox** / **CheckboxGroup** — 多选 (indeterminate 半选, min/max 限制)
5. **Radio** / **RadioGroup** — 单选 (button 样式, border)
6. **Switch** — 开关 (active/inactive 文字, loading)
7. **Select** — 下拉选择 ⚠️ (需要 Popper 定位基础)
8. **Slider** — 滑块 (范围选择, 刻度, input 联动)
9. **Form** / **FormItem** — 表单容器 (label-width, required, 校验, error message)
10. **Rate** — 评分 (半星, 文字辅助, 只读)

## 关键挑战

**Select 组件**: 这是第一个需要使用 Popper/Portal 弹出定位的组件。如果 Popper 基建尚未完成，Select 降级为使用 `div().absolute()` 的相对定位方案。

## 依赖关系

```
Input → (无依赖, 优先开发)
Textarea → (无依赖)
InputNumber → Input (共享基础样式)
Switch → (无依赖)
Checkbox/Radio → (无依赖)
Slider → (无依赖)
Rate → (无依赖)
Form/FormItem → (无依赖, 状态管理 Model)
Select → Popper基建 or 简化方案
```

## 推荐开发顺序

Input → Textarea → Switch → Checkbox → Radio → InputNumber → Slider → Rate → Form → Select
