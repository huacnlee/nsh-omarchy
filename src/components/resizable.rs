//! Omarchy resizable panels: [`resizable`] styles `nsh_base`'s
//! `ResizablePanelGroup` (panels side by side or stacked, resized by
//! dragging the handles between them), [`resizable_panel`] is one panel.
//!
//! Each handle is a 1px line in the divider color, the accent under the
//! pointer and while dragged; its band is wider than the line for an easy
//! grab. Screen readers adjust a handle (increment and decrement move it
//! 10 points); the keyboard does not reach handles (`nsh_base` reports no
//! focus for them).
//!
//! Motion: the line's color is a fast color transition (`DURATION_FAST`);
//! panels follow the drag frame by frame, natively.

use nsh_base::{ResizablePanelElement, ResizablePanelGroupElement};
use nsh_core::element::StyleRefinement;
use nsh_core::geometry::Axis;
use nsh_core::prelude::*;

use crate::tokens;

/// The smallest a panel gets by default.
const PANEL_MIN: f32 = 64.0;

/// A group of panels laid out along `axis` (`Axis::Horizontal`: side by
/// side), its children [`resizable_panel`]s. It fills its parent; give
/// the parent a size. `.on_resize(..)` reports the panels' sizes after a
/// drag, to pass back as their `size`.
pub fn resizable(axis: Axis) -> ResizablePanelGroupElement {
    nsh_base::resizable_panel_group()
        .axis(axis)
        .handle_accessibility_label(match axis {
            Axis::Horizontal => "Resize panels horizontally",
            Axis::Vertical => "Resize panels vertically",
        })
        .handle_style(|s: StyleRefinement| {
            s.bg(move || tokens::divider())
                .transition_colors()
                .duration(tokens::DURATION_FAST)
                .group_hover(|s| s.bg(move || tokens::accent()))
                .group_active(|s| s.bg(move || tokens::accent()))
        })
}

/// A panel of a [`resizable`] group, at least 64 points along its axis.
/// Set `.size(px(..))` for the size it wants, `.size_range(..)` for its
/// limits; one without a size shares what the others leave.
pub fn resizable_panel() -> ResizablePanelElement {
    nsh_base::resizable_panel()
        .min_size(px(PANEL_MIN))
        .min_w(px(0.0))
        .min_h(px(0.0))
}
