//! The components: one `#[component]` each, drawn with the tokens and the
//! private drawing helpers of [`crate::components`]. TSX imports them from
//! `nsh-omarchy`; Rust builds them with the constructors at the crate root
//! (this module is private, so the macro's no-argument constructors are
//! not public).

#[macro_use]
mod shorthand;

pub mod actions;
pub mod button;
#[cfg(feature = "chart")]
pub mod charts;
pub mod choices;
pub mod data;
pub mod display;
pub mod fields;
pub mod icon;
pub mod menu;
pub mod navigation;
pub mod omarchy;
pub mod overlays;

use nsh_core::prelude::*;

/// Registers every component and module TSX imports from `nsh-omarchy`.
pub(crate) fn register(registry: &mut Registry) {
    registry.module::<omarchy::Omarchy>();
    actions::register(registry);
    choices::register(registry);
    fields::register(registry);
    navigation::register(registry);
    overlays::register(registry);
    data::register(registry);
    display::register(registry);
}
