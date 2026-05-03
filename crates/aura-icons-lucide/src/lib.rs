include!(concat!(env!("OUT_DIR"), "/generated.rs"));
include!(concat!(env!("OUT_DIR"), "/generated_assets.rs"));

use std::borrow::Cow;
use gpui::{AssetSource, SharedString};

pub struct IconAssets;

pub fn create_asset_source() -> IconAssets { IconAssets }

impl AssetSource for IconAssets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        Ok(load_icon(path))
    }

    fn list(&self, _path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

impl IconName {
    pub fn svg_path(&self) -> String {
        self.file().to_string()
    }
}

impl aura_icons::IntoIconPath for IconName {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(self.svg_path())
    }
}
