# Liora Native Package Signing Policy

Liora remains pure Rust + GPUI native. Signing and notarization only apply to generated installer/package artifacts; they do not introduce Tauri, WebView, HTML/CSS, DOM, or browser runtime architecture.

## Enforcement mode

`cargo run -p xtask -- package release-readiness` supports two modes:

- default: local/preview builds may be unsigned; missing signing/notarization inputs are reported as warnings;
- strict: set `LIORA_REQUIRE_SIGNING=true` to fail readiness when the current platform inputs are missing;
- RPM releases: `GITHUB_REF=refs/tags/v*` or `LIORA_PACKAGE_CHANNEL=release` always requires a signing key, even when `LIORA_ALLOW_UNSIGNED_RELEASE=true`. macOS/Windows retain their existing opt-in signing gates.

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

### RPM build and release signing

`xtask package` passes `LIORA_RPM_SIGNING_KEY` (a path to an ASCII-armored, unencrypted OpenPGP private key) to the existing `cargo-generate-rpm --signing-key` backend. CI pins backend version `0.21.0`. Signing happens while generating the RPM, **before** package manifests and SHA-256 checksums are calculated. No `rpmsign` dependency, distro detection, verification bypass, or private key on the user's installation machine is needed.

Use a dedicated RSA signing key (3072 bits or stronger, OpenPGP v4) for compatibility with RPM 4.x and RPM 6.x. Keep this key stable between releases; changing it requires a new explicit trust decision. The backend does not accept an encrypted-key passphrase: use a dedicated unencrypted signing export stored only in protected release secrets, not a personal key. Keep an offline backup and revoke/rotate it if compromised. Do not generate a new identity for every release.

#### 1. Generate and export the publisher key

Use a trusted owner-controlled machine with GnuPG 2.4+ and Bash. Generate this identity once, not in CI or during end-user installation. Use a dedicated Bash session; keep it open for the following steps. Replace the example email with a release contact you control.

```bash
set -euo pipefail
umask 077
RPM_SIGNING_DIR="$(mktemp -d "$HOME/liora-rpm-signing.XXXXXX")"
export GNUPGHOME="$RPM_SIGNING_DIR/gnupg"
mkdir -m 700 "$GNUPGHOME"
RPM_KEY_UID="Liora RPM Release Signing <releases@your-domain.example>"

gpg --batch --pinentry-mode loopback --passphrase '' \
  --quick-generate-key "$RPM_KEY_UID" rsa4096 sign 2y
RPM_FINGERPRINT="$(gpg --batch --with-colons --list-secret-keys "$RPM_KEY_UID" |
  awk -F: '$1 == "fpr" { print $10; exit }')"
[[ "$RPM_FINGERPRINT" =~ ^[[:xdigit:]]{40}$ ]]
gpg --batch --armor --export-secret-keys "$RPM_FINGERPRINT" \
  > "$RPM_SIGNING_DIR/rpm-signing-key.asc"
gpg --batch --armor --export "$RPM_FINGERPRINT" \
  > "$RPM_SIGNING_DIR/RPM-GPG-KEY-liora"
gpg --show-keys --with-fingerprint "$RPM_SIGNING_DIR/RPM-GPG-KEY-liora"
printf 'Fingerprint: %s\nSigning directory: %s\n' "$RPM_FINGERPRINT" "$RPM_SIGNING_DIR"
```

The explicit `rsa4096 sign 2y` arguments create a signing-only RSA-4096 primary key with a two-year expiry. The isolated GnuPG home avoids selecting a personal key; the 40-hex-digit check rejects a non-v4 fingerprint. Do not truncate a longer fingerprint or substitute a short key ID. The private export has no passphrase because the current backend cannot unlock encrypted keys. `umask 077` protects the new directory/files; keep all private material outside the checkout, logs, chat, and release assets.

#### 2. Configure GitHub Actions

Install GitHub CLI (`gh`) and authenticate with an account allowed to manage the target repository's Actions secrets and variables. Inspect the public-key fingerprint above before setting the two separate values; change `GH_REPO` for a fork.

| GitHub Actions setting | Value |
| --- | --- |
| Repository **Secret** `LIORA_RPM_SIGNING_PRIVATE_KEY` | Entire contents of `rpm-signing-key.asc`, including `BEGIN/END PGP PRIVATE KEY BLOCK`; not a file path or public key |
| Repository **Variable** `LIORA_RPM_SIGNING_FINGERPRINT` | Full 40-hex-digit **primary-key** fingerprint; public, not a short key ID or a separately generated random value |

```bash
gh auth login
GH_REPO="yhyzgn/liora" # Change this for your fork.
gh secret set LIORA_RPM_SIGNING_PRIVATE_KEY --repo "$GH_REPO" \
  < "$RPM_SIGNING_DIR/rpm-signing-key.asc"
gh variable set LIORA_RPM_SIGNING_FINGERPRINT --repo "$GH_REPO" \
  --body "$RPM_FINGERPRINT"
gh secret list --repo "$GH_REPO"
gh variable get LIORA_RPM_SIGNING_FINGERPRINT --repo "$GH_REPO"
```

The file is sent to `gh secret set` through standard input; never put private key text in `--body`, a shell command, or a `.env` file in the repository. `gh secret list` verifies the secret's name exists, not its value. Compare the value returned by `gh variable get` with the fingerprint displayed on your trusted generation machine.

Web UI alternative: open **Settings → Secrets and variables → Actions**. Under **Secrets → New repository secret**, paste the entire armored private-key file under `LIORA_RPM_SIGNING_PRIVATE_KEY`. Under **Variables → New repository variable**, paste only the 40-digit fingerprint under `LIORA_RPM_SIGNING_FINGERPRINT`. Do not reverse the two fields. The current package job is not bound to a GitHub Environment, so environment-only secrets/variables are not available to it; use repository settings or explicitly bind the job before adopting environment-scoped credentials. Repository secrets alone do not add reviewer approval: protect release tags, workflow write access, and the owner account.

Tagged RPM releases already enforce signing; you do not need to set `LIORA_REQUIRE_SIGNING=true` just to enable them. That optional strict setting also affects macOS/Windows readiness, so configure their signing inputs before enabling it globally.

#### 3. Publish the public identity and back it up

Publish `RPM-GPG-KEY-liora` and the full fingerprint through an owner-controlled channel whose credentials are independent of GitHub, for example a separately managed release website. A fingerprint generated from this trusted key is not a random secret: users compare it with the downloaded public key before trusting it. CI compares its pinned variable with the private key's fingerprint, but publishing both public files on the same GitHub Release alone does not establish independent publisher trust.

Keep an encrypted offline backup of `rpm-signing-key.asc`, the public key, and the automatically generated revocation certificate under `$GNUPGHOME/openpgp-revocs.d/`. Confirm you can recover the backup before removing the local working directory. Never publish the private key; keep the revocation certificate private until you deliberately revoke this identity. The same signing identity must survive ordinary release cycles; a fresh key for every release forces users to make a new trust decision every time.

#### 4. Sign a local build

In the same session (or with `LIORA_RPM_SIGNING_KEY` pointing to a restored private export), install the pinned backend and build/package Gallery:

```bash
cargo install cargo-generate-rpm --version 0.21.0 --locked
export LIORA_RPM_SIGNING_KEY="$RPM_SIGNING_DIR/rpm-signing-key.asc"
cargo run -p xtask -- package build --app gallery
cargo run -p xtask -- package ci --app gallery --format rpm --skip-build
```

`LIORA_RPM_SIGNING_KEY` is a local file path, unlike the GitHub Secret, which contains the file's text. End users installing downloaded RPMs need only the verified public key, never any private key.

#### CI verification

For tagged GitHub releases, configure protected secret `LIORA_RPM_SIGNING_PRIVATE_KEY` with the armored private key and repository variable `LIORA_RPM_SIGNING_FINGERPRINT` with the **full 40-hex-character OpenPGP v4 fingerprint**, published to users over an independent trusted channel. `package.yml` compares the secret's actual key fingerprint against this separately provisioned value before exporting any public key or building RPMs; it then writes the key under a private `$RUNNER_TEMP` directory and writes `RPM-GPG-KEY-liora` plus the verified fingerprint to `target/packages/`. Both font variants must pass `rpmkeys --define '_pkgverify_level all' --checksig` against an isolated RPM keyring containing only the exported public key. The private material is removed even on failure and is never uploaded. Both public files are release assets covered by `SHA256SUMS.txt`.

`LIORA_REQUIRE_SIGNING=true` also provisions and verifies the RPM key for non-tag Gallery packaging. Workflow dry-runs and Docs-only raw-binary runs do not load RPM private keys; readiness receives the existing `--dry-run` and `--app` arguments. Actual RPM generation enforces signing independently, so those readiness arguments cannot permit an unsigned release RPM. The manifest marks only the exact RPM produced by the successful signed backend invocation as signed, never unrelated files left from earlier builds.

Missing signing credentials, an invalid key, an unverifiable signature, or missing font variants stop the release. Preview RPMs may remain unsigned and are not suitable for signature-enforcing installation. The already published v0.5.0 RPMs are not retroactively signed by this change.

#### Renewal and rotation

Before expiry, open a dedicated Bash session with the original private key imported into its isolated GnuPG home. Set `RPM_SIGNING_DIR`, `GNUPGHOME`, `RPM_FINGERPRINT`, and `GH_REPO` to the trusted saved values from setup. Renew the same primary key, then export and upload the updated secret:

```bash
gpg --batch --quick-set-expire "$RPM_FINGERPRINT" 2y
gpg --batch --armor --export-secret-keys "$RPM_FINGERPRINT" \
  > "$RPM_SIGNING_DIR/rpm-signing-key.asc"
gpg --batch --armor --export "$RPM_FINGERPRINT" \
  > "$RPM_SIGNING_DIR/RPM-GPG-KEY-liora"
gh secret set LIORA_RPM_SIGNING_PRIVATE_KEY --repo "$GH_REPO" \
  < "$RPM_SIGNING_DIR/rpm-signing-key.asc"
```

Renewal keeps the primary-key fingerprint unchanged, so do not change the pinned variable. Redistribute the refreshed public key through the independent channel, tell users to re-import it, and refresh the encrypted offline backup. If the key is compromised or intentionally replaced instead, revoke the old identity, generate a new dedicated key, update **both** the private-key secret and fingerprint variable, and independently announce the new fingerprint plus the old key's revocation. Never silently replace the trust anchor.

### RPM install and update trust

A checksum downloaded beside a package proves consistency, not publisher identity. The RPM's signed OpenPGP primary-key fingerprint must match the full fingerprint published by the owner through an independent trusted channel. The attached fingerprint asset and a public key on the same GitHub release are not independent trust anchors.

After downloading the public key and the desired signed RPM from the same release:

```bash
gpg --show-keys --with-fingerprint RPM-GPG-KEY-liora
# Compare the complete fingerprint with the independently published value.
sudo rpm --import RPM-GPG-KEY-liora
rpmkeys --define '_pkgverify_level all' --checksig ./liora-gallery.rpm
```

Only continue if signature verification succeeds. Reusing a trusted publisher key means this import is normally a one-time step, not something to repeat silently for each update.

| System | Install or update the verified local RPM |
| --- | --- |
| Fedora, RHEL, Rocky, AlmaLinux, other DNF systems | `sudo dnf --setopt=localpkg_gpgcheck=1 install ./liora-gallery.rpm` |
| openSUSE / SLES | `sudo zypper install ./liora-gallery.rpm` |
| RPM-only systems with dependencies already satisfied | `sudo rpm -Uvh ./liora-gallery.rpm` |

Fedora 45 / RPM 6 enforce verifiable signatures by default. Older RPM systems can accept unsigned packages or skip local-package checks by default, so keep the explicit signature check and DNF `localpkg_gpgcheck=1` rather than depending on a distro version. An RPM signature does not guarantee compatible dependency names or ABI on every distribution; native install smoke remains required.

`liora-updater` continues to verify release SHA-256 and return a **manual** RPM installation plan. It never imports keys, elevates privileges, self-signs downloaded packages, or disables system verification. Applications must present the trust and install action to the user.

### Fedora local autosigning

Fedora's `/usr/lib/rpm/rpm-setup-autosign` creates a host/user-specific passwordless key and configures **rpmbuild**. It does not automatically sign this project's `cargo-generate-rpm` output. Use it only for local build testing, importing the public key as instructed by the tool. For the native backend, explicitly export the generated private key and set `LIORA_RPM_SIGNING_KEY`; on systems without that helper, create/export a dedicated OpenPGP key with GnuPG instead. Do not automatically re-sign downloaded release RPMs: that would replace publisher provenance with local trust.

Sources: [Fedora 45 signature enforcement](https://fedoraproject.org/wiki/Changes/Enforcing_signature_checking_by_default), [rpm-setup-autosign](https://rpm.org/docs/6.0.x/man/rpm-setup-autosign.1), [RPM verification policy](https://rpm.org/docs/6.0.x/man/rpm-config.5), [cargo-generate-rpm signing implementation](https://github.com/cat-in-136/cargo-generate-rpm/blob/v0.21.0/src/main.rs), [openSUSE Zypper](https://doc.opensuse.org/documentation/tumbleweed/zypper/).

Setup references: [GnuPG key management](https://www.gnupg.org/documentation/manuals/gnupg/OpenPGP-Key-Management.html), [GitHub CLI secret set](https://cli.github.com/manual/gh_secret_set), [GitHub CLI variable set](https://cli.github.com/manual/gh_variable_set).
