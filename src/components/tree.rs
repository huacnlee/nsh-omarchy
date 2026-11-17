//! Omarchy tree: [`tree`] styles `nsh_base`'s virtualized `Tree` (items
//! that hold items, expanded and collapsed, one selected).
//!
//! Keyboard (the tree is one Tab stop): Up and Down move the selection,
//! Right expands a folder or enters it, Left collapses it or goes to its
//! parent, Home and End jump to the ends, Return and Space expand or act,
//! and typing a label's start finds it.
//!
//! Motion: a row's hover wash is a fast color transition; a folder's
//! chevron turns a quarter as it expands (`DURATION_BASE`).

use nsh_base::{TreeElement, TreeEntry};
use nsh_core::element::StyleRefinement;
use nsh_core::prelude::*;

use crate::tokens;

/// Width of a selected row's accent bar.
const MARKER: f32 = 2.0;
/// How far each level of the tree is indented.
const INDENT: f32 = tokens::SPACE_4;
/// The chevron's box.
const CHEVRON: f32 = 12.0;

/// A tree of rows a control tall: a folder's chevron that turns as it
/// expands, a label indented by its depth, a hover wash, the selected
/// row's fill and accent bar, the focus border on the keys' row while the
/// keyboard has the tree. Give it a height, its `.items(..)`, and wire
/// `.expanded(..)` / `.on_expanded_change(..)` and `.selected(..)` /
/// `.on_selected_change(..)` to the owner's state (or leave them out for
/// the tree's own).
pub fn tree(accessibility_label: impl Into<SharedString>) -> TreeElement {
    nsh_base::tree()
        .accessibility_label(accessibility_label.into())
        .bg(move || tokens::background())
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
        .list_style(|s: StyleRefinement| s.py(px(tokens::SPACE_1)))
        .item_class("group")
        .item_style(|s: StyleRefinement| {
            s.h(px(tokens::CONTROL_MD))
                .flex_row()
                .items_stretch()
                .border_l(px(MARKER))
                .border_color(move || AdaptiveColor::TRANSPARENT)
                .transition_colors()
                .duration(tokens::DURATION_FAST)
                .selected(|s| {
                    s.bg(move || tokens::selected_fill())
                        .border_l_color(move || tokens::accent())
                })
        })
        .render_item(tree_row)
}

/// An item's row content: what the item is (its state is the row's).
fn tree_row(entry: &TreeEntry) -> View {
    let folder = entry.folder;
    let disabled = entry.disabled;
    view()
        .flex_1()
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_1))
        .pl(px(tokens::SPACE_1 + entry.depth as f32 * INDENT))
        .pr(px(tokens::SPACE_2))
        .border_1()
        .border_color(move || AdaptiveColor::TRANSPARENT)
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .group_hover(|s| s.bg(move || tokens::hover_fill()))
        .group_focus_visible(|s| s.border_color(move || tokens::focus_border()))
        .when(disabled, |row| row.opacity(0.45))
        .child(match folder {
            true => text("\u{203A}")
                .w(px(CHEVRON))
                .text_center()
                .text_size(px(tokens::TEXT_LG))
                .text_color(move || tokens::secondary())
                .transition_transform()
                .duration(tokens::DURATION_BASE)
                .app_class(Some(SharedString::from("group-aria-expanded:rotate-90")))
                .into_element(),
            false => view().w(px(CHEVRON)).into_element(),
        })
        .child(
            text(entry.label.clone())
                .flex_1()
                .min_w(px(0.0))
                .number_of_lines(1)
                .text_color(move || {
                    if disabled {
                        tokens::disabled()
                    } else {
                        tokens::foreground()
                    }
                }),
        )
}
