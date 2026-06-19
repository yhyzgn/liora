#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rerun-if-changed=../../packaging/icons/liora-docs.ico");
    winresource::WindowsResource::new()
        .set_icon("../../packaging/icons/liora-docs.ico")
        .set("ProductName", "Liora Docs")
        .set("FileDescription", "Native GPUI documentation app for Liora")
        .set("CompanyName", "Liora")
        .compile()
        .expect("failed to embed Liora Docs Windows resources");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
