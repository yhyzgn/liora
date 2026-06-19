use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(error) = try_main() {
        println!("cargo:error=failed to generate lucide icon bindings: {error}");
        std::process::exit(1);
    }
}

fn try_main() -> io::Result<()> {
    let out_dir = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "OUT_DIR is not set"))?;
    let svg_dir = Path::new("assets/svgs");

    if !svg_dir.is_dir() || fs::read_dir(svg_dir).map_or(true, |mut d| d.next().is_none()) {
        println!("cargo:warning=No SVG icons found. Run: ./scripts/sync-lucide.sh");
    }

    let mut entries = Vec::new();
    if svg_dir.is_dir() {
        for entry in fs::read_dir(svg_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "svg") {
                let stem = path_component_to_str(path.file_stem(), &path, "file stem")?;
                let file_name = path_component_to_str(path.file_name(), &path, "file name")?;
                entries.push((to_pascal_case(stem), file_name.to_string()));
            }
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let generated = out_dir.join("generated.rs");
    let mut f = fs::File::create(&generated)?;
    writeln!(f, "// Auto-generated — {} icons", entries.len())?;
    writeln!(
        f,
        "/// Strongly typed names for the bundled Lucide SVG icon assets."
    )?;
    writeln!(f, "///")?;
    writeln!(
        f,
        "/// Pass a variant to `liora_icons::Icon::new(...)` to render that asset as"
    )?;
    writeln!(
        f,
        "/// a native GPUI SVG element without hard-coding asset paths."
    )?;
    writeln!(f, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
    writeln!(f, "pub enum IconName {{")?;
    for (v, file) in &entries {
        let kebab = file.strip_suffix(".svg").unwrap_or(file);
        writeln!(f, "    /// The `{}` Lucide icon asset.", kebab)?;
        writeln!(f, "    {},", v)?;
    }
    writeln!(f, "}}")?;
    writeln!(f, "impl IconName {{")?;
    writeln!(
        f,
        "    /// Returns the bundled SVG file name for this Lucide icon."
    )?;
    writeln!(f, "    pub fn file(&self) -> &'static str {{")?;
    writeln!(f, "        match self {{")?;
    for (v, file) in &entries {
        writeln!(f, "    IconName::{} => \"{}\",", v, file)?;
    }
    writeln!(f, "        }}")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;
    println!("cargo:rerun-if-changed=assets/svgs/");
    Ok(())
}

fn path_component_to_str<'a>(
    component: Option<&'a std::ffi::OsStr>,
    path: &Path,
    label: &str,
) -> io::Result<&'a str> {
    component
        .and_then(|component| component.to_str())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("icon SVG {} is not valid UTF-8: {}", label, path.display()),
            )
        })
}

fn to_pascal_case(s: &str) -> String {
    s.split(&['-', '_', ' '])
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        })
        .collect()
}
