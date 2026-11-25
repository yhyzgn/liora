//! Page skeleton with Header, Aside, Main, and Footer.

use gpui::IntoElement;
use liora_components::{Container, Flex, Space, Text, Title};

pub fn container_layout() -> impl IntoElement {
    Flex::new().height_units(300.0).w_full().border().child(
        Container::new()
            .header(
                Space::new()
                    .wrap()
                    .child(Title::new("Header").h5())
                    .child(Text::new("Wrapping header content"))
                    .child(Text::new("Theme / Language / Frame")),
            )
            .header_height_auto()
            .aside(Flex::new().padding_md().child(Text::new("Aside Sidebar")))
            .footer(Text::new("Footer"))
            .child(
                Flex::new()
                    .padding_md()
                    .child(Text::new("Main Content Area")),
            ),
    )
}
