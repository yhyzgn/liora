use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=../../Cargo.toml");
    println!("cargo:rerun-if-changed=../../Cargo.lock");
    println!("cargo:rustc-check-cfg=cfg(liora_gpui_latest_api)");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let workspace_root = Path::new(&manifest_dir).join("../..");
    let root_cargo = fs::read_to_string(workspace_root.join("Cargo.toml")).unwrap_or_default();
    let root_lock = fs::read_to_string(workspace_root.join("Cargo.lock")).unwrap_or_default();

    if root_cargo.contains("https://github.com/zed-industries/zed")
        || root_lock.contains("git+https://github.com/zed-industries/zed")
    {
        println!("cargo:rustc-cfg=liora_gpui_latest_api");
    }
}
