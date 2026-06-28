use gpui::{AnyView, App, Context, Render, Window, prelude::*, rgb};
use liora_components::layout_helpers::{page, row_md, section, showcase_card, showcase_grid};
use liora_components::{Kbd, Space, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| KbdDemo).into()
}

struct KbdDemo;

impl Render for KbdDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Kbd 快捷键",
            "用于展示键盘快捷键、菜单尾部提示、命令面板入口和帮助说明。",
            Space::new().vertical().gap_xl().child(section(
                "Kbd showcase",
                "快捷键示例统一在卡片中展示，避免 keycap 在页面上随机排布。",
                showcase_grid(vec![
                    showcase_card(
                        "基础 keycap",
                        "常见组合键、单键和跨平台快捷提示。",
                        row_md(vec![
                            Kbd::new("⌘K").into_any_element(),
                            Kbd::new("Ctrl").into_any_element(),
                            Kbd::new("Shift").into_any_element(),
                            Kbd::new("Enter").into_any_element(),
                            Kbd::new("Esc").into_any_element(),
                        ]),
                    )
                    .into_any_element(),
                    showcase_card(
                        "尺寸",
                        "小尺寸适合菜单尾部，大尺寸适合空状态或帮助页。",
                        row_md(vec![
                            Kbd::new("⌘").small().into_any_element(),
                            Kbd::new("Tab").into_any_element(),
                            Kbd::new("Space").large().into_any_element(),
                        ]),
                    )
                    .into_any_element(),
                    showcase_card(
                        "语义颜色",
                        "自定义背景和前景色用于强调危险或成功快捷键。",
                        row_md(vec![
                            Kbd::new("Esc")
                                .color(rgb(0xdc2626).into())
                                .into_any_element(),
                            Kbd::new("⌘S")
                                .bg(rgb(0xdcfce7).into())
                                .color(rgb(0x166534).into())
                                .into_any_element(),
                        ]),
                    )
                    .into_any_element(),
                    showcase_card(
                        "组合行",
                        "用 Space/Text 组成命令帮助、菜单项和快捷键说明。",
                        Space::new()
                            .gap_md()
                            .child(Text::new("Open command palette"))
                            .child(
                                Space::new()
                                    .gap_xs()
                                    .child(Kbd::new("⌘"))
                                    .child(Kbd::new("K")),
                            ),
                    )
                    .into_any_element(),
                ]),
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn kbd_demo_is_dedicated_and_rich() {
        let source = include_str!("kbd_demo.rs");
        assert!(source.contains("Kbd 快捷键"));
        assert!(source.contains("尺寸"));
        assert!(source.contains("语义颜色和组合行"));
        assert!(source.contains("use liora_components::{Kbd, Space, Text};"));
    }
}
