//! Omarchy slider: an inset rail with an accent fill up to a square thumb,
//! on `nsh_base`'s slider (one value, or a range with two thumbs).
//!
//! Keys on the focused thumb: Left/Down and Right/Up step, PageUp/PageDown
//! jump a page, Home/End go to the ends; a press on the rail moves the
//! nearer thumb there and a drag follows the pointer.
//!
//! States: the thumb is an accent outline on the background; hover fills it
//! with the selection, a drag and focus-visible fill it with the accent
//! (focus also brightens its border). Colors fade over `DURATION_FAST`;
//! the thumb follows the value at once, so a drag tracks the pointer
//! exactly. Disabled greys the fill and the thumb and takes no changes.

use nsh_base::{SliderElement, slider as base_slider};
use nsh_core::prelude::*;

use crate::tokens;

/// The rail's thickness.
const RAIL: f32 = 4.0;
/// The thumb's side.
const THUMB: f32 = 16.0;
/// The thumb's border.
const THUMB_BORDER: f32 = 2.0;

/// The fill: `tokens::accent`, `tokens::disabled` while disabled.
const RANGE_CLASS: &str = "bg-omarchy-accent disabled:bg-foreground/38";
/// The thumb: an accent outline on `tokens::background`; hover fills it
/// with `tokens::selection`, a drag and focus-visible with the accent, and
/// focus-visible brightens the outline (`tokens::bright`); disabled greys
/// the outline.
const THUMB_CLASS: &str = "border-omarchy-accent bg-background hover:bg-selection \
     active:bg-omarchy-accent focus-visible:bg-omarchy-accent focus-visible:border-bright \
     disabled:border-foreground/38";

/// An Omarchy slider, full width, `CONTROL_MD` tall. Controlled: set
/// `.value(..)` (a number, or `(start, end)` for a range), `.min(..)`,
/// `.max(..)`, `.step(..)`, `.accessibility_label(..)` and write the value
/// `.on_change(..)` reports. Children (the value as text) sit after the rail.
///
/// ```ignore
/// slider()
///     .accessibility_label("Volume")
///     .value(move || SliderValue::Single(volume.get()))
///     .on_change(move |v: &SliderValue| volume.set(v.end()))
/// ```
pub fn slider() -> SliderElement {
    base_slider()
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_3))
        .w_full()
        .track_style(|s| {
            s.flex_1()
                .h(px(tokens::CONTROL_MD))
                .justify_center()
                // Room for half a thumb past each end of the rail.
                .px(px(THUMB / 2.0))
        })
        .indicator_style(|s| s.h(px(RAIL)).bg(move || tokens::inset()))
        // State colors (hover, a drag, keyboard focus, disabled) are only
        // reachable by class on these parts; the classes name the same
        // theme tokens `crate::tokens` reads (see `theme.rs`), and the
        // inline styles below set geometry and motion only, as an inline
        // style is over the classes.
        .range_class(RANGE_CLASS)
        .range_style(|s| s.transition_colors().duration(tokens::DURATION_FAST))
        .thumb_class(THUMB_CLASS)
        .thumb_style(|s| {
            s.w(px(THUMB))
                .h(px(THUMB))
                .border(px(THUMB_BORDER))
                .transition_colors()
                .duration(tokens::DURATION_FAST)
        })
}
