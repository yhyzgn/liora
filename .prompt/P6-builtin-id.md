# P6 Built-in Unique ID — 控件内置唯一 ID 规范

> 上游: `.prompt/P5-advanced.md`

## 目标

确保全库每个控件都有默认的内置唯一 ID，事件冲突防护应由组件库自身保证，而非依赖使用者凭良心设置。

## 动机

根据 P0-P5 阶段积累的经验，GPUI 中多个控件实例共用相同的 Element ID 会导致：
- 交互事件冲突（点击无反应、hover 穿透、状态错乱）
- 多个实例只需一个能正常工作
- 开发者需手写 ID 前缀避免冲突，容易遗漏

**组件库的职责是开箱即用、零配置无冲突，ID 唯一性规范必须内建到每个控件中。**

## 要求

### 1. 每个控件必须有默认内置唯一 ID

```rust
// ✅ 正确 — Button 内部自动生成全局唯一 ID
Button::new("Save").primary()

// ❌ 禁止 — 依赖开发者手动设置唯一 ID
Button::new("Save").primary().id("my-unique-id")
```

### 2. 全局唯一，非仅局部唯一

- 不能仅基于调用位置 (`track_caller`) 生成 ID — 同一个 helper 函数中循环创建多个实例时 ID 相同
- 不能仅基于组件名称 — 同类型组件多实例冲突
- 必须结合运行时唯一标识（如 UUID、atomic counter、EntityId 等）

### 3. 实现策略（分层）

| 优先级 | 策略 | 适用场景 |
|--------|------|---------|
| A | 基于 `EntityId` 生成 | 有状态组件（View-based），每个 entity 有天然唯一 ID |
| B | 基于 `std::sync::atomic::AtomicU64` 全局递增计数器 | 无状态组件（RenderOnce），每次构造分配唯一序号 |
| C | 基于 UUID (`uuid::Uuid`) | 需要跨会话/跨窗口唯一性的场景 |
| D | 暴露 `.id(impl Into<SharedString>)` 作为可覆盖项 | 用户需要显式指定 ID 时 |

### 4. 组件内交互子元素 ID 前缀

每个组件内部的交互子元素（按钮、图标、输入框等）必须以组件唯一 ID 为前缀：

```rust
let component_id = format!("button-{}", self.uid);
// 内部子元素:
//   "{component_id}-icon-start"
//   "{component_id}-icon-end"
//   "{component_id}-text"
```

### 5. 全局计数器基础设施

在 `liora-core` 中新增全局 ID 生成器：

```rust
// crates/liora-core/src/lib.rs 或新建 crates/liora-core/src/unique_id.rs
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// 生成全局唯一递增序号
pub fn next_unique_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// 生成带前缀的唯一 ID 字符串
pub fn unique_id(prefix: &str) -> SharedString {
    format!("{}-{}", prefix, next_unique_id()).into()
}
```

## 检查清单

对 `crates/liora-components/src/` 下每一个组件文件，逐项检查：

- [ ] 是否有默认内置唯一 ID？
- [ ] 内部交互子元素 ID 是否以组件 ID 为前缀？
- [ ] 多实例共存时是否存在 ID 冲突？
- [ ] `#[track_caller]` 默认 ID 是否在循环/helper 函数中能保持唯一？
- [ ] 是否提供 `.id(custom_id)` 覆盖入口（非强制，但建议）？

## 涉及文件范围

- `crates/liora-core/src/` — 新增全局 ID 生成器
- `crates/liora-components/src/*.rs` — 所有现有组件逐一改造
- `crates/liora-components/src/lib.rs` — 确保 unique_id 模块可访问

## 验证标准

1. `cargo check` 0 errors, 0 warnings
2. Gallery 中同类型多实例控件交互互不干扰
3. 同一个 demo helper 函数中循环创建的控件每个都正常工作
