#[cfg(target_os = "windows")]
const WINDOWS_APP_MANIFEST: &str =
    include_str!("../../packaging/windows/common-controls-v6.manifest");

#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rerun-if-changed=../../packaging/icons/liora-gallery.ico");
    println!("cargo:rerun-if-changed=../../packaging/windows/common-controls-v6.manifest");
    winresource::WindowsResource::new()
        .set_icon("../../packaging/icons/liora-gallery.ico")
        .set_manifest(WINDOWS_APP_MANIFEST)
        .set("ProductName", "Liora Gallery")
        .set("FileDescription", "Native GPUI component gallery for Liora")
        .set("CompanyName", "Liora")
        .compile()
        .expect("failed to embed Liora Gallery Windows resources");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
