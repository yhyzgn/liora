use crate::UpdaterError;
use std::{
    fs, io,
    path::{Component, Path, PathBuf},
};

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "cache/unix.rs"]
mod native;
#[cfg(windows)]
#[path = "cache/windows.rs"]
mod native;

/// Creates or reuses a protected updater cache beneath the OS cache directory
/// (`liora-updates`) or an explicit absolute base directory.
///
/// Use the returned, resolved path for downloading and launching the artifact.
/// On Linux, macOS, and Windows the resolved ancestors must prevent other local
/// users from replacing their children. The app and version directories must
/// belong to the current user and prevent other users from modifying them or
/// files created in the version directory. Administrators and other processes
/// running as the same user are outside this protection.
///
/// A base may resolve through OS aliases. App and version directories may not
/// be aliases. An unsafe or unavailable cache produces an error.
pub fn private_update_cache_dir(
    app: &str,
    version: &str,
    override_base: Option<&Path>,
) -> Result<PathBuf, UpdaterError> {
    // A deliberately portable subset, including ordinary SemVer build metadata.
    for name in [app, version] {
        if name.is_empty()
            || !name.as_bytes()[0].is_ascii_alphanumeric()
            || name.ends_with('.')
            || !name.bytes().all(|c| c.is_ascii_alphanumeric() || b"._-+".contains(&c))
        {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid update cache name").into());
        }
    }

    let default;
    let base = match override_base {
        Some(base) => base,
        None => {
            default = dirs::cache_dir()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no OS cache directory"))?
                .join("liora-updates");
            &default
        }
    };
    if !base.is_absolute() || base.components().any(|c| c == Component::ParentDir) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "update cache base must be absolute and contain no parent components",
        ).into());
    }

    #[cfg(any(target_os = "linux", target_os = "macos", windows))]
    {
        let (existing, missing) = resolve_base(base)?;
        let guard = native::Guard::new()?;
        for ancestor in existing.ancestors().filter(|p| p.has_root()) {
            guard.check(ancestor, false)?;
        }
        let mut path = existing;
        for component in missing.into_iter().rev() {
            path.push(component);
            ensure_directory(&guard, &path, false)?;
        }
        for name in [app, version] {
            path.push(name);
            ensure_directory(&guard, &path, true)?;
        }
        Ok(path)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
    Err(io::Error::new(io::ErrorKind::Unsupported, "no native update cache protection for this platform").into())
}

#[cfg(any(target_os = "linux", target_os = "macos", windows))]
fn resolve_base(mut base: &Path) -> io::Result<(PathBuf, Vec<&std::ffi::OsStr>)> {
    let mut missing = Vec::new();
    loop {
        match base.canonicalize() {
            Ok(resolved) => return Ok((resolved, missing)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                match fs::symlink_metadata(base) {
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                    // A dangling alias is not a missing directory we can create.
                    Ok(_) => return Err(unsafe_cache(base, "unresolved directory alias")),
                    Err(error) => return Err(error),
                }
                missing.push(base.file_name().ok_or(error)?);
                base = base.parent().expect("an absolute named path has a parent");
            }
            Err(error) => return Err(error),
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos", windows))]
fn ensure_directory(guard: &native::Guard, path: &Path, private: bool) -> io::Result<()> {
    match guard.check(path, private) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            match guard.create_private_dir(path) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error),
            }
            guard.check(path, private)
        }
        result => result,
    }
}

#[cfg(any(target_os = "linux", target_os = "macos", windows))]
fn unsafe_cache(path: &Path, reason: &str) -> io::Error {
    io::Error::new(io::ErrorKind::PermissionDenied, format!("unsafe update cache {}: {reason}", path.display()))
}

#[cfg(all(test, any(target_os = "linux", target_os = "macos")))]
pub(crate) mod tests {
    use super::*;
    use std::{
        os::unix::fs::{DirBuilderExt, PermissionsExt, symlink},
        sync::atomic::{AtomicU64, Ordering},
    };

    static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

    pub(crate) struct Fixture(pub(crate) PathBuf);

    impl Fixture {
        pub(crate) fn new(name: &str) -> Self {
            // The contract rejects shared writable ancestors even for a private leaf.
            let home = dirs::home_dir().unwrap().canonicalize().unwrap();
            loop {
                let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
                let path = home.join(format!(".liora-cache-test-{name}-{}-{sequence}", std::process::id()));
                match fs::DirBuilder::new().mode(0o700).create(&path) {
                    Ok(()) => return Self(path),
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                    Err(error) => panic!("cannot create owned cache fixture: {error}"),
                }
            }
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    #[test]
    fn same_name_fixtures_do_not_reuse_each_others_directories() {
        let first = Fixture::new("isolation");
        let second = Fixture::new("isolation");
        assert_ne!(first.0, second.0);
    }

    #[test]
    fn accepts_canonical_private_cache_override() {
        let fixture = Fixture::new("private");
        let base = fixture.0.join("private");
        let alias = fixture.0.join("alias");
        native::Guard::new().unwrap().create_private_dir(&base).unwrap();
        symlink(&base, &alias).unwrap();

        let path = private_update_cache_dir("app", "1.2.3", Some(&alias)).unwrap();
        assert_eq!(path, base.join("app").join("1.2.3"));
        for directory in [path.as_path(), path.parent().unwrap()] {
            let metadata = fs::symlink_metadata(directory).unwrap();
            assert!(metadata.is_dir());
            assert_eq!(metadata.permissions().mode() & 0o077, 0);
        }
        assert_eq!(private_update_cache_dir("app", "1.2.3", Some(&alias)).unwrap(), path);
    }

    #[test]
    fn rejects_writable_cache_ancestor_before_creating_children() {
        let fixture = Fixture::new("writable");
        let writable = fixture.0.join("shared");
        let base = writable.join("private");
        fs::create_dir(&writable).unwrap();
        fs::set_permissions(&writable, fs::Permissions::from_mode(0o777)).unwrap();
        native::Guard::new().unwrap().create_private_dir(&base).unwrap();

        let error = private_update_cache_dir("app", "1.2.3", Some(&base)).unwrap_err();
        assert!(matches!(error, UpdaterError::Io(error) if error.kind() == io::ErrorKind::PermissionDenied));
        assert!(!base.join("app").exists());
    }

    #[test]
    fn rejects_invalid_names_and_nonabsolute_bases_without_creating_children() {
        let fixture = Fixture::new("invalid");
        for name in ["", ".", "..", "a/b", "a\\b", "a:", "trailing."] {
            assert!(matches!(
                private_update_cache_dir(name, "1.2.3", Some(&fixture.0)),
                Err(UpdaterError::Io(error)) if error.kind() == io::ErrorKind::InvalidInput
            ));
            assert!(private_update_cache_dir("app", name, Some(&fixture.0)).is_err());
        }
        for base in [Path::new("relative"), Path::new("/safe/../elsewhere")] {
            assert!(matches!(
                private_update_cache_dir("app", "1.2.3", Some(base)),
                Err(UpdaterError::Io(error)) if error.kind() == io::ErrorKind::InvalidInput
            ));
        }
        assert_eq!(fs::read_dir(&fixture.0).unwrap().count(), 0);
    }

    #[test]
    fn rejects_app_and_version_aliases_without_changing_the_target() {
        let fixture = Fixture::new("leaf-alias");
        let target = fixture.0.join("protected");
        native::Guard::new().unwrap().create_private_dir(&target).unwrap();
        fs::write(target.join("keep"), b"protected bytes").unwrap();
        let app = fixture.0.join("app");
        symlink(&target, &app).unwrap();
        assert!(private_update_cache_dir("app", "1.2.3", Some(&fixture.0)).is_err());
        fs::remove_file(&app).unwrap();
        native::Guard::new().unwrap().create_private_dir(&app).unwrap();
        symlink(&target, app.join("1.2.3")).unwrap();
        assert!(private_update_cache_dir("app", "1.2.3", Some(&fixture.0)).is_err());
        assert_eq!(fs::read(target.join("keep")).unwrap(), b"protected bytes");
        assert_eq!(fs::read_dir(&target).unwrap().count(), 1);
    }

    #[test]
    fn rejects_dangling_base_and_broad_private_directories() {
        let fixture = Fixture::new("unsafe-leaf");
        let dangling = fixture.0.join("dangling");
        symlink(fixture.0.join("absent"), &dangling).unwrap();
        assert!(private_update_cache_dir("app", "1.2.3", Some(&dangling)).is_err());
        assert!(!fixture.0.join("absent").exists());
        let app = fixture.0.join("app");
        native::Guard::new().unwrap().create_private_dir(&app).unwrap();
        fs::set_permissions(&app, fs::Permissions::from_mode(0o750)).unwrap();
        assert!(private_update_cache_dir("app", "1.2.3", Some(&fixture.0)).is_err());
        assert!(!app.join("1.2.3").exists());
        fs::set_permissions(&app, fs::Permissions::from_mode(0o700)).unwrap();
        let version = app.join("1.2.3");
        fs::create_dir(&version).unwrap();
        fs::set_permissions(&version, fs::Permissions::from_mode(0o770)).unwrap();
        assert!(private_update_cache_dir("app", "1.2.3", Some(&fixture.0)).is_err());
    }

    #[test]
    fn default_cache_uses_the_os_cache_root_without_temp_fallback() {
        let fixture = Fixture::new("default");
        let app = fixture.0.file_name().unwrap().to_str().unwrap().strip_prefix('.').unwrap();
        let path = private_update_cache_dir(app, "1.2.3+build", None).unwrap();
        let base = dirs::cache_dir().unwrap().join("liora-updates").canonicalize().unwrap();
        assert_eq!(path, base.join(app).join("1.2.3+build"));
        assert_eq!(fs::metadata(&path).unwrap().permissions().mode() & 0o077, 0);
        fs::remove_dir_all(base.join(app)).unwrap();
    }
}
