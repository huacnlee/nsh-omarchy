//! The display pages.

use nsh_core::prelude::*;

use super::placeholder;
use crate::registry::Page;

pub fn panel() -> AnyElement {
    placeholder("panel")
}

pub fn separator() -> AnyElement {
    placeholder("separator")
}

pub fn badge() -> AnyElement {
    placeholder("badge")
}

pub fn keycap() -> AnyElement {
    placeholder("keycap")
}

pub fn empty_state() -> AnyElement {
    placeholder("empty_state")
}

pub fn avatar() -> AnyElement {
    placeholder("avatar")
}

pub fn icon() -> AnyElement {
    placeholder("icon")
}

pub fn typography() -> AnyElement {
    placeholder("typography")
}

pub fn markdown() -> AnyElement {
    placeholder("markdown")
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Display", id: "panel", title: "Panel", make: panel },
    Page { group: "Display", id: "separator", title: "Separator", make: separator },
    Page { group: "Display", id: "badge", title: "Badge", make: badge },
    Page { group: "Display", id: "keycap", title: "Keycap", make: keycap },
    Page { group: "Display", id: "empty_state", title: "Empty state", make: empty_state },
    Page { group: "Display", id: "avatar", title: "Avatar", make: avatar },
    Page { group: "Display", id: "icon", title: "Icon", make: icon },
    Page { group: "Display", id: "typography", title: "Typography", make: typography },
    Page { group: "Display", id: "markdown", title: "Markdown", make: markdown },
];
