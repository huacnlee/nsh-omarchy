//! Omarchy's look on NSH: components for Rust and TSX apps, drawn with the
//! active Omarchy theme.
//!
//! ```ignore
//! use nsh_omarchy as om;
//!
//! #[component]
//! fn App() -> impl IntoElement {
//!     om::init();
//!     view().child(om::button("Save").primary().on_press(|| save()))
//! }
//! ```
//!
//! A TSX app installs the crate with `nsh add nsh-omarchy` and imports the
//! same components: `import { Button } from "nsh-omarchy"`. The API is
//! described in `docs/api.md`.

mod components;
mod system_theme;
mod ui;

mod actions;
mod choices;
mod data;
mod display;
mod fields;
mod navigation;
mod overlays;
mod types;

pub mod theme;
pub mod tokens;

pub use actions::*;
pub use choices::*;
pub use data::*;
pub use display::*;
pub use fields::*;
pub use navigation::*;
pub use overlays::*;
pub use types::*;
pub use components::button::{ButtonVariant, ControlSize};
pub use components::icon::IconName;
pub use system_theme::{ThemeLoadError, follow_system, stop_following};
pub use theme::{ActiveTheme, Appearance, Theme, active_theme};

use nsh_core::prelude::*;

/// Applies the Omarchy system theme (Tokyo Night where there is none) and
/// follows its changes. Call it once, in the root component's body.
pub fn init() {
    system_theme::follow_system();
}

/// What a TSX app imports from `nsh-omarchy` (installed with
/// `nsh add nsh-omarchy`): the `Omarchy` theme module and the components.
#[native_module("nsh-omarchy")]
fn native(registry: &mut Registry) {
    ui::register(registry);
}
