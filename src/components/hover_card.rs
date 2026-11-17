//! Omarchy hover cards: a preview of what the trigger points at.
//!
//! [`hover_card`] is `nsh_base::hover_card()` with the popover's look and
//! entrance. It opens after the pointer rests 400 ms on the trigger and
//! closes 200 ms after the pointer left both; Escape and a press outside
//! close it. Base follows the pointer only, so keep what a keyboard user
//! needs reachable outside the card.
//!
//! ```ignore
//! hover_card()
//!     .trigger(text("@omarchy"))
//!     .child(popover_title("Omarchy"))
//!     .child(text("An opinionated Arch setup."))
//! ```

use nsh_core::prelude::*;

use super::popover::{popup_ending, popup_starting, popup_surface};
use crate::tokens;

const HOVER_OPEN_DELAY_MS: f64 = 400.0;
const HOVER_CLOSE_DELAY_MS: f64 = 200.0;

/// A hover card with Omarchy's popup look. Give it a `.trigger(..)` and
/// children (made on each opening).
pub fn hover_card() -> nsh_base::HoverCardElement {
    nsh_base::hover_card()
        .open_delay(HOVER_OPEN_DELAY_MS)
        .close_delay(HOVER_CLOSE_DELAY_MS)
        .offset(px(tokens::SPACE_1))
        .margin(px(tokens::SPACE_2))
        .popup_style(|s| popup_surface(s).w(px(280.0)))
        .popup_starting_style(popup_starting)
        .popup_ending_style(popup_ending)
}
