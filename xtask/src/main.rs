use std::{env, fs, path::PathBuf, process::Command};

use aura_packager::{
    KnownApp, PackageFormat, PackageManifest, Platform, cargo_packager_formats,
    collect_package_artifacts, generated_config_path, package_out_dir, release_binaries_dir,
    render_cargo_packager_config, supplemental_formats, validate_packaging_layout,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("package") => package(args.collect()),
        Some("help") | Some("--help") | Some("-h") | None => {
            print_help();
            Ok(())
        }
        Some(other) => Err(format!("unknown xtask command '{other}'")),
    }
}

fn package(args: Vec<String>) -> Result<(), String> {
    let command = PackageCommand::parse(args)?;
    match command.action {
        PackageAction::Validate => validate(),
        PackageAction::Build => build(command.apps),
        PackageAction::Package => package_formats(command),
    }
}

fn validate() -> Result<(), String> {
    let root = workspace_root()?;
    let report = validate_packaging_layout(&root);
    if report.is_ok() {
        println!("packaging layout OK");
        return Ok(());
    }

    for error in report.errors {
        eprintln!("- {error}");
    }
    Err("packaging layout validation failed".into())
}

fn build(apps: Vec<KnownApp>) -> Result<(), String> {
    for app in apps {
        let status = Command::new("cargo")
            .args(["build", "--release", "-p", app.package()])
            .status()
            .map_err(|error| format!("failed to spawn cargo build: {error}"))?;
        if !status.success() {
            return Err(format!("cargo build failed for {}", app.package()));
        }
    }
    Ok(())
}

fn package_formats(command: PackageCommand) -> Result<(), String> {
    validate()?;
    if !command.skip_build {
        build(command.apps.clone())?;
    }

    let root = workspace_root()?;
    let platform = Platform::current();
    let formats: Vec<_> = if command.format == PackageFormat::PlatformDefaults {
        PackageFormat::defaults_for(platform).to_vec()
    } else {
        vec![command.format]
    };

    let mut manifest = PackageManifest::default();

    for app in command.apps {
        let metadata = app.metadata();
        let cargo_formats = cargo_packager_formats(&formats);
        let supplemental = supplemental_formats(&formats);
        let out_dir = package_out_dir(&root, &metadata, platform);
        let binaries_dir = release_binaries_dir(&root);

        if !cargo_formats.is_empty() {
            let config_path = generated_config_path(&root, &metadata);
            let config_text = render_cargo_packager_config(
                &root,
                &metadata,
                &cargo_formats,
                &out_dir,
                &binaries_dir,
            );
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
            }
            fs::write(&config_path, config_text)
                .map_err(|error| format!("failed to write {}: {error}", config_path.display()))?;

            let args = cargo_packager_args(&config_path, &out_dir, &binaries_dir, &cargo_formats);
            println!(
                "cargo-packager config: app={} path={}",
                metadata.package,
                config_path.display()
            );
            if command.dry_run {
                println!("dry-run: cargo {}", args.join(" "));
            } else {
                run_cargo_packager(&args)?;
            }
        }

        for format in supplemental {
            println!(
                "supplemental package pending: app={} platform={} format={} output={}",
                metadata.package,
                platform.as_str(),
                format.as_str(),
                out_dir.display()
            );
        }

        if !command.dry_run {
            let artifacts = collect_package_artifacts(
                &metadata.package,
                app_version(),
                platform,
                &out_dir,
                &formats,
            )
            .map_err(|error| {
                format!(
                    "failed to collect package artifacts from {}: {error}",
                    out_dir.display()
                )
            })?;
            if artifacts.is_empty() {
                println!(
                    "no package artifacts discovered yet: app={} output={}",
                    metadata.package,
                    out_dir.display()
                );
            } else {
                println!(
                    "discovered {} package artifact(s): app={} output={}",
                    artifacts.len(),
                    metadata.package,
                    out_dir.display()
                );
            }
            manifest.extend(artifacts);
        }
    }

    if !command.dry_run {
        write_manifest_outputs(&root, &manifest)?;
    }

    Ok(())
}

fn write_manifest_outputs(
    root: &std::path::Path,
    manifest: &PackageManifest,
) -> Result<(), String> {
    let package_dir = root.join("target/packages");
    fs::create_dir_all(&package_dir)
        .map_err(|error| format!("failed to create {}: {error}", package_dir.display()))?;
    let manifest_path = package_dir.join("package-manifest.json");
    fs::write(&manifest_path, manifest.to_json_pretty())
        .map_err(|error| format!("failed to write {}: {error}", manifest_path.display()))?;
    let checksum_path = package_dir.join("checksums.txt");
    fs::write(&checksum_path, manifest.checksums_txt())
        .map_err(|error| format!("failed to write {}: {error}", checksum_path.display()))?;
    if manifest.is_empty() {
        println!(
            "package manifest written with no artifacts yet: {}",
            manifest_path.display()
        );
    } else {
        println!(
            "package manifest written: {} ({} artifact(s))",
            manifest_path.display(),
            manifest.artifacts.len()
        );
        println!("checksums written: {}", checksum_path.display());
    }
    Ok(())
}

fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn cargo_packager_args(
    config_path: &std::path::Path,
    out_dir: &std::path::Path,
    binaries_dir: &std::path::Path,
    formats: &[PackageFormat],
) -> Vec<String> {
    let format_arg = formats
        .iter()
        .filter_map(|format| format.cargo_packager_format())
        .collect::<Vec<_>>()
        .join(",");
    vec![
        "packager".into(),
        "--config".into(),
        config_path.display().to_string(),
        "--out-dir".into(),
        out_dir.display().to_string(),
        "--binaries-dir".into(),
        binaries_dir.display().to_string(),
        "--formats".into(),
        format_arg,
    ]
}

fn run_cargo_packager(args: &[String]) -> Result<(), String> {
    let status = Command::new("cargo")
        .args(args)
        .status()
        .map_err(|error| format!("failed to spawn cargo packager: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(
            "cargo packager failed; install backend with `cargo install cargo-packager --locked` and ensure platform tools are available"
                .into(),
        )
    }
}

#[derive(Debug)]
struct PackageCommand {
    action: PackageAction,
    apps: Vec<KnownApp>,
    format: PackageFormat,
    dry_run: bool,
    skip_build: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PackageAction {
    Validate,
    Build,
    Package,
}

impl PackageCommand {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut action = PackageAction::Package;
        let mut app: Option<KnownApp> = None;
        let mut all_apps = false;
        let mut format = PackageFormat::PlatformDefaults;
        let mut dry_run = false;
        let mut skip_build = false;
        let mut iter = args.into_iter();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "validate" => action = PackageAction::Validate,
                "build" => action = PackageAction::Build,
                "--all-apps" => all_apps = true,
                "--dry-run" => dry_run = true,
                "--skip-build" => skip_build = true,
                "--app" => {
                    let value = iter.next().ok_or("--app requires a value")?;
                    app = Some(value.parse()?);
                }
                "--format" => {
                    let value = iter.next().ok_or("--format requires a value")?;
                    format = value.parse()?;
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                other => return Err(format!("unknown package argument '{other}'")),
            }
        }

        let apps = if all_apps {
            aura_packager::known_apps().to_vec()
        } else {
            vec![app.unwrap_or(KnownApp::Gallery)]
        };

        Ok(Self {
            action,
            apps,
            format,
            dry_run,
            skip_build,
        })
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    env::current_dir().map_err(|error| format!("failed to read current directory: {error}"))
}

fn print_help() {
    println!(
        "Aura xtask\n\n  cargo xtask package validate\n  cargo xtask package build --app gallery\n  cargo xtask package --app docs --format appimage\n  cargo xtask package --app docs --format deb --dry-run --skip-build\n  cargo xtask package --all-apps --format platform-defaults\n\nOptions:\n  --app <gallery|docs>\n  --all-apps\n  --format <appimage|deb|rpm|tar.gz|app|dmg|nsis|msi|platform-defaults>\n  --dry-run      generate backend config and print cargo-packager invocation\n  --skip-build   reuse target/release binaries instead of building first"
    );
}
