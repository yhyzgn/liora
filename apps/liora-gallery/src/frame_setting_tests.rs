use super::*;

#[gpui::test]
fn frame_setting_retains_navigation_input_and_bounds(cx: &mut gpui::TestAppContext) {
    use gpui::{Bounds, Focusable, WindowBounds, WindowOptions, point};
    cx.update(|cx| init_liora_with_options(cx, gallery_liora_options()));
    let startup = cx.update(|cx| open_gallery_window(cx).unwrap());
    let root = startup
        .update(cx, |root, _, _| root.downcast::<Gallery>().unwrap())
        .unwrap();
    root.update(cx, |gallery, cx| {
        gallery.selected = gallery
            .entries
            .iter()
            .position(|entry| entry.name == "Input 输入框")
            .unwrap();
        gallery.nav_query = "input".into();
        gallery
            .nav_filter
            .update(cx, |input, cx| input.set_value("Input", cx));
    });
    let bounds = WindowBounds::Windowed(Bounds::new(
        point(px(73.), px(91.)),
        size(px(1100.), px(760.)),
    ));
    let mut handle: gpui::AnyWindowHandle = cx.update(|cx| {
        cx.open_window(
            WindowOptions {
                window_bounds: Some(bounds),
                ..Default::default()
            },
            |window, _| {
                window.set_rem_size(px(19.));
                root.clone()
            },
        )
        .unwrap()
        .into()
    });
    startup
        .update(cx, |_, window, _| window.remove_window())
        .unwrap();
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
                let gallery = root.read(cx);
                assert_eq!(gallery.entries[gallery.selected].name, "Input 输入框");
                assert_eq!(gallery.nav_filter.read(cx).value().as_ref(), "Input");
                assert_eq!(gallery.nav_query, "input");
                assert_eq!(gallery.frame_mode, mode);
            })
            .unwrap();
    }
    handle
        .update(cx, |_, window, _| window.remove_window())
        .unwrap();
}
