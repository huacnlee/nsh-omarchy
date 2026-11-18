//! Omarchy tables: [`table`] for a few rows the app writes (a header of
//! secondary titles, divider rows, a hover wash, a selected row's fill),
//! and [`data_table`] for any number of rows, styled over
//! `nsh_base`'s `DataTable` (virtualized, sortable, keyboard-selectable).
//!
//! Motion: a row's hover and selection washes are fast color transitions
//! (`DURATION_FAST`).

use nsh_base::text_view::PartStyle;
use nsh_base::{
    ButtonElement, DataTableAlign, DataTableClasses, DataTableColumn, DataTableElement,
    DataTableStyles, TableBodyElement, TableCellElement, TableElement, TableHeadElement,
    TableHeaderElement, TableRowElement, button,
};
use nsh_core::element::{IntoProp, Style, StyleRefinement};
use nsh_core::prelude::*;

use crate::tokens;

// ---- Static table -------------------------------------------------------

/// A static table with a table's semantics inside a 1px border. Its
/// children are a [`table_header`] and a [`table_body`].
pub fn table(accessibility_label: impl Into<SharedString>) -> TableElement {
    nsh_base::table()
        .accessibility_label(accessibility_label.into())
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::background())
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
}

/// The header group: its row of [`table_head`] titles on a raised strip,
/// with the table's border under it.
pub fn table_header() -> TableHeaderElement {
    nsh_base::table_header()
        .bg(move || tokens::surface())
        .border_b_1()
        .border_color(move || tokens::border())
}

/// A cell's column line: a divider at its trailing edge, but the last
/// column's (the table's border closes it).
const COLUMN_LINE: &str = "last:border-r-0";

/// The header's row of column titles.
pub fn table_header_row() -> TableRowElement {
    nsh_base::table_row()
        .items_center()
        .min_h(px(tokens::CONTROL_MD))
}

/// A column title: small, semibold, secondary. `width` fixes the column's
/// width; `None` lets it share the row.
pub fn table_head(title: impl Into<SharedString>, width: Option<f32>) -> TableHeadElement {
    sized(nsh_base::table_head(), width)
        .self_stretch()
        .px(px(tokens::SPACE_2))
        .justify_center()
        .border_r_1()
        .border_color(move || tokens::divider())
        .app_class(SharedString::from(COLUMN_LINE))
        .child(
            text(title.into())
                .number_of_lines(1)
                .text_size(px(tokens::TEXT_SM))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::secondary()),
        )
}

/// A numeric column's title: as [`table_head`], at the end.
pub fn table_head_numeric(title: impl Into<SharedString>, width: Option<f32>) -> TableHeadElement {
    table_head(title, width).items_end()
}

/// The body group of the rows.
pub fn table_body() -> TableBodyElement {
    nsh_base::table_body()
}

/// A body row: pressable and focusable (role `row`, Tab reaches it, Space
/// and Return press it) with a divider above it (but the first, under the
/// header's border), a hover wash and, while
/// `selected`, the selected fill and bright text. Give it
/// `.on_press(..)` to select it and `.accessibility_label(..)` to name it.
pub fn table_row<M>(selected: impl IntoProp<bool, M>) -> ButtonElement {
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

/// A text cell. `width` fixes the column's width; `None` shares the row.
pub fn table_cell(content: impl Into<SharedString>, width: Option<f32>) -> TableCellElement {
    column_line(sized(nsh_base::table_cell(), width))
        .px(px(tokens::SPACE_2))
        .py(px(tokens::SPACE_1))
        .justify_center()
        .child(text(content.into()).number_of_lines(1))
}

/// A numeric cell: monospaced tabular figures at the end.
pub fn table_cell_numeric(
    content: impl Into<SharedString>,
    width: Option<f32>,
) -> TableCellElement {
    column_line(sized(nsh_base::table_cell(), width))
        .px(px(tokens::SPACE_2))
        .py(px(tokens::SPACE_1))
        .justify_center()
        .items_end()
        .child(
            text(content.into())
                .number_of_lines(1)
                .font_family(move || tokens::font_mono())
                .tabular_nums(),
        )
}

/// The divider at a cell's trailing edge, the row's full height.
fn column_line(cell: TableCellElement) -> TableCellElement {
    cell.border_r_1()
        .border_color(move || tokens::divider())
        .app_class(SharedString::from(COLUMN_LINE))
}

fn sized<S: Styled>(element: S, width: Option<f32>) -> S {
    match width {
        Some(w) => element.w(px(w)).flex_none(),
        None => element.flex_1().min_w(px(0.0)),
    }
}

// ---- DataTable ----------------------------------------------------------

/// A column of a [`data_table`]: the `key` of its field in the rows, its
/// title and its width. Set `sortable` on it for a sort button.
pub fn data_column(
    key: impl Into<SharedString>,
    title: impl Into<SharedString>,
    width: f32,
) -> DataTableColumn {
    let mut column = DataTableColumn::default();
    column.key = key.into();
    column.title = Some(title.into());
    column.width = Some(px(width));
    column
}

/// A numeric column: at the end, in monospaced tabular figures.
pub fn data_column_numeric(
    key: impl Into<SharedString>,
    title: impl Into<SharedString>,
    width: f32,
) -> DataTableColumn {
    let mut column = data_column(key, title, width);
    column.align = DataTableAlign::End;
    column.cell_class = Some("font-mono tabular-nums".into());
    column
}

/// A virtualized table of any number of rows: secondary titles on a raised
/// strip under the table's border, divider rows and column lines, a
/// hover wash, the selected row's fill and bright text, the sorted column's title bright with its chevron in the accent.
/// Rows select by a press and by the arrow keys, Home, End, Page Up and
/// Down while the table has the keyboard. Sortable columns report the next
/// sort with `on_sort_change`: the owner sorts its rows and sets `.sort(..)`.
/// Give it a height.
pub fn data_table(accessibility_label: impl Into<SharedString>) -> DataTableElement {
    nsh_base::data_table()
        .accessibility_label(accessibility_label.into())
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
