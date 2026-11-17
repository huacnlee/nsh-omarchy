//! [`otp_field`]: a numeric one-time code entered into square cells.
//!
//! `nsh_base::otp_input` owns the keyboard with one native field over the
//! cells (typing, paste, autofill, full-width digits as ASCII ones,
//! Backspace); this module draws the cells in Omarchy's control look: a
//! `normal_fill` well with a `control_border`, the cell that takes the next
//! digit in the accent border while the code has the focus, with a
//! blinking accent caret while it is empty (steady with reduced motion).
//! The name avoids `nsh_base::otp_input`.

use nsh_base::{OtpInputElement, otp_input};
use nsh_core::element::StyleRefinement;
use nsh_core::prelude::*;

use super::input::state_classes::{CONTROL_BORDER, DISABLED, FOCUS_BORDER};
use crate::tokens;

/// A cell's width; its height is `CONTROL_LG`.
const CELL_WIDTH: f32 = 28.0;
/// The caret's height inside an empty cell.
const CARET_HEIGHT: f32 = 14.0;

/// A one-time code of `length` digits, one cell each. Controlled:
/// `.value(..)` (an `Option<SharedString>`) plus
/// `.on_change(move |code: &SharedString| ..)`; `.on_complete(..)` hears
/// the full code. `.groups(2)` splits six cells 3 + 3, `.masked(true)`
/// shows bullets. Give it an `accessibility_label` naming what the code
/// is for.
pub fn otp_field(length: u32) -> OtpInputElement {
    otp_input()
        .length(length)
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_3))
        .group_style(|s: StyleRefinement| s.flex_row().gap(px(tokens::SPACE_1)))
        .cell_class(SharedString::from(format!(
            "{CONTROL_BORDER} {FOCUS_BORDER} {DISABLED}"
        )))
        .cell_style(|s: StyleRefinement| {
            s.w(px(CELL_WIDTH))
                .h(px(tokens::CONTROL_LG))
                .items_center()
                .justify_center()
                .rounded(px(tokens::RADIUS))
                // Its border color is the class's, so `focus:` can win.
                .border_1()
                .bg(move || tokens::normal_fill())
                .text_size(px(tokens::TEXT_LG))
                .text_color(move || tokens::bright())
                .font_family(move || tokens::font_mono())
                .transition_colors()
                .duration(tokens::DURATION_FAST)
        })
        .caret_style(|s: StyleRefinement| {
            s.w(px(tokens::FOCUS_BORDER))
                .h(px(CARET_HEIGHT))
                .bg(move || tokens::accent())
        })
}
