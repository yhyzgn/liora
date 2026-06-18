# P20 — Theme and Interaction Polish

> 上游: `.prompt/P15-quality-hardening.md` / `.prompt/P19-dashboard-state-and-data-flow.md`
> 状态: Complete
> 目标: 收口 Liora 的 System/Light/Dark 主题模式、语义 token、浮层遮罩和关键交互状态一致性。

## Scope

P20 不新增大型业务组件，聚焦主题与交互一致性：

- System / Light / Dark 三模式作为正式主题入口。
- 深色模式下 subtle semantic token 不再用白色混合，而是低透明度语义色 overlay。
- 常见浮层遮罩改用 `theme.neutral.overlay`，全屏 Loading mask 改用 `theme.neutral.mask`。
- 关键控件避免写死浅色边框 / 浅色背景 / 红白关闭按钮。
- Gallery 增加 Theme dogfooding 页面；Docs 增加 Theme System 页面和 compile-checked snippet。
- 增加源码级回归测试防止重新引入硬编码 light/dark 颜色。

## Completed changes

- `liora-theme`
  - 新增 dark semantic subtle token 生成路径：`ColorFamily::new_dark(...)`。
  - `primary/info/success/warning/danger.light_9/light_8/light_7` 在 dark 模式下为透明 overlay，避免 Table hover、Picker chip、Upload hover 等区域过亮。
  - 增加 light/dark subtle token 回归测试。

- `liora-components`
  - `Dialog` / `Drawer` / `Tour` 遮罩改用 `theme.neutral.overlay`。
  - `Loading::full_screen()` 背景改用 `theme.neutral.mask`。
  - `CodeEditor` 行号 gutter 边框改用 `theme.neutral.border`。
  - `AppWindowFrame` 自定义关闭按钮 hover 改用 `theme.danger.base` + `theme.neutral.inverted`。
  - 增加 `visual_theme_consistency_tests` 覆盖遮罩、CodeEditor、WindowFrame token 使用。

- `liora-gallery`
  - 新增 `Theme 主题系统` demo，展示当前 `ThemeMode`、语义色 token 和按钮交互状态。

- `liora-docs`
  - 新增 `Theme` 页面，说明 System/Light/Dark、`observe_window_appearance`、token 使用原则和 dark subtle token 策略。
  - 新增 `theme/system_mode.rs` compile-checked snippet。
  - 增加 Docs 页面注册和 snippet loader 回归测试。

## Non-goals

- 不替换 Liora 的主题品牌色。
- 不把所有用户自定义颜色 demo 强制改为语义色；自定义颜色 API 的存在是刻意能力。
- 不执行截图级视觉自动化；当前验证以源码回归、编译、测试、Docs/Gallery smoke 为主。
- 不引入 WebView/HTML/CSS/DOM/browser runtime。

## Completion evidence

- `cargo fmt --all --check`
- `cargo check --workspace --all-targets`
- `cargo test --workspace`
- `cargo check -p liora-docs --bin check_snippets`
- `cargo doc --workspace --no-deps`
- `cargo run -p xtask -- package validate`
- `cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build`
- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run`
- `timeout 10s cargo run -p liora-gallery`
- `timeout 10s cargo run -p liora-docs`
- `git diff --check -- . ':(exclude).omx'`
