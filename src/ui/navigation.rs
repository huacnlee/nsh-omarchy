//! Navigation: tabs, the sidebar, accordions, collapsibles, pagination,
//! the title bar. (The menus are in [`super::menu`].)

use nsh_core::prelude::*;

/// Registers this area's components for TSX.
pub(crate) fn register(registry: &mut Registry) {
    super::menu::register(registry);
}
