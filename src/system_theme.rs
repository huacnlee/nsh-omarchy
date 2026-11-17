//! Reading and following the Omarchy system theme.
//!
//! Owned by the theme agent: the parser of `colors.toml` (ANSI and semantic
//! formats), the theme locations and the watcher live here.

use crate::theme::Theme;

/// Stops following the system theme (an explicit theme was applied).
pub fn stop_following() {}

/// Applies the system theme now and follows its changes. Call it from the
/// root component's body.
pub fn follow_system() {
    Theme::system_or_default().apply_palette();
}

impl Theme {
    /// The current system theme, or Tokyo Night when it cannot be read.
    pub fn system_or_default() -> Self {
        Theme::tokyo_night()
    }
}
