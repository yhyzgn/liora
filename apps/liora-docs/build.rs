#[cfg(target_os = "windows")]
const WINDOWS_APP_MANIFEST: &str =
    include_str!("../../packaging/windows/common-controls-v6.manifest");

#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rerun-if-changed=../../packaging/icons/liora-docs.ico");
    println!("cargo:rerun-if-changed=../../packaging/windows/common-controls-v6.manifest");
    winresource::WindowsResource::new()
        .set_icon("../../packaging/icons/liora-docs.ico")
        .set_manifest(WINDOWS_APP_MANIFEST)
        .set("ProductName", "Liora Docs")
        .set("FileDescription", "Native GPUI documentation app for Liora")
        .set("CompanyName", "Liora")
        .compile()
        .expect("failed to embed Liora Docs Windows resources");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
