//! Docs authoring rule: one effect should own one adjacent code snippet.

struct DocsExample<'a> {
    title: &'a str,
    live_demo_key: &'a str,
    snippet_path: &'a str,
}

fn button_type_example() -> DocsExample<'static> {
    DocsExample {
        title: "类型",
        live_demo_key: "ButtonTypes",
        snippet_path: "button/types.rs",
    }
}

fn main() {
    let _ = button_type_example();
}
