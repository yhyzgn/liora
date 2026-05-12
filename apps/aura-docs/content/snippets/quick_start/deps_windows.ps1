# Run in an elevated PowerShell when configuring a Windows development machine.
# Install Rust with the MSVC toolchain from https://rustup.rs/ first.

rustup default stable-x86_64-pc-windows-msvc
rustup component add rustfmt clippy

git config --system core.longpaths true
New-ItemProperty `
  -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" `
  -Name "LongPathsEnabled" `
  -Value 1 `
  -PropertyType DWORD `
  -Force

# Also install "Desktop development with C++" from Visual Studio Build Tools.
# Keep GPU drivers current; GPUI/Zed's Windows renderer needs a working graphics backend.
