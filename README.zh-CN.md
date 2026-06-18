<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/liora-logo.svg">
    <img src="assets/liora-logo.svg" alt="Liora — 纯 Rust + GPUI 原生 UI 组件库" width="220">
  </picture>

  <p><strong>面向 Rust 桌面应用的企业级 GPUI 原生组件库。</strong></p>
  <p>纯 Rust。GPUI Native。API 体系参考 Element Plus。无 Tauri、无 WebView、无浏览器运行时。</p>

  <p>
    <a href="README.md">English</a>
    ·
    <a href="docs/release-candidate-checklist.md">Release Candidate Checklist</a>
    ·
    <a href="CONTRIBUTING.md">Contributing</a>
    ·
    <a href="CHANGELOG.md">Changelog</a>
  </p>

  <p>
    <img alt="Rust 2024" src="https://img.shields.io/badge/Rust-2024-dea584?logo=rust&logoColor=white">
    <img alt="GPUI native" src="https://img.shields.io/badge/GPUI-native-7c3aed">
    <img alt="Pure Rust" src="https://img.shields.io/badge/runtime-pure%20rust%20native-10b981">
    <img alt="LicenseRef-Liora" src="https://img.shields.io/badge/license-LicenseRef--Liora-64748b">
    <img alt="Release candidate" src="https://img.shields.io/badge/0.1.0-RC%20ready-0ea5e9">
  </p>
</div>

---

## Liora 是什么？

**Liora** 是一个用于构建精致、企业级 **Rust 原生桌面 UI** 的 [GPUI](https://github.com/zed-industries/zed) 组件库工作区。它把 Element Plus 风格的组件分类和 API 体验带到 Rust Native 桌面应用中，覆盖基础元素、表单、反馈浮层、导航、数据展示、高级输入、原生图表、虚拟化数据视图、代码展示/编辑、系统托盘，以及安装器打包基础设施。

Liora 明确不是 Web 外壳。基于 Liora 的应用必须保持 **纯 Rust + GPUI 原生** 路线：

- 不使用 Tauri runtime；
- 不引入 WebView、HTML/CSS/DOM 或浏览器应用壳；
- 不引入 Web 图表运行时、SVG DOM 图表层或前端构建链；
- Gallery 和 Docs 是维护中的 GPUI 原生应用，不是独立 sample 产品。

## 为什么选择 Liora？

Rust 桌面团队通常需要的不只是几个底层 primitive。Liora 专注补齐从 GPUI 底层布局代码到真实产品界面之间的“企业级组件层”。

| 需求 | Liora 的回答 |
|---|---|
| 原生桌面 UI | GPUI 元素树、原生窗口、原生文本/布局/绘制路径。 |
| 企业组件覆盖 | 参考 Element Plus 的分类体系，覆盖表单、反馈、数据、导航、图表和高级控件。 |
| 真实产品 dogfooding | `liora-gallery` 与 `liora-docs` 覆盖组合布局、主题切换、搜索过滤、托盘、文档渲染和 Dashboard 模式。 |
| 主题系统 | Light、Dark、System 三种主题入口，语义 token 与组件级 variant。 |
| 发布准备 | `liora-packager` + `xtask package` 校验安装器 metadata、manifest、checksum、签名策略和 release gate。 |
| 清晰架构边界 | 可复用组件进入 `liora-components`；业务 sample model 与 dashboard glue 留在 Gallery/Docs 应用层。 |

## 能力亮点

- **70+ 原生 UI 组件**，覆盖 Basic、Form、Feedback、Data、Navigation、Others 等分类。
- **Element Plus 启发的 API 风格**，结合 Rust builder 与 GPUI 渲染范式。
- **原生图表**：Line、Area、Bar、Pie、Ring、Sparkline、scale、grid、legend、降采样与 hover hit testing。
- **高级控件**：CodeEditor、CodeBlock、二维码、Timer、SignalMeter、HeatBar、SegmentRatioBar、拖动列表、Tour、TreeSelect、Mention、InputTag、Watermark、VirtualizedTable/Tree。
- **浮层与交互系统**：Tooltip、Popover、Popconfirm、Dialog、Drawer、Dropdown、Message、Notification、MessageBox、Loading、Preview、Tour。
- **原生 Docs 渲染器**：Markdown 只作为输入格式，最终渲染为 Liora/GPUI 原生节点；代码片段与 Markdown 分离并参与编译检查。
- **系统托盘 facade**：`liora-tray` 基于 `tray-icon` + `muda`，支持动态图标、N 级菜单、checkbox 菜单、稳定命令与进程常驻。
- **安装器流水线**：metadata 校验、`cargo-packager` 配置生成、RPM 补充配置、portable `.tar.gz`、manifest、checksum、release notes 和 CI dry-run gate。
- **质量门禁**：workspace fmt/check/test、Docs snippet check、package validate、release-readiness、GUI startup smoke。

## 组件覆盖

| 分类 | 组件与能力 |
|---|---|
| Basic 基础 | Button、ButtonGroup、Icon、Link、Text、Title、Paragraph、Space、Divider、Row、Col、Container、Scrollbar、Splitter、CodeBlock |
| Form 表单 | Input、InputNumber、Textarea、Checkbox、CheckboxGroup、Radio、RadioGroup、Switch、Select、Slider、Form、FormItem、Rate、DatePicker、TimePicker、DateTimePicker、Upload、Cascader、Transfer、ColorPicker、Autocomplete、InputTag、Mention、TreeSelect |
| Feedback / Overlay 反馈浮层 | Tooltip、Popover、Popconfirm、Dialog、Drawer、Message、Notification、Alert、Loading、MessageBox、Dropdown、Card、Collapse、Preview、Tour |
| Navigation 导航 | Menu、Tabs、Breadcrumb、Steps、PageHeader、Affix、Backtop、Anchor |
| Data 数据展示 | Table、VirtualizedTable、VirtualizedTree、VirtualizedList、Progress、Skeleton、Empty、Result、Descriptions、Timeline、Tree、Pagination、Statistic、Segmented、Tag、Avatar、Badge、Calendar、Carousel、Image、Watermark |
| Charts / Metrics 图表指标 | LineChart、AreaChart、BarChart、PieChart、RingChart、Sparkline、SignalMeter、HeatBar、SegmentRatioBar |
| Editing / Utility 编辑与工具 | CodeEditor、QrCode、Timer、Label、Operation、横向/纵向拖动列表模式 |
| Platform / App shell 平台能力 | `liora-tray`、自定义窗口框架 polish、Gallery shell patterns、Docs adoption pages、packaging helpers |

## 仓库结构

```text
liora/
├── crates/
│   ├── liora-core/            # 全局配置、主题初始化、popper/portal 状态、唯一 ID
│   ├── liora-theme/           # 语义 token、Light/Dark/System 主题支持
│   ├── liora-components/      # 可复用 GPUI 组件
│   ├── liora-icons/           # 原生 icon trait 与 helper
│   ├── liora-icons-lucide/    # 生成的 Lucide icon 名称与 path 适配
│   ├── liora-tray/            # 面向 GPUI app 的 tray-icon + muda facade
│   └── liora-packager/        # package metadata、manifest、checksum、backend config
├── apps/
│   ├── liora-gallery/         # 原生组件看板与 dogfooding shell
│   └── liora-docs/            # 原生文档 app 与 Markdown renderer
├── xtask/                     # cargo run -p xtask -- package ...
├── packaging/                 # icons、desktop/metainfo、macOS/Windows/Linux package 资源
├── docs/                      # 技术计划、RC checklist、GitHub metadata 建议
├── .prompt/                   # 阶段提示词与维护契约
├── .memory/                   # 项目状态、决策、组件清单、会话历史
└── Cargo.toml                 # workspace root
```

## 快速开始

### 1. 安装依赖

安装 Rust stable，以及 GPUI 在当前平台需要的原生依赖。Linux 常见开发依赖包括 GTK3、Wayland/X11、xkbcommon、fontconfig/freetype、Vulkan、ALSA 与 `pkg-config`。仓库中也提供了面向 Fedora 的 `scripts/install-fedora-deps.sh`。

### 2. 运行原生 Gallery

```bash
cargo run -p liora-gallery
```

Gallery 是组件 demo、主题切换、搜索过滤、托盘控制、toast 和产品式组合布局的 canonical 可视化表面。

### 3. 运行原生 Docs app

```bash
cargo run -p liora-docs
```

Docs app 是 canonical 采用说明表面。它会把 Markdown 内容渲染成原生 Liora/GPUI 元素，并展示来自 `apps/liora-docs/content/snippets/` 的编译检查片段。

### 4. 检查工作区

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p liora-docs --bin check_snippets
cargo doc --workspace --no-deps
```

## 最小应用形态

一个使用 Liora 的 GPUI 应用应先初始化主题/配置，初始化所需全局服务，注册组件 key bindings，再打开 GPUI 窗口。

```rust
use gpui::App;
use liora_components::init_liora;

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        // 初始化 Liora core/theme 状态、组件服务和 key bindings。
        init_liora(cx);

        // cx.open_window(...)
    });
}
```

`liora_components::init_liora(cx)` 默认跟随系统主题，并统一初始化组件服务与 key bindings。当产品需要显式选择启动主题时，使用 `liora_components::init_liora_with_mode(cx, ThemeMode::Light | ThemeMode::Dark | ThemeMode::System)`。对于 `Input`、`Switch`、`Select`、`CodeEditor` 等有内部状态的控件，使用 `Entity<T>` 以保证 focus 和内部状态在重渲染后仍然稳定。Gallery 和 Docs 是应用壳初始化、key binding 注册、主题切换、托盘、toast 与组合模式的维护中参考实现。

## 组件 API 示例

Liora 组件采用 builder 风格，并通过 GPUI 原生元素渲染：

```rust
use gpui::{div, IntoElement, RenderOnce};
use liora_components::{Button, Space, Tag, Text, Title};

struct WelcomePanel;

impl RenderOnce for WelcomePanel {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        div()
            .child(Title::new("Native Rust UI").level(2))
            .child(Text::new("Build enterprise desktop screens with GPUI and Liora."))
            .child(
                Space::new()
                    .child(Button::new("Open Gallery").primary())
                    .child(Tag::new("Pure Rust").success()),
            )
    }
}
```

组件在 render 路径中从 Liora 全局配置读取主题。不要使用 `.build(theme)` 这类传入主题对象的 API 模式。

## 文档地图

| 资源 | 用途 |
|---|---|
| `apps/liora-docs` | 原生文档 app、Adoption Guide、组件页和 release 文档。 |
| `apps/liora-gallery` | 原生组件 Gallery 与 app-shell dogfooding 表面。 |
| `apps/liora-docs/content/pages/` | 由原生 Docs app 渲染的 Markdown 页面。 |
| `apps/liora-docs/content/snippets/` | Markdown 引用的外部代码片段，并由 `check_snippets` 检查。 |
| `docs/release-candidate-checklist.md` | 本地 `0.1.0` RC source of truth。 |
| `docs/packaging-installer-technical-plan.md` | 打包架构与平台 release 说明。 |
| `assets/github-repository-metadata.md` | 推荐的 GitHub About 描述与 SEO topics。 |
| `.prompt/` 和 `.memory/` | 阶段契约、当前状态、决策、组件清单与会话历史。 |

## 原生打包与 Release Readiness

仓库内打包准备由 `liora-packager` 和 `xtask` 提供：

```bash
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
```

打包流水线可生成 AppImage、`.deb`、`.rpm`、macOS `.app` / `.dmg`、Windows NSIS / MSI，以及 Liora portable `.tar.gz` 的后端配置或产物。正式发布仍属于 protected release 环境工作：macOS notarization、Windows signing、破坏性系统级安装/卸载 smoke，以及真实 `vX.Y.Z` GitHub Release 发布必须由 owner-controlled release path 执行。

当前 `0.1.0` RC gate 见 `docs/release-candidate-checklist.md`。

## 开发工作流

组件或功能变更应保持小步、可验证：

1. 在 `crates/liora-components/src/` 实现或更新组件。
2. 需要时从 `crates/liora-components/src/lib.rs` 导出 API。
3. 在 `apps/liora-gallery/src/demos/` 新增或更新 Gallery demo。
4. 新增或更新原生 Docs 内容和外部 snippets。
5. 为行为、计算、解析或 release 边界添加聚焦测试。
6. 先运行最小相关检查，再运行 workspace gate。
7. 如果代码行为、组件清单或阶段状态变化，同步更新 `.memory/`。

## GitHub SEO metadata

推荐 GitHub 项目简介：

> Pure Rust + GPUI native enterprise UI component library for desktop apps — Element Plus-inspired components, charts, docs, tray integration, and installer packaging.

推荐 topics 维护在 `assets/github-repository-metadata.md`。根据 GitHub Docs，topics 有助于项目发现，应使用小写字母、数字和连字符，单个 topic 不超过 50 个字符，总数不超过 20 个。

## 当前状态

本地实现阶段已完成到 **P21 Release Candidate Readiness**。仓库已具备 owner-controlled `0.1.0` release-candidate 验证条件。Protected release-only 事项仍不属于普通本地开发：真实签名凭据、公证、破坏性安装器 smoke、正式 license 替换，以及公开 release 发布。

## 贡献

提交 PR 前请阅读 `CONTRIBUTING.md`。关键边界：

- 保持 Liora 为纯 Rust + GPUI native。
- 不引入 Tauri、WebView、HTML/CSS/DOM、browser runtime 或 web chart shell。
- 不重新新增独立 `examples/minimal-app` 或 `examples/dashboard-app`。
- 不把业务 sample model 或 dashboard-only helper 放进 `liora-components`。
- `.omx/**` 为本地运行时状态，不能进入提交。

## License

Liora 当前使用 `LicenseRef-Liora`；见 `LICENSE.md`。在项目 owner 明确替换为 OSS 或商业 license 条款前，不要假设本项目为开源 license。
