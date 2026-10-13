# Liora 图标系统设计文档

> 状态：Active
> 最后更新：2026-07-06
> 范围：`liora-icons`、内置 `liora-icons-*` 图标库、`liora-icons-optimizer`、应用打包资源集成。

## 1. 背景与目标

Liora 是基于 Rust + GPUI 的原生组件库。图标系统需要同时满足两个目标：

1. **开发体验顺滑**：业务代码直接写强类型图标枚举，例如 `Icon::new(IconName::Search)`，不手写 SVG 路径字符串。
2. **发布体积可控**：内置图标库数量很大，发布时不能把未使用的全集 SVG 都打进二进制或安装包。

因此当前架构采用：

```text
liora-icons          通用 Icon 容器、IntoIconPath trait、IconAssetSource 资源解析
liora-icons-*        每个内置图标库一个独立强类型 crate
liora-icons-optimizer 构建期自动扫描实际使用图标，只复制需要的 SVG 资源
liora-packager/xtask 打包时把 optimizer 输出目录带入 raw binary / portable / installer
```

关键边界：

- `Icon` 是无业务、无图标库绑定的通用渲染容器。
- 图标库 crate 只负责把上游 SVG 文件映射成 `IconName` 枚举和虚拟资源路径。
- optimizer 是 SDK 内部通用能力，不允许耦合 Gallery / Docs 等 demo 应用业务结构。
- 应用业务代码不从 optimizer 清单引用图标，图标使用方式保持不变。

## 2. 模块分层

| 模块 | 职责 | 不应该做的事 |
| --- | --- | --- |
| `liora-icons` | 提供 `Icon`、`IntoIconPath`、`inline_svg_asset_path`、`icon_svg_asset_path`、`IconAssetSource`。 | 不内置某个图标库全集，不感知 Gallery/Docs 页面。 |
| `liora-icons-lucide` | Lucide 强类型枚举与 SVG 文件映射。 | 不处理打包、不扫描业务源码。 |
| `liora-icons-antd` | Ant Design Icons 强类型枚举与 SVG 文件映射。 | 不把 AntD 专属命名规则泄漏给 `Icon` 容器。 |
| `liora-icons-ionic` | Ionicons 强类型枚举与 SVG 文件映射。 | 不耦合具体应用。 |
| `liora-icons-tabler` | Tabler Icons 强类型枚举与 SVG 文件映射。 | 不耦合具体应用。 |
| `liora-icons-carbon` | Carbon Icons 强类型枚举与 SVG 文件映射。 | 不耦合具体应用。 |
| `liora-icons-material` | Material Design Icons 强类型枚举与 SVG 文件映射。 | 不耦合具体应用。 |
| `liora-icons-optimizer` | 构建期扫描 `IconName::...` 使用点，生成最小 SVG bundle 和报告。 | 不要求业务代码改写成清单引用，不写死 app 路径。 |
| `liora-packager` / `xtask package` | 把 optimizer 产物放进最终应用资源目录。 | 不分析源码、不决定哪些图标被使用。 |

## 3. 通用 Icon 容器

`Icon` 位于 `crates/liora-icons`，是图标系统唯一的渲染入口。它接收任何实现 `IntoIconPath` 的输入：

```rust
use liora_icons::Icon;
use liora_icons_lucide::IconName;

Icon::new(IconName::Search).size_units(18.0)
```

核心职责：

- **尺寸**：`.size(...)` / `.size_units(...)` / `.size_xs()` / `.size_md()` / `.size_lg()` / `.size_xl()`。
- **颜色**：默认使用主题 `neutral.icon`，也可 `.color(...)` 显式指定。
- **交互状态**：支持 `.group_hover_color(...)` 和 `.group_hover_primary(...)`。
- **变换**：支持 `.rotation(...)` 和 `.render_scale(...)`，Spinner 等动画控件可复用。
- **资源解析**：将 `asset_path` 交给 GPUI `svg().path(...)`，由 `IconAssetSource` 加载真实 SVG 字节。

`Icon` 不解析具体图标库，不知道 `Search`、`Home`、`SaveOutlined` 等业务含义。

## 4. 内置图标库

Liora 当前内置 6 个强类型 SVG 图标库。所有图标库都遵循同一 API 形状：

```rust
pub enum IconName { ... }

impl IconName {
    pub const fn all() -> &'static [IconName];
    pub const fn set(&self) -> &'static str;
    pub fn file(&self) -> &'static str;
    pub fn svg_path(&self) -> String;
}

impl liora_icons::IntoIconPath for IconName { ... }
impl gpui::IntoElement for IconName { ... }
```

| 图标库 | Crate | Facade 模块 | 上游来源 | 命名规则示例 |
| --- | --- | --- | --- | --- |
| Lucide | `liora-icons-lucide` | `liora::icons_lucide` | `lucide-icons/lucide` | `search.svg` → `IconName::Search` |
| Ant Design Icons | `liora-icons-antd` | `liora::icons_antd` | `ant-design/ant-design-icons` | `save-outlined.svg` → `IconName::SaveOutlined` |
| Ionicons | `liora-icons-ionic` | `liora::icons_ionic` | `ionic-team/ionicons` | `add-circle-outline.svg` → `IconName::AddCircleOutline` |
| Tabler Icons | `liora-icons-tabler` | `liora::icons_tabler` | `tabler/tabler-icons` | `home-filled.svg` → `IconName::HomeFilled` |
| Carbon Icons | `liora-icons-carbon` | `liora::icons_carbon` | `carbon-design-system/carbon` | `checkmark-filled.svg` → `IconName::CheckmarkFilled` |
| Material Design Icons | `liora-icons-material` | `liora::icons_material` | `google/material-design-icons` | `search-outlined.svg` → `IconName::SearchOutlined` |

上游同步脚本和来源记录放在各图标库自己的 `scripts/` 与 `assets/UPSTREAM.md` 中。新增图标库时必须保持同构 API，不能让 `Icon` 为某个图标库增加专属分支。

## 5. 图标库代码生成

每个 `liora-icons-*` crate 都在 `build.rs` 中扫描自身 `assets/svgs/`：

1. 遍历 `.svg` 文件。
2. 将文件名 stem 转成 PascalCase Rust 枚举变体。
3. 在 `OUT_DIR/generated.rs` 中生成 `IconName`、`all()`、`set()`、`file()`。
4. `src/lib.rs` 通过 `include!(concat!(env!("OUT_DIR"), "/generated.rs"));` 引入。

当前实现保留完整 SVG 文件作为运行时资源，而不是在 generated enum 中内联 SVG 文本。这一点很重要：

- 业务代码得到强类型枚举。
- release 可以只携带实际使用的 SVG 文件。
- optimizer 可以按文件粒度复制资源。

命名约束：

- 文件名中的非 ASCII 字母数字分隔符会被视为单词边界。
- 空名称兜底为 `Icon`。
- 数字开头的变体会加 `I` 前缀，例如 `3d-rotation.svg` → `IconName::I3dRotation`。
- 新图标库必须与 optimizer 中的 `to_pascal_case` 规则保持一致，否则扫描到的 variant 无法映射到 SVG 文件。

## 6. 虚拟资源路径与运行时加载

强类型 `IconName` 不直接返回 SVG 文本，而是返回虚拟资源路径：

```text
liora-icon://<set>/<file>.svg
```

开发期会携带源码 fallback：

```text
liora-icon://lucide/search.svg?dev=/repo/crates/liora-icons-lucide/assets/svgs/search.svg
```

`IconAssetSource` 的解析顺序：

1. `LIORA_ICON_ASSETS_DIR/<set>/<file>`。
2. 可执行文件旁：`<exe-dir>/assets/liora-icons/<set>/<file>`。
3. Linux/macOS 包资源路径：`<exe-dir>/../assets/liora-icons/...`、`<exe-dir>/../Resources/assets/liora-icons/...`。
4. Linux 系统安装路径：`/usr/lib/<binary>/assets/liora-icons/<set>/<file>`。
5. 当前工作目录：`./assets/liora-icons/<set>/<file>`。
6. 开发构建目录：`target/liora/icons/apps/<app>/assets/liora-icons/<set>/<file>`。
7. 虚拟路径中的 `dev=` 源码 fallback。
8. 找不到时返回可见 fallback placeholder，避免静默空白。

调试缺失图标可设置：

```bash
LIORA_ICON_DEBUG=1 cargo run -p your-app
```

## 7. Icon Optimizer

`liora-icons-optimizer` 是构建期资源瘦身工具。它解决的问题是：内置图标库全集很大，但生产应用通常只使用很少一部分图标。

### 7.1 目标

- **业务代码不变**：继续写 `Icon::new(IconName::Search)`。
- **无需手动清单**：不让开发者维护 Cargo.toml 或 TOML 清单。
- **自动增删**：源码新增图标使用点后自动复制；删除使用点后下次构建自动从 bundle 移除。
- **SDK 不耦合业务**：只基于 Cargo metadata 和源码扫描，不写死 Gallery/Docs 路径。
- **可报告、可调试**：生成 markdown 报告，列出扫描根、复制图标、缺失图标、运行时搜索路径。

### 7.2 接入方式

应用 crate 添加构建依赖：

```toml
[build-dependencies]
liora-icons-optimizer = "0.4"
```

在应用唯一的 Cargo build script 中调用：

```rust
fn main() {
    liora_icons_optimizer::Optimizer::new()
        .bundle_auto()
        .run();

    // existing build.rs logic can stay here.
}
```

如果应用已有 `build.rs`，直接把 builder 调用合进去即可。Cargo 只执行一个 build script；如果要使用非默认文件名，可以在 `[package] build = "..."` 指定，但最终仍是一个入口。

高级用法：

```rust
fn main() {
    let report = liora_icons_optimizer::Optimizer::new()
        .scan_dir("src")
        .scan_dir("content/snippets")
        .bundle_auto()
        .try_run()
        .expect("icon optimization should complete");

    // 可选：把 report.copied 再嵌入 raw executable 兜底资源。
}
```

### 7.3 默认输出

默认输出目录：

```text
target/liora/icons/apps/<app>/assets/liora-icons/<set>/<file>.svg
```

默认报告：

```text
target/liora/icons/reports/<app>.md
```

`run()` 失败会通过 `cargo:error` 终止构建；`try_run()` 返回 `Result<OptimizationReport, OptimizerError>`，适合应用 build script 自行处理错误或生成额外嵌入资源。

### 7.4 扫描规则

optimizer 扫描应用源码目录（默认 `src/`，可追加 `scan_dir(...)`），并通过 `cargo metadata` 发现当前应用可达的 Liora 依赖源码。它会识别：

```rust
liora_icons_lucide::IconName::Search
liora::icons_lucide::IconName::Search
use liora_icons_lucide::IconName; IconName::Search
use liora_icons_lucide::IconName as LucideIcon; LucideIcon::Search
use liora::icons_tabler::IconName as TablerIcon; TablerIcon::HomeFilled
liora_icons_carbon::IconName::all()
```

`IconName::all()` 会打包对应图标库全集，只适合图标浏览器或文档页面。普通生产应用应引用具体枚举变体。

不会处理：

- 应用自己的 `assets/icons/*.svg`。
- `file:///...` 路径。
- `inline_svg_asset_path(...)`。
- 运行时字符串拼接出来的图标名。

这些资源属于应用自己的 assets/packager 流程，不应该混入 optimizer 输出目录。

### 7.5 打包集成

`liora-packager` / `xtask package` 读取：

```text
target/liora/icons/apps/<app>/assets/liora-icons
```

并放进最终应用资源目录：

```text
assets/liora-icons/<set>/<file>.svg
```

因此：

- raw executable 如果旁边带了 `assets/liora-icons`，可直接解析图标。
- Linux portable / deb / rpm / AppImage、macOS app/dmg、Windows installer 都能携带最小资源集。
- 如果需要单文件 raw executable，应用可基于 `try_run()` 的 `OptimizationReport` 生成 `OUT_DIR` 嵌入 bundle 作为兜底；Gallery/Docs 就是普通应用层这样接入，不代表 SDK 耦合它们。

## 8. 新增图标库流程

新增 `liora-icons-<set>` 时必须完成：

1. 新建独立 crate，并依赖 `liora-icons`。
2. 将同步后的 SVG 放入 `assets/svgs/`，记录 `assets/UPSTREAM.md`。
3. 提供同步脚本，确保来源可复现。
4. build script 生成同构 `IconName` API：`all()`、`set()`、`file()`。
5. `src/lib.rs` 实现 `svg_path()`、`IntoIconPath`、`IntoElement`。
6. 在顶层 facade `liora` 中暴露模块（如适用）。
7. 在 `liora-icons-optimizer` 的 `ICON_SETS` 中注册 set id、crate 名、facade 模块名。
8. 在 README / Docs 图标库页面补充说明和示例。
9. 添加测试，至少验证虚拟路径能通过 `IconAssetSource` 加载真实 SVG。

禁止事项：

- 禁止在 `liora-icons` 里写某个图标库的专用逻辑。
- 禁止让 optimizer 扫描 Gallery/Docs 的业务专属路径作为 SDK 行为。
- 禁止要求业务代码从 optimizer 生成清单中引用图标。
- 禁止把应用业务 SVG 放到 `assets/liora-icons`，该目录保留给 optimizer 输出。

## 9. 当前开发者使用范式

### 9.1 使用 facade

```rust
use liora::{components::Icon, icons_lucide::IconName};

Icon::new(IconName::Search).size_units(18.0)
```

### 9.2 使用拆分 crate

```rust
use liora_icons::Icon;
use liora_icons_antd::IconName as AntdIcon;

Icon::new(AntdIcon::SaveOutlined).size_lg()
```

### 9.3 自定义 SVG

```rust
use liora_icons::{Icon, inline_svg_asset_path};

Icon::new(inline_svg_asset_path(
    r#"<svg viewBox=\"0 0 24 24\"><path d=\"M4 4h16v16H4z\"/></svg>"#,
))
```

自定义业务图标不经过 optimizer；应用应自行决定如何随包分发。

## 10. 验证清单

图标系统相关修改完成前至少验证：

```bash
cargo test -p liora-icons
cargo test -p liora-icons-lucide
cargo test -p liora-icons-antd
cargo test -p liora-icons-ionic
cargo test -p liora-icons-tabler
cargo test -p liora-icons-carbon
cargo test -p liora-icons-material
cargo test -p liora-icons-optimizer
cargo check -p liora-gallery -p liora-docs
cargo run --release -p xtask -- package validate
```

如果修改了打包资源路径，还需要额外检查 release raw binary、portable archive 和 installer 中是否包含 `assets/liora-icons/<set>/<file>.svg`。
