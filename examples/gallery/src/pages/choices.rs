//! The choices pages.

use nsh_core::prelude::*;

use super::placeholder;
use crate::registry::Page;

pub fn checkbox() -> AnyElement {
    placeholder("checkbox")
}

pub fn switch() -> AnyElement {
    placeholder("switch")
}

pub fn radio() -> AnyElement {
    placeholder("radio")
}

pub fn slider() -> AnyElement {
    placeholder("slider")
}

pub fn progress() -> AnyElement {
    placeholder("progress")
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Forms", id: "checkbox", title: "Checkbox", make: checkbox },
    Page { group: "Forms", id: "switch", title: "Switch", make: switch },
    Page { group: "Forms", id: "radio", title: "Radio", make: radio },
    Page { group: "Forms", id: "slider", title: "Slider", make: slider },
    Page { group: "Forms", id: "progress", title: "Progress", make: progress },
];
