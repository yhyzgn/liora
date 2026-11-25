//! Native glyph coverage probe; requires installed CJK and emoji fallback fonts.
//! Pass a private output directory; capture the window after `ready`, then create `captured`.
use gpui::{
    AppContext, Bounds, FontWeight, IsZero, RenderGlyphParams, WindowBounds, WindowOptions, point,
    px, size,
};
use liora_core::{
    EmbeddedFont, FontConfig, Options, init_liora_with_options, is_font_family_available,
    load_embedded_fonts, ui_font_family, ui_font_weight,
};
use std::{path::PathBuf, time::Duration};

fn main() {
    let output = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .expect("pass private output directory"),
    );
    assert!(output.is_dir());
    let platform = gpui_platform::current_platform(false);
    let rasterizer = platform.text_system();
    gpui::Application::with_platform(platform)
        .with_assets(liora_icons::IconAssetSource)
        .run(move |cx| {
            let report = load_embedded_fonts(
                cx,
                [EmbeddedFont::new(
                    "MiSans-Medium.ttf",
                    include_bytes!("../assets/fonts/MiSans/MiSans-Medium.ttf").as_slice(),
                )],
            );
            assert!(report.failures.is_empty(), "{report:?}");
            assert!(is_font_family_available(cx, "MiSans"));
            init_liora_with_options(
                cx,
                Options::system().with_fonts(
                    FontConfig::system()
                        .with_ui_families(["MiSans", "Segoe UI", "Arial"])
                        .with_ui_weight(FontWeight::MEDIUM),
                ),
            );
            let handle = cx
                .open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                            point(px(0.), px(0.)),
                            size(px(900.), px(240.)),
                        ))),
                        ..Default::default()
                    },
                    |_, cx| cx.new(|_| liora_gallery::demos::typography_demo::MultilingualDemo),
                )
                .expect("open native glyph window");
            let executor = cx.background_executor().clone();
            cx.spawn(async move |cx| {
                executor.timer(Duration::from_millis(300)).await;
                cx.update_window(handle.into(), |_, window, cx| {
                    window.draw(cx).clear(cx);
                    let mut font = gpui::font(ui_font_family(cx).expect("configured UI font"));
                    font.weight = ui_font_weight(cx).expect("configured UI font weight");
                    let font_size = px(cx.global::<liora_core::Config>().theme.font_size.md);
                    let system = window.text_system();
                    let primary = system.resolve_font(&font);
                    for text in ["Liora", "中文", "日本語", "한국어", "😀", "🚀", "e\u{301}"]
                    {
                        let mut run = window.text_style().to_run(text.len());
                        run.font = font.clone();
                        let line = system.shape_line(text.into(), font_size, &[run], None);
                        assert!(line.width() > px(0.), "empty layout for {text}");
                        for (index, _) in text.char_indices() {
                            // Combining marks can share their base's source cluster.
                            if text == "e\u{301}" && index != 0 {
                                continue;
                            }
                            assert!(
                                line.runs.iter().any(|run| run
                                    .glyphs
                                    .iter()
                                    .any(|glyph| glyph.index == index && glyph.id.0 != 0)),
                                "missing glyph cluster for {text} at {index}: {line:?}"
                            );
                        }
                        assert!(
                            line.runs
                                .iter()
                                .all(|run| run.glyphs.iter().all(|g| g.id.0 != 0))
                        );
                        if text == "😀" || text == "🚀" {
                            assert!(line.runs.iter().any(|run| run.font_id != primary));
                        }
                        for run in &line.runs {
                            for glyph in &run.glyphs {
                                let params = RenderGlyphParams {
                                    font_id: run.font_id,
                                    glyph_id: glyph.id,
                                    font_size,
                                    subpixel_variant: point(0, 0),
                                    scale_factor: window.scale_factor(),
                                    is_emoji: glyph.is_emoji,
                                    subpixel_rendering: false,
                                    dilation: 0,
                                };
                                let bounds = rasterizer
                                    .glyph_raster_bounds(&params)
                                    .expect("native glyph raster bounds");
                                assert!(
                                    !bounds.is_zero(),
                                    "empty native raster for {text}: {params:?}"
                                );
                                let (_, pixels) = rasterizer
                                    .rasterize_glyph(&params, bounds)
                                    .expect("native glyph raster pixels");
                                let visible = if glyph.is_emoji {
                                    pixels.chunks_exact(4).any(|pixel| pixel[3] != 0)
                                } else {
                                    pixels.iter().any(|pixel| *pixel != 0)
                                };
                                assert!(visible, "invisible native raster for {text}: {params:?}");
                            }
                        }
                        println!("GLYPHS {text}: {line:?}");
                    }
                })
                .expect("shape native multilingual glyphs");
                std::fs::write(output.join("ready"), "glyph assertions passed\n")
                    .expect("write capture readiness");
                for _ in 0..200 {
                    if output.join("captured").exists() {
                        println!("PASS native multilingual glyphs and capture handshake");
                        cx.update(|cx| cx.quit());
                        return;
                    }
                    executor.timer(Duration::from_millis(50)).await;
                }
                panic!("native screenshot handshake timed out");
            })
            .detach();
        });
}
