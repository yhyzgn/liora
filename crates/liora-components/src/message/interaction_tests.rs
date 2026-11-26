use super::*;
use gpui::TestAppContext;

fn drain(app: &TestAppContext) {
    for _ in 0..100 {
        if !app.executor().tick() {
            return;
        }
    }
    panic!("message tasks did not park");
}

fn advance(app: &TestAppContext, millis: u64) {
    app.executor().advance_clock(Duration::from_millis(millis));
    drain(app);
}

#[test]
fn replaced_manager_does_not_expire_its_successor_messages() {
    let app = TestAppContext::single();
    app.update(crate::init_liora);
    app.update(|cx| show_message("old", MessageType::Info, cx));
    drain(&app);
    advance(&app, 1000);
    let current = app.update(|cx| {
        let current = cx.new(|_| MessageManager::new());
        cx.set_global(MessageManagerGlobal(current.clone()));
        show_message("new", MessageType::Success, cx);
        current
    });
    drain(&app);
    advance(&app, 2000);
    assert_eq!(current.read_with(&app, |value, _| value.messages.len()), 1);
    advance(&app, 1000);
    assert_eq!(current.read_with(&app, |value, _| value.messages.len()), 0);
    app.quit();
}

fn messages(
    manager: &Entity<MessageManager>,
    app: &TestAppContext,
) -> Vec<(SharedString, MessageType)> {
    manager.read_with(app, |value, _| {
        value
            .messages
            .iter()
            .map(|entry| (entry.item.content.clone(), entry.item.msg_type))
            .collect()
    })
}

#[test]
fn stacked_messages_expire_after_their_own_three_second_deadlines() {
    let app = TestAppContext::single();
    app.update(crate::init_liora);
    let manager = app.update(|cx| {
        toast_info("first", cx);
        cx.global::<MessageManagerGlobal>().0.clone()
    });
    drain(&app);
    advance(&app, 1000);
    app.update(|cx| toast_success("second", cx));
    drain(&app);
    advance(&app, 1999);
    assert_eq!(
        messages(&manager, &app),
        [
            ("first".into(), MessageType::Info),
            ("second".into(), MessageType::Success)
        ]
    );
    advance(&app, 1);
    assert_eq!(
        messages(&manager, &app),
        [("second".into(), MessageType::Success)]
    );
    advance(&app, 1000);
    assert!(messages(&manager, &app).is_empty());
    app.quit();
}

#[test]
fn toast_macros_deliver_formatted_content_and_semantic_types() {
    let app = TestAppContext::single();
    app.update(crate::init_liora);
    let name = "Liora";
    crate::toast_info!("{}, {} messages", name, 4);
    crate::toast_success!("{name} completed");
    crate::toast_warning!(SharedString::from("check input"));
    crate::toast_error!("operation failed");
    drain(&app);
    let manager = app.update(|cx| cx.global::<MessageManagerGlobal>().0.clone());
    assert_eq!(
        messages(&manager, &app),
        [
            ("Liora, 4 messages".into(), MessageType::Info),
            ("Liora completed".into(), MessageType::Success),
            ("check input".into(), MessageType::Warning),
            ("operation failed".into(), MessageType::Error),
        ]
    );
    advance(&app, 3000);
    assert!(messages(&manager, &app).is_empty());
    app.quit();
}

#[test]
fn toast_functions_preserve_content_and_semantic_types() {
    let app = TestAppContext::single();
    app.update(crate::init_liora);
    app.update(|cx| {
        toast_info("info", cx);
        toast_success("success", cx);
        toast_warning("warning", cx);
        toast_error("error", cx);
    });
    let manager = app.update(|cx| cx.global::<MessageManagerGlobal>().0.clone());
    assert_eq!(
        messages(&manager, &app),
        [
            ("info".into(), MessageType::Info),
            ("success".into(), MessageType::Success),
            ("warning".into(), MessageType::Warning),
            ("error".into(), MessageType::Error),
        ]
    );
    app.quit();
}

#[test]
fn dropping_manager_releases_pending_message_tasks_without_retaining_entity() {
    let app = TestAppContext::single();
    app.update(crate::init_liora);
    let manager = app.update(|cx| {
        toast_info("first", cx);
        cx.global::<MessageManagerGlobal>().0.clone()
    });
    let weak = manager.downgrade();
    app.update(|cx| {
        cx.remove_global::<MessageManagerGlobal>();
    });
    drop(manager);
    drain(&app);
    assert!(weak.upgrade().is_none());
    advance(&app, 5000);
    app.quit();
}
