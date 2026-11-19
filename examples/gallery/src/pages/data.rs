//! The data pages.

use nsh_core::prelude::*;

use super::placeholder;
use crate::registry::Page;

pub fn table() -> AnyElement {
    placeholder("table")
}

pub fn list() -> AnyElement {
    placeholder("list")
}

pub fn tree() -> AnyElement {
    placeholder("tree")
}

pub fn calendar() -> AnyElement {
    placeholder("calendar")
}

pub fn date_picker() -> AnyElement {
    placeholder("date_picker")
}

pub fn color_picker() -> AnyElement {
    placeholder("color_picker")
}

pub fn resizable() -> AnyElement {
    placeholder("resizable")
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Data", id: "table", title: "Table", make: table },
    Page { group: "Data", id: "list", title: "List", make: list },
    Page { group: "Data", id: "tree", title: "Tree", make: tree },
    Page { group: "Data", id: "calendar", title: "Calendar", make: calendar },
    Page { group: "Data", id: "date_picker", title: "Date picker", make: date_picker },
    Page { group: "Data", id: "color_picker", title: "Color picker", make: color_picker },
    Page { group: "Data", id: "resizable", title: "Resizable", make: resizable },
];
