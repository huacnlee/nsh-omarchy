//! Omarchy pagination on `nsh_base::pagination()`.
//!
//! `nsh_base` owns the pages shown (first, last, the current one and its
//! neighbours, ellipses between), the keyboard (each item a Tab stop,
//! Return/Space press it, focus follows the page), Previous/Next disabled at
//! the ends and the navigation landmark. [`pagination_bar`] draws each item
//! as a compact square cell: a hover wash and pressed fill, a 1px focus
//! border, and for the current page a selected fill with a 2px accent bar.
//!
//! Chain `.page(..)`, `.total_pages(..)` and `.on_change(..)` (controlled).

use nsh_base::{PaginationElement, PaginationItemKind, PaginationItemState, pagination};
use nsh_core::prelude::*;

use crate::tokens;

/// A styled pagination row. Set `.page(..)` and `.total_pages(..)` and
/// follow `.on_change(..)`; `.compact(true)` keeps Previous and Next only.
pub fn pagination_bar() -> PaginationElement {
    pagination()
        .items_center()
        .gap(px(2.0))
        .accessibility_label("Pages")
        .previous_accessibility_label("Previous page")
        .next_accessibility_label("Next page")
        // The item is the group its cell's states follow.
        .item_style(|s| s.group())
        .render_item(|state: &PaginationItemState| pagination_cell(state))
}

/// One item's cell, drawn from its state.
fn pagination_cell(state: &PaginationItemState) -> AnyElement {
    let label: SharedString = match state.kind {
        PaginationItemKind::Previous => "‹".into(),
        PaginationItemKind::Next => "›".into(),
        PaginationItemKind::Ellipsis => "…".into(),
        PaginationItemKind::Page => state.page.to_string().into(),
    };
    let current = state.current;
    let (ellipsis, disabled) = (state.kind == PaginationItemKind::Ellipsis, state.disabled);
    let inert = ellipsis || disabled;
    let cell = view()
        .flex_row()
        .items_center()
        .justify_center()
        .h(px(tokens::CONTROL_SM))
        .min_w(px(tokens::CONTROL_SM))
        .px(px(tokens::SPACE_1))
        .border_1()
        .border_color(move || AdaptiveColor::TRANSPARENT)
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .child(
            text(label)
                .text_size(px(tokens::TEXT_MD))
                .font_weight(if current {
                    FontWeight::SEMIBOLD
                } else {
                    FontWeight::NORMAL
                })
                .text_color(move || {
                    if ellipsis {
                        tokens::secondary()
                    } else if disabled {
                        tokens::disabled()
                    } else if current {
                        tokens::bright()
                    } else {
                        tokens::foreground()
                    }
                }),
        );
    let cell = if inert {
        cell
    } else {
        cell.group_hover(|s| s.bg(move || tokens::hover_fill()))
            .group_active(|s| s.bg(move || tokens::pressed_fill()))
            .group_focus_visible(|s| s.border_color(move || tokens::focus_border()))
    };
    if current {
        cell.bg(move || tokens::selected_fill())
            .child(
                view()
                    .absolute()
                    .left(px(0.0))
                    .right(px(0.0))
                    .bottom(px(0.0))
                    .h(px(2.0))
                    .bg(move || tokens::accent()),
            )
            .into_element()
    } else {
        cell.into_element()
    }
}
