//! Omarchy tree: [`tree`] styles `nsh_base`'s virtualized `Tree` (items
//! that hold items, expanded and collapsed, one selected).
//!
//! Keyboard (the tree is one Tab stop): Up and Down move the selection,
//! Right expands a folder or enters it, Left collapses it or goes to its
//! parent, Home and End jump to the ends, Return and Space expand or act,
//! and typing a label's start finds it.
//!
//! Every row state (hover, press, the keys' focus, selection) fills the
//! whole row, edge to edge: it is drawn on the item, never on its content.
//!
//! Motion: a row's hover wash is a fast color transition; a folder's
//! chevron turns a quarter as it expands (`DURATION_BASE`).

use nsh_base::{TreeElement, TreeEntry};
use nsh_core::element::StyleRefinement;
use nsh_core::prelude::*;

use crate::tokens;
use crate::tokens::classes::{CLEAR_BORDER, FOCUS_FILL, FOCUS_VISIBLE_BORDER, HOVER, PRESSED};
/// How far each level of the tree is indented.
const INDENT: f32 = tokens::SPACE_4;
/// The chevron's box.
const CHEVRON: f32 = 12.0;

/// A tree of rows a control tall: a folder's chevron that turns as it
/// expands, a label indented by its depth, a hover wash, the selected
/// row's fill and bright text, the focus border on the keys' row while the
/// keyboard has the tree, each across the full row. Give it a height, its `.items(..)`, a `.render_item(..)` of
/// [`tree_row`]s, and wire
/// `.expanded(..)` / `.on_expanded_change(..)` and `.selected(..)` /
/// `.on_selected_change(..)` to the owner's state (or leave them out for
/// the tree's own).
pub fn tree() -> TreeElement {
    nsh_base::tree()
        .bg(move || tokens::background())
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
        .list_style(|s: StyleRefinement| s.py(px(tokens::SPACE_1)))
        // The states are the item's, the full-width row: hover, press and
        // the keys' focus by class (only a class reaches them on an item),
        // the selection by its style (over the classes).
        .item_class(SharedString::from(item_class()))
        .item_style(|s: StyleRefinement| {
            s.h(px(tokens::CONTROL_MD))
                .flex_row()
                .items_stretch()
                .border(px(tokens::FOCUS_BORDER))
                .transition_colors()
                .duration(tokens::DURATION_FAST)
                .selected(|s| {
                    s.bg(move || tokens::selected_fill())
                        .text_color(move || tokens::bright())
                })
        })
}

/// The classes of an item's row: a group for its content, and its states.
fn item_class() -> String {
    format!("group {CLEAR_BORDER} {HOVER} {PRESSED} {FOCUS_FILL} {FOCUS_VISIBLE_BORDER}")
}

/// An item's row content: what the item is, with its `icon` before the
/// label (its state is the row's).
pub fn tree_row(entry: &TreeEntry, glyph: Option<super::icon::IconName>) -> View {
    let folder = entry.folder;
    let disabled = entry.disabled;
    view()
        .flex_1()
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_1))
        .pl(px(tokens::SPACE_1 + entry.depth as f32 * INDENT))
        .pr(px(tokens::SPACE_2))
        .when(disabled, |row| row.opacity(0.45))
        .child(match folder {
            true => super::icon::indicator(super::icon::IconName::ChevronRight, CHEVRON)
                .transition_transform()
                .duration(tokens::DURATION_BASE)
                .app_class(Some(SharedString::from("group-aria-expanded:rotate-90")))
                .into_element(),
            false => view().w(px(CHEVRON)).into_element(),
        })
        .child(glyph.map(|name| super::icon::icon(name)))
        .child(
            text(entry.label.clone())
                .flex_1()
                .min_w(px(0.0))
                .number_of_lines(1)
                // The row's colour (bright while selected) unless disabled.
                .when(disabled, |label| {
                    label.text_color(move || tokens::disabled())
                }),
        )
}

#[cfg(test)]
mod tests {
    use nsh_core::tailwind::{Theme, parse};

    /// The row states are the item's (the full-width row), so its classes
    /// carry the hover, press and focus fills and they all parse.
    #[test]
    fn the_row_states_are_on_the_full_width_item() {
        let class = super::item_class();
        for state in [
            "hover:bg-",
            "active:bg-",
            "focus-visible:bg-",
            "focus-visible:border-",
        ] {
            assert!(class.contains(state), "{class} lacks {state}");
        }
        let mut theme = Theme::default();
        theme.set_tokens(&crate::Theme::tokyo_night().to_tokens());
        let list = parse(&theme, &class);
        assert!(list.unknown().is_empty(), "{class}: {:?}", list.unknown());
    }
}
