#!/usr/bin/env bash
set -euo pipefail

xcode-select --install || true
sudo xcodebuild -license accept
brew install cmake
