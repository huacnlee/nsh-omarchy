//! The overlays pages.

use nsh_core::prelude::*;

use super::placeholder;
use crate::registry::Page;

pub fn dialog() -> AnyElement {
    placeholder("dialog")
}

pub fn alert_dialog() -> AnyElement {
    placeholder("alert_dialog")
}

pub fn sheet() -> AnyElement {
    placeholder("sheet")
}

pub fn popover() -> AnyElement {
    placeholder("popover")
}

pub fn tooltip() -> AnyElement {
    placeholder("tooltip")
}

pub fn hover_card() -> AnyElement {
    placeholder("hover_card")
}

pub fn toast() -> AnyElement {
    placeholder("toast")
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Overlays", id: "dialog", title: "Dialog", make: dialog },
    Page { group: "Overlays", id: "alert_dialog", title: "Alert dialog", make: alert_dialog },
    Page { group: "Overlays", id: "sheet", title: "Sheet", make: sheet },
    Page { group: "Overlays", id: "popover", title: "Popover", make: popover },
    Page { group: "Overlays", id: "tooltip", title: "Tooltip", make: tooltip },
    Page { group: "Overlays", id: "hover_card", title: "Hover card", make: hover_card },
    Page { group: "Overlays", id: "toast", title: "Toast", make: toast },
];
