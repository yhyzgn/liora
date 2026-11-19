# Linux AT-SPI Busy/Disabled leaf backport

## Explicit policy amendment and source

The former blanket ban on workspace `[patch.crates-io]` is amended for exactly one application-root override, not reinterpreted as a typo. `third_party/accesskit_atspi_common` is the complete official `accesskit_atspi_common 0.19.1` crate plus two upstream mapping backports and a regression test. All other crate/source overrides, GPUI forks, local GPUI paths and official-Zed source patches remain forbidden. Publishable SDK manifests remain unchanged and contain no patches or local GPUI paths. GPUI remains official Zed `f0d896f2c18a543335eb155f729d36b624695591` with registry fallback `0.2.2`; AccessKit versions and dependency edges are unchanged.

- [Official 0.19.1 crate](https://crates.io/crates/accesskit_atspi_common/0.19.1): archive SHA-256 `023da0e5097f46df7092d5280b02efb9bbf8d93298daeced42652463e357d636`, source commit [`c88605b96d04431f9c3c792464a0f2f253480e94`](https://github.com/AccessKit/accesskit/tree/c88605b96d04431f9c3c792464a0f2f253480e94/platforms/atspi-common).
- [Upstream #780](https://github.com/AccessKit/accesskit/commit/58412b89d64a00b2f79563c2ae8dc6b9e06cb68e): only its Busy-state hunk, not the additional Document interface.
- [Upstream #788](https://github.com/AccessKit/accesskit/commit/6ee0558b6315b3ef1594db24ce45a030ecac7cb5): Disabled controls no longer report Enabled/Sensitive; do not emulate this with ReadOnly or change roles/names.
- Production change is two `src/node.rs` hunks, 9 added/4 removed lines. The additional `src/adapter.rs` test checks same-node forward/reverse availability and Busy updates. Original `LICENSE-APACHE`, `LICENSE-MIT` and `LICENSE.chromium` are retained from the same official source commit.

The leaf is excluded from workspace members and SDK publication targets. The allowlist authenticates the manifest shape and package identity, not arbitrary future source edits; review the production diff and retain upstream provenance when changing this directory. No publication, tag or signing operation is authorized by this exception.

## Downstream application setup

**An SDK update alone does not deliver this fix to arbitrary hosts.** Cargo reads patches from the active application/workspace root, not dependency manifests. Its patch applies across that root's resolved dependency graph but is not inherited when Liora is used as a dependency. See [Cargo's overriding-dependencies reference](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section).

Acquire the leaf from the exact Liora source revision being consumed, not a moving branch or a registry package with the same version. From an existing Liora checkout at the reviewed revision, record the full commit ID, transfer the complete directory including licenses and verify its file hashes; for example, from the downstream application's root:

```bash
# LIORA_CHECKOUT is an existing checkout containing the reviewed backport.
# LIORA_REV is its full immutable 40-hex commit, recorded with your application.
# Do not substitute the GPUI revision: this is the Liora source revision.
test "${#LIORA_REV}" -eq 40
mkdir -p third_party
git -C "$LIORA_CHECKOUT" archive "$LIORA_REV" third_party/accesskit_atspi_common \
  | tar -x -C .
```

Keep the existing official-GPUI patch in the application's root `Cargo.toml`, and add the leaf in the **same** table (do not create duplicate tables):

```toml
[patch.crates-io]
gpui = { git = "https://github.com/zed-industries/zed", rev = "f0d896f2c18a543335eb155f729d36b624695591", version = "=0.2.2" }
accesskit_atspi_common = { path = "third_party/accesskit_atspi_common", version = "=0.19.1" }
```

Retain the direct GPUI/gpui_platform dependencies and platform features from the SDK version's adoption guide. After resolving the application lockfile, inspect `cargo metadata --format-version 1` or `cargo tree -i accesskit_atspi_common`: the selected package must point to this application's actual vendor directory with `source = null`, not merely say `0.19.1`. Without the explicit leaf patch it still resolves to registry `0.19.1`, which lacks these mappings. Record the resolved source and recheck native states in the host's own controls; compilation and the version string are not native accessibility evidence.

## Enforced checks and evidence boundary

Ordinary CI and SDK release preflight both invoke the same implementation and subprocess fixtures:

```bash
python scripts/audit_sdk_policy.py
python scripts/test_audit_sdk_policy.py -v
```

Clean controls allow the baseline without patches (for removal/rollback) and the sole exact leaf exception. Negative controls exercise disallowed names, paths, versions, entries, sources, TOML spelling equivalents, renamed packages, replacements, config overrides, malformed/missing manifests and wrong GPUI revisions through the actual CLI. Original SDK metadata and independent consumer compilation checks remain in place.

With the leaf override present, this same lightweight audit also parses `Cargo.lock`: exactly one active `accesskit_atspi_common 0.19.1` entry must have no registry/git source or checksum, and the patch must not appear under `patch.unused`. Wrong/duplicate/missing/registry-selected entries and malformed/missing lockfiles fail actual CLI negative controls; baseline without patches may retain the original registry lock. All protected SDK manifests are audited even if removed from the explicit members list while still reachable as workspace path dependencies.

CI additionally copies the excluded leaf to a temporary standalone root, seeds its lockfile from the workspace resolution and runs the regression explicitly after workspace checks have fetched the dependencies:

```bash
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
cp -R third_party/accesskit_atspi_common "$tmp/leaf"
cp Cargo.lock "$tmp/leaf/Cargo.lock"
cargo test --offline --manifest-path "$tmp/leaf/Cargo.toml" --all-features --lib
```

The original frozen Button native receipt reported unavailable controls as Enabled/Sensitive and omitted Busy despite verified SDK/GPUI forwarding. The unchanged leaf candidate was separately verified with both canonical apps and a same-SDK-node dynamic AT-SPI probe; that proof concerns the explicit patched graph, not all downstream hosts, screen readers, macOS or Windows. The existing release consumer smoke is compilation evidence, not proof that it inherited this patch.

Remove the root override and vendored directory once the pinned stack can resolve a compatible official release with both fixes; rerun source policy, dependency-resolution and native state gates together. Do not upgrade GPUI/AccessKit or relax publication protections merely to retire the exception. SDK Markdown/PdfReader audit deferral remains unchanged.
