//! Omarchy popovers: anchored, non-modal surfaces.
//!
//! [`popover`] is `nsh_base::popover()` with Omarchy's popup look: the
//! raised surface, a 1px border, square corners and a short fade-and-rise
//! entrance (`DURATION_BASE`, skipped under reduced motion). Base owns the
//! rest: the trigger toggles it, a press outside and Escape close it, and
//! the focus goes back to the trigger as it closes.
//!
//! ```ignore
//! popover()
//!     .trigger(overlay_action_button("Details", OverlayTone::Neutral, || {}))
//!     .child(popover_title("Workspace 3"))
//!     .child(text("2 windows"))
//! ```

use nsh_core::element::StyleRefinement;
use nsh_core::prelude::*;

use crate::tokens;

/// Omarchy's popup chrome on a part's style: surface, 1px border, square,
/// body text, and the transition its entrance and exit run.
pub(crate) fn popup_surface(s: StyleRefinement) -> StyleRefinement {
    s.min_w(px(200.0))
        .max_w(px(320.0))
        .p(px(tokens::SPACE_3))
        .gap(px(tokens::SPACE_2))
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(move || tokens::border())
        .bg(move || tokens::surface())
        .text_color(move || tokens::foreground())
        .text_size(px(tokens::TEXT_MD))
        .transition_all()
        .duration(tokens::DURATION_BASE)
        .ease_out()
}

/// Where a popup starts its entrance: transparent, `ENTER_DISTANCE` above.
pub(crate) fn popup_starting(s: StyleRefinement) -> StyleRefinement {
    s.opacity(0.0).translate_y(px(-tokens::ENTER_DISTANCE))
}

/// Where a popup goes as it exits: transparent.
pub(crate) fn popup_ending(s: StyleRefinement) -> StyleRefinement {
    s.opacity(0.0)
}

/// An anchored popover with Omarchy's popup look. Give it a `.trigger(..)`
/// (pressing it toggles the popover) and children (made on each opening).
/// Set `.open(..)` with `.on_open_change(..)` only when something else must
/// open it.
pub fn popover() -> nsh_base::PopoverElement {
    nsh_base::popover()
        .offset(px(tokens::SPACE_1))
        .margin(px(tokens::SPACE_2))
        .popup_style(popup_surface)
        .popup_starting_style(popup_starting)
        .popup_ending_style(popup_ending)
}

/// A popover's heading: emphasized, semibold.
pub fn popover_title(title: impl Into<SharedString>) -> impl IntoElement {
    text(title.into())
        .text_size(px(tokens::TEXT_MD))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::bright())
}
