use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use aura_packager::{
    KnownApp, PackageFormat, PackageManifest, Platform, cargo_packager_formats,
    collect_package_artifacts, generated_config_path, generated_rpm_config_path, package_out_dir,
    release_binaries_dir, render_cargo_packager_config, render_generate_rpm_config,
    supplemental_formats, validate_packaging_layout,
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
        PackageAction::Smoke => smoke(command.apps, command.format),
        PackageAction::InstallSmoke => {
            install_smoke(command.apps, command.format, command.execute_install)
        }
        PackageAction::Package | PackageAction::Ci => package_formats(command),
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

fn smoke(apps: Vec<KnownApp>, format: PackageFormat) -> Result<(), String> {
    let root = workspace_root()?;
    let platform = Platform::current();
    let formats: Vec<_> = if format == PackageFormat::PlatformDefaults {
        PackageFormat::defaults_for(platform).to_vec()
    } else {
        vec![format]
    };

    let mut checked = 0usize;
    for app in apps {
        let metadata = app.metadata();
        let out_dir = package_out_dir(&root, &metadata, platform);
        let artifacts = collect_package_artifacts(
            &metadata.package,
            &app_version(),
            platform,
            &target_triple(),
            git_short_sha(&root).as_deref(),
            &out_dir,
            &formats,
        )
        .map_err(|error| {
            format!(
                "failed to inspect package artifacts from {}: {error}",
                out_dir.display()
            )
        })?;

        if artifacts.is_empty() {
            return Err(format!(
                "no package artifacts found for smoke: app={} output={} formats={}",
                metadata.package,
                out_dir.display(),
                formats
                    .iter()
                    .map(|format| format.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        for artifact in artifacts {
            smoke_artifact(&root, &metadata, platform, artifact.format, &artifact.path)?;
            checked += 1;
        }
    }

    println!("package smoke OK: checked {checked} artifact(s)");
    Ok(())
}

fn install_smoke(
    apps: Vec<KnownApp>,
    format: PackageFormat,
    execute_install: bool,
) -> Result<(), String> {
    let root = workspace_root()?;
    let platform = Platform::current();
    let formats: Vec<_> = if format == PackageFormat::PlatformDefaults {
        PackageFormat::defaults_for(platform).to_vec()
    } else {
        vec![format]
    };

    let mut plans = Vec::new();
    let mut checked = 0usize;
    for app in apps {
        let metadata = app.metadata();
        let out_dir = package_out_dir(&root, &metadata, platform);
        let artifacts = collect_package_artifacts(
            &metadata.package,
            &app_version(),
            platform,
            &target_triple(),
            git_short_sha(&root).as_deref(),
            &out_dir,
            &formats,
        )
        .map_err(|error| {
            format!(
                "failed to inspect package artifacts from {}: {error}",
                out_dir.display()
            )
        })?;

        if artifacts.is_empty() {
            return Err(format!(
                "no package artifacts found for install-smoke: app={} output={} formats={}",
                metadata.package,
                out_dir.display(),
                formats
                    .iter()
                    .map(|format| format.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        for artifact in artifacts {
            smoke_artifact(&root, &metadata, platform, artifact.format, &artifact.path)?;
            let plan = install_smoke_plan(&metadata, platform, artifact.format, &artifact.path);
            println!("{plan}");
            if execute_install {
                execute_install_smoke(&root, &metadata, platform, artifact.format, &artifact.path)?;
            }
            plans.push(plan);
            checked += 1;
        }
    }

    write_install_smoke_plan(&root, &plans, execute_install)?;
    println!(
        "install-smoke {} OK: checked {checked} artifact(s)",
        if execute_install { "execute" } else { "plan" }
    );
    Ok(())
}

fn install_smoke_plan(
    app: &aura_packager::AppMetadata,
    platform: Platform,
    format: PackageFormat,
    path: &Path,
) -> String {
    let path = path.display();
    match (platform, format) {
        (Platform::Linux, PackageFormat::Deb) => format!(
            "install-smoke plan: app={} format=deb\n  install: sudo dpkg -i {path} || sudo apt-get -f install -y\n  smoke: timeout 15s {} || true  # GUI app may stay open until timeout\n  uninstall: sudo apt-get remove -y {}",
            app.package, app.binary, app.package
        ),
        (Platform::Linux, PackageFormat::Rpm) => format!(
            "install-smoke plan: app={} format=rpm\n  install: sudo rpm -Uvh {path}\n  smoke: timeout 15s {} || true  # GUI app may stay open until timeout\n  uninstall: sudo rpm -e {}",
            app.package, app.binary, app.package
        ),
        (Platform::Linux, PackageFormat::AppImage) => format!(
            "install-smoke plan: app={} format=appimage\n  install: chmod +x {path}\n  smoke: timeout 15s {path} || true  # GUI app may stay open until timeout\n  uninstall: rm -f <copied-appimage-if-installed>",
            app.package
        ),
        (_, PackageFormat::TarGz) => format!(
            "install-smoke plan: app={} format=tar.gz\n  install: mkdir -p target/install-smoke && tar -xzf {path} -C target/install-smoke\n  smoke: verify launcher, bin/{}, icons and README exist\n  uninstall: rm -rf target/install-smoke/{}-*",
            app.package, app.binary, app.package
        ),
        (Platform::Macos, PackageFormat::App) => format!(
            "install-smoke plan: app={} format=app\n  install: cp -R {path} /tmp/aura-install-smoke/\n  smoke: open -W -n /tmp/aura-install-smoke/*.app --args --smoke if supported\n  uninstall: rm -rf /tmp/aura-install-smoke",
            app.package
        ),
        (Platform::Macos, PackageFormat::Dmg) => format!(
            "install-smoke plan: app={} format=dmg\n  install: hdiutil attach {path}; copy .app to /tmp/aura-install-smoke\n  smoke: open copied .app if runner policy allows\n  uninstall: hdiutil detach <mount>; rm -rf /tmp/aura-install-smoke",
            app.package
        ),
        (Platform::Windows, PackageFormat::Nsis) => format!(
            "install-smoke plan: app={} format=nsis\n  install: {path} /S /D=%TEMP%\\AuraInstallSmoke\\{}\n  smoke: run installed {}.exe with timeout if runner policy allows\n  uninstall: invoke generated uninstaller silently, then remove temp directory",
            app.package, app.binary, app.binary
        ),
        (Platform::Windows, PackageFormat::Msi) => format!(
            "install-smoke plan: app={} format=msi\n  install: msiexec /i {path} /qn TARGETDIR=%TEMP%\\AuraInstallSmoke\\{}\n  smoke: run installed {}.exe with timeout if runner policy allows\n  uninstall: msiexec /x {path} /qn",
            app.package, app.binary, app.binary
        ),
        (_, other) => format!(
            "install-smoke plan: app={} format={}\n  artifact: {path}\n  install/uninstall smoke is platform-specific and currently plan-only",
            app.package,
            other.as_str()
        ),
    }
}

fn execute_install_smoke(
    root: &Path,
    app: &aura_packager::AppMetadata,
    platform: Platform,
    format: PackageFormat,
    path: &Path,
) -> Result<(), String> {
    match format {
        PackageFormat::TarGz => execute_portable_install_smoke(root, app, platform, path),
        _ => Err(format!(
            "refusing to execute install-smoke for format={} without a dedicated safe executor; rerun without --execute-install to generate the plan",
            format.as_str()
        )),
    }
}

fn execute_portable_install_smoke(
    root: &Path,
    app: &aura_packager::AppMetadata,
    platform: Platform,
    path: &Path,
) -> Result<(), String> {
    smoke_portable_tar_gz(app, platform, path)?;
    let install_root = root.join("target").join("install-smoke").join(&app.package);
    if install_root.exists() {
        fs::remove_dir_all(&install_root)
            .map_err(|error| format!("failed to reset {}: {error}", install_root.display()))?;
    }
    fs::create_dir_all(&install_root)
        .map_err(|error| format!("failed to create {}: {error}", install_root.display()))?;

    let status = Command::new("tar")
        .arg("-xzf")
        .arg(path)
        .arg("-C")
        .arg(&install_root)
        .status()
        .map_err(|error| format!("failed to spawn tar for install-smoke: {error}"))?;
    if !status.success() {
        return Err(format!(
            "portable install-smoke extraction failed: {}",
            path.display()
        ));
    }

    let mut found_launcher = false;
    for entry in fs::read_dir(&install_root)
        .map_err(|error| format!("failed to list {}: {error}", install_root.display()))?
    {
        let entry =
            entry.map_err(|error| format!("failed to read install-smoke entry: {error}"))?;
        let candidate = entry.path().join(&app.binary);
        let bin_candidate = entry.path().join("bin").join(&app.binary);
        if candidate.is_file() && bin_candidate.is_file() {
            found_launcher = true;
            break;
        }
    }
    if !found_launcher {
        return Err(format!(
            "portable install-smoke did not find launcher and bin binary under {}",
            install_root.display()
        ));
    }

    fs::remove_dir_all(&install_root).map_err(|error| {
        format!(
            "failed to uninstall portable smoke dir {}: {error}",
            install_root.display()
        )
    })?;
    println!(
        "install-smoke execute OK: app={} format=tar.gz install-root={}",
        app.package,
        install_root.display()
    );
    Ok(())
}

fn write_install_smoke_plan(
    root: &Path,
    plans: &[String],
    execute_install: bool,
) -> Result<(), String> {
    let package_dir = root.join("target").join("packages");
    fs::create_dir_all(&package_dir)
        .map_err(|error| format!("failed to create {}: {error}", package_dir.display()))?;
    let path = package_dir.join("install-smoke-plan.md");
    let mut text = String::from("# Aura package install/uninstall smoke plan\n\n");
    text.push_str(if execute_install {
        "Mode: execute selected safe install smoke paths.\n\n"
    } else {
        "Mode: plan-only. No system package installation was performed.\n\n"
    });
    for plan in plans {
        text.push_str("```text\n");
        text.push_str(plan);
        text.push_str("\n```\n\n");
    }
    fs::write(&path, text)
        .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
    println!("install-smoke plan written: {}", path.display());
    Ok(())
}

fn smoke_artifact(
    _root: &Path,
    app: &aura_packager::AppMetadata,
    platform: Platform,
    format: PackageFormat,
    path: &Path,
) -> Result<(), String> {
    match format {
        PackageFormat::TarGz => smoke_portable_tar_gz(app, platform, path),
        PackageFormat::Deb => require_magic(path, b"!<arch>\n", "deb ar archive"),
        PackageFormat::Rpm => require_magic(path, &[0xed, 0xab, 0xee, 0xdb], "rpm lead"),
        PackageFormat::Nsis => require_magic(path, b"MZ", "Windows executable"),
        PackageFormat::Msi => require_magic(
            path,
            &[0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1],
            "MSI compound document",
        ),
        PackageFormat::AppImage | PackageFormat::Dmg | PackageFormat::App => {
            let len = fs::metadata(path)
                .map_err(|error| format!("failed to stat {}: {error}", path.display()))?
                .len();
            if len == 0 {
                Err(format!("package artifact is empty: {}", path.display()))
            } else {
                println!(
                    "package smoke OK: app={} format={} path={} bytes={}",
                    app.package,
                    format.as_str(),
                    path.display(),
                    len
                );
                Ok(())
            }
        }
        PackageFormat::PlatformDefaults => Ok(()),
    }
}

fn smoke_portable_tar_gz(
    app: &aura_packager::AppMetadata,
    platform: Platform,
    path: &Path,
) -> Result<(), String> {
    let output = Command::new("tar")
        .arg("-tzf")
        .arg(path)
        .output()
        .map_err(|error| format!("failed to spawn tar for {}: {error}", path.display()))?;
    if !output.status.success() {
        return Err(format!(
            "portable tar.gz listing failed for {}: {}",
            path.display(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let entries = String::from_utf8_lossy(&output.stdout);
    let top = entries
        .lines()
        .find_map(|entry| entry.split('/').next())
        .filter(|entry| !entry.is_empty())
        .ok_or_else(|| format!("portable tar.gz is empty: {}", path.display()))?;

    let required = [
        format!("{top}/bin/{}", app.binary),
        format!("{top}/icons/{}.png", app.icon_stem),
        format!("{top}/icons/{}.svg", app.icon_stem),
        format!("{top}/{}", app.binary),
        format!("{top}/README.md"),
    ];
    for required_entry in required {
        if !tar_listing_contains(&entries, &required_entry) {
            return Err(format!(
                "portable tar.gz missing required entry `{required_entry}` in {}",
                path.display()
            ));
        }
    }

    if platform == Platform::Linux {
        for required_entry in [
            format!("{top}/share/applications/{}.desktop", app.binary),
            format!("{top}/share/metainfo/{}.metainfo.xml", app.binary),
        ] {
            if !tar_listing_contains(&entries, &required_entry) {
                return Err(format!(
                    "portable tar.gz missing Linux metadata entry `{required_entry}` in {}",
                    path.display()
                ));
            }
        }
    }

    println!(
        "package smoke OK: app={} format=tar.gz path={}",
        app.package,
        path.display()
    );
    Ok(())
}

fn tar_listing_contains(entries: &str, required_entry: &str) -> bool {
    entries
        .lines()
        .any(|entry| entry.trim_end_matches('/') == required_entry)
}

fn require_magic(path: &Path, magic: &[u8], label: &str) -> Result<(), String> {
    let bytes =
        fs::read(path).map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    if bytes.starts_with(magic) {
        println!(
            "package smoke OK: {} path={} bytes={}",
            label,
            path.display(),
            bytes.len()
        );
        Ok(())
    } else {
        Err(format!(
            "package artifact has unexpected header for {label}: {}",
            path.display()
        ))
    }
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
        let target_triple = target_triple();
        let git_sha = git_short_sha(&root);

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
            if format == PackageFormat::TarGz {
                let archive_path = portable_tar_gz_path(
                    &out_dir,
                    &metadata.package,
                    &app_version(),
                    platform,
                    &target_triple,
                );
                println!(
                    "portable tar.gz package: app={} path={}",
                    metadata.package,
                    archive_path.display()
                );
                if command.dry_run {
                    println!(
                        "dry-run: stage release binary, icons, metadata, README and launcher; tar -czf {} -C <staging-parent> <staging-dir>",
                        archive_path.display()
                    );
                } else {
                    build_portable_tar_gz(
                        &root,
                        &metadata,
                        platform,
                        &target_triple,
                        git_sha.as_deref(),
                        &out_dir,
                        &archive_path,
                    )?;
                }
            } else if format == PackageFormat::Rpm && platform == Platform::Linux {
                let config_path = generated_rpm_config_path(&root, &metadata);
                if let Some(parent) = config_path.parent() {
                    fs::create_dir_all(parent).map_err(|error| {
                        format!("failed to create {}: {error}", parent.display())
                    })?;
                }
                fs::write(&config_path, render_generate_rpm_config(&root, &metadata)).map_err(
                    |error| format!("failed to write {}: {error}", config_path.display()),
                )?;
                let args = generate_rpm_args(&root, app, &config_path, &out_dir);
                println!(
                    "generate-rpm config: app={} path={}",
                    metadata.package,
                    config_path.display()
                );
                if command.dry_run {
                    println!("dry-run: cargo {}", args.join(" "));
                } else {
                    run_generate_rpm(&args)?;
                }
            } else {
                println!(
                    "supplemental package pending: app={} platform={} format={} output={}",
                    metadata.package,
                    platform.as_str(),
                    format.as_str(),
                    out_dir.display()
                );
            }
        }

        if !command.dry_run {
            let artifacts = collect_package_artifacts(
                &metadata.package,
                &app_version(),
                platform,
                &target_triple,
                git_sha.as_deref(),
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

fn portable_tar_gz_path(
    out_dir: &Path,
    package: &str,
    version: &str,
    platform: Platform,
    target_triple: &str,
) -> PathBuf {
    out_dir.join(format!(
        "{package}-{version}-{}-{target_triple}.tar.gz",
        platform.as_str()
    ))
}

fn build_portable_tar_gz(
    root: &Path,
    app: &aura_packager::AppMetadata,
    platform: Platform,
    target_triple: &str,
    git_sha: Option<&str>,
    out_dir: &Path,
    archive_path: &Path,
) -> Result<(), String> {
    let binary_path = root.join("target/release").join(&app.binary);
    if !binary_path.is_file() {
        return Err(format!(
            "release binary not found for portable tar.gz: {}; run `cargo run -p xtask -- package build --app {}` first or omit --skip-build",
            binary_path.display(),
            app.app.key()
        ));
    }

    fs::create_dir_all(out_dir)
        .map_err(|error| format!("failed to create {}: {error}", out_dir.display()))?;

    let stage_name = format!(
        "{}-{}-{}-{target_triple}",
        app.package,
        app_version(),
        platform.as_str()
    );
    let stage_root = out_dir.join("portable-staging").join(&stage_name);
    if stage_root.exists() {
        fs::remove_dir_all(&stage_root)
            .map_err(|error| format!("failed to reset {}: {error}", stage_root.display()))?;
    }

    fs::create_dir_all(stage_root.join("bin"))
        .map_err(|error| format!("failed to create staging bin directory: {error}"))?;
    fs::create_dir_all(stage_root.join("icons"))
        .map_err(|error| format!("failed to create staging icons directory: {error}"))?;
    fs::create_dir_all(stage_root.join("share/applications"))
        .map_err(|error| format!("failed to create staging desktop directory: {error}"))?;
    fs::create_dir_all(stage_root.join("share/metainfo"))
        .map_err(|error| format!("failed to create staging metainfo directory: {error}"))?;

    copy_file(&binary_path, &stage_root.join("bin").join(&app.binary))?;
    copy_file(
        &root
            .join("packaging/icons")
            .join(format!("{}.png", app.icon_stem)),
        &stage_root
            .join("icons")
            .join(format!("{}.png", app.icon_stem)),
    )?;
    copy_file(
        &root
            .join("packaging/icons")
            .join(format!("{}.svg", app.icon_stem)),
        &stage_root
            .join("icons")
            .join(format!("{}.svg", app.icon_stem)),
    )?;

    if platform == Platform::Linux {
        copy_file(
            &app.linux_desktop_path(root),
            &stage_root
                .join("share/applications")
                .join(format!("{}.desktop", app.binary)),
        )?;
        copy_file(
            &app.linux_metainfo_path(root),
            &stage_root
                .join("share/metainfo")
                .join(format!("{}.metainfo.xml", app.binary)),
        )?;
    }

    write_portable_launcher(&stage_root.join(app.binary.as_str()), &app.binary)?;
    write_portable_readme(
        &stage_root.join("README.md"),
        app,
        platform,
        target_triple,
        git_sha,
    )?;

    if archive_path.exists() {
        fs::remove_file(archive_path)
            .map_err(|error| format!("failed to replace {}: {error}", archive_path.display()))?;
    }
    let parent = stage_root
        .parent()
        .ok_or_else(|| format!("staging path has no parent: {}", stage_root.display()))?;
    let status = Command::new("tar")
        .arg("-czf")
        .arg(archive_path)
        .arg("-C")
        .arg(parent)
        .arg(&stage_name)
        .status()
        .map_err(|error| format!("failed to spawn tar: {error}"))?;
    if !status.success() {
        return Err("portable tar.gz backend failed; ensure system `tar` is available".into());
    }
    println!("portable tar.gz written: {}", archive_path.display());
    Ok(())
}

fn copy_file(source: &Path, dest: &Path) -> Result<(), String> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }
    fs::copy(source, dest).map_err(|error| {
        format!(
            "failed to copy {} to {}: {error}",
            source.display(),
            dest.display()
        )
    })?;
    Ok(())
}

fn write_portable_launcher(path: &Path, binary: &str) -> Result<(), String> {
    let script = format!(
        "#!/usr/bin/env sh\nset -eu\nDIR=\"$(CDPATH= cd -- \"$(dirname -- \"$0\")\" && pwd)\"\nexec \"$DIR/bin/{binary}\" \"$@\"\n"
    );
    fs::write(path, script)
        .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(path)
            .map_err(|error| format!("failed to stat {}: {error}", path.display()))?
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)
            .map_err(|error| format!("failed to chmod {}: {error}", path.display()))?;
    }
    Ok(())
}

fn write_portable_readme(
    path: &Path,
    app: &aura_packager::AppMetadata,
    platform: Platform,
    target_triple: &str,
    git_sha: Option<&str>,
) -> Result<(), String> {
    let git_line = git_sha
        .map(|sha| format!("- Git SHA: `{sha}`\n"))
        .unwrap_or_default();
    let text = format!(
        "# {name} portable archive\n\nThis archive contains a pure Rust + GPUI native Aura application. It does not bundle or require Tauri, WebView, HTML/CSS, or browser runtime architecture.\n\n## Metadata\n\n- Package: `{package}`\n- Version: `{version}`\n- Platform: `{platform}`\n- Target triple: `{target_triple}`\n{git_line}\n## Run\n\n```bash\n./{binary}\n# or\n./bin/{binary}\n```\n\nLinux desktop metadata is included under `share/` when the archive is built on Linux.\n",
        name = app.name,
        package = app.package,
        version = app_version(),
        platform = platform.as_str(),
        binary = app.binary,
    );
    fs::write(path, text).map_err(|error| format!("failed to write {}: {error}", path.display()))
}

fn target_triple() -> String {
    env::var("CARGO_BUILD_TARGET")
        .or_else(|_| env::var("TARGET"))
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            rustc_host_triple()
                .unwrap_or_else(|| format!("{}-{}", env::consts::ARCH, env::consts::OS))
        })
}

fn rustc_host_triple() -> Option<String> {
    let output = Command::new("rustc").arg("-vV").output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| line.strip_prefix("host: ").map(str::to_string))
}

fn git_short_sha(root: &Path) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .current_dir(root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!sha.is_empty()).then_some(sha)
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
    let release_notes_path = package_dir.join("release-notes.md");
    fs::write(&release_notes_path, manifest.release_notes_markdown())
        .map_err(|error| format!("failed to write {}: {error}", release_notes_path.display()))?;
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
        println!("release notes written: {}", release_notes_path.display());
    }
    Ok(())
}

fn app_version() -> String {
    env::var("AURA_PACKAGE_VERSION").unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_string())
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

fn generate_rpm_args(
    root: &std::path::Path,
    app: KnownApp,
    config_path: &std::path::Path,
    out_dir: &std::path::Path,
) -> Vec<String> {
    let package_dir = root.join("apps").join(app.package());
    vec![
        "generate-rpm".into(),
        "--package".into(),
        package_dir.display().to_string(),
        "--output".into(),
        out_dir.display().to_string(),
        "--metadata-overwrite".into(),
        format!("{}#package.metadata.generate-rpm", config_path.display()),
        "--auto-req".into(),
        "builtin".into(),
    ]
}

fn run_generate_rpm(args: &[String]) -> Result<(), String> {
    let status = Command::new("cargo")
        .args(args)
        .status()
        .map_err(|error| format!("failed to spawn cargo generate-rpm: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(
            "cargo generate-rpm failed; install backend with `cargo install cargo-generate-rpm --locked` and ensure rpm build prerequisites are available"
                .into(),
        )
    }
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
    execute_install: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PackageAction {
    Validate,
    Build,
    Smoke,
    InstallSmoke,
    Package,
    Ci,
}

impl PackageCommand {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut action = PackageAction::Package;
        let mut app: Option<KnownApp> = None;
        let mut all_apps = false;
        let mut format = PackageFormat::PlatformDefaults;
        let mut dry_run = false;
        let mut skip_build = false;
        let mut execute_install = false;
        let mut iter = args.into_iter();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "validate" => action = PackageAction::Validate,
                "build" => action = PackageAction::Build,
                "smoke" => action = PackageAction::Smoke,
                "install-smoke" => action = PackageAction::InstallSmoke,
                "ci" => action = PackageAction::Ci,
                "--all-apps" => all_apps = true,
                "--dry-run" => dry_run = true,
                "--skip-build" => skip_build = true,
                "--execute-install" => execute_install = true,
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
            execute_install,
        })
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    env::current_dir().map_err(|error| format!("failed to read current directory: {error}"))
}

fn print_help() {
    println!(
        "Aura xtask\n\n  cargo run -p xtask -- package validate\n  cargo run -p xtask -- package build --app gallery\n  cargo run -p xtask -- package --app docs --format appimage\n  cargo run -p xtask -- package --app docs --format deb --dry-run --skip-build\n  cargo run -p xtask -- package ci --all-apps --format platform-defaults\n  cargo run -p xtask -- package smoke --all-apps --format platform-defaults\n\nOptions:\n  --app <gallery|docs>\n  --all-apps\n  --format <appimage|deb|rpm|tar.gz|app|dmg|nsis|msi|platform-defaults>\n  --dry-run      generate backend config and print cargo-packager invocation\n  --skip-build   reuse target/release binaries instead of building first"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_triple_prefers_explicit_env() {
        // SAFETY: This unit test mutates process environment and does not spawn
        // threads. It restores the variables before returning.
        unsafe {
            env::set_var("CARGO_BUILD_TARGET", "aarch64-apple-darwin");
            env::remove_var("TARGET");
        }
        assert_eq!(target_triple(), "aarch64-apple-darwin");
        unsafe {
            env::remove_var("CARGO_BUILD_TARGET");
        }
    }

    #[test]
    fn portable_tar_gz_name_contains_package_version_platform_and_target() {
        let path = portable_tar_gz_path(
            Path::new("out"),
            "aura-gallery",
            "1.2.3-preview.4.abcdef0",
            Platform::Linux,
            "x86_64-unknown-linux-gnu",
        );
        assert_eq!(
            path,
            Path::new("out")
                .join("aura-gallery-1.2.3-preview.4.abcdef0-linux-x86_64-unknown-linux-gnu.tar.gz")
        );
    }

    #[test]
    fn portable_tar_listing_checks_required_entries() {
        let entries = "aura-gallery/\naura-gallery/bin/aura-gallery\naura-gallery/README.md\n";
        assert!(tar_listing_contains(
            entries,
            "aura-gallery/bin/aura-gallery"
        ));
        assert!(tar_listing_contains(entries, "aura-gallery/README.md"));
        assert!(!tar_listing_contains(entries, "aura-gallery/missing"));
    }

    #[test]
    fn generate_rpm_args_use_metadata_overwrite_branch() {
        let args = generate_rpm_args(
            Path::new("/repo/aura"),
            KnownApp::Gallery,
            Path::new("/repo/aura/target/aura-packager/GenerateRpm.gallery.toml"),
            Path::new("/repo/aura/target/packages/aura-gallery/linux"),
        );
        assert!(args.contains(&"generate-rpm".into()));
        assert!(args.contains(&"--metadata-overwrite".into()));
        assert!(args.contains(
            &"/repo/aura/target/aura-packager/GenerateRpm.gallery.toml#package.metadata.generate-rpm"
                .into()
        ));
    }

    #[test]
    fn install_smoke_plan_contains_install_and_uninstall_steps() {
        let app = KnownApp::Gallery.metadata();
        let plan = install_smoke_plan(
            &app,
            Platform::Linux,
            PackageFormat::Deb,
            Path::new("target/packages/aura-gallery/linux/aura-gallery.deb"),
        );

        assert!(plan.contains("sudo dpkg -i"));
        assert!(plan.contains("sudo apt-get remove"));
        assert!(plan.contains("aura-gallery"));
    }

    #[test]
    fn install_smoke_command_parse_defaults_to_plan_only() {
        let command = PackageCommand::parse(vec![
            "install-smoke".into(),
            "--all-apps".into(),
            "--format".into(),
            "tar.gz".into(),
        ])
        .unwrap();

        assert_eq!(command.action, PackageAction::InstallSmoke);
        assert_eq!(command.format, PackageFormat::TarGz);
        assert!(!command.execute_install);
        assert_eq!(command.apps.len(), aura_packager::known_apps().len());
    }

    #[test]
    fn install_smoke_execute_flag_is_explicit() {
        let command = PackageCommand::parse(vec![
            "install-smoke".into(),
            "--format".into(),
            "tar.gz".into(),
            "--execute-install".into(),
        ])
        .unwrap();

        assert!(command.execute_install);
    }
}
