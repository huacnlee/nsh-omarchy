//! Gallery pages, one module per group.

use nsh_core::prelude::*;

pub mod actions;
pub mod choices;
pub mod fields;
pub mod navigation;
pub mod overlays;
pub mod display;
pub mod data;
pub mod charts;

/// A page not built yet.
pub fn placeholder(name: &'static str) -> AnyElement {
    text(format!("{name}: coming soon"))
        .text_color(move || crate::tokens::secondary())
        .into_element()
}
