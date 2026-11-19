//! The types several components share.

use nsh_core::prelude::*;

pub use nsh_core::geometry::Axis;
pub use nsh_core::overlay::{Align, Placement};

/// A semantic color for a fill or an action: a progress bar, an alert
/// dialog's action.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PropEnum)]
#[non_exhaustive]
pub enum Tone {
    /// The theme's accent: the default.
    #[default]
    Accent,
    /// Done, healthy.
    Success,
    /// Needs attention.
    Warning,
    /// Destructive or failed.
    Danger,
}
