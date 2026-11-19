//! Omarchy tables: the looks of a static table's parts (a header of
//! secondary titles, divider rows, a hover wash, a selected row's fill),
//! and [`data_table`] for any number of rows, styled over `nsh_base`'s
//! `DataTable` (virtualized, sortable, keyboard-selectable).
//!
//! Motion: a row's hover and selection washes are fast color transitions
//! (`DURATION_FAST`).

use nsh_base::text_view::PartStyle;
use nsh_base::{ButtonElement, DataTableClasses, DataTableElement, DataTableStyles, button};
use nsh_core::element::{IntoProp, Style, StyleRefinement};
use nsh_core::prelude::*;

use crate::tokens;

// ---- Static table -------------------------------------------------------

/// A static table's frame: a 1px border on the background.
pub fn table_look(table: View) -> View {
    table
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::background())
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
}

/// The header group: its row of titles on a raised strip, with the table's
/// border under it.
pub fn header_look(header: View) -> View {
    header
        .bg(move || tokens::surface())
        .border_b_1()
        .border_color(move || tokens::border())
}

/// A cell's column line: a divider at its trailing edge, but the last
/// column's (the table's border closes it).
const COLUMN_LINE: &str = "last:border-r-0";

/// The header's row of column titles.
pub fn header_row_look(row: View) -> View {
    row.flex_row().items_center().min_h(px(tokens::CONTROL_MD))
}

/// A column title: small, semibold, secondary, at the end when `numeric`.
/// `width` fixes the column's width; `None` lets it share the row.
pub fn head_look(
    head: View,
    title: impl IntoReactive<SharedString>,
    width: impl Fn() -> Option<f32> + 'static,
    numeric: impl Fn() -> bool + 'static,
) -> View {
    sized(head, width, numeric)
        .self_stretch()
        .px(px(tokens::SPACE_2))
        .justify_center()
        .border_r_1()
        .border_color(move || tokens::divider())
        .app_class(SharedString::from(COLUMN_LINE))
        .child(
            text(title)
                .number_of_lines(1)
                .text_size(px(tokens::TEXT_SM))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::secondary()),
        )
}

/// A body row: pressable and focusable (role `row`, Tab reaches it, Space
/// and Return press it) with a divider above it (but the first, under the
/// header's border), a hover wash and, while `selected`, the selected fill
/// and bright text.
pub fn body_row<M>(selected: impl IntoProp<bool, M>) -> ButtonElement {
    button()
        .role(AccessibilityRole::Row)
        .selected(selected)
        .selected_style(|s: StyleRefinement| {
            s.bg(move || tokens::selected_fill())
                .text_color(move || tokens::bright())
        })
        .flex_row()
        .items_stretch()
        .min_h(px(tokens::CONTROL_MD))
        .border_t_1()
        .border_t_color(move || tokens::divider())
        .app_class(SharedString::from("first:border-t-0"))
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| s.bg(move || tokens::hover_fill()))
}

/// A text cell with the column line at its trailing edge; a `numeric` one
/// in monospaced tabular figures at the end. `width` fixes the column's
/// width; `None` shares the row.
pub fn cell_look(
    cell: View,
    content: Prop<SharedString>,
    width: impl Fn() -> Option<f32> + 'static,
    numeric: impl Fn() -> bool + Copy + 'static,
) -> View {
    sized(cell, width, numeric)
        .px(px(tokens::SPACE_2))
        .py(px(tokens::SPACE_1))
        .justify_center()
        .border_r_1()
        .border_color(move || tokens::divider())
        .app_class(SharedString::from(COLUMN_LINE))
        .child(dynamic(move || {
            let label = text(move || content.get()).number_of_lines(1);
            match numeric() {
                true => label
                    .font_family(move || tokens::font_mono())
                    .tabular_nums()
                    .into_element(),
                false => label.into_element(),
            }
        }))
}

/// A column's width (`None`: a share of the row) and alignment (`numeric`:
/// at the end).
fn sized(
    element: View,
    width: impl Fn() -> Option<f32> + 'static,
    numeric: impl Fn() -> bool + 'static,
) -> View {
    element.style_with(move |s| {
        let s = match width() {
            Some(w) => s.w(px(w)).flex_none(),
            None => s.flex_1().min_w(px(0.0)),
        };
        match numeric() {
            true => s.items_end(),
            false => s,
        }
    })
}

// ---- DataTable ----------------------------------------------------------

/// A virtualized table of any number of rows: secondary titles on a raised
/// strip under the table's border, divider rows and column lines, a
/// hover wash, the selected row's fill and bright text, the sorted column's title bright with its chevron in the accent.
/// Rows select by a press and by the arrow keys, Home, End, Page Up and
/// Down while the table has the keyboard. Sortable columns report the next
/// sort with `on_sort_change`: the owner sorts its rows and sets `.sort(..)`.
/// Give it a height.
pub fn data_table() -> DataTableElement {
    nsh_base::data_table()
        .row_height(px(tokens::CONTROL_MD))
        .header_height(px(tokens::CONTROL_MD))
        .font_size(px(tokens::TEXT_MD))
        .column_selectable(false)
        .movable_columns(false)
        .classes(data_table_classes())
        .styles(data_table_styles())
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::background())
        .text_color(move || tokens::foreground())
}

fn data_table_classes() -> DataTableClasses {
    // The lines and fills as theme classes, which follow a theme change:
    // the header under the table's border, a divider under every row and
    // at every column's trailing edge (the last too: the columns may end
    // before the table does), through the titles as through the rows.
    const COLUMN: &str = "border-r border-foreground/12";
    let mut classes = DataTableClasses::default();
    classes.header = Some("bg-surface border-b border-border".into());
    classes.header_cell =
        Some(format!("px-2 {COLUMN} text-[11px] font-semibold text-text-secondary").into());
    classes.header_cell_sorted = Some("text-bright".into());
    classes.row = Some("border-b border-foreground/12".into());
    classes.row_hover = Some("bg-foreground/8".into());
    classes.row_selected = Some("bg-foreground/18 text-bright".into());
    classes.cell = Some(format!("px-2 {COLUMN}").into());
    classes
}

/// One part's typed style.
fn part(f: impl FnOnce(StyleRefinement) -> StyleRefinement) -> Option<PartStyle> {
    Some(PartStyle::from(Style::from(f(StyleRefinement::default()))))
}

fn data_table_styles() -> DataTableStyles {
    let mut styles = DataTableStyles::default();
    styles.sort_indicator = part(|s| s.text_color(move || tokens::secondary()));
    styles.sort_ascending = part(|s| s.selected(|s| s.text_color(move || tokens::accent())));
    styles.sort_descending = part(|s| s.selected(|s| s.text_color(move || tokens::accent())));
    styles.resize_handle = part(|s| s.bg(move || tokens::divider()));
    styles.row = part(|s| s.transition_colors().duration(tokens::DURATION_FAST));
    styles.row_active = part(|s| s.bg(move || tokens::hover_fill()));
    styles.cell_selected = part(|s| s.border_1().border_color(move || tokens::focus_border()));
    styles.fixed_edge = part(|s| s.bg(move || tokens::divider()));
    styles.empty = part(|s| s.text_color(move || tokens::secondary()));
    styles
}
