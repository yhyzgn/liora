# Liora Native Package Signing Policy

Liora remains pure Rust + GPUI native. Signing and notarization only apply to generated installer/package artifacts; they do not introduce Tauri, WebView, HTML/CSS, DOM, or browser runtime architecture.

## Enforcement mode

`cargo run -p xtask -- package release-readiness` supports two modes:

- default: unsigned builds are allowed, but missing signing/notarization inputs are reported as warnings;
- strict: set `LIORA_REQUIRE_SIGNING=true` to fail the release readiness gate when the current platform signing inputs are missing.

## macOS inputs

Required for strict signed macOS release gates:

- `LIORA_MACOS_CODESIGN_IDENTITY` — Developer ID Application identity used by `codesign`;
- `LIORA_MACOS_NOTARY_APPLE_ID` — Apple ID for `notarytool`;
- `LIORA_MACOS_NOTARY_TEAM_ID` — Apple Developer Team ID;
- `LIORA_MACOS_NOTARY_PASSWORD` — app-specific password or keychain credential for notarization.

Expected release sequence once credentials are available:

1. build `.app` and `.dmg` with `cargo run -p xtask -- package ci --all-apps --format platform-defaults`;
2. sign `.app` bundles and `.dmg` artifacts with `codesign`;
3. submit `.dmg` artifacts with `xcrun notarytool`;
4. staple notarization tickets with `xcrun stapler`;
5. rerun package smoke and install-smoke plan.

## Windows inputs

Required for strict signed Windows release gates:

- `LIORA_WINDOWS_SIGNTOOL_CERT_PATH` — certificate/PFX path or CI materialized secret path;
- `LIORA_WINDOWS_SIGNTOOL_CERT_PASSWORD` — certificate password;
- `LIORA_WINDOWS_TIMESTAMP_URL` — timestamp server URL, for example a vendor-approved RFC 3161 endpoint.

Expected release sequence once credentials are available:

1. build NSIS/MSI artifacts with the release tag numeric version;
2. sign `.exe` and `.msi` artifacts with `signtool sign /fd SHA256 /tr <timestamp> /td SHA256`;
3. verify signatures with `signtool verify /pa`;
4. rerun package smoke and install-smoke plan.

## Linux

Linux installer formats are not blocked by platform code-signing credentials in the current policy. Release integrity is provided by generated SHA-256 checksums, package manifests, and GitHub Release artifact provenance. Future work may add minisign/cosign, repository metadata signing, or distro-specific signing after owner policy is decided.

## GitHub Secrets mapping

Suggested secret names mirror the environment variables above. The workflow should map secrets to these variables only in protected release environments, not on untrusted pull requests.
