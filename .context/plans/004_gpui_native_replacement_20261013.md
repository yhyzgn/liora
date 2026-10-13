# 004 GPUI Native Replacement — 2026-10-13

- 状态: 已完成（安全替换已应用；其余候选保留原实现）
- 负责人: Codex
- 当前任务: `.context/tasks/004_gpui_native_replacement_20261013.md`

## 目标

审计 003 GPUI refresh 后 Liora 的手写低级效果，使用官方 GPUI 原生能力替换行为等价且可验证的重复实现。

## 决策

- `Switch` 的 thumb 位移动画可安全替换：旧 `thumb_from_checked`、`slide_snap` 只服务于一次状态切换，GPUI spring 能直接表达同一目标并保留 retarget 速度。
- 进度条、滚动条、tooltip、timer、SVG 资源和复杂拖拽不做强行替换：它们包含 Liora 专属视觉/业务语义，或 GPUI 新 API 只覆盖底层机制而非完整组件契约。
- 保留旧 motion helper 公共 API，避免无必要的破坏性变更。

## 完成标准

- Switch 使用稳定 ID 的 `with_spring`，删除仅用于旧插值的状态字段。
- 回归测试锁定原有切换结构和新 spring 接入。
- workspace check/test、fmt、CTX、diff whitespace 通过。
- 文档记录替换范围和明确未替换候选。
