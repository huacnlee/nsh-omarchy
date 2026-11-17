//! Gallery pages of the `fields` group. Each `pub fn` is one page.

use nsh_core::prelude::*;

pub fn input() -> AnyElement {
    super::placeholder("input")
}

pub fn textarea() -> AnyElement {
    super::placeholder("textarea")
}

pub fn number_input() -> AnyElement {
    super::placeholder("number_input")
}

pub fn otp_input() -> AnyElement {
    super::placeholder("otp_input")
}

pub fn select() -> AnyElement {
    super::placeholder("select")
}

pub fn combobox() -> AnyElement {
    super::placeholder("combobox")
}

