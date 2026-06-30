#!/usr/bin/env python3
"""Runtime smoke verification for Liora release artifacts.

The script is intentionally runner-safe: it runs GUI binaries only long enough to
prove that process startup does not immediately fail, treats a timeout as a
successful long-running GUI launch, and cleans up temporary install locations.
"""

from __future__ import annotations

import argparse
import csv
import json
import os
import platform as py_platform
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Iterable, Sequence

TIMEOUT_SECONDS = 15


@dataclass
class CheckResult:
    platform: str
    artifact: str
    kind: str
    action: str
    status: str
    notes: str
    duration_seconds: float


def rel(path: Path, root: Path) -> str:
    try:
        return path.resolve().relative_to(root.resolve()).as_posix()
    except ValueError:
        return path.as_posix()


def run_command(
    command: Sequence[str],
    *,
    timeout: int = TIMEOUT_SECONDS,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
) -> tuple[str, str, float]:
    started = time.monotonic()
    try:
        proc = subprocess.run(
            list(command),
            cwd=str(cwd) if cwd else None,
            env=env,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
            check=False,
        )
    except subprocess.TimeoutExpired as error:
        elapsed = time.monotonic() - started
        stdout = error.stdout if isinstance(error.stdout, str) else ""
        stderr = error.stderr if isinstance(error.stderr, str) else ""
        return "timeout", (stdout + stderr).strip(), elapsed
    elapsed = time.monotonic() - started
    output = (proc.stdout + proc.stderr).strip()
    if proc.returncode == 0:
        return "success", output, elapsed
    return f"exit-{proc.returncode}", output, elapsed


def shell_quote(command: Sequence[str]) -> str:
    return " ".join(command)


def truncate(text: str, limit: int = 500) -> str:
    text = " ".join(text.split())
    if len(text) <= limit:
        return text
    return text[: limit - 1] + "…"


def smoke_executable(path: Path, platform: str, root: Path) -> CheckResult:
    if platform != "windows":
        path.chmod(path.stat().st_mode | 0o755)

    env = os.environ.copy()
    if platform == "linux":
        env.setdefault("WINIT_UNIX_BACKEND", "x11")
        xvfb = shutil.which("xvfb-run")
        command = [xvfb, "-a", str(path)] if xvfb else [str(path)]
    else:
        command = [str(path)]

    status, output, elapsed = run_command(command, env=env)
    if status in {"success", "timeout"}:
        note = "process exited cleanly" if status == "success" else "process stayed alive until smoke timeout"
        return CheckResult(platform, rel(path, root), "Raw executable", shell_quote(command), "pass", note, elapsed)
    return CheckResult(
        platform,
        rel(path, root),
        "Raw executable",
        shell_quote(command),
        "fail",
        truncate(output or status),
        elapsed,
    )


def find_raw_executables(root: Path, platform: str) -> list[Path]:
    ext = ".exe" if platform == "windows" else ""
    names = {f"liora-docs{ext}", f"liora-gallery{ext}"}
    candidates = []
    for path in root.rglob("*"):
        if path.is_file() and path.name in names and "liora-release-binaries" in path.as_posix():
            candidates.append(path)
    return sorted(candidates)


def verify_tar_gz(path: Path, platform: str, root: Path) -> CheckResult:
    started = time.monotonic()
    with tempfile.TemporaryDirectory(prefix="liora-tar-smoke-") as tmp:
        tmp_path = Path(tmp)
        try:
            with tarfile.open(path, "r:gz") as archive:
                archive.extractall(tmp_path)
            executables = [p for p in tmp_path.rglob("liora-gallery") if p.is_file()]
            launchers = [p for p in tmp_path.rglob("liora-gallery") if p.is_file()]
            readmes = list(tmp_path.rglob("README.md"))
            if not executables or not readmes:
                return CheckResult(platform, rel(path, root), "Gallery package", "extract tar.gz", "fail", "missing binary or README after extraction", time.monotonic() - started)
            smoke = smoke_executable(executables[0], platform, root)
            status = "pass" if smoke.status == "pass" else "fail"
            notes = f"extracted portable archive; launchers={len(launchers)} readmes={len(readmes)}; smoke={smoke.status}: {smoke.notes}"
            return CheckResult(platform, rel(path, root), "Gallery package", "extract + run portable binary", status, notes, time.monotonic() - started)
        except Exception as error:  # noqa: BLE001 - report any artifact-specific failure.
            return CheckResult(platform, rel(path, root), "Gallery package", "extract tar.gz", "fail", truncate(str(error)), time.monotonic() - started)


def verify_appimage(path: Path, platform: str, root: Path) -> CheckResult:
    if platform != "linux":
        return CheckResult(platform, rel(path, root), "Gallery package", "skip AppImage", "skip", "AppImage is Linux-only", 0.0)
    direct = smoke_executable(path, platform, root)
    if direct.status == "pass":
        direct.kind = "Gallery package"
        direct.action = "chmod + run AppImage"
        return direct

    started = time.monotonic()
    with tempfile.TemporaryDirectory(prefix="liora-appimage-smoke-") as tmp:
        tmp_path = Path(tmp)
        path.chmod(path.stat().st_mode | 0o755)
        status, output, _ = run_command([str(path), "--appimage-extract"], cwd=tmp_path, timeout=60)
        app_run = tmp_path / "squashfs-root" / "AppRun"
        if status != "success" or not app_run.exists():
            return CheckResult(platform, rel(path, root), "Gallery package", "run AppImage or extract fallback", "fail", truncate(direct.notes + " | " + output), time.monotonic() - started)
        smoke = smoke_executable(app_run, platform, root)
        return CheckResult(platform, rel(path, root), "Gallery package", "AppImage extract fallback + run AppRun", smoke.status, f"direct run failed; extract fallback {smoke.status}: {smoke.notes}", time.monotonic() - started)


def sudo_command(command: Sequence[str]) -> list[str]:
    if os.name == "nt" or hasattr(os, "geteuid") and os.geteuid() == 0:
        return list(command)
    sudo = shutil.which("sudo")
    return [sudo, *command] if sudo else list(command)


def verify_deb(path: Path, platform: str, root: Path) -> CheckResult:
    if platform != "linux":
        return CheckResult(platform, rel(path, root), "Gallery package", "skip deb", "skip", "deb is Linux-only", 0.0)
    started = time.monotonic()
    package = "liora-gallery"
    run_command(sudo_command(["apt-get", "remove", "-y", package]), timeout=90)
    install_status, install_output, _ = run_command(sudo_command(["apt-get", "install", "-y", str(path.resolve())]), timeout=180)
    try:
        if install_status != "success":
            return CheckResult(platform, rel(path, root), "Gallery package", "apt-get install", "fail", truncate(install_output), time.monotonic() - started)
        binary = Path("/usr/bin/liora-gallery")
        smoke = smoke_executable(binary, platform, root) if binary.exists() else CheckResult(platform, str(binary), "Gallery package", "run installed binary", "fail", "installed binary not found", 0.0)
        return CheckResult(platform, rel(path, root), "Gallery package", "apt-get install + run + remove", smoke.status, f"install OK; smoke={smoke.status}: {smoke.notes}", time.monotonic() - started)
    finally:
        run_command(sudo_command(["apt-get", "remove", "-y", package]), timeout=90)


def verify_rpm(path: Path, platform: str, root: Path) -> CheckResult:
    if platform != "linux":
        return CheckResult(platform, rel(path, root), "Gallery package", "skip rpm", "skip", "rpm is Linux-only", 0.0)
    started = time.monotonic()
    package = "liora-gallery"
    run_command(sudo_command(["rpm", "-e", package, "--nodeps"]), timeout=90)
    install_status, install_output, _ = run_command(sudo_command(["rpm", "-Uvh", "--nodeps", str(path.resolve())]), timeout=180)
    try:
        if install_status != "success":
            return CheckResult(platform, rel(path, root), "Gallery package", "rpm -Uvh --nodeps", "fail", truncate(install_output), time.monotonic() - started)
        binary = Path("/usr/bin/liora-gallery")
        smoke = smoke_executable(binary, platform, root) if binary.exists() else CheckResult(platform, str(binary), "Gallery package", "run installed binary", "fail", "installed binary not found", 0.0)
        return CheckResult(platform, rel(path, root), "Gallery package", "rpm install + run + erase", smoke.status, f"install OK; smoke={smoke.status}: {smoke.notes}", time.monotonic() - started)
    finally:
        run_command(sudo_command(["rpm", "-e", package, "--nodeps"]), timeout=90)


def verify_dmg(path: Path, platform: str, root: Path) -> CheckResult:
    if platform != "macos":
        return CheckResult(platform, rel(path, root), "Gallery package", "skip dmg", "skip", "dmg is macOS-only", 0.0)
    started = time.monotonic()
    with tempfile.TemporaryDirectory(prefix="liora-dmg-mount-") as mount, tempfile.TemporaryDirectory(prefix="liora-app-smoke-") as app_tmp:
        mount_path = Path(mount)
        attach_status, attach_output, _ = run_command(["hdiutil", "attach", str(path), "-mountpoint", str(mount_path), "-nobrowse", "-readonly"], timeout=90)
        try:
            if attach_status != "success":
                return CheckResult(platform, rel(path, root), "Gallery package", "hdiutil attach", "fail", truncate(attach_output), time.monotonic() - started)
            apps = sorted(mount_path.rglob("*.app"))
            if not apps:
                return CheckResult(platform, rel(path, root), "Gallery package", "inspect mounted dmg", "fail", "no .app bundle found", time.monotonic() - started)
            dest = Path(app_tmp) / apps[0].name
            shutil.copytree(apps[0], dest)
            status, output, elapsed = run_command(["open", "-n", str(dest)], timeout=TIMEOUT_SECONDS)
            if status in {"success", "timeout"}:
                return CheckResult(platform, rel(path, root), "Gallery package", "mount dmg + copy app + open", "pass", "open accepted copied app bundle", time.monotonic() - started)
            return CheckResult(platform, rel(path, root), "Gallery package", "mount dmg + copy app + open", "fail", truncate(output), elapsed)
        finally:
            run_command(["hdiutil", "detach", str(mount_path), "-quiet"], timeout=60)


def find_installed_exe(install_dir: Path) -> Path | None:
    for path in install_dir.rglob("liora-gallery.exe"):
        if path.is_file():
            return path
    return None


def verify_nsis(path: Path, platform: str, root: Path) -> CheckResult:
    if platform != "windows":
        return CheckResult(platform, rel(path, root), "Gallery package", "skip nsis", "skip", "NSIS setup is Windows-only", 0.0)
    started = time.monotonic()
    with tempfile.TemporaryDirectory(prefix="liora-nsis-") as tmp:
        install_dir = Path(tmp) / "LioraGallery"
        status, output, _ = run_command([str(path), "/S", f"/D={install_dir}"], timeout=180)
        try:
            if status != "success":
                return CheckResult(platform, rel(path, root), "Gallery package", "silent NSIS install", "fail", truncate(output), time.monotonic() - started)
            exe = find_installed_exe(install_dir)
            if not exe:
                return CheckResult(platform, rel(path, root), "Gallery package", "silent NSIS install", "fail", "installed liora-gallery.exe not found", time.monotonic() - started)
            smoke = smoke_executable(exe, platform, root)
            return CheckResult(platform, rel(path, root), "Gallery package", "silent NSIS install + run + uninstall", smoke.status, f"install OK; smoke={smoke.status}: {smoke.notes}", time.monotonic() - started)
        finally:
            uninstallers = list(install_dir.rglob("uninstall*.exe")) + list(install_dir.rglob("Uninstall*.exe"))
            for uninstaller in uninstallers[:1]:
                run_command([str(uninstaller), "/S"], timeout=120)


def verify_msi(path: Path, platform: str, root: Path) -> CheckResult:
    if platform != "windows":
        return CheckResult(platform, rel(path, root), "Gallery package", "skip msi", "skip", "MSI is Windows-only", 0.0)
    started = time.monotonic()
    with tempfile.TemporaryDirectory(prefix="liora-msi-") as tmp:
        install_dir = Path(tmp) / "LioraGallery"
        status, output, _ = run_command(["msiexec", "/i", str(path), "/qn", f"TARGETDIR={install_dir}"], timeout=180)
        try:
            if status != "success":
                return CheckResult(platform, rel(path, root), "Gallery package", "silent MSI install", "fail", truncate(output), time.monotonic() - started)
            exe = find_installed_exe(install_dir) or find_installed_exe(Path(os.environ.get("ProgramFiles", "C:/Program Files")))
            if not exe:
                return CheckResult(platform, rel(path, root), "Gallery package", "silent MSI install", "fail", "installed liora-gallery.exe not found", time.monotonic() - started)
            smoke = smoke_executable(exe, platform, root)
            return CheckResult(platform, rel(path, root), "Gallery package", "silent MSI install + run + uninstall", smoke.status, f"install OK; smoke={smoke.status}: {smoke.notes}", time.monotonic() - started)
        finally:
            run_command(["msiexec", "/x", str(path), "/qn"], timeout=180)


def find_package_artifacts(root: Path, platform: str) -> list[Path]:
    suffixes = {
        "linux": (".AppImage", ".deb", ".rpm", ".tar.gz"),
        "macos": (".dmg",),
        "windows": ("setup.exe", ".msi"),
    }[platform]
    out: list[Path] = []
    for path in root.rglob("*"):
        if not path.is_file():
            continue
        text = path.as_posix()
        if "liora-release-gallery-packages" not in text and "release-assets" not in text:
            continue
        name = path.name
        if any(name.endswith(suffix) for suffix in suffixes):
            out.append(path)
    return sorted(out)


def verify_package(path: Path, platform: str, root: Path) -> CheckResult:
    name = path.name
    if name.endswith(".tar.gz"):
        return verify_tar_gz(path, platform, root)
    if name.endswith(".AppImage"):
        return verify_appimage(path, platform, root)
    if name.endswith(".deb"):
        return verify_deb(path, platform, root)
    if name.endswith(".rpm"):
        return verify_rpm(path, platform, root)
    if name.endswith(".dmg"):
        return verify_dmg(path, platform, root)
    if name.endswith("setup.exe"):
        return verify_nsis(path, platform, root)
    if name.endswith(".msi"):
        return verify_msi(path, platform, root)
    return CheckResult(platform, rel(path, root), "Gallery package", "unknown", "skip", "unsupported artifact extension", 0.0)


def write_reports(results: list[CheckResult], platform: str, out_dir: Path) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    json_path = out_dir / f"runtime-verification-{platform}.json"
    md_path = out_dir / f"runtime-verification-{platform}.md"
    json_path.write_text(json.dumps([asdict(result) for result in results], indent=2), encoding="utf-8")

    passed = sum(1 for result in results if result.status == "pass")
    failed = sum(1 for result in results if result.status == "fail")
    skipped = sum(1 for result in results if result.status == "skip")
    lines = [
        f"# Runtime verification — {platform}",
        "",
        f"Host: `{py_platform.platform()}`",
        "",
        f"Summary: **{passed} passed**, **{failed} failed**, **{skipped} skipped**.",
        "",
        "| Status | Kind | Artifact | Action | Notes |",
        "| --- | --- | --- | --- | --- |",
    ]
    for result in results:
        icon = {"pass": "✅", "fail": "❌", "skip": "⏭️"}.get(result.status, result.status)
        notes = result.notes.replace("|", "\\|")
        lines.append(f"| {icon} {result.status} | {result.kind} | `{result.artifact}` | `{result.action}` | {notes} |")
    lines.append("")
    md_path.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--platform", required=True, choices=["linux", "macos", "windows"])
    parser.add_argument("--dist", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    root = args.dist.resolve()
    results: list[CheckResult] = []

    raw = find_raw_executables(root, args.platform)
    packages = find_package_artifacts(root, args.platform)
    if not raw:
        results.append(CheckResult(args.platform, str(root), "Raw executable", "discover", "fail", "no raw executables found", 0.0))
    if not packages:
        results.append(CheckResult(args.platform, str(root), "Gallery package", "discover", "fail", "no package artifacts found", 0.0))

    for path in raw:
        results.append(smoke_executable(path, args.platform, root))
    for path in packages:
        results.append(verify_package(path, args.platform, root))

    write_reports(results, args.platform, args.out)
    failed = [result for result in results if result.status == "fail"]
    if failed:
        for result in failed:
            print(f"FAIL {result.platform} {result.artifact}: {result.notes}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
