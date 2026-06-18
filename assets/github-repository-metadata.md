# GitHub Repository Metadata for Liora

This file contains the recommended GitHub About metadata for discoverability and SEO. It is documentation only; repository admins must apply these values in the GitHub UI or with an authorized GitHub API/CLI flow.

## Repository description

Use this as the GitHub repository **Description**:

> Pure Rust + GPUI native enterprise UI component library for desktop apps — Element Plus-inspired components, charts, docs, tray integration, and installer packaging.

Shorter fallback if GitHub UI space feels tight:

> Pure Rust + GPUI native enterprise UI components for desktop apps, with charts, docs, tray integration, and installer packaging.

## Website / homepage

Recommended homepage value before a public docs site exists:

```text
https://github.com/yhyzgn/liora#readme
```

If a public project site or hosted docs page is later published, replace the homepage with that URL and keep the README link in the project description/docs.

## Topics

GitHub topics make repositories easier to find by subject area. GitHub Docs currently specify that topic names should use lowercase letters, numbers, and hyphens, be 50 characters or less, and use no more than 20 topics. Source: https://docs.github.com/articles/classifying-your-repository-with-topics

Recommended topic set, ordered by search intent:

```text
rust
gpui
native-ui
desktop-app
ui-components
component-library
rust-ui
gui
enterprise-ui
element-plus
charts
system-tray
installer
packaging
markdown-renderer
code-editor
dark-theme
cross-platform
native-desktop
rust-desktop
```

## SEO rationale

- **rust**, **rust-ui**, **rust-desktop**: captures language-specific discovery intent.
- **gpui**: targets the exact native UI framework audience.
- **native-ui**, **native-desktop**, **desktop-app**, **gui**: covers broader desktop UI searches.
- **ui-components**, **component-library**, **enterprise-ui**: communicates the library category and enterprise use case.
- **element-plus**: connects the API inspiration and component taxonomy.
- **charts**, **system-tray**, **installer**, **packaging**, **markdown-renderer**, **code-editor**, **dark-theme**: indexes the most differentiated capabilities beyond basic components.
- **cross-platform**: aligns with the packaging matrix and native app release path.

## Suggested social preview text

If preparing a release announcement, use:

> Liora brings Element Plus-style enterprise UI coverage to pure Rust + GPUI native desktop apps: components, charts, docs, tray integration, and installer packaging without a WebView or browser runtime.

## Boundaries to keep out of metadata

Avoid these claims until owner policy changes or a public release exists:

- Do not call Liora open source while the license remains `LicenseRef-Liora`.
- Do not imply crates are published unless the owner enables publishing.
- Do not imply official GPUI/Zed affiliation.
- Do not advertise Tauri, WebView, HTML/CSS, DOM, or browser runtime support.
