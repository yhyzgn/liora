# 004 GPUI Native Replacement Execution — 2026-10-13

- 状态: 已完成
- 计划: `.context/plans/004_gpui_native_replacement_20261013.md`

## 已应用

- `crates/liora-components/src/switch.rs`：以 GPUI `MotionSpring::Snappy.animation_to(...).with_spring(...)` 替换手写 `thumb_from_checked` + `slide_snap`；稳定 thumb ID 保留原生位置/速度。
- 保留 `motion.rs` 的旧弹性函数和 duration API，作为兼容公共 API；当前无其他生产调用依赖旧 `slide_snap`。

## 明确未替换

- Progress：目标值、圆形 canvas、渐变和范围约束需要组件级策略，不能直接把 overshoot spring 塞进百分比/几何。
- Scrollbar/VirtualizedList：GPUI 提供滚动物理和列表测量，但 Liora 自定义 thumb、拖拽、主题和业务 API 仍是上层职责。
- Tooltip/long press：GPUI 内建 tooltip/gesture 不会自动赋予 Liora portal tooltip 同等语义。
- Timer、SVG asset fallback、复杂 drag：没有行为等价的一对一替换。

## 验证

- `cargo test -p liora-components switch_thumb_uses_native_retargeted_spring_motion --locked`：PASS。
- `cargo check --workspace --all-targets --locked`：PASS。
- `cargo test --workspace --locked`：PASS，44 suites、668 passed、0 failed、1 既有 ignored。
- `cargo check -p liora-docs --bin check_snippets --locked`：PASS。
- `cargo fmt --all`、CTX validate、`git diff --check`：PASS。
- `timeout 30s cargo run -p liora-gallery --example gpui_upgrade_smoke --locked`：PASS，退出码 0。
- 严格 `cargo clippy --workspace --all-targets --locked -- -D warnings`：仍 FAIL 于 003 已记录的未改动 optimizer/packager/tray 基线问题；Switch 本次未新增 Clippy 错误。
- 严格 Clippy 和真实跨平台 GUI/GPU 验证沿用 003 的已知缺口；本次没有新增告警抑制或平台声明。
