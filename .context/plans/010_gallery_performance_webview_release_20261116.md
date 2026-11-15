# 010 Gallery Performance and 0.6.1 Release Plan - 2026-11-16

- 状态: 进行中
- 任务: `.context/tasks/010_gallery_performance_webview_release_20261116.md`

## Scope and constraints

- Investigate Gallery performance report #2 without claiming a Gallery fix when no reproducible details are available.
- Keep the independently verified Docs live-demo prefix-index optimization accurately described; do not present it as a Gallery fix.
- Prepare Liora 0.6.1, including the standalone opt-in `liora-webview` crate. Default SDK, Gallery, Docs, and native controls remain WRY-free.
- Follow protected release workflow requirements. Do not publish or tag until local acceptance gates pass and the user explicitly requests release execution.

## Phases

1. Record issue #2 evidence and preserve its open status pending reproducible steps.
2. Update package versions, all affected READMEs/docs, changelog, release notes, and release-candidate checklist.
3. Run focused documentation, manifest, packaging, formatting, and context validation; record exact results.
4. Commit and push the prepared changes, then monitor GitHub Actions CI.
5. After CI passes and required protected environments/signing credentials are verified, push the release tag; verify app/SDK workflows, crate publication, assets, and SDK installation. Reply on issue #2 with verified findings and request missing reproduction details.

## Release boundaries

- `liora-webview` requires GPUI child-window embedding, a system engine, and platform runtime dependencies. Linux validation is limited to X11/XWayland with GTK3 and WebKitGTK 4.1; macOS/Windows runtime compatibility is not claimed.
- Release publication remains protected-environment work. Never infer success from a pushed tag; verify completed workflow and registry evidence.
