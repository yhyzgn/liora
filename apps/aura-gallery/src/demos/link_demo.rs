use aura_components::{Link, Space, Title};
use aura_icons_lucide::IconName;
use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};

pub fn render(cx: &mut App) -> Entity<LinkDemo> {
    cx.new(|_| LinkDemo)
}

pub struct LinkDemo;

impl Render for LinkDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Space::new()
            .vertical()
            .gap_md()
            .child(hdr("Variants 类型"))
            .child(row(vec![
                Link::new("Default").href("https://github.com"),
                Link::new("Primary").primary().href("https://github.com"),
                Link::new("Success").success().href("https://github.com"),
                Link::new("Warning").warning().href("https://github.com"),
                Link::new("Danger").danger().href("https://github.com"),
                Link::new("Info")
                    .info()
                    .href("https://github.com".to_string()),
            ]))
            .child(hdr("Underline 下划线"))
            .child(row(vec![
                Link::new("With underline").href("https://github.com"),
                Link::new("No underline")
                    .underline(false)
                    .href("https://github.com"),
            ]))
            .child(hdr("Disabled 禁用"))
            .child(row(vec![
                Link::new("Disabled")
                    .disabled(true)
                    .href("https://github.com"),
            ]))
            .child(hdr("Icons 图标"))
            .child(row(vec![
                Link::new("GitHub")
                    .icon_start(IconName::ExternalLink)
                    .href("https://github.com"),
                Link::new("Home")
                    .icon_start(IconName::House)
                    .href("https://example.com"),
            ]))
    }
}

fn hdr(s: &str) -> impl IntoElement {
    Title::new(s.to_string()).h3()
}

fn row(elements: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().wrap().gap_lg().children(elements)
}
