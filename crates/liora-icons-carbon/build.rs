use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(error) = try_main() {
        println!("cargo:error=failed to generate icon bindings: {error}");
        std::process::exit(1);
    }
}

fn try_main() -> io::Result<()> {
    let out_dir = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "OUT_DIR is not set"))?;
    let svg_dir = Path::new("assets/svgs");

    if !svg_dir.is_dir() || fs::read_dir(svg_dir).map_or(true, |mut d| d.next().is_none()) {
        println!("cargo:warning=No SVG icons found. Run the crate sync script.");
    }

    let mut entries = Vec::new();
    if svg_dir.is_dir() {
        for entry in fs::read_dir(svg_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "svg") {
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
    writeln!(f, "/// Strongly typed names for bundled SVG icon assets.")?;
    writeln!(f, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
    writeln!(f, "pub enum IconName {{")?;
    for (variant, file) in &entries {
        let kebab = file.strip_suffix(".svg").unwrap_or(file);
        writeln!(f, "    /// The `{}` icon asset.", kebab)?;
        writeln!(f, "    {},", variant)?;
    }
    writeln!(f, "}}")?;
    writeln!(f, "impl IconName {{")?;
    writeln!(f, "    /// Returns every icon in generated enum order.")?;
    writeln!(f, "    pub const fn all() -> &'static [IconName] {{")?;
    writeln!(f, "        &[")?;
    for (variant, _) in &entries {
        writeln!(f, "            IconName::{},", variant)?;
    }
    writeln!(f, "        ]")?;
    writeln!(f, "    }}")?;
    writeln!(
        f,
        "    /// Returns the bundled SVG file name for this icon."
    )?;
    writeln!(f, "    pub fn file(&self) -> &'static str {{")?;
    writeln!(f, "        match self {{")?;
    for (variant, file) in &entries {
        writeln!(f, "            IconName::{} => {:?},", variant, file)?;
    }
    writeln!(f, "        }}")?;
    writeln!(f, "    }}")?;
    writeln!(f, "    /// Returns the embedded SVG source for this icon.")?;
    writeln!(f, "    pub fn svg_source(&self) -> &'static str {{")?;
    writeln!(f, "        match self {{")?;
    for (variant, file) in &entries {
        writeln!(
            f,
            "            IconName::{} => include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/assets/svgs/{}\")),",
            variant, file
        )?;
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

fn to_pascal_case(value: &str) -> String {
    let mut out = String::new();
    for part in value.split(|ch: char| !ch.is_ascii_alphanumeric()) {
        if part.is_empty() {
            continue;
        }
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    if out.is_empty() {
        out.push_str("Icon");
    }
    if out.as_bytes().first().is_some_and(u8::is_ascii_digit) {
        out.insert(0, 'I');
    }
    out
}
