use gpui::IntoElement;
use liora_components::{Button, Label, Operation};
use liora_icons_lucide::IconName;

pub fn operation_basic() -> impl IntoElement {
    Operation::new(
        Label::new("执行操作").icon(IconName::Play),
        Button::new("Run").small(),
    )
    .description("左侧可带说明文本，右侧操作区域保持末端对齐。")
    .status("手动")
}
