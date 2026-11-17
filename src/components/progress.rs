//! Omarchy progress: a thin inset track with an accent fill, on
//! `nsh_base`'s progress bar (determinate, 0 to 100).

use nsh_base::{ProgressElement, progress as base_progress};
use nsh_core::prelude::*;

use crate::tokens;

/// The bar's thickness.
const BAR: f32 = 6.0;

/// An Omarchy progress bar, full width. Set `.value(..)` (a percentage,
/// clamped to 0..=100; NaN is 0) and `.accessibility_label(..)` for what is
/// in progress. Children (a caption, the value as text) sit under the bar.
///
/// ```ignore
/// progress().accessibility_label("Download").value(move || done.get())
/// ```
pub fn progress() -> ProgressElement {
    base_progress()
        .w_full()
        .gap(px(tokens::SPACE_1))
        .text_size(px(tokens::TEXT_SM))
        .text_color(move || tokens::secondary())
        .track_style(|s| s.w_full().h(px(BAR)).bg(move || tokens::inset()))
        .indicator_style(|s| s.h_full().bg(move || tokens::accent()))
}

/// A progress bar whose fill is `color` instead of the accent: a status
/// (`tokens::success`, `tokens::warning`, `tokens::danger`).
pub fn progress_tone(color: fn() -> AdaptiveColor) -> ProgressElement {
    progress().indicator_style(move |s| s.h_full().bg(move || color()))
}
