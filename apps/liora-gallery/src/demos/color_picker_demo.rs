use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::{Card, ColorPicker, Space, Text};

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| ColorPickerDemo::new(cx)).into()
}

struct ColorPickerDemo {
    basic: Entity<ColorPicker>,
    custom: Entity<ColorPicker>,
    compact: Entity<ColorPicker>,
    disabled: Entity<ColorPicker>,
}

impl ColorPickerDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| ColorPicker::new("#409eff").width_md()),
            custom: cx.new(|_| {
                ColorPicker::new("#13c2c2").width_md().presets([
                    "#13C2C2", "#52C41A", "#FAAD14", "#F5222D", "#722ED1", "#EB2F96",
                ])
            }),
            compact: cx.new(|_| {
                ColorPicker::new("#F56C6C")
                    .show_label(false)
                    .close_on_click_outside(false)
                    .close_on_escape(false)
            }),
            disabled: cx.new(|_| ColorPicker::new("#909399").disabled(true).width_md()),
        }
    }
}

impl Render for ColorPickerDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "ColorPicker 颜色选择器",
            "点击颜色方块弹出类似取色器的面板，支持自由选择色相、明度/饱和度和 alpha 透明度。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "展示完整取色器弹层和当前颜色文本。",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.basic.clone())
                            .child(Text::new("点击颜色方块打开 popup；在大色板中选择颜色，右侧切换 hue，下方选择 alpha。支持 #RGB、#RRGGBB 和 rgba 展示。")),
                    ),
                ))
                .child(section("自定义 Popup 预设色", "替换底部快捷色板。", Card::new(self.custom.clone())))
                .child(section("隐藏文本标签", "仅展示颜色方块触发器，并禁用点击外部/ESC 自动关闭。", Card::new(self.compact.clone())))
                .child(section("禁用状态", "禁用后不可打开弹层。", Card::new(self.disabled.clone()))),
        )
    }
}
