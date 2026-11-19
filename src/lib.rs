//! Omarchy's presentation language on NSH's `nsh_base` behavior.
//!
//! Call [`init`] in the root component's body, then build screens from the
//! constructors in [`components`]. Every color comes from [`tokens`], which
//! follow the active [`Theme`].

pub mod components;
pub mod gallery;
pub mod system_theme;
pub mod theme;
pub mod tokens;
pub mod tsx;

pub use components::*;
pub use system_theme::follow_system;
pub use theme::{ActiveTheme, Appearance, Theme, active_theme};

use nsh_core::prelude::*;

/// Applies the Omarchy system theme (Tokyo Night when unavailable) and
/// follows its changes. Call it once, in the root component's body.
pub fn init() {
    system_theme::follow_system();
}

/// The UI set a TSX entry may import from `nsh-omarchy`: the `Omarchy`
/// theme module, the charts, `Markdown`, `Icon` and the `Gallery`.
#[native_module("nsh-omarchy")]
fn native(registry: &mut Registry) {
    registry
        .module::<tsx::Omarchy>()
        .register::<tsx::LineChart>()
        .register::<tsx::AreaChart>()
        .register::<tsx::BarChart>()
        .register::<tsx::PieChart>()
        .register::<components::text_view::Markdown>()
        .register::<tsx::Icon>()
        .register::<tsx::Button>()
        .register::<tsx::IconButton>()
        .register::<tsx::DropdownMenu>()
        .register::<tsx::ContextMenu>()
        .register::<tsx::MenuItem>()
        .register::<tsx::MenuSeparator>()
        .register::<tsx::MenuLabel>()
        .register::<gallery::Gallery>();
}
