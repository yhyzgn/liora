#!/usr/bin/env bash
set -euo pipefail

sudo apt-get update
sudo apt-get install -y \
  build-essential gcc g++ make cmake clang lld llvm pkg-config git curl \
  libfontconfig-dev libfreetype-dev libssl-dev \
  libwayland-dev libx11-xcb-dev libxkbcommon-x11-dev \
  libvulkan1 mesa-vulkan-drivers vulkan-tools \
  xdg-desktop-portal
