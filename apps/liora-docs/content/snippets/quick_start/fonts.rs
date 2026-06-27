//! Application-level font setup used by native Liora apps.
//!
//! Load font resources first, then select ordered UI/code fallback families in
//! `LioraOptions`. This keeps raw executables small while installers can mount
//! a full `assets/fonts` directory next to the app.

use gpui::App;
use liora_components::{
    FontConfig, FontLoadMode, FontLoadOptions, LioraOptions, init_liora_with_options,
    load_app_fonts, set_font_config,
};
use std::{borrow::Cow, path::PathBuf};

fn app_font_dirs(app_binary: &str) -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    // Source-tree development: `cargo run -p your-app`.
    dirs.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/fonts"));

    // Portable/install layouts: keep large font families outside the binary.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            dirs.push(exe_dir.join("assets/fonts"));
            dirs.push(exe_dir.join("..").join("assets/fonts"));
            dirs.push(exe_dir.join("..").join("Resources").join("assets/fonts"));
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    dirs.push(
        PathBuf::from("/usr/lib")
            .join(app_binary)
            .join("assets/fonts"),
    );

    dirs
}

pub fn init_with_packaged_fonts(cx: &mut App) {
    let mut options = FontLoadOptions::new(FontLoadMode::ExternalThenEmbedded)
        .embedded(
            "PingFangSC-Regular.ttf",
            Cow::Borrowed(
                include_bytes!("../../../assets/fonts/PingFangSC/PingFangSC-Regular.ttf")
                    .as_slice(),
            ),
        )
        .require_family("PingFang SC");

    for dir in app_font_dirs("my-liora-app") {
        options = options.external_dir(dir);
    }

    let report = load_app_fonts(cx, options);
    if !report.failures.is_empty() || !report.required_families_available() {
        eprintln!("font load report: {report:?}");
    }

    init_liora_with_options(
        cx,
        LioraOptions::system().with_fonts(
            FontConfig::system()
                .with_ui_families(["PingFang SC", "Segoe UI", "Arial"])
                .with_code_families(["Consolas", "JetBrains Mono", "SF Mono", "Monospace"]),
        ),
    );
}

pub fn switch_to_system_fallbacks(cx: &mut App) {
    set_font_config(
        cx,
        FontConfig::system()
            .with_ui_families(["Segoe UI", "PingFang SC", "Arial"])
            .with_code_families(["Consolas", "JetBrains Mono", "SF Mono", "Monospace"]),
    );
}
