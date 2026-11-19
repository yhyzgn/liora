"""Audit SDK metadata and the single application-root AccessKit backport exception."""

from pathlib import Path
import sys
import tomllib

ZED = "https://github.com/zed-industries/zed"
REV = "f0d896f2c18a543335eb155f729d36b624695591"
LEAF = "third_party/accesskit_atspi_common"
SDK_CRATES = "liora-theme liora-icons-optimizer liora-tray liora-packager liora-updater liora-core liora-icons liora-icons-lucide liora-icons-antd liora-icons-ionic liora-icons-tabler liora-icons-carbon liora-icons-material liora-components liora liora-webview".split()
APPROVED_PATCH = {
    "crates-io": {
        "accesskit_atspi_common": {"path": LEAF, "version": "=0.19.1"},
    },
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load(path: Path) -> dict:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def check_dependencies(manifest: dict, *, workspace: bool, publish: bool) -> None:
    scopes = [manifest, *manifest.get("target", {}).values()]
    for scope in scopes:
        for kind in ("dependencies", "dev-dependencies", "build-dependencies"):
            for name, dependency in scope.get(kind, {}).items():
                package = dependency.get("package", name) if isinstance(dependency, dict) else name
                require(package != "open-gpui" and name != "open-gpui", "GPUI forks are forbidden")
                if name not in ("gpui", "gpui_platform") and package not in ("gpui", "gpui_platform"):
                    continue
                require(name == package, "renamed GPUI dependencies are forbidden")
                if not workspace:
                    require(isinstance(dependency, dict) and dependency.get("workspace") is True,
                            f"{name} must inherit the official workspace dependency")
                    require(set(dependency) <= {"workspace", "features", "default-features", "optional"},
                            f"{name} must not override its workspace source")
                require(not (publish and kind != "dev-dependencies" and name == "gpui_platform"),
                        "gpui_platform must stay app/example-only")


def audit(root: Path) -> None:
    root = root.resolve()
    manifest = load(root / "Cargo.toml")
    patch = manifest.get("patch", {})
    require(patch == {} or patch == APPROVED_PATCH, "only the exact AccessKit leaf patch is allowed")
    require("replace" not in manifest and "source" not in manifest and "paths" not in manifest,
            "workspace replacement/source overrides are forbidden")
    dependencies = manifest["workspace"]["dependencies"]
    official = {"git": ZED, "rev": REV, "default-features": False}
    require(dependencies["gpui"] == {**official, "version": "0.2.2"},
            "gpui must retain the exact official revision and 0.2.2 registry fallback")
    require(dependencies["gpui_platform"] == official,
            "gpui_platform must retain the same official revision without a registry version")
    check_dependencies(manifest["workspace"], workspace=True, publish=False)

    members = manifest["workspace"]["members"]
    protected = [f"crates/{crate}" for crate in SDK_CRATES]
    manifests = [root / member / "Cargo.toml" for member in dict.fromkeys([*members, *protected])]
    require(all(path.resolve().is_relative_to(root) for path in manifests), "workspace members must stay in the repository")
    if patch:
        leaf = root / LEAF
        require(leaf.resolve() == leaf, "vendored leaf must not redirect through a symlink")
        package = load(leaf / "Cargo.toml")["package"]
        require(package["name"] == "accesskit_atspi_common" and package["version"] == "0.19.1",
                "vendored leaf identity must be accesskit_atspi_common 0.19.1")
        require(LEAF in manifest["workspace"].get("exclude", []), "vendored leaf must be excluded from the SDK workspace")
        require(leaf / "Cargo.toml" not in manifests, "vendored leaf must not be a workspace member")
        manifests.append(leaf / "Cargo.toml")
        lock = load(root / "Cargo.lock")
        selected = [entry for entry in lock["package"] if entry["name"] == "accesskit_atspi_common"]
        require(len(selected) == 1 and selected[0]["version"] == "0.19.1"
                and "source" not in selected[0] and "checksum" not in selected[0],
                "active lock entry must select the local accesskit_atspi_common 0.19.1")
        require(not any(entry.get("name") == "accesskit_atspi_common"
                        for entry in lock.get("patch", {}).get("unused", [])),
                "AccessKit leaf patch must not be unused")

    config_directories = {root}
    for path in manifests:
        member = load(path)
        require(not any(key in member for key in ("patch", "replace", "source", "paths")),
                f"{path}: member source overrides are forbidden")
        publish = member.get("package", {}).get("publish") is True
        check_dependencies(member, workspace=False, publish=publish)
        for directory in (path.parent, *path.parent.parents):
            if directory.is_relative_to(root):
                config_directories.add(directory)
    for directory in config_directories:
        for name in ("config", "config.toml"):
            path = directory / ".cargo" / name
            if path.exists():
                config = load(path)
                require(not any(key in config for key in ("patch", "replace", "source", "paths")),
                        f"{path}: Cargo configuration source overrides are forbidden")

    for crate in SDK_CRATES:
        path = root / "crates" / crate / "Cargo.toml"
        package = load(path)["package"]
        require(package["publish"] is True, f"{crate} should be publishable to crates.io")
        require(package["license-file"] == "../../LICENSE.md", f"{crate}: incorrect license file")
        require(package["repository"] == "https://github.com/yhyzgn/liora", f"{crate}: incorrect repository")
        require(package.get("readme") == "README.md", f"{crate} should publish its crate-local README.md")
        require(bool(package.get("description")), f"{crate}: description is required")
        require(bool((path.parent / "README.md").read_text(encoding="utf-8").strip()),
                f"{crate} README.md should not be empty")


if __name__ == "__main__":
    try:
        audit(Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parents[1])
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f"SDK policy FAILED: {error}", file=sys.stderr)
        sys.exit(1)
    print("crates.io SDK metadata and source policy OK")
