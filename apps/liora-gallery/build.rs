#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rerun-if-changed=../../packaging/icons/liora-gallery.ico");
    winresource::WindowsResource::new()
        .set_icon("../../packaging/icons/liora-gallery.ico")
        .set("ProductName", "Liora Gallery")
        .set("FileDescription", "Native GPUI component gallery for Liora")
        .set("CompanyName", "Liora")
        .compile()
        .expect("failed to embed Liora Gallery Windows resources");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
