#!/usr/bin/env bash
set -euo pipefail

# Install Fedora system dependencies required to build/run Aura Gallery (GPUI).
# Usage:
#   ./scripts/install-fedora-deps.sh          # install packages + smoke-check system deps
#   ./scripts/install-fedora-deps.sh --verify # also run `cargo check` afterwards
#
# Why these packages:
# - gcc-c++ fixes: freetype-sys failed with "failed to find tool c++".
# - fontconfig-devel fixes: yeslogic-fontconfig-sys could not find fontconfig.pc.
# - freetype-devel/pkgconf-pkg-config provide freetype2/fontconfig pkg-config metadata.
# - Wayland/X11/Vulkan/XKB/XCB packages cover GPUI gallery features:
#   gpui = ["wayland", "x11", "font-kit"].

VERIFY=0
if [[ "${1:-}" == "--verify" ]]; then
  VERIFY=1
elif [[ "${1:-}" != "" ]]; then
  echo "Unknown argument: $1" >&2
  echo "Usage: $0 [--verify]" >&2
  exit 2
fi

if [[ ! -f /etc/fedora-release ]]; then
  echo "This script is intended for Fedora Linux." >&2
  echo "Continuing anyway because dnf-compatible systems may still work." >&2
fi

if command -v sudo >/dev/null 2>&1; then
  SUDO=sudo
elif command -v doas >/dev/null 2>&1; then
  SUDO=doas
elif [[ "$(id -u)" -eq 0 ]]; then
  SUDO=""
else
  echo "Need sudo/doas/root privileges to install packages." >&2
  exit 1
fi

packages=(
  # Build toolchain
  gcc
  gcc-c++
  make
  cmake
  pkgconf-pkg-config

  # Native build helpers used by transitive crates such as clang-sys/bindgen paths
  clang
  clang-devel
  llvm-devel

  # Font stack required by GPUI font-kit / yeslogic-fontconfig-sys / freetype-sys
  fontconfig-devel
  freetype-devel

  # Wayland backend
  wayland-devel
  wayland-protocols-devel
  libxkbcommon-devel
  libxkbcommon-x11-devel

  # X11 / XCB backend
  libX11-devel
  libXcursor-devel
  libXi-devel
  libXrandr-devel
  libxcb-devel
  xcb-util-devel
  xcb-util-wm-devel
  xcb-util-keysyms-devel
  xcb-util-image-devel
  xcb-util-renderutil-devel

  # GPU/runtime headers and loaders commonly needed by wgpu/GPUI on Linux
  mesa-libGL-devel
  mesa-libEGL-devel
  vulkan-loader
  vulkan-loader-devel
  mesa-vulkan-drivers

  # Common TLS/native dependency fallback for Rust desktop dependency graphs
  openssl-devel
)

echo "Installing Aura/GPUI Fedora dependencies..."
$SUDO dnf install -y "${packages[@]}"

echo
echo "Checking key tools and pkg-config entries..."
missing=0
for tool in c++ pkg-config; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "MISSING tool: $tool" >&2
    missing=1
  else
    echo "OK tool: $tool -> $(command -v "$tool")"
  fi
done

for pc in fontconfig freetype2 x11 xcb wayland-client xkbcommon; do
  if ! pkg-config --exists "$pc"; then
    echo "MISSING pkg-config module: $pc" >&2
    missing=1
  else
    echo "OK pkg-config: $pc $(pkg-config --modversion "$pc" 2>/dev/null || true)"
  fi
done

if ! command -v cargo >/dev/null 2>&1; then
  echo "WARN: cargo not found. Install Rust separately via rustup if needed: https://rustup.rs/" >&2
else
  echo "OK tool: cargo -> $(command -v cargo)"
fi

if [[ "$missing" -ne 0 ]]; then
  echo "One or more required tools/pkg-config modules are still missing." >&2
  exit 1
fi

if [[ "$VERIFY" -eq 1 ]]; then
  if [[ ! -f Cargo.toml ]]; then
    echo "--verify expects to be run from the Aura repository root." >&2
    exit 1
  fi
  echo
  echo "Running cargo check..."
  cargo check
fi

echo
echo "Done. Try: cargo run -p aura-gallery"
