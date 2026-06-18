include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use std::borrow::Cow;

impl IconName {
    pub fn svg_path(&self) -> String {
        format!("{}/assets/svgs/{}", env!("CARGO_MANIFEST_DIR"), self.file())
    }
}

impl liora_icons::IntoIconPath for IconName {
    fn icon_path(&self) -> Cow<'static, str> {
        Cow::Owned(self.svg_path())
    }
}

impl gpui::IntoElement for IconName {
    type Element = gpui::Component<liora_icons::Icon>;
    fn into_element(self) -> Self::Element {
        liora_icons::Icon::new(self).into_element()
    }
}
