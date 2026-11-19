//! The navigation pages.

use nsh_core::prelude::*;

use super::placeholder;
use crate::registry::Page;

pub fn tabs() -> AnyElement {
    placeholder("tabs")
}

pub fn menu() -> AnyElement {
    placeholder("menu")
}

pub fn context_menu() -> AnyElement {
    placeholder("context_menu")
}

pub fn accordion() -> AnyElement {
    placeholder("accordion")
}

pub fn collapsible() -> AnyElement {
    placeholder("collapsible")
}

pub fn pagination() -> AnyElement {
    placeholder("pagination")
}

pub fn sidebar() -> AnyElement {
    placeholder("sidebar")
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Navigation", id: "tabs", title: "Tabs", make: tabs },
    Page { group: "Navigation", id: "menu", title: "Menu", make: menu },
    Page { group: "Navigation", id: "context_menu", title: "Context menu", make: context_menu },
    Page { group: "Navigation", id: "accordion", title: "Accordion", make: accordion },
    Page { group: "Navigation", id: "collapsible", title: "Collapsible", make: collapsible },
    Page { group: "Navigation", id: "pagination", title: "Pagination", make: pagination },
    Page { group: "Navigation", id: "sidebar", title: "Sidebar", make: sidebar },
];
