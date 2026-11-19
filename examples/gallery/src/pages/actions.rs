//! The actions pages.

use nsh_core::prelude::*;

use super::placeholder;
use crate::registry::Page;

pub fn button() -> AnyElement {
    placeholder("button")
}

pub fn toggle() -> AnyElement {
    placeholder("toggle")
}

pub fn toggle_group() -> AnyElement {
    placeholder("toggle_group")
}

pub fn button_group() -> AnyElement {
    placeholder("button_group")
}

pub fn link() -> AnyElement {
    placeholder("link")
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Actions", id: "button", title: "Button", make: button },
    Page { group: "Actions", id: "toggle", title: "Toggle", make: toggle },
    Page { group: "Actions", id: "toggle_group", title: "Toggle group", make: toggle_group },
    Page { group: "Actions", id: "button_group", title: "Button group", make: button_group },
    Page { group: "Actions", id: "link", title: "Link", make: link },
];
