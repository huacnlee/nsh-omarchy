//! Gallery pages of the `choices` group. Each `pub fn` is one page.

use nsh_core::prelude::*;

pub fn checkbox() -> AnyElement {
    super::placeholder("checkbox")
}

pub fn switch() -> AnyElement {
    super::placeholder("switch")
}

pub fn radio() -> AnyElement {
    super::placeholder("radio")
}

pub fn slider() -> AnyElement {
    super::placeholder("slider")
}

pub fn progress() -> AnyElement {
    super::placeholder("progress")
}

