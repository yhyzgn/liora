<claude-mem-context>
# Memory Context

# claude-mem status

This project has no memory yet. The current session will seed it; subsequent sessions will receive auto-injected context for relevant past work.

Memory injection starts on your second session in a project.

`/learn-codebase` is available if the user wants to front-load the entire repo into memory in a single pass (~5 minutes on a typical repo, optional). Otherwise memory builds passively as work happens.

Live activity: http://localhost:37777
How it works: `/how-it-works`

This message disappears once the first observation lands.
</claude-mem-context>

# Project Development Rules

## Official-source-first rule

- For all GPUI, Zed, platform integration, packaging, release, CI, updater, and third-party API work, verify behavior against official upstream sources before implementing.
- Preferred evidence order: official repository source code and examples, official documentation, official release notes, then local verified behavior.
- Do not guess APIs from memory, do not invent unsupported patterns, and do not use unofficial forks such as `open-gpui`. Liora must stay on official `zed-industries/zed` GPUI unless the owner explicitly approves a temporary local patch for app-only validation.
- When upstream behavior is unclear, inspect the exact dependency revision in `Cargo.lock` or the official upstream commit and document the rationale in code comments, tests, or commit notes as appropriate.
- Keep implementations pure Rust + native GPUI. Do not introduce WebView/Tauri/HTML/CSS/DOM runtime paths.
