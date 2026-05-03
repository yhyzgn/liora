# P6 Engineering — 工程化交付

> 上游: `.prompt/P5-advanced.md`

## 目标

将 Aura 从"能用的组件库"升级为"专业的企业级产品"。

## 任务清单

### 1. aura-gallery 完善
- [ ] 每个组件的交互式 Demo (不只是静态展示)
- [ ] 左侧导航 sidebar (可折叠分类树)
- [ ] 主题切换按钮 (light/dark 一键切换)
- [ ] 组件搜索 (输入过滤)
- [ ] 窗口标题 "Aura Gallery — Native Component Library"

### 2. docs 文档站
- [ ] Vitepress 搭建 (在 `apps/docs/`)
- [ ] 首页 (hero + 特色 + 快速开始)
- [ ] 快速上手指南
- [ ] 全局配置文档
- [ ] 主题定制指南 (自定义 Design Token)
- [ ] 每个组件 API 文档页面:
  - Props (Builder 方法表格)
  - Events (回调表格)
  - Slots (子元素说明)
  - 代码示例 + 截图
- [ ] 从 Element-Plus 迁移指南
- [ ] 暗黑模式使用指南

### 3. 测试体系
- [ ] 单元测试: Theme 计算, 工具函数
- [ ] 组件测试: GPUI TestApp 集成测试
- [ ] 视觉回归: 截图对比 (参考 Zed 的测试方案)
- [ ] `cargo test` 全部通过

### 4. CI/CD
- [ ] GitHub Actions workflow:
  - `cargo check` (全平台)
  - `cargo clippy`
  - `cargo test`
  - `cargo doc`
- [ ] PR 模板
- [ ] Issue 模板

### 5. 发布
- [ ] crates.io 发布策略 (aura-core → aura-theme → aura-icons → aura-components)
- [ ] CHANGELOG.md
- [ ] 版本号规范 (SemVer)
- [ ] LICENSE (Apache-2.0, 与 GPUI 一致)

### 6. 社区建设 (可选)
- [ ] CONTRIBUTING.md (贡献指南)
- [ ] CODE_OF_CONDUCT.md
- [ ] GitHub 仓库 README (badge, 截图, 快速开始)
- [ ] 组件开发规范 (如何贡献新组件)
