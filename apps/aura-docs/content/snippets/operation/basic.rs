use aura_components::{Button, Label, Operation};
use aura_icons_lucide::IconName;
use gpui::IntoElement;

pub fn operation_basic() -> impl IntoElement {
    Operation::new(
        Label::new("执行操作").icon(IconName::Play),
        Button::new("Run").small(),
    )
    .description("左侧可带说明文本，右侧操作区域保持末端对齐。")
    .status("手动")
}
