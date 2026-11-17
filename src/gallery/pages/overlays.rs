//! Gallery pages of the `overlays` group. Each `pub fn` is one page.

use nsh_core::prelude::*;

pub fn dialog() -> AnyElement {
    super::placeholder("dialog")
}

pub fn alert_dialog() -> AnyElement {
    super::placeholder("alert_dialog")
}

pub fn sheet() -> AnyElement {
    super::placeholder("sheet")
}

pub fn popover() -> AnyElement {
    super::placeholder("popover")
}

pub fn tooltip() -> AnyElement {
    super::placeholder("tooltip")
}

pub fn hover_card() -> AnyElement {
    super::placeholder("hover_card")
}

pub fn toast() -> AnyElement {
    super::placeholder("toast")
}

