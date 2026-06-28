//! Render several bundled icon libraries through the same `Icon` component.

use gpui::{IntoElement, prelude::*};
use liora_components::{Space, Text};
use liora_icons::Icon;
use liora_icons_antd::IconName as AntdIconName;
use liora_icons_carbon::IconName as CarbonIconName;
use liora_icons_ionic::IconName as IonicIconName;
use liora_icons_material::IconName as MaterialIconName;
use liora_icons_tabler::IconName as TablerIconName;

pub fn icon_libraries_row() -> impl IntoElement {
    Space::new().wrap().gap_md().children([
        labeled_icon(Icon::new(AntdIconName::SaveOutlined).size_lg(), "AntD"),
        labeled_icon(
            Icon::new(IonicIconName::AddCircleOutline).size_lg(),
            "Ionic",
        ),
        labeled_icon(Icon::new(TablerIconName::HomeFilled).size_lg(), "Tabler"),
        labeled_icon(
            Icon::new(CarbonIconName::CheckmarkFilled).size_lg(),
            "Carbon",
        ),
        labeled_icon(
            Icon::new(MaterialIconName::SearchOutlined).size_lg(),
            "Material",
        ),
    ])
}

fn labeled_icon(icon: Icon, label: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .align_center()
        .gap_xs()
        .child(icon)
        .child(Text::new(label).nowrap())
}
