"""Exercise the real SDK audit CLI against disposable Cargo/TOML fixtures."""

from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[1]
AUDIT = ROOT / "scripts/audit_sdk_policy.py"
BASE = (ROOT / "Cargo.toml").read_text().split("\n[patch.crates-io]")[0]
PIN = "743b465820bdfbea7a8105c98ef3dce9d12f01b6"
LEAF = "third_party/accesskit_atspi_common"
PATCH = '\n[patch.crates-io]\naccesskit_atspi_common = { path = "' + LEAF + '", version = "=0.19.1" }\n'


class SdkPolicyTests(unittest.TestCase):
    def check_case(self, name, expected, manifest=BASE, *, leaf=None, files=None):
        with tempfile.TemporaryDirectory(prefix="liora-policy-test-") as temporary:
            root = Path(temporary)
            (root / "Cargo.toml").write_text(manifest)
            shutil.copyfile(ROOT / "Cargo.lock", root / "Cargo.lock")
            for member in tomllib.loads(BASE)["workspace"]["members"]:
                directory = root / member
                directory.mkdir(parents=True)
                shutil.copyfile(ROOT / member / "Cargo.toml", directory / "Cargo.toml")
                readme = ROOT / member / "README.md"
                if readme.exists():
                    shutil.copyfile(readme, directory / "README.md")
            directory = root / LEAF
            directory.mkdir(parents=True)
            (directory / "Cargo.toml").write_text(
                leaf if leaf is not None else '[package]\nname = "accesskit_atspi_common"\nversion = "0.19.1"\n'
            )
            for relative, content in (files or {}).items():
                path = root / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                if content is None:
                    path.unlink()
                else:
                    path.write_text(content)
            result = subprocess.run(
                [sys.executable, str(AUDIT), str(root)],
                capture_output=True, text=True, check=False,
            )
            print(f"{name}: exit={result.returncode}")
            self.assertEqual(result.returncode, expected, result.stdout + result.stderr)

    def test_clean_controls(self):
        self.check_case("baseline", 0)
        self.check_case("approved-leaf", 0, BASE + PATCH)
        quoted = PATCH.replace("[patch.crates-io]", "[patch.'crates-io']")
        self.check_case("equivalent-approved-TOML", 0, BASE + quoted)

    def test_override_negative_controls(self):
        cases = {
            "wrong-name": PATCH.replace("accesskit_atspi_common =", "other ="),
            "wrong-path": PATCH.replace(LEAF, "third_party/elsewhere"),
            "wrong-version": PATCH.replace("=0.19.1", "=0.19.0"),
            "extra-entry": PATCH + 'serde = { path = "other" }\n',
            "gpui-registry": '\n[patch.crates-io]\ngpui = { path = "third_party/zed/crates/gpui" }\n',
            "gpui-official-source": '\n[patch."https://github.com/zed-industries/zed"]\ngpui = { path = "third_party/zed/crates/gpui" }\n',
            "quoted-forbidden": "\n[patch.'crates-io']\ngpui = { path = 'bad' }\n",
            "dotted-forbidden": "\n[patch]\n'crates-io'.gpui.path = 'bad'\n",
            "alternate-source": '\n[patch."https://example.invalid/index"]\naccesskit_atspi_common = { path = "' + LEAF + '" }\n',
            "replace": '\n[replace]\n"accesskit_atspi_common:0.19.1" = { path = "' + LEAF + '" }\n',
            "renamed": PATCH.replace('path =', 'package = "gpui", path ='),
            "unexpected-key": PATCH.replace('path =', 'optional = true, path ='),
            "unexpected-git": PATCH.replace('path =', 'git = "https://github.com/AccessKit/accesskit", path ='),
            "source-table": '\n[source.crates-io]\nreplace-with = "other"\n',
        }
        for name, patch in cases.items():
            with self.subTest(name=name):
                self.check_case(name, 1, BASE + patch)

    def test_pin_and_leaf_negative_controls(self):
        for name, manifest, leaf, files in [
            ("wrong-revision", BASE.replace(PIN, "a" * 40), None, None),
            ("gpui-fork", BASE.replace("https://github.com/yhyzgn/liora", "https://example.invalid/fork"), None, None),
            ("gpui-official-without-readiness", BASE.replace("https://github.com/yhyzgn/liora", "https://github.com/zed-industries/zed"), None, None),
            ("mixed-gpui-platform-source", BASE.replace('gpui_platform = { git = "https://github.com/yhyzgn/liora"', 'gpui_platform = { git = "https://github.com/zed-industries/zed"'), None, None),
            ("gpui-no-fallback", BASE.replace('version = "0.2.2", git =', 'git ='), None, None),
            ("wrong-leaf-package", BASE + PATCH, '[package]\nname = "gpui"\nversion = "0.19.1"\n', None),
            ("wrong-leaf-version", BASE + PATCH, '[package]\nname = "accesskit_atspi_common"\nversion = "0.19.0"\n', None),
            ("missing-leaf-manifest", BASE + PATCH, None, {LEAF + "/Cargo.toml": None}),
            ("malformed-leaf", BASE + PATCH, "[package", None),
            ("malformed-root", "[workspace", None, None),
        ]:
            with self.subTest(name=name):
                self.check_case(name, 1, manifest, leaf=leaf, files=files)

    def test_implicit_sdk_member_cannot_escape_source_policy(self):
        core_path = "crates/liora-core/Cargo.toml"
        core = (ROOT / core_path).read_text()
        manifest = (BASE + PATCH).replace('    "crates/liora-core",\n', "")
        override = core.replace("gpui.workspace = true", 'gpui = { path = "../../third_party/zed/crates/gpui" }')
        self.check_case("implicit-sdk-member-gpui-path", 1, manifest, files={core_path: override})
        self.check_case("implicit-sdk-member-patch", 1, manifest, files={core_path: core + PATCH})

    def test_selected_leaf_lock_controls(self):
        lock = (ROOT / "Cargo.lock").read_text()
        entry = 'name = "accesskit_atspi_common"\nversion = "0.19.1"\n'
        registry = entry + 'source = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "023da0e5097f46df7092d5280b02efb9bbf8d93298daeced42652463e357d636"\n'
        self.check_case("baseline-registry-lock", 0, files={"Cargo.lock": lock.replace(entry, registry)})
        cases = {
            "registry-leaf-selected": lock.replace(entry, registry),
            "wrong-selected-version": lock.replace(entry, entry.replace("0.19.1", "0.19.2")),
            "duplicate-selected-leaf": lock + '\n[[package]]\n' + entry,
            "unused-leaf-patch": lock + '\n[[patch.unused]]\n' + entry,
            "missing-selected-leaf": lock.replace(entry, 'name = "other"\nversion = "0.19.1"\n'),
            "malformed-lock": "[[package]",
            "missing-lock": None,
        }
        for name, content in cases.items():
            with self.subTest(name=name):
                self.check_case(name, 1, BASE + PATCH, files={"Cargo.lock": content})

    def test_sdk_and_config_negative_controls(self):
        core_path = "crates/liora-core/Cargo.toml"
        core = (ROOT / core_path).read_text()
        cases = {
            "sdk-patch": {core_path: core + PATCH},
            "sdk-path-gpui": {core_path: core.replace("gpui.workspace = true", 'gpui = { path = "../../third_party/zed/crates/gpui" }')},
            "sdk-renamed-gpui": {core_path: core + '\n[dependencies.renamed]\npackage = "gpui"\npath = "../../third_party/zed/crates/gpui"\n'},
            "sdk-not-publishable": {core_path: core.replace("publish = true", "publish = false")},
            "sdk-wrong-license": {core_path: core.replace("../../LICENSE.md", "other")},
            "sdk-empty-readme": {"crates/liora-core/README.md": ""},
            "config-source": {".cargo/config.toml": '[source.crates-io]\nreplace-with = "other"\n'},
            "config-patch": {".cargo/config": PATCH},
            "config-paths": {".cargo/config.toml": 'paths = ["other"]\n'},
            "member-config-source": {"crates/.cargo/config.toml": '[source.crates-io]\nreplace-with = "other"\n'},
            "malformed-config": {".cargo/config.toml": "[source"},
        }
        for name, files in cases.items():
            with self.subTest(name=name):
                self.check_case(name, 1, BASE + PATCH, files=files)


if __name__ == "__main__":
    unittest.main()
