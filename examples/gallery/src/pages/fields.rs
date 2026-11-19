//! The fields pages.

use nsh_core::prelude::*;

use super::placeholder;
use crate::registry::Page;

pub fn input() -> AnyElement {
    placeholder("input")
}

pub fn textarea() -> AnyElement {
    placeholder("textarea")
}

pub fn number_input() -> AnyElement {
    placeholder("number_input")
}

pub fn otp_input() -> AnyElement {
    placeholder("otp_input")
}

pub fn select() -> AnyElement {
    placeholder("select")
}

pub fn combobox() -> AnyElement {
    placeholder("combobox")
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Forms", id: "input", title: "Input", make: input },
    Page { group: "Forms", id: "textarea", title: "Textarea", make: textarea },
    Page { group: "Forms", id: "number_input", title: "Number input", make: number_input },
    Page { group: "Forms", id: "otp_input", title: "Otp input", make: otp_input },
    Page { group: "Forms", id: "select", title: "Select", make: select },
    Page { group: "Forms", id: "combobox", title: "Combobox", make: combobox },
];
