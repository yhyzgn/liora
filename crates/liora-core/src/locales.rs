//! Runtime locale helpers for Liora applications.
//!
//! Liora keeps locales deliberately low-coupling: component code asks for a stable
//! key at render time, while applications choose where translation resources
//! come from. The default implementation loads external TOML language files;
//! advanced applications can provide any [`Translator`] implementation.

use gpui::{App, Context, SharedString, Window};
use std::{
    collections::HashMap,
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Typed translation key used by [`tr`].
///
/// The value remains a dot-separated resource path internally so external TOML
/// language files stay simple and low-coupling, but application/component code
/// can use generated or macro-defined constants instead of hardcoded strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Locales(&'static str);

impl Locales {
    /// Creates a typed key from a static resource path.
    pub const fn new(key: &'static str) -> Self {
        Self(key)
    }

    /// Returns the dot-separated resource path used by translation providers.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for Locales {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl fmt::Display for Locales {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// User-facing text that can be either a literal string or a typed locale key.
///
/// Component constructors accept this type through `impl Into<LocalizedText>`,
/// so callers can pass generated keys directly, e.g.
/// `Button::new(locales::common::ok)`. The actual translation is resolved at
/// render time from the current [`LocalesContext`], which keeps runtime
/// language switching working without hardcoded string keys at call sites.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LocalizedText {
    /// Literal user-facing text that should not be translated.
    Literal(SharedString),
    /// Typed locale key resolved against the active locale during render.
    Key(Locales),
}

impl LocalizedText {
    /// Creates a literal localized text source.
    pub fn literal(text: impl Into<SharedString>) -> Self {
        Self::Literal(text.into())
    }

    /// Creates a localized text source from a typed key.
    pub const fn key(key: Locales) -> Self {
        Self::Key(key)
    }

    /// Resolves this text source against the active locale context.
    pub fn resolve(&self, cx: &impl LocalesContext) -> SharedString {
        match self {
            Self::Literal(text) => text.clone(),
            Self::Key(key) => tr(cx, *key),
        }
    }

    /// Returns a stable seed for element ids, tests, and debug output.
    pub fn stable_seed(&self) -> &str {
        match self {
            Self::Literal(text) => text.as_ref(),
            Self::Key(key) => key.as_str(),
        }
    }

    /// Returns whether this text source is definitely empty before translation.
    pub fn is_empty_source(&self) -> bool {
        match self {
            Self::Literal(text) => text.is_empty(),
            Self::Key(_) => false,
        }
    }
}

impl From<Locales> for LocalizedText {
    fn from(value: Locales) -> Self {
        Self::Key(value)
    }
}

impl From<SharedString> for LocalizedText {
    fn from(value: SharedString) -> Self {
        Self::Literal(value)
    }
}

impl From<&'static str> for LocalizedText {
    fn from(value: &'static str) -> Self {
        Self::Literal(value.into())
    }
}

impl From<String> for LocalizedText {
    fn from(value: String) -> Self {
        Self::Literal(value.into())
    }
}

/// Defines typed locale key constants grouped by resource table.
///
/// Applications normally use the generated [`locales`] module, which is rebuilt
/// by Cargo from `assets/locales/*.toml`. This macro remains available for
/// application-specific key modules outside Liora's default resources.
///
/// ```
/// liora_core::locales! {
///     pub mod app_keys {
///         docs { subtitle }
///     }
/// }
/// # let _ = app_keys::docs::subtitle;
/// ```
#[macro_export]
macro_rules! locales {
    (
        $vis:vis mod $module:ident {
            $(
                $group:ident { $($key:ident),+ $(,)? }
            )+
        }
    ) => {
        $vis mod $module {
            $crate::locales!(@groups pub, $( $group { $($key),+ } )+);
        }
    };
    (@groups $vis:vis, $( $group:ident { $($key:ident),+ } )+) => {
        $(
            $vis mod $group {
                $(
                    #[allow(dead_code, non_upper_case_globals)]
                    pub const $key: $crate::locales::Locales =
                        $crate::locales::Locales::new(concat!(stringify!($group), ".", stringify!($key)));
                )+
            }
        )+
    };
}

include!(concat!(env!("OUT_DIR"), "/locales_keys.rs"));

/// Stable locale identifier such as `"zh-CN"` or `"en-US"`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LocaleId(SharedString);

impl LocaleId {
    /// Creates a locale identifier from an application-supplied string.
    pub fn new(locale: impl Into<SharedString>) -> Self {
        Self(locale.into())
    }

    /// Returns the underlying locale string.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl Default for LocaleId {
    fn default() -> Self {
        Self::new("en-US")
    }
}

impl From<&str> for LocaleId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for LocaleId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<SharedString> for LocaleId {
    fn from(value: SharedString) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for LocaleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Text direction associated with a locale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextDirection {
    /// Left-to-right writing direction.
    #[default]
    Ltr,
    /// Right-to-left writing direction.
    Rtl,
}

/// Pluggable translation provider.
///
/// Applications can replace the default TOML-backed map with a custom provider
/// that reads from a database, remote service, Fluent/ICU runtime, or any other
/// source. Components depend only on this trait through [`tr`].
pub trait Translator: Send + Sync {
    /// Returns a translated value for `key` under `locale`, or `None` when the
    /// provider does not know that key.
    fn translate(&self, locale: &LocaleId, key: &str) -> Option<SharedString>;

    /// Returns whether this provider has any resources for `locale`.
    fn has_locale(&self, _locale: &LocaleId) -> bool {
        false
    }
}

/// In-memory translator loaded from external language files or explicit maps.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocalesMap {
    locales: HashMap<LocaleId, HashMap<SharedString, SharedString>>,
}

impl LocalesMap {
    /// Creates an empty translation map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a map containing Liora's small built-in safety fallback.
    pub fn builtin() -> Self {
        let mut map = Self::new();
        map = map.with_locale("en-US", builtin_locale_entries(BUILTIN_EN_US_TOML));
        map = map.with_locale("zh-CN", builtin_locale_entries(BUILTIN_ZH_CN_TOML));
        map
    }

    /// Adds or replaces a locale with key/value pairs.
    pub fn with_locale(
        mut self,
        locale: impl Into<LocaleId>,
        entries: impl IntoIterator<Item = (impl Into<SharedString>, impl Into<SharedString>)>,
    ) -> Self {
        self.insert_locale(locale, entries);
        self
    }

    /// Adds or replaces a locale with key/value pairs.
    pub fn insert_locale(
        &mut self,
        locale: impl Into<LocaleId>,
        entries: impl IntoIterator<Item = (impl Into<SharedString>, impl Into<SharedString>)>,
    ) {
        self.locales.insert(
            locale.into(),
            entries
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        );
    }

    /// Overrides entries in a locale while preserving all other locale keys.
    pub fn override_locale(
        mut self,
        locale: impl Into<LocaleId>,
        entries: impl IntoIterator<Item = (impl Into<SharedString>, impl Into<SharedString>)>,
    ) -> Self {
        let values = self.locales.entry(locale.into()).or_default();
        for (key, value) in entries {
            values.insert(key.into(), value.into());
        }
        self
    }

    /// Loads one locale from a TOML file.
    pub fn load_locale_file(
        &mut self,
        locale: impl Into<LocaleId>,
        path: impl AsRef<Path>,
    ) -> Result<(), LocalesLoadError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|source| LocalesLoadError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let entries =
            parse_toml_translations(&content).map_err(|source| LocalesLoadError::Parse {
                path: path.to_path_buf(),
                source,
            })?;
        self.insert_locale(locale, entries);
        Ok(())
    }

    /// Loads all `*.toml` files from a directory, using file stems as locale ids.
    pub fn load_dir(&mut self, dir: impl AsRef<Path>) -> Result<Vec<LocaleId>, LocalesLoadError> {
        let dir = dir.as_ref();
        let mut loaded = Vec::new();
        let entries = fs::read_dir(dir).map_err(|source| LocalesLoadError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let mut files = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|source| LocalesLoadError::Io {
                path: dir.to_path_buf(),
                source,
            })?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("toml") {
                files.push(path);
            }
        }
        files.sort();
        for path in files {
            let Some(locale) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            let locale = LocaleId::from(locale);
            self.load_locale_file(locale.clone(), &path)?;
            loaded.push(locale);
        }
        Ok(loaded)
    }

    /// Returns true when this map contains any entries for `locale`.
    pub fn has_locale(&self, locale: &LocaleId) -> bool {
        self.locales.contains_key(locale)
    }

    /// Returns the loaded locale ids.
    pub fn locales(&self) -> impl Iterator<Item = &LocaleId> {
        self.locales.keys()
    }
}

impl Translator for LocalesMap {
    fn translate(&self, locale: &LocaleId, key: &str) -> Option<SharedString> {
        self.locales
            .get(locale)
            .and_then(|entries| entries.get(key).cloned())
    }

    fn has_locale(&self, locale: &LocaleId) -> bool {
        self.has_locale(locale)
    }
}

/// Complete locale runtime config.
#[derive(Clone)]
pub struct LocalesConfig {
    /// Currently active locale.
    pub locale: LocaleId,
    /// Fallback locale used when the active locale misses a key.
    pub fallback_locale: LocaleId,
    /// Text direction reserved for direction-aware components and shells.
    pub direction: TextDirection,
    /// Translation resources loaded from external TOML files or explicit maps.
    pub resources: LocalesMap,
    /// Optional application-provided translator. When present it is consulted
    /// before the file-backed resources so apps can fully override Liora's
    /// lookup behavior without forcing components to know about that system.
    pub translator: Option<Arc<dyn Translator>>,
    /// Optional external locales directory used by [`switch_locale_from_dir`].
    pub resource_dir: Option<PathBuf>,
    /// Monotonic version incremented whenever locale/resources change.
    pub version: u64,
}

impl Default for LocalesConfig {
    fn default() -> Self {
        Self {
            locale: LocaleId::from("en-US"),
            fallback_locale: LocaleId::from("en-US"),
            direction: TextDirection::Ltr,
            resources: LocalesMap::builtin(),
            translator: None,
            resource_dir: None,
            version: 0,
        }
    }
}

impl fmt::Debug for LocalesConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalesConfig")
            .field("locale", &self.locale)
            .field("fallback_locale", &self.fallback_locale)
            .field("direction", &self.direction)
            .field("resource_dir", &self.resource_dir)
            .field("version", &self.version)
            .finish_non_exhaustive()
    }
}

impl PartialEq for LocalesConfig {
    fn eq(&self, other: &Self) -> bool {
        self.locale == other.locale
            && self.fallback_locale == other.fallback_locale
            && self.direction == other.direction
            && self.resources == other.resources
            && self.resource_dir == other.resource_dir
            && self.version == other.version
            && match (&self.translator, &other.translator) {
                (Some(a), Some(b)) => Arc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}

impl Eq for LocalesConfig {}

impl LocalesConfig {
    /// Creates the default locales config.
    pub fn system() -> Self {
        Self::default()
    }

    /// Returns a config with an active locale.
    pub fn with_locale(mut self, locale: impl Into<LocaleId>) -> Self {
        self.locale = locale.into();
        self.direction = direction_for_locale(&self.locale);
        self
    }

    /// Returns a config with a fallback locale.
    pub fn with_fallback_locale(mut self, locale: impl Into<LocaleId>) -> Self {
        self.fallback_locale = locale.into();
        self
    }

    /// Returns a config with a text direction.
    pub fn with_direction(mut self, direction: TextDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Returns a config with file-backed resources.
    pub fn with_resources(mut self, resources: LocalesMap) -> Self {
        self.resources = resources;
        self
    }

    /// Loads all TOML files from `dir` into this config and remembers the dir
    /// for later runtime locale loading.
    pub fn try_with_locales_dir(mut self, dir: impl AsRef<Path>) -> Result<Self, LocalesLoadError> {
        let dir = dir.as_ref();
        self.resources.load_dir(dir)?;
        self.resource_dir = Some(dir.to_path_buf());
        Ok(self)
    }

    /// Returns a config using a custom translator implementation.
    pub fn with_translator(mut self, translator: impl Translator + 'static) -> Self {
        self.translator = Some(Arc::new(translator));
        self
    }

    /// Returns a config using a shared custom translator implementation.
    pub fn with_shared_translator(mut self, translator: Arc<dyn Translator>) -> Self {
        self.translator = Some(translator);
        self
    }

    /// Resolves a translation with active-locale and fallback-locale lookup.
    pub fn translate(&self, key: &str) -> SharedString {
        self.translator
            .as_ref()
            .and_then(|translator| translator.translate(&self.locale, key))
            .or_else(|| {
                self.translator
                    .as_ref()
                    .and_then(|translator| translator.translate(&self.fallback_locale, key))
            })
            .or_else(|| self.resources.translate(&self.locale, key))
            .or_else(|| self.resources.translate(&self.fallback_locale, key))
            .or_else(|| builtin_locale_value(BUILTIN_EN_US_TOML, key))
            .unwrap_or_else(|| key.into())
    }

    /// Returns true when either configured provider has resources for `locale`.
    pub fn has_locale(&self, locale: &LocaleId) -> bool {
        self.translator
            .as_ref()
            .is_some_and(|translator| translator.has_locale(locale))
            || self.resources.has_locale(locale)
    }
}

/// Error returned while reading external language resources.
#[derive(Debug)]
pub enum LocalesLoadError {
    /// Failed to read a file or directory.
    Io {
        /// Path being read.
        path: PathBuf,
        /// Source IO error.
        source: std::io::Error,
    },
    /// Failed to parse a TOML language file.
    Parse {
        /// File being parsed.
        path: PathBuf,
        /// Source TOML parser error.
        source: toml::de::Error,
    },
    /// A requested locale does not exist in the configured locales directory.
    MissingLocaleFile {
        /// Locale that was requested.
        locale: LocaleId,
        /// Expected TOML file path.
        path: PathBuf,
    },
}

impl fmt::Display for LocalesLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(
                f,
                "failed to read locale resource {}: {source}",
                path.display()
            ),
            Self::Parse { path, source } => write!(
                f,
                "failed to parse locale resource {}: {source}",
                path.display()
            ),
            Self::MissingLocaleFile { locale, path } => {
                write!(f, "missing locale resource {locale} at {}", path.display())
            }
        }
    }
}

impl Error for LocalesLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
            Self::MissingLocaleFile { .. } => None,
        }
    }
}

/// Flattens a TOML language file into dot-separated translation keys.
pub fn parse_toml_translations(
    source: &str,
) -> Result<Vec<(SharedString, SharedString)>, toml::de::Error> {
    let value: toml::Value = toml::from_str(source)?;
    let mut out = Vec::new();
    flatten_toml_value(None, &value, &mut out);
    out.sort_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
    Ok(out)
}

fn flatten_toml_value(
    prefix: Option<&str>,
    value: &toml::Value,
    out: &mut Vec<(SharedString, SharedString)>,
) {
    match value {
        toml::Value::String(text) => {
            if let Some(prefix) = prefix {
                out.push((prefix.into(), text.as_str().into()));
            }
        }
        toml::Value::Table(table) => {
            for (key, value) in table {
                let next = if let Some(prefix) = prefix {
                    format!("{prefix}.{key}")
                } else {
                    key.clone()
                };
                flatten_toml_value(Some(&next), value, out);
            }
        }
        _ => {}
    }
}

const BUILTIN_EN_US_TOML: &str = include_str!("../assets/locales/en-US.toml");
const BUILTIN_ZH_CN_TOML: &str = include_str!("../assets/locales/zh-CN.toml");

fn builtin_locale_entries(source: &'static str) -> Vec<(SharedString, SharedString)> {
    parse_toml_translations(source).expect("built-in locale resources must be valid TOML")
}

fn builtin_locale_value(source: &'static str, key: &str) -> Option<SharedString> {
    builtin_locale_entries(source)
        .into_iter()
        .find_map(|(entry_key, value)| (entry_key.as_ref() == key).then_some(value))
}

/// Minimal context abstraction used by translation helpers.
pub trait LocalesContext {
    /// Returns the active locale config.
    fn locales_config(&self) -> &LocalesConfig;
}

impl LocalesContext for LocalesConfig {
    fn locales_config(&self) -> &LocalesConfig {
        self
    }
}

impl LocalesContext for App {
    fn locales_config(&self) -> &LocalesConfig {
        &self.global::<crate::Config>().locales
    }
}

impl<T> LocalesContext for Context<'_, T> {
    fn locales_config(&self) -> &LocalesConfig {
        &self.global::<crate::Config>().locales
    }
}

/// Returns the active locale.
pub fn current_locale(cx: &impl LocalesContext) -> LocaleId {
    cx.locales_config().locale.clone()
}

/// Returns the configured fallback locale.
pub fn fallback_locale(cx: &impl LocalesContext) -> LocaleId {
    cx.locales_config().fallback_locale.clone()
}

/// Returns the active locale version.
pub fn locales_version(cx: &impl LocalesContext) -> u64 {
    cx.locales_config().version
}

/// Translates `key` using the active locale config.
pub fn tr(cx: &impl LocalesContext, key: Locales) -> SharedString {
    cx.locales_config().translate(key.as_str())
}

/// Replaces the complete locale config.
pub fn set_locales_config(cx: &mut App, mut locales: LocalesConfig) {
    locales.version = cx
        .global::<crate::Config>()
        .locales
        .version
        .saturating_add(1);
    cx.global_mut::<crate::Config>().locales = locales;
}

/// Replaces the current custom translator while preserving locale and file resources.
pub fn set_translator(cx: &mut App, translator: impl Translator + 'static) {
    let config = cx.global_mut::<crate::Config>();
    config.locales.translator = Some(Arc::new(translator));
    config.locales.version = config.locales.version.saturating_add(1);
}

/// Replaces the current shared custom translator while preserving locale and file resources.
pub fn set_shared_translator(cx: &mut App, translator: Arc<dyn Translator>) {
    let config = cx.global_mut::<crate::Config>();
    config.locales.translator = Some(translator);
    config.locales.version = config.locales.version.saturating_add(1);
}

/// Clears the custom translator and uses only file-backed/built-in resources.
pub fn clear_translator(cx: &mut App) {
    let config = cx.global_mut::<crate::Config>();
    config.locales.translator = None;
    config.locales.version = config.locales.version.saturating_add(1);
}

/// Updates the active locale without refreshing a window.
pub fn set_locale(cx: &mut App, locale: impl Into<LocaleId>) -> Result<(), LocalesLoadError> {
    let locale = locale.into();
    let config = cx.global_mut::<crate::Config>();
    config.locales.locale = locale;
    config.locales.direction = direction_for_locale(&config.locales.locale);
    config.locales.version = config.locales.version.saturating_add(1);
    Ok(())
}

/// Updates the active locale and refreshes the current window immediately.
pub fn apply_locale(
    window: &mut Window,
    cx: &mut App,
    locale: impl Into<LocaleId>,
) -> Result<(), LocalesLoadError> {
    set_locale(cx, locale)?;
    window.refresh();
    Ok(())
}

/// Loads one TOML locale file into the current file-backed resources.
pub fn load_locale_file(
    cx: &mut App,
    locale: impl Into<LocaleId>,
    path: impl AsRef<Path>,
) -> Result<(), LocalesLoadError> {
    let config = cx.global_mut::<crate::Config>();
    config.locales.resources.load_locale_file(locale, path)?;
    config.locales.version = config.locales.version.saturating_add(1);
    Ok(())
}

/// Loads every TOML file from `dir` into the current file-backed resources.
pub fn load_locales_dir(
    cx: &mut App,
    dir: impl AsRef<Path>,
) -> Result<Vec<LocaleId>, LocalesLoadError> {
    let dir = dir.as_ref();
    let config = cx.global_mut::<crate::Config>();
    let loaded = config.locales.resources.load_dir(dir)?;
    config.locales.resource_dir = Some(dir.to_path_buf());
    config.locales.version = config.locales.version.saturating_add(1);
    Ok(loaded)
}

/// Ensures `locale` is loaded from `dir`, switches to it, and refreshes `window`.
pub fn switch_locale_from_dir(
    window: &mut Window,
    cx: &mut App,
    locale: impl Into<LocaleId>,
    dir: impl AsRef<Path>,
) -> Result<(), LocalesLoadError> {
    let locale = locale.into();
    if !cx.global::<crate::Config>().locales.has_locale(&locale) {
        let path = dir.as_ref().join(format!("{}.toml", locale.as_str()));
        if !path.exists() {
            return Err(LocalesLoadError::MissingLocaleFile { locale, path });
        }
        load_locale_file(cx, locale.clone(), path)?;
    }
    apply_locale(window, cx, locale)
}

/// Returns a direction guess for a locale.
pub fn direction_for_locale(locale: &LocaleId) -> TextDirection {
    let language = locale
        .as_str()
        .split(['-', '_'])
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    match language.as_str() {
        "ar" | "fa" | "he" | "ur" => TextDirection::Rtl,
        _ => TextDirection::Ltr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    crate::locales! {
        mod test_keys {
            demo { title, empty_state }
        }
    }

    #[test]
    fn typed_locales_preserve_dot_paths() {
        assert_eq!(empty::description.as_str(), "empty.description");
        assert_eq!(message_box::confirm.as_str(), "message_box.confirm");
        assert_eq!(test_keys::demo::empty_state.as_str(), "demo.empty_state");
    }

    #[test]
    fn toml_translations_flatten_nested_tables() {
        let entries = parse_toml_translations(
            r#"
[demo]
ready = "Ready"
[select]
no_data = "No data"
"#,
        )
        .unwrap();
        assert!(entries.contains(&("demo.ready".into(), "Ready".into())));
        assert!(entries.contains(&("select.no_data".into(), "No data".into())));
    }

    #[test]
    fn locales_map_uses_locale_specific_values() {
        let map = LocalesMap::new().with_locale("zh-CN", [("test.only", "测试")]);
        assert_eq!(
            map.translate(&LocaleId::from("zh-CN"), "test.only")
                .as_deref(),
            Some("测试")
        );
        assert_eq!(
            map.translate(&LocaleId::from("en-US"), "test.only")
                .as_deref(),
            None
        );
    }

    #[test]
    fn locales_config_falls_back_to_fallback_locale_then_builtin_then_key() {
        let map = LocalesMap::new().with_locale("en-US", [("demo.hello", "Hello")]);
        let config = LocalesConfig::system()
            .with_locale("zh-CN")
            .with_fallback_locale("en-US")
            .with_translator(map);
        assert_eq!(config.translate("demo.hello").as_ref(), "Hello");
        assert_eq!(config.translate("common.cancel").as_ref(), "取消");
        assert_eq!(config.translate("missing.key").as_ref(), "missing.key");
    }

    #[test]
    fn load_dir_uses_file_stems_as_locale_ids() {
        let dir = temp_dir("liora-locales");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("en-US.toml"), "[test]\nonly = \"Test\"\n").unwrap();
        fs::write(dir.join("zh-CN.toml"), "[test]\nonly = \"测试\"\n").unwrap();

        let mut map = LocalesMap::new();
        let loaded = map.load_dir(&dir).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(
            map.translate(&LocaleId::from("zh-CN"), "test.only")
                .as_deref(),
            Some("测试")
        );

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn direction_detects_rtl_language_prefixes() {
        assert_eq!(
            direction_for_locale(&LocaleId::from("ar-SA")),
            TextDirection::Rtl
        );
        assert_eq!(
            direction_for_locale(&LocaleId::from("zh-CN")),
            TextDirection::Ltr
        );
    }

    fn temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{label}-{unique}"))
    }
}
