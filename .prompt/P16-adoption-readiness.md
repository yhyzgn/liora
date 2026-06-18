# P16 — Public API & Adoption Readiness

> 上游: `.prompt/P12-packaging.md` / `.prompt/P15-quality-hardening.md`
> 状态: Complete
> 目标: 让外部 Rust/GPUI 项目能在 10 分钟内理解 Aura、跑起最小示例、找到 API/Docs/发布流程，并知道如何贡献与发布。

## Scope

P16 不继续新增大组件，而是完成对外采用闭环：

- 根 README：项目定位、快速启动、示例、文档、验证命令、发布边界。
- 采用示例已回流到 Gallery/Docs；不再维护独立 `examples/minimal-app` workspace package。
- Public API 文档入口：核心 crate-level Rustdoc 说明组件导出、初始化、主题、托盘、图标和打包边界。
- Docs Adoption 页面：在 native Docs app 中说明从最小示例到真实项目采用的步骤。
- 贡献与发布说明：`CONTRIBUTING.md` / `CHANGELOG.md` 与 P12 release-readiness 流程对齐。
- 回归测试：锁住 README、Docs adoption 页面、minimal app、workflow/readiness 入口。

## Non-goals

- 不引入 Tauri/WebView/HTML/CSS/DOM/browser runtime。
- 不把当前 `LicenseRef-Aura` 伪装成正式 OSS license。
- 不执行真实 public release 或签名/公证；这些仍由 owner/protected environment 控制。
- 不为了文档重构大量组件 API。

## Completion evidence

- Gallery/Docs 是 adoption 的 compile-checked surfaces；不再要求 `aura-minimal-app`。
- `cargo doc --workspace --no-deps` 通过。
- Docs app 包含 Adoption Guide，且测试覆盖 README/Docs/workflow 入口。
- Full local gates pass and changes are committed/pushed.
