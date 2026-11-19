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
pub const PANEL_MIN: f32 = 64.0;

/// What screen readers say of a handle of a group laid out along `axis`.
pub fn handle_label(axis: Axis) -> SharedString {
    match axis {
        Axis::Horizontal => "Resize panels horizontally",
        _ => "Resize panels vertically",
    }
    .into()
}

/// A group of panels, its children [`resizable_panel`]s: the handles'
/// look. It fills its parent; give the parent a size.
pub fn resizable() -> ResizablePanelGroupElement {
    nsh_base::resizable_panel_group()
        .handle_style(|s: StyleRefinement| {
            s.bg(move || tokens::divider())
                .transition_colors()
                .duration(tokens::DURATION_FAST)
                .group_hover(|s| s.bg(move || tokens::accent()))
                .group_active(|s| s.bg(move || tokens::accent()))
        })
}

/// A panel of a [`resizable`] group: its content may shrink below its own
/// size. Set its size and limits.
pub fn resizable_panel() -> ResizablePanelElement {
    nsh_base::resizable_panel()
        .min_w(px(0.0))
        .min_h(px(0.0))
}
