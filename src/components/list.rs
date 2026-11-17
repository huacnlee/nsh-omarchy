//! Omarchy lists: [`list`], a native virtualized list for any number of
//! rows (only the rows near the viewport exist, their cells recycled),
//! [`list_row`], its pressable row with the hover wash and the selected
//! fill, and [`scrollbar`], a square thumb over a scroll
//! view or list.
//!
//! A list's cell is made once per cell, not per row: read the row's data
//! through `at.index()` in bindings, and keep per-row state (selection,
//! expansion) in data keyed by the row's id, never in a signal in the cell.
//!
//! Motion: hover and selection washes are fast color transitions
//! (`DURATION_FAST`); the scrollbar's thumb fades in while scrolling.

use nsh_base::{ButtonElement, ScrollbarElement, ScrollbarTarget, button};
use nsh_core::element::{IntoProp, StyleRefinement};
use nsh_core::prelude::*;

use crate::tokens;

/// Width of a scrollbar's track.
const SCROLLBAR_TRACK: f32 = 8.0;
/// Inset of a scrollbar's thumb in its track.
const SCROLLBAR_INSET: f32 = 1.0;

/// A virtualized list of `count` rows, each made by `row` from its
/// position (call `at.index()` in bindings). Its rows are estimated at a
/// control's height; it hides the platform's indicator, so pair it with a
/// [`scrollbar`] over it. Give it a height (`.h(..)` or `.flex_1()`).
pub fn list<R: IntoElement>(
    count: impl IntoReactive<usize>,
    row: impl Fn(Position) -> R + 'static,
) -> VirtualList {
    virtual_list(count, row)
        .estimated_item_size(px(tokens::CONTROL_MD))
        .scrollbar(false)
        .bg(move || tokens::background())
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
}

/// A list's row: a focusable button (Tab reaches it, Space and Return
/// press it) a control tall, with a hover wash, the focus border and,
/// while `selected`, the selected fill and bright text.
/// Give it `.on_press(..)` and its content.
pub fn list_row<M>(selected: impl IntoProp<bool, M>) -> ButtonElement {
    button()
        .selected(selected)
        .selected_style(|s: StyleRefinement| {
            s.bg(move || tokens::selected_fill())
                .text_color(move || tokens::bright())
        })
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_2))
        .h(px(tokens::CONTROL_MD))
        .px(px(tokens::SPACE_2))
        .border_1()
        .border_color(move || AdaptiveColor::TRANSPARENT)
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
}

/// A scrollbar over the scroll view or list `target` names (its
/// [`ElementRef`]): a square thumb in a slim, faintly filled track. Place it in a relative parent, over the
/// target's trailing edge (`.absolute().top(..).bottom(..).right(..)`).
pub fn scrollbar(target: impl Into<ScrollbarTarget>) -> ScrollbarElement {
    nsh_base::scrollbar()
        .scroll(target.into())
        .accessibility_label("Scroll")
        .w(px(SCROLLBAR_TRACK))
        .p(px(SCROLLBAR_INSET))
        .bg(move || tokens::normal_fill())
        .thumb_style(|s: StyleRefinement| s.w_full().bg(move || tokens::foreground().opacity(0.35)))
}
