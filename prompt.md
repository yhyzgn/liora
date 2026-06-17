# Aura UI — AI Development Prompt

> **用途**: 本文件供任何 AI 开发工具（OpenCode / Claude / Gemini / Codex / Cursor 等）在接手 Aura 项目时读取，确保上下文连贯、开发风格一致。

---

## 1. 项目速览

**Aura** 是一套基于 [GPUI](https://github.com/zed-industries/zed) 的企业级 Rust Native 组件库，参照 [Element-Plus](https://element-plus.org/zh-CN/) 的 API 规范和组件分类体系。目标是为 Rust 桌面应用提供开箱即用的高级 UI 控件。

| 属性 | 值 |
|------|-----|
| 语言 | Rust edition 2024 |
| UI 框架 | GPUI 0.2.2 (git = "https://github.com/zed-industries/zed") |
| 参考规范 | Element-Plus 2.x (https://element-plus.org/zh-CN/) |
| 架构 | Cargo Workspace Monorepo |
| 目标 | ~76+ 个企业级组件, 分阶段交付；P9 作为延后高级组件补全 backlog；P10 原生统计图组件；P11 系统托盘/进程常驻阶段；P12 原生安装器打包阶段；P13 高级控件扩展阶段；P14 延后高级组件补全阶段 |

---

## 2. 关键文档索引

| 文档 | 路径 | 用途 |
|------|------|------|
| 架构设计 | `architecture-design.md` | 完整技术方案、组件清单、Token 体系、里程碑 |
| 工程结构 | `structure.txt` | Workspace 目录树速览 |
| 初始调研 | `chat.txt` | Gemini 技术可行性分析 |
| **记忆库** | `.memory/` | 当前状态、架构决策、组件清单、会话历史 |
| **阶段提示词** | `.prompt/` | 各阶段开发指令，链式继承 |
| **P9 延后高级组件** | `.prompt/P9-deferred-advanced.md` | P5 跳过/延后的高级组件 backlog，后续需要时补充 |
| **P10 统计图组件** | `.prompt/P10-charts.md` | 原生 GPUI 统计图控件：Line/Area/Bar/Pie/Ring/Sparkline/Axis/Grid/Legend/Tooltip |
| **P11 托盘常驻** | `.prompt/P11-tray.md` | `aura-tray` 跨平台系统托盘、动态图标、CheckBox/N 级菜单与 GPUI 常驻进程桥接 |
| **P12 原生打包** | `.prompt/P12-packaging.md` | `aura-packager` / `xtask package` / `packaging/` / CI installer pipeline，含 install-smoke plan，纯 Rust + GPUI 原生应用打包 |
| **P13 高级控件扩展** | `.prompt/P13-component-expansion.md` | 已实现：二维码、代码编辑器、信号图、热力/比例条、拖动列表、Timer、Label/Operation，以及 BarChart 独立迷你柱样式与 Chart/Button/Tag/Radio/Checkbox 增强 |
| **P14 延后高级组件补全** | `.prompt/P14-deferred-advanced.md` | 正在实现 P9 deferred backlog；Wave 1 已完成 Carousel、Calendar、InputTag；Wave 2 已完成 Mention、Watermark；Wave 3 已完成 TreeSelect；Wave 4 已完成 Tour；Wave 5 已完成 VirtualizedTable；Wave 6 已完成 VirtualizedTree |

---

## 3. 工程结构

```
aura/
├── Cargo.toml                       # [workspace] root
├── crates/
│   ├── aura-core/       lib.rs      # Global 配置、ElementExt trait、Z-Index
│   ├── aura-theme/      lib.rs      # Design Tokens、亮/暗主题、ButtonVariant/Size
│   ├── aura-components/ src/        # 全部业务组件 (button.rs, input.rs, ...)
│   ├── aura-tray/       src/        # 系统托盘 facade (tray-icon + muda)
│   │   └── lib.rs
│   ├── aura-packager/   src/        # P12 打包领域逻辑：metadata/format/checksum/manifest/backend config
│   └── aura-icons/      lib.rs      # Icon trait、图标函数
├── apps/
│   ├── aura-gallery/    src/        # 组件看板 (GPUI 窗口)
│       ├── main.rs
│       ├── category.rs
│       └── demos/
│           ├── mod.rs               # Demo 注册表 registry()
│           └── *_demo.rs            # 各组件 Demo 页面
│   └── aura-docs/                   # 官方原生文档主程序 (GPUI 窗口)
│       ├── content/
│       │   ├── pages/               # 每个文档/控件一份 Markdown
│       │   └── snippets/            # 外部 .rs 代码片段
│       └── src/
│           ├── main.rs
│           └── markdown.rs          # P8: Markdown AST → Aura 原生元素树
├── xtask/                            # P12 统一工程命令入口：cargo run -p xtask -- package ...
├── packaging/                        # P12 icons、desktop、metainfo、entitlements、Windows installer resources
├── .github/workflows/package.yml      # P12 Linux/macOS/Windows packaging matrix
├── .memory/                          # 🧠 记忆库 (跨会话状态)
│   ├── state.md                     # 当前阶段 + 进度
│   ├── decisions.md                 # 架构决策记录
│   ├── inventory.md                 # 组件清单与完成状态
│   └── sessions.md                  # 会话历史
├── .prompt/                          # 📋 阶段提示词链
│   ├── P0-foundation.md
│   ├── P1-basic-elements.md
│   ├── P2-form-controls.md
│   ├── P3-popper-feedback.md
│   ├── P4-nav-data.md
│   ├── P5-advanced.md
│   ├── P6-builtin-id.md
│   ├── P7-demo-self-contained.md
│   ├── P8-engineering.md
│   ├── P9-deferred-advanced.md
│   ├── P10-charts.md
│   ├── P11-tray.md
│   └── P12-packaging.md
├── prompt.md                         # 📌 本文件 (AI 入口)
├── architecture-design.md
└── structure.txt
```

---

## 4. 开发工作流 🔄

### 4.1 每次对话开始

```
1. 阅读本文件 (prompt.md)
2. 阅读 .memory/state.md  了解当前阶段和进度
3. 阅读 .prompt/<current-phase>.md  了解当前阶段任务
4. 开始工作
```


### 4.1.1 阶段状态提示

- P5 当前请求范围已结束；Carousel、Calendar、TreeSelect、InputTag、Mention、Watermark、Tour、VirtualizedTable、VirtualizedTree 已移入 `.prompt/P9-deferred-advanced.md`。
- P8 当前技术路线已调整为 **Aura Docs 主程序**：官方文档在 GPUI 原生窗口中渲染，且独立为 `aura-docs` 主程序；`aura-gallery` 保持组件看板，不再承担官方文档入口。
- P9 deferred backlog 已迁移并由 P14 完成；不要再把 Carousel/Calendar/TreeSelect/InputTag/Mention/Watermark/Tour/Virtualized* 当作等待项。
- P10 原生统计图阶段已完成：Line/Area/Bar/Pie/Ring/Sparkline、scale/axis/grid/legend、降采样，以及 Line/Area Overlay/Bar/Pie/Ring 原生 hover tooltip / hit testing 均已落地。后续缓存策略只有在真实 profiling 证据出现时再作为独立性能阶段推进。
- P11 是系统托盘/进程常驻阶段：开发 `aura-tray`，采用 `tray-icon` + `muda`，支持动态图标、CheckBox 菜单、二级/三级/N 级菜单，并在 Gallery/Docs 中提供丰富用例。
- P12 是打包 readiness 阶段：`aura-packager` + `xtask package` + `packaging/` + CI workflow 已落地并有 runner-safe smoke/install-smoke gates。剩余主要是签名/公证、真实系统级安装卸载、license policy、release tag 实跑等外部策略/runner 事项。应用必须保持纯 Rust + GPUI native，严禁转成 Tauri 或引入 WebView/HTML/CSS/DOM/browser runtime。
- P13 主体已实现：二维码、代码编辑器、信号图、热力/比例条、横向/纵向拖动列表、Timer、Label、Operation，以及 Chart/Button/Tag/Radio/Checkbox 等高自定义能力；详见 `.prompt/P13-component-expansion.md`。
- P14 延后高级组件补全阶段已完成：Carousel、Calendar、InputTag、Mention、Watermark、TreeSelect、Tour、VirtualizedTable、VirtualizedTree 均已实现并有 Gallery/Docs/snippets/tests。详见 `.prompt/P14-deferred-advanced.md`。

### 4.2 每个组件/功能开发流程

```
┌─────────────────────────────────────────────────┐
│ 1. 编码                                          │
│    └── 创建/修改 crates/aura-components/src/<name>.rs
│    └── 在 lib.rs 中 pub mod + pub use            │
│                                                  │
│ 2. Demo (必须)                                    │
│    └── 创建 apps/aura-gallery/src/demos/<name>_demo.rs
│    └── render() -> AnyElement   │
│    └── 在 demos/mod.rs 注册表添加 DemoEntry      │
│                                                  │
│ 3. 验证 (必须)                                    │
│    └── cargo check  (0 errors, 0 warnings)       │
│                                                  │
│ 4. 提交 (通过后)                                   │
│    └── git add -A                                │
│    └── git commit -m "✨ component: add <Name>"  │
│    └── git push origin master                    │
│                                                  │
│ 5. 记忆更新 (必须)                                 │
│    └── 更新 .memory/inventory.md 组件状态         │
│    └── 更新 .memory/sessions.md 会话记录          │
│    └── 里程碑完成时更新 .memory/state.md           │
└─────────────────────────────────────────────────┘
```

### 4.3 阶段完成时

```
1. 更新 .memory/state.md   (标记阶段 done, 更新 next)
2. 审查 .prompt/<next-phase>.md   (确保上下文准确)
3. 如有架构决策，更新 .memory/decisions.md
4. Git commit + push
```

### 4.4 阶段回退/调整时

```
1. 更新 .memory/state.md   (回退 phase status)
2. 更新 .prompt/<affected-phase>.md  (调整任务描述)
3. 更新所有后续 .prompt/ 文件 (级联影响)
4. Git commit + push
```

---

## 5. 关键架构约束

### 5.1 组件 API 风格（codex 范式）

```rust
// ✅ 正确 — RenderOnce + IntoElement，主题从 cx.global 自动读取
Button::new("Save").primary().large()
Icon::new(IconName::House).size(24.0)
CodeBlock::new("cargo run -p aura-docs").shell().copyable(true)

// 实现范式
impl RenderOnce for MyComponent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        // ...
    }
}
impl IntoElement for MyComponent {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}

// ❌ 禁止 — .build(theme) 传参模式
```

### 5.2 类型和 Context

```rust
// GPUI 0.2.2 关键类型
gpui::App              // 应用全局
gpui::Context<'_, V>   // 视图上下文 (可读 Global)
gpui::Window           // 窗口句柄
gpui::AnyElement       // 类型擦除的 Element (Demo 注册表用)
gpui::IntoElement      // 渲染目标 trait
gpui::InteractiveElement // hover/on_mouse_up 等交互 trait

// 读取主题
fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    let theme = _cx.aura(); // ContextExt trait
    // ...
}
```

### 5.3 GPUI Features

```toml
# 库 crate — 不启用平台 feature
aura-core/Cargo.toml:     gpui = { workspace = true }
aura-theme/Cargo.toml:    gpui = { workspace = true }

# App — 显式启用
aura-gallery/Cargo.toml:
  gpui = { workspace = true, features = ["wayland", "x11", "font-kit"] }
  gpui_platform = { workspace = true, features = ["wayland", "x11"] }
```

### 5.4 不在 GPUI 0.2.2 中的 API

```
❌ RenderOnce    → 使用 Render
❌ ViewContext   → 使用 Context<'_, V>
❌ WindowContext → 使用 Window + App
❌ .when()       → 使用 if/else 手动构建
❌ .when_some()  → 使用 if let 手动构建
❌ .active()     → 需要先 .id() → Stateful<Div>, 普通 Div 无此方法
❌ .on_click()   → 需要 StatefulInteractiveElement (先 .id())
✅ .on_mouse_up(MouseButton::Left, ...) → InteractiveElement 上可用
✅ .hover(|style| ...) → InteractiveElement 上可用
✅ .cursor_pointer()   → 在 hover 闭包内使用
```

---

## 6. P8 Aura Docs 主程序规约

P8 的目标不是搭建网页文档站，而是把 `aura-gallery` 维持组件看板，文档另以 `aura-docs` 独立主程序交付。

### 6.1 绝对边界

- 100% GPUI 原生窗口运行。
- 文档渲染基于 Rust、GPUI 元素树、Aura 组件、Flex 布局和原生滚动容器。
- 禁止引入 Web 文档站、浏览器渲染路径、跨端转译运行时或网页排版模型。
- Markdown 只允许作为输入文本格式；解析后必须映射为 Aura/GPUI 原生节点。

### 6.2 Markdown 自举架构

- `pulldown-cmark` 只负责 Markdown AST/Event 解析。
- 富文本折行、样式、段落布局由 Aura Typography 组件负责。
- P8 需要优先补齐富文本文本片段与段落能力：多样式片段在同一段落中流式拼接、自动换行、不截断。
- `apps/aura-docs/src/markdown.rs` 负责 `render_markdown(md_text: &str) -> gpui::AnyElement`。
- Renderer 使用 `Vec` 栈管理块级容器，使用文本样式上下文管理 strong/emphasis/code 等内联状态。
- 文档内容按单页文件维护：`apps/aura-docs/content/pages/<page>.md`；组件页使用 `<component>.md`。
- 代码示例与 Markdown 分离：`apps/aura-docs/content/snippets/<page>/<case>.rs`。
- Markdown fenced code 通过 `src` 引用外部片段，例如 <code>```rust src="button/types.rs"</code>，`src` 路径相对于 `content/snippets/`。

### 6.3 Live Demo 注入

Markdown 中的特殊语法：

```text
::AuraDemo{component="Button"}::
```

必须被解析为真实 Aura/GPUI view node，而不是普通文本。插入后的组件必须保留 hover、click 等真实交互能力。

---


## 6.5 P10 统计图组件阶段规约

P10 目标是在 `aura-components` 中新增企业级统计图控件，全部运行在 GPUI 原生渲染路径中。严禁引入 ECharts、Canvas/WebView、SVG DOM、HTML/CSS、WASM 或跨端图表运行时。

技术路线：

- 首选 GPUI 官方能力：`canvas(...)`、`PathBuilder`、`Window::paint_path`、`Window::paint_quad`、TextSystem/`Text`/`Paragraph`。
- 图表绘制基础设施沉淀在组件库内，例如 `chart.rs` / `chart_scale.rs` / `chart_axis.rs` / `chart_shape.rs`。
- `https://github.com/vicanso/zedis` 的 Metrics 页面可作为 GPUI 图表案例参考：它通过 `canvas` 绘制 Area/Line/Bar，并将 scale、axis、grid、shape 拆层；但 Aura 必须实现自己的 API、主题、测试与文档。
- 主题颜色优先来自 `Theme` 的语义色，必要时新增 chart palette token。

首批交付控件：

1. `LineChart` — 折线图，支持多 series、平滑/直线、点标记、空数据。
2. `AreaChart` — 面积图，支持填充透明度、堆叠后续扩展。
3. `BarChart` — 柱状图，支持竖向柱、分组后续扩展。
4. `PieChart` / `RingChart` — 饼图/环图，支持百分比、legend。
5. `Sparkline` — 迷你趋势图，用于 Statistic/Card 中嵌入。
6. 基础设施：linear/band/point scales、axis、grid、legend、tooltip/hover hit test。

每个图表必须：新增组件文件、导出 API、Gallery demo、Docs 页面与 snippet、单元测试、`cargo check/test/run` 验证后提交推送。


## 6.6 P12 原生安装器打包规约

P12 目标是为 `aura-gallery`、`aura-docs` 以及未来 Aura GPUI 主程序建立跨平台原生安装器/发布产物流水线。

### 6.6.1 绝对边界

- Aura app 必须保持 **纯 Rust + GPUI native**。
- 严禁把 `aura-gallery`、`aura-docs` 或未来 Aura 主程序改造成 Tauri 应用。
- 严禁引入 WebView、HTML/CSS/DOM、browser runtime 或前端构建链作为应用运行时。
- 可以使用独立 packaging tools，但它们只能处理产物打包，不能改变应用架构。

### 6.6.2 当前架构

| 模块 | 职责 |
|---|---|
| `crates/aura-packager` | 打包领域逻辑：app metadata、format enum、checksum、manifest、cargo-packager config、RPM metadata |
| `xtask` | 统一入口：`cargo run -p xtask -- package ...` / `cargo run -p xtask -- package ci ...` |
| `packaging/` | 静态平台资源：icons、Linux desktop/metainfo、macOS entitlements、Windows nsis/wix folders |
| `.github/workflows/package.yml` | Linux/macOS/Windows packaging matrix，dry-run workflow_dispatch，`v*` tag 真实打包并自动发布 GitHub Release |

### 6.6.3 已完成能力

- `cargo run -p xtask -- package validate`
- `cargo run -p xtask -- package build --app <gallery|docs>`
- `cargo run -p xtask -- package --app <gallery|docs> --format <format>`
- `cargo run -p xtask -- package ci --all-apps --format platform-defaults`
- `cargo run -p xtask -- package smoke --all-apps --format platform-defaults`
- `--dry-run --skip-build` 生成后端配置并打印真实命令。
- `cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run` 生成 runner-safe install/uninstall 计划，不依赖真实 artifact。
- `cargo-packager` config generation：AppImage、deb、app、dmg、NSIS、MSI/WiX。
- `cargo-generate-rpm` supplemental backend config generation：RPM。
- Aura supplemental portable `.tar.gz` backend：收集 release binary、icons、Linux desktop/metainfo、README、启动脚本，并输出 `<package>-<version>-<platform>-<target-triple>.tar.gz`。
- Linux deb/rpm runtime dependency metadata：GTK3、Ayatana/AppIndicator、X11/Wayland、xkbcommon、fontconfig/freetype、Vulkan、ALSA、xdg-utils。
- 真实打包后生成：`package-manifest.json`、`checksums.txt`、`release-notes.md`，manifest 包含 version、platform、target triple、git sha。
- GitHub Actions 在 `main` push 上构建 preview 包（`<base>-preview.<run_number>.<short_sha>`），在 `v*` tag 上自动下载各平台 release artifacts，基于 git commits 按类型分组收集 changelog，创建/更新 GitHub Release 并上传全部构建产物。
- main Aura logo 已选择第 3 套 ribbon，落到 `packaging/icons/aura.*`。

### 6.6.4 当前验证基线

```bash
cargo check -p xtask -p aura-packager
cargo test -p aura-packager
cargo run -p xtask -- package validate
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
```

Dry-run 预期生成：

```text
target/aura-packager/Packager.gallery.toml
target/aura-packager/Packager.docs.toml
target/aura-packager/GenerateRpm.gallery.toml
target/aura-packager/GenerateRpm.docs.toml
```

### 6.6.5 P12 剩余工作

GitHub preview runner `27613242837` / commit `5a3615d` 已通过 Linux/macOS/Windows packaging matrix：真实 package generation、artifact smoke、raw binary/package artifact upload 均成功。下一位开发者接手时按此优先级推进：

1. **Signing / notarization**：macOS `codesign`/`notarytool`/`stapler`，Windows `signtool`/timestamp，CI secrets 与 unsigned fallback。
2. **GitHub Release automation 后续增强**：基础自动 Release、push preview 构建、按提交类型分组 changelog 已接入；后续可补 release draft/prerelease 策略、非 tar 后端 artifact 命名清洗、按平台聚合 manifest。
3. **Install / uninstall smoke**：已补 artifact smoke 与 `xtask package install-smoke --dry-run` plan-only gate；portable `.tar.gz` 支持显式 `--execute-install` 安全解压/删除验证。仍待在 dedicated runners 和明确 policy 下放开真正系统级 deb/rpm/AppImage/macOS/Windows 安装/卸载执行 gate。
4. **License / metadata cleanup**：当前无明确 `LICENSE`，RPM 暂用 `LicenseRef-Aura`。
5. **CI release-tag iteration**：preview runner 已通过；仍需在 `v*` tag 上验证 release job、GitHub Release asset 上传与 Windows MSI。

完整细节见 `.prompt/P12-packaging.md`。


## 6.7 P13 高级控件扩展规约

P13 主体已实现，目标是补齐 Dashboard / 低代码配置 / 数据监控 / 内容编辑场景中的高级控件与既有控件高自定义能力。

已落地范围：

- 新增：`QrCode`、`CodeEditor`、`SignalMeter`、`HeatBar`、`SegmentRatioBar`、`HorizontalList`、`Timer`、`Label`、`Operation`。
- 增强：`RingChart` 外置 legend/value pattern、`LineChart` per-series 线型、`BarChart` 独立迷你柱样式与 value range colors、`RingProgress` gradient/completion color、`Button` gradient/custom color states、`Tag` flow layout、`Radio`/`Checkbox` option render customization、既有垂直列表 item drag。
- QR/CodeEditor 如需新增依赖，必须先做 dependency review；CodeEditor 第一阶段复用已有 `syntect` 高亮，语法检查只做 provider/diagnostics 扩展点，不硬绑定 LSP。
- 所有新增/增强控件必须同步 Gallery demo、Docs 页面、外部 snippets 和核心状态/计算测试；已有控件增强必须落在原组件、原 demo、原 docs/snippets 上，不另建平行控件。

完整实现与维护规约见 `.prompt/P13-component-expansion.md`。

执行原则：已有控件直接增强原实现（例如 Tag flow、Progress/RingProgress 渐变、LineChart 线型、BarChart 独立迷你柱样式/区间色、Button 自定义色、Radio/Checkbox option 自定义），不要新增替代控件或平行页面。

## 7. Gallery Demo 规约

### 7.1 Demo 函数签名

```rust
// apps/aura-gallery/src/demos/<name>_demo.rs
pub fn render() -> gpui::AnyElement {
    gpui::Component::new(NameDemo).into_any_element()
}

struct NameDemo;

impl gpui::RenderOnce for NameDemo {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;
        div().flex().flex_col().gap_4()
            .child(section_header(theme, "Variants 变体"))
            .child(demo_row(vec![...]))
    }
}
```

### 7.2 注册表 (增量添加)

```rust
// apps/aura-gallery/src/demos/mod.rs
pub fn registry() -> Vec<DemoEntry> {
    vec![
        DemoEntry {
            name: "Button 按钮",
            category: Category::Basic,
            description: "常用的操作按钮",
            render: button_demo::render,  // ← 函数指针
        },
        // 👇 新增组件只需在这里加一项
        DemoEntry {
            name: "Link 链接",
            category: Category::Basic,
            description: "文字链接",
            render: link_demo::render,
        },
    ]
}
```

### 7.3 Category 分类

```rust
Category::Basic       // ⊞ Basic 基础
Category::Form        // ☰ Form 表单
Category::Data        // ⊟ Data 数据
Category::Navigation  // ☈ Navigation 导航
Category::Feedback    // ⚡ Feedback 反馈
Category::Others      // ⋯ Others 其他
```

---

## 8. 记忆系统 🧠

### 8.1 记忆库更新时机

| 事件 | 更新文件 |
|------|---------|
| 任何代码变更 | `.memory/sessions.md` (追加记录) |
| 组件完成 | `.memory/inventory.md` (标记 ✅) |
| 阶段完成 | `.memory/state.md` (更新 phase/next) |
| 架构决策 | `.memory/decisions.md` (追加 ADR) |
| 发现 API 差异 | `.memory/sessions.md` (Key Discoveries) |

### 8.2 记忆库格式

所有 .memory/ 文件使用 Markdown，保持简洁、结构化、可追加。新条目追加在文件末尾或对应位置。

---

## 9. Git 提交规范

### 9.1 Commit Message 格式

```
<emoji> <scope>: <subject>

<body — 可选，多行详细说明>

<footer — 可选，关联 issue>
```

### 9.2 Emoji 参考

| Emoji | 用途 |
|-------|------|
| ✨ `:sparkles:` | 新组件/新功能 |
| 🎨 `:art:` | 样式/主题/Token 调整 |
| 🏗️ `:building_construction:` | 架构/结构变更 |
| 🐛 `:bug:` | Bug 修复 |
| ♻️ `:recycle:` | 重构 |
| 📝 `:memo:` | 文档 |
| 🧪 `:test_tube:` | 测试 |
| 🔧 `:wrench:` | 配置/工具 |
| 🚀 `:rocket:` | 发布/CI |
| 🧠 `:brain:` | 记忆库更新 |
| 📋 `:clipboard:` | 阶段提示词更新 |

### 9.3 示例

```
✨ button: add icon_start/icon_end support

- Add .icon_start(AnyElement) and .icon_end(AnyElement) builder methods
- Update demo with icon examples
- Register in gallery registry

Closes #P1-button-icons
```

---

## 10. 阶段导航

```
当前阶段 → 读取 .memory/state.md 获取
├── P0 Foundation          ✅ → .prompt/P0-foundation.md
├── P1 Basic Elements      ⬜ → .prompt/P1-basic-elements.md
├── P2 Form Controls       ⬜ → .prompt/P2-form-controls.md
├── P3 Popper + Feedback   ⬜ → .prompt/P3-popper-feedback.md
├── P4 Nav + Data          ⬜ → .prompt/P4-nav-data.md
├── P5 Advanced            ⬜ → .prompt/P5-advanced.md
├── P6 Built-in Unique ID  ⬜ → .prompt/P6-builtin-id.md
├── P7 Demo Self-Contained ⬜ → .prompt/P7-demo-self-contained.md
├── P8 Native Docs App ✅ → .prompt/P8-engineering.md
├── P9 Deferred Advanced ⏸️ → .prompt/P9-deferred-advanced.md
├── P10 Charts ✅ → .prompt/P10-charts.md
├── P11 Tray ✅ → .prompt/P11-tray.md
├── P12 Packaging ✅ → .prompt/P12-packaging.md
├── P13 Component Expansion ✅ → .prompt/P13-component-expansion.md
└── P14 Deferred Advanced 🧭 → .prompt/P14-deferred-advanced.md
```

---

## 11. 快速命令

```bash
# 编译检查
cargo check

# 运行组件看板
cargo run -p aura-gallery

# 编译所有 crate
cargo build

# 运行测试 (如果有)
cargo test

# P12 打包资源校验
cargo run -p xtask -- package validate

# P12 打包 dry-run（生成后端配置，不真实产物）
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build

# P12 install/uninstall plan-only gate（不需要真实产物）
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run

# P12 当前平台真实打包（需要先安装后端工具）
cargo run -p xtask -- package ci --all-apps --format platform-defaults

# 清理构建
cargo clean
```

---

## 12. 启动检查清单 ⚡

接手本项目时的最小行动集：

- [ ] 读取 `prompt.md` (本文件)
- [ ] 读取 `.memory/state.md` (当前阶段)
- [ ] 读取 `.prompt/<current-phase>.md` (当前任务)
- [ ] 运行 `cargo check` 确认编译基线
- [ ] 运行 `cargo run -p aura-gallery` 确认看板基线
- [ ] 开始工作, 按 §4.2 流程推进
