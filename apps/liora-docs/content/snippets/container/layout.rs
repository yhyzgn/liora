//! Page skeleton with Header, Aside, Main, and Footer.

use gpui::IntoElement;
use liora_components::{Container, Flex, Text, Title};

pub fn container_layout() -> impl IntoElement {
    Flex::new().height_units(300.0).w_full().border().child(
        Container::new()
            .header(Title::new("Header").h5())
            .aside(Flex::new().padding_md().child(Text::new("Aside Sidebar")))
            .footer(Text::new("Footer"))
            .child(
                Flex::new()
                    .padding_md()
                    .child(Text::new("Main Content Area")),
            ),
    )
}
