mod category;
mod demos;

use aura_core::{ContextExt, init_aura};
use aura_theme::Theme;
use aura_components::{Switch, Checkbox, RadioGroup, Input};
use aura_icons_lucide::IconName;
use gpui::{
    App, Bounds, Context, Entity, Render, Window, WindowBounds, WindowOptions, KeyBinding, div, prelude::*, px, size,
};

pub struct Gallery {
    switch_demo_on: Entity<Switch>,
    switch_demo_off: Entity<Switch>,
    switch_demo_disabled: Entity<Switch>,
    cb_checked: Entity<Checkbox>,
    cb_unchecked: Entity<Checkbox>,
    cb_labeled: Entity<Checkbox>,
    cb_disabled: Entity<Checkbox>,
    radio_group: Entity<RadioGroup>,
    input_plain: Entity<Input>,
    input_placeholder: Entity<Input>,
    input_icon: Entity<Input>,
    input_clearable: Entity<Input>,
    input_disabled: Entity<Input>,
    textarea: Entity<Input>,
}

fn run_gallery() {
    gpui_platform::application().run(|cx: &mut App| {
        init_aura(cx, Theme::light());
        Input::register_key_bindings(cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(Bounds::centered(None, size(px(1200.0), px(800.0)), cx))),
                ..Default::default()
            },
            |_, cx| {
                let switch_on = cx.new(|cx| Switch::new(true, cx));
                let switch_off = cx.new(|cx| Switch::new(false, cx));
                let switch_disabled = cx.new(|cx| Switch::new(false, cx).disabled(true));
                let cb_checked = cx.new(|cx| Checkbox::new(true, cx));
                let cb_unchecked = cx.new(|cx| Checkbox::new(false, cx));
                let cb_labeled = cx.new(|cx| Checkbox::new(false, cx).label("Label"));
                let cb_disabled = cx.new(|cx| Checkbox::new(true, cx).disabled(true));
                let radio_group = cx.new(|cx| RadioGroup::new(vec!["Option A", "Option B", "Option C"], 1, cx));
                let input_plain = cx.new(|cx| Input::new("", cx));
                let input_placeholder = cx.new(|cx| Input::new("", cx).placeholder("Type something..."));
                let input_icon = cx.new(|cx| Input::new("", cx).placeholder("Search").icon_prefix(IconName::Search).clearable(true));
                let input_clearable = cx.new(|cx| Input::new("Clear me", cx).clearable(true));
                let input_disabled = cx.new(|cx| Input::new("Disabled", cx).disabled(true));
                let textarea = cx.new(|cx| Input::new("", cx).placeholder("Multi-line textarea..."));
                cx.new(|_| Gallery {
                    switch_demo_on: switch_on, switch_demo_off: switch_off, switch_demo_disabled: switch_disabled,
                    cb_checked, cb_unchecked, cb_labeled, cb_disabled,
                    radio_group,
                    input_plain, input_placeholder, input_icon, input_clearable, input_disabled, textarea,
                })
            },
        ).unwrap();
        cx.activate(true);
    });
}

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.aura();
        let registry = demos::registry();

        let header = div().flex().flex_col().gap_1().mb_4().pb_4().border_b_1().border_color(theme.neutral.border)
            .child(div().text_2xl().text_color(theme.neutral.text_1).font_weight(gpui::FontWeight::BOLD).child("Aura UI"))
            .child(div().text_size(px(theme.font_size.md)).text_color(theme.neutral.text_3).child(format!("Native Component Library · {} components", registry.len())));

        let mut body = div().flex().flex_col().size_full().bg(theme.neutral.body).gap_8().p_8()
            .id("gallery-body").overflow_y_scroll().child(header);

        for entry in &registry {
            body = body.child(
                div().flex().flex_col().gap_4().p_4().border_1().border_color(theme.neutral.divider).rounded(px(theme.radius.lg)).bg(theme.neutral.card)
                    .child(div().flex().flex_col().gap_1()
                        .child(div().text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1).font_weight(gpui::FontWeight::BOLD).child(entry.name))
                        .child(div().text_size(px(theme.font_size.sm)).text_color(theme.neutral.text_3).child(entry.description)))
                    .child((entry.render)())
            );
        }

        // Switch demo (View-based, embedded directly)
        body = body.child(
            div().flex().flex_col().gap_4().p_4().border_1().border_color(theme.neutral.divider).rounded(px(theme.radius.lg)).bg(theme.neutral.card)
                .child(div().flex().flex_col().gap_1()
                    .child(div().text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1).font_weight(gpui::FontWeight::BOLD).child("Switch 开关"))
                    .child(div().text_size(px(theme.font_size.sm)).text_color(theme.neutral.text_3).child("ON/OFF 切换开关")))
                .child(div().flex().flex_row().gap_4().items_center()
                    .child(self.switch_demo_on.clone())
                    .child(self.switch_demo_off.clone())
                    .child(self.switch_demo_disabled.clone()))
        );

        // Checkbox demo
        body = body.child(
            div().flex().flex_col().gap_4().p_4().border_1().border_color(theme.neutral.divider).rounded(px(theme.radius.lg)).bg(theme.neutral.card)
                .child(div().flex().flex_col().gap_1()
                    .child(div().text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1).font_weight(gpui::FontWeight::BOLD).child("Checkbox 多选"))
                    .child(div().text_size(px(theme.font_size.sm)).text_color(theme.neutral.text_3).child("多选框")))
                .child(div().flex().flex_row().gap_4().items_center()
                    .child(self.cb_checked.clone())
                    .child(self.cb_unchecked.clone())
                    .child(self.cb_labeled.clone())
                    .child(self.cb_disabled.clone()))
        );

        // Radio demo
        body = body.child(
            div().flex().flex_col().gap_4().p_4().border_1().border_color(theme.neutral.divider).rounded(px(theme.radius.lg)).bg(theme.neutral.card)
                .child(div().flex().flex_col().gap_1()
                    .child(div().text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1).font_weight(gpui::FontWeight::BOLD).child("Radio 单选"))
                    .child(div().text_size(px(theme.font_size.sm)).text_color(theme.neutral.text_3).child("单选按钮")))
                .child(div().flex().flex_row().gap_4().items_center()
                    .child(self.radio_group.clone()))
        );

        // Input demo
        body = body.child(
            div().flex().flex_col().gap_4().p_4().border_1().border_color(theme.neutral.divider).rounded(px(theme.radius.lg)).bg(theme.neutral.card)
                .child(div().flex().flex_col().gap_1()
                    .child(div().text_size(px(theme.font_size.lg)).text_color(theme.neutral.text_1).font_weight(gpui::FontWeight::BOLD).child("Input 输入框"))
                    .child(div().text_size(px(theme.font_size.sm)).text_color(theme.neutral.text_3).child("文本输入（展示层，编辑功能待实现）")))
                .child(div().flex().flex_col().gap_2()
                    .child(self.input_placeholder.clone())
                    .child(self.input_icon.clone())
                    .child(self.input_clearable.clone())
                    .child(self.input_disabled.clone())
                    .child(self.textarea.clone()))
        );

        body
    }
}

#[cfg(not(target_family = "wasm"))] fn main() { run_gallery(); }
#[cfg(target_family = "wasm")] #[wasm_bindgen::prelude::wasm_bindgen(start)] pub fn start() { gpui_platform::web_init(); run_gallery(); }
