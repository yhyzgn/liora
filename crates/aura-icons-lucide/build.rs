use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let svg_dir = Path::new("assets/svgs");

    // Auto-sync if empty
    if !svg_dir.is_dir() || fs::read_dir(svg_dir).map_or(true, |mut d| d.next().is_none()) {
        let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/sync-lucide.sh");
        if script.exists() {
            println!("cargo:warning=Syncing Lucide icons...");
            let _ = Command::new("bash").arg(&script).status();
        }
    }

    let mut entries = Vec::new();
    let mut embed = String::new();

    if svg_dir.is_dir() {
        for entry in fs::read_dir(svg_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "svg") {
                let stem = path.file_stem().unwrap().to_str().unwrap();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let content = fs::read_to_string(&path).unwrap_or_default();
                let variant = to_pascal_case(stem);
                entries.push((variant, file_name.to_string()));
                // Embed full paths for the enums file
                embed.push_str(&format!(
                    "        \"{}\" => Some(std::borrow::Cow::Borrowed(r###\"{}\"###.as_bytes())),\n",
                    file_name, content
                ));
            }
        }
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    // ---- generated.rs: IconName enum ----
    let generated = Path::new(&out_dir).join("generated.rs");
    let mut f = fs::File::create(&generated).unwrap();
    writeln!(f, "// Auto-generated — {} icons", entries.len()).unwrap();
    writeln!(f, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]").unwrap();
    writeln!(f, "pub enum IconName {{").unwrap();
    for (variant, _) in &entries { writeln!(f, "    {},", variant).unwrap(); }
    writeln!(f, "}}").unwrap();
    writeln!(f, "impl IconName {{").unwrap();
    writeln!(f, "    pub fn file(&self) -> &'static str {{ match self {{").unwrap();
    for (v, file) in &entries { writeln!(f, "        IconName::{} => \"{}\",", v, file).unwrap(); }
    writeln!(f, "    }}}}").unwrap();
    writeln!(f, "}}").unwrap();

    // ---- generated_assets.rs: embedded SVG content ----
    let assets = Path::new(&out_dir).join("generated_assets.rs");
    let mut fa = fs::File::create(&assets).unwrap();
    writeln!(fa, "pub fn load_icon(path: &str) -> Option<std::borrow::Cow<'static, [u8]>> {{").unwrap();
    writeln!(fa, "    match path {{").unwrap();
    fa.write_all(embed.as_bytes()).unwrap();
    writeln!(fa, "        _ => None,").unwrap();
    writeln!(fa, "    }}").unwrap();
    writeln!(fa, "}}").unwrap();

    println!("cargo:rerun-if-changed=assets/svgs/");
}

fn to_pascal_case(s: &str) -> String {
    s.split(&['-', '_', ' ']).map(|w| {
        let mut c = w.chars();
        match c.next() { None=>String::new(), Some(f)=>f.to_uppercase().chain(c).collect() }
    }).collect()
}
