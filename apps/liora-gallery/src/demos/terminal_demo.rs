use gpui::{AnyView, App, Context, Entity, Render, Subscription, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Space, Terminal, TerminalEvent, TerminalSize, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let terminal = cx.new(|cx| Terminal::new(TerminalSize::new(12, 72).unwrap(), 500, cx));
        terminal.update(cx, |terminal, cx| terminal.process_output(
            b"Liora native VT screen (no shell or PTY)\r\n\x1b[31mred\x1b[0m / \x1b[32mgreen\x1b[0m / \x1b[1mbold\x1b[0m\r\nDrag to select; Ctrl-Shift-C copies.\r\nKeys emit bytes to the host event log.\r\n", cx));
        let subscription = cx.subscribe(&terminal, |this: &mut TerminalDemo, _, event, cx| {
            this.latest = match event {
                TerminalEvent::InputBytes(bytes) => format!("Host received input bytes: {bytes:02x?}"),
                TerminalEvent::Resized(size) => format!("Host received resize: {} rows × {} cols", size.rows(), size.cols()),
            };
            cx.notify();
        });
        TerminalDemo { terminal, latest: "No input received yet".into(), alternate: false, _subscription: subscription }
    }).into()
}

struct TerminalDemo {
    terminal: Entity<Terminal>,
    latest: String,
    alternate: bool,
    _subscription: Subscription,
}

impl Render for TerminalDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        page("Terminal 终端", "原生 VT/ANSI 屏幕；宿主负责进程与 I/O，本示例不启动 shell。",
            section("真实屏幕与输入事件", "可拖选复制、滚动历史、切换备用屏幕，按键在下方显示实际编码字节。",
                Space::new().vertical().gap_md()
                    .child(Space::new().wrap().gap_sm()
                        .child(Button::new("写入进度 / CR 覆盖").on_click(cx.listener(|this, _, _, cx| {
                            this.terminal.update(cx, |terminal, cx| terminal.process_output(b"progress 10%\rprogress 90%\r\n", cx));
                        })))
                        .child(Button::new("追加 30 行").on_click(cx.listener(|this, _, _, cx| {
                            let bytes = (1..=30).map(|line| format!("scrollback line {line}\r\n")).collect::<String>();
                            this.terminal.update(cx, |terminal, cx| terminal.process_output(bytes.as_bytes(), cx));
                        })))
                        .child(Button::new("切换备用屏幕").on_click(cx.listener(|this, _, _, cx| {
                            this.alternate = !this.alternate;
                            let bytes: &[u8] = if this.alternate { b"\x1b[?1049h\x1b[2J\x1b[HAlternate screen\r\n\x1b[44mBlue background\x1b[0m" } else { b"\x1b[?1049l" };
                            this.terminal.update(cx, |terminal, cx| terminal.process_output(bytes, cx));
                        })))
                        .child(Button::new("切换尺寸").on_click(cx.listener(|this, _, _, cx| {
                            this.terminal.update(cx, |terminal, cx| {
                                let rows = if terminal.size().rows() == 12 { 16 } else { 12 };
                                terminal.resize(TerminalSize::new(rows, 72).unwrap(), cx);
                            });
                        }))))
                    .child(self.terminal.clone())
                    .child(Text::new(self.latest.clone()).wrap())))
    }
}
