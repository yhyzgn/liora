//! Disabled Link state.

use liora_components::Link;

pub fn disabled_link() -> Link {
    Link::new("Disabled")
        .disabled(true)
        .href("https://github.com")
}
