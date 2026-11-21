use super::*;

#[gpui::test]
fn frame_setting_retains_navigation_input_and_bounds(cx: &mut gpui::TestAppContext) {
    use gpui::{point, size, Bounds, Focusable, WindowBounds, WindowOptions};
    cx.update(|cx| crate::init_liora_with_options(cx, crate::docs_liora_options()));
    let bounds = WindowBounds::Windowed(Bounds::new(
        point(px(73.), px(91.)),
        size(px(1100.), px(760.)),
    ));
    let (root, startup) = cx.update(|cx| {
        let root = render_docs_shell(
            WindowFrameMode::System,
            crate::set_docs_frame_mode,
            crate::request_docs_window_close,
            cx,
        );
        root.update(cx, |docs, _| {
            docs.update_status = UpdatePanelStatus::Checking
        });
        let handle = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(bounds),
                    ..Default::default()
                },
                |window, _| {
                    window.set_rem_size(px(19.));
                    root.clone()
                },
            )
            .unwrap();
        (root, handle)
    });
    root.update(cx, |docs, cx| {
        docs.selected = DOC_PAGES
            .iter()
            .position(|page| page.title == "Input")
            .unwrap();
        docs.nav_query = "input".into();
        docs.nav_filter
            .update(cx, |input, cx| input.set_value("Input", cx));
    });
    let mut handle: gpui::AnyWindowHandle = startup.into();
    for mode in [WindowFrameMode::Custom, WindowFrameMode::System] {
        handle
            .update(cx, |_, window, cx| {
                let switch = root.read(cx).frame_mode_switch.clone();
                window.focus(&switch.read(cx).focus_handle(cx), cx);
                window.draw(cx).clear(cx);
                window.dispatch_action(Box::new(liora_components::switch::SwitchToggle), cx);
            })
            .unwrap();
        cx.run_until_parked();
        let windows = cx.windows();
        assert_eq!(windows.len(), 1);
        handle = windows[0];
        handle
            .update(cx, |actual, window, cx| {
                assert_eq!(actual.entity_id(), root.entity_id());
                assert_eq!(window.window_bounds(), bounds);
                assert_eq!(window.rem_size(), px(19.));
                let docs = root.read(cx);
                assert_eq!(DOC_PAGES[docs.selected].title, "Input");
                assert_eq!(docs.nav_filter.read(cx).value().as_ref(), "Input");
                assert_eq!(docs.nav_query, "input");
                assert_eq!(docs.frame_mode, mode);
            })
            .unwrap();
    }
    handle
        .update(cx, |_, window, _| window.remove_window())
        .unwrap();
}
