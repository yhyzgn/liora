fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../packaging/icons/liora.ico");
        res.set("FileDescription", "Liora Gallery");
        res.set("ProductName", "Liora");
        res.set("CompanyName", "Liora Contributors");
        res.set("OriginalFilename", "liora-gallery.exe");
        let _ = res.compile();
    }

    liora_locales_codegen::generate_locales_from_package("liora_core::Locales");
}
