//! Real Linux/Cosmic code-font regression; requires a display and installed monospace fonts.
//! Run with the official pinned Zed assets/fonts/lilex/Lilex-Regular.ttf as the sole argument.
//! Lilex and the higher-priority default candidates must not already be installed.
use gpui::{
    AppContext, Bounds, Context, IntoElement, Render, SharedString, Window, WindowBounds,
    WindowOptions, div, point, prelude::*, px, size,
};
use liora_components::{CodeBlock, Paragraph, Text};
use liora_core::{
    Config, EmbeddedFont, FontConfig, Options, code_font_family, init_liora_with_options,
    is_font_family_available, load_custom_fonts, load_embedded_fonts, set_font_config,
};
use std::{borrow::Cow, time::Duration};

struct CodeSurface;
impl Render for CodeSurface {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        div()
            .p_4()
            .flex()
            .flex_col()
            .gap_4()
            .child(CodeBlock::new("iiii\nWWWW"))
            .child(Paragraph::new().child(Text::new("iiii WWWW").code_style(theme)))
    }
}

fn assert_spacing(window: &Window, family: SharedString, monospace: bool) {
    let text_system = window.text_system();
    let font = gpui::font(family.clone());
    let id = text_system.resolve_font(&font);
    let i = text_system
        .advance(id, px(16.), 'i')
        .expect("Latin i glyph")
        .width;
    let w = text_system
        .advance(id, px(16.), 'W')
        .expect("Latin W glyph")
        .width;
    let mut run = window.text_style().to_run(4);
    run.font = font;
    let narrow = text_system
        .shape_line("iiii".into(), px(16.), &[run.clone()], None)
        .width();
    let wide = text_system
        .shape_line("WWWW".into(), px(16.), &[run], None)
        .width();
    assert!(i > px(0.) && w > px(0.) && narrow > px(0.) && wide > px(0.));
    assert_eq!(
        (i - w).abs() < px(0.01),
        monospace,
        "{family}: i={i:?}, W={w:?}"
    );
    assert_eq!(
        (narrow - wide).abs() < px(0.01),
        monospace,
        "{family}: iiii={narrow:?}, WWWW={wide:?}"
    );
    println!("family={family} i={i:?} W={w:?} iiii={narrow:?} WWWW={wide:?}");
}

fn main() {
    let path = std::env::args_os()
        .nth(1)
        .expect("pass pinned Lilex-Regular.ttf path");
    let lilex = std::fs::read(path).expect("read pinned Lilex font");
    gpui_platform::application().with_assets(liora_icons::IconAssetSource).run(move |cx| {
        init_liora_with_options(cx, Options::system());
        let initial = code_font_family(cx);
        let handle = cx.open_window(WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                point(px(0.), px(0.)), size(px(640.), px(320.)),
            ))),
            ..Default::default()
        }, |_, cx| cx.new(|_| CodeSurface)).unwrap();
        let executor = cx.background_executor().clone();
        cx.spawn(async move |cx| {
            executor.timer(Duration::from_millis(100)).await;
            cx.update_window(handle.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
                assert_spacing(window, initial.clone(), true);
                assert!(is_font_family_available(cx, initial.as_ref()), "default must name an available family");
                for family in ["Consolas", "JetBrains Mono", "SF Mono", "Menlo", "Monaco", "Lilex"] {
                    assert!(!is_font_family_available(cx, family), "registration-refresh fixture requires {family} absent");
                }
                let report = load_embedded_fonts(cx, [EmbeddedFont::new(
                    "MiSans-Medium.ttf",
                    include_bytes!("../assets/fonts/MiSans/MiSans-Medium.ttf").as_slice(),
                )]);
                assert!(report.failures.is_empty());
                assert!(is_font_family_available(cx, "MiSans"));
                set_font_config(cx, FontConfig::system().with_code_families(["not installed", "MiSans", initial.as_ref()]));
                assert_eq!(code_font_family(cx), "MiSans");
                window.draw(cx).clear(cx);
                assert_spacing(window, code_font_family(cx), false);
                cx.global_mut::<Config>().set_font_config(FontConfig::system());
                assert_eq!(code_font_family(cx), initial);
                load_custom_fonts(cx, [Cow::Owned(lilex)]).expect("register Lilex with SDK");
                assert!(is_font_family_available(cx, "Lilex"));
                assert_eq!(code_font_family(cx), "Lilex", "successful registration refreshes cached default");
                window.draw(cx).clear(cx);
                assert_spacing(window, code_font_family(cx), true);
                set_font_config(cx, FontConfig::system().with_code_families(["MiSans"]));
                assert_eq!(code_font_family(cx), "MiSans");
                set_font_config(cx, FontConfig::system());
                assert_eq!(code_font_family(cx), "Lilex");
                window.draw(cx).clear(cx);
            }).unwrap();
            println!("PASS: native Latin advances/shaped widths, proportional override, runtime policy and SDK-registration refresh");
            cx.update(|cx| cx.quit());
        }).detach();
    });
}
