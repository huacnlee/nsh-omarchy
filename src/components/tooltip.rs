//! Omarchy tooltips: a small square label next to a control.
//!
//! [`tooltip`] is `nsh_base::tooltip()` styled: the raised surface, a 1px
//! control border, `TEXT_SM`, a fast fade (skipped under reduced motion).
//! Base shows it after the pointer rests 400 ms, at once when the keyboard
//! focuses the control, and hides it on Escape. It never takes presses.
//! Attach it with the element's `.tooltip(..)`; keep an accessible label on
//! icon-only controls, a tooltip only adds to it.
//!
//! ```ignore
//! button().accessibility_label("Save").tooltip(tooltip("Save  ⌘S"))
//! ```

use nsh_core::prelude::*;

use crate::tokens;

/// How long the pointer rests on the trigger before the tooltip shows.
const TOOLTIP_DELAY_MS: f64 = 400.0;

/// A tooltip saying `label`.
pub fn tooltip(label: impl Into<SharedString>) -> nsh_base::TooltipElement {
    nsh_base::tooltip()
        .delay(TOOLTIP_DELAY_MS)
        .offset(px(tokens::SPACE_1))
        .margin(px(tokens::SPACE_1))
        .max_w(px(320.0))
        .px(px(tokens::SPACE_2))
        .py(px(tokens::SPACE_1))
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(move || tokens::control_border())
        .bg(move || tokens::surface())
        .transition_opacity()
        .duration(tokens::DURATION_FAST)
        .starting_style(|s| s.opacity(0.0))
        .ending_style(|s| s.opacity(0.0))
        .child(
            text(label.into())
                .text_size(px(tokens::TEXT_SM))
                .text_color(move || tokens::foreground()),
        )
}

/// Wraps `trigger` (a component element such as an `nsh_base` button,
/// which takes no `.tooltip`) in a view that shows `tooltip(label)` while
/// the pointer rests on it. A host element (`view()`, `text()`) takes
/// `.tooltip(tooltip(..))` itself, which also shows it on keyboard focus.
pub fn with_tooltip(trigger: impl IntoElement, label: impl Into<SharedString>) -> View {
    view().child(trigger).tooltip(tooltip(label))
}
