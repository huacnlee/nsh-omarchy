//! Data: tables, the data table, lists, trees, scrollbars and resizable
//! panels.

use nsh_base::ScrollbarTarget;
use nsh_core::element::IntoProp;
use nsh_core::prelude::*;

use crate::ui::data::{data_table, list as list_ui, resizable, table, tree as tree_ui};

pub use crate::ui::data::data_table::{
    DataRow, DataTable, DataTableElement, SortDirection, TableColumn, TableSort,
};
pub use crate::ui::data::list::{ListItem, ListItemElement, Scrollbar, ScrollbarElement};
pub use crate::ui::data::resizable::{
    ResizablePanel, ResizablePanelElement, ResizablePanelGroup, ResizablePanelGroupElement,
};
pub use crate::ui::data::table::{
    Table, TableBody, TableBodyElement, TableCell, TableCellElement, TableElement, TableHead,
    TableHeadElement, TableHeader, TableHeaderElement, TableRow, TableRowElement,
};
pub use crate::ui::data::tree::{Tree, TreeElement, TreeItem};

/// A table of a few rows the app writes, inside a 1px border; its children
/// are a [`table_header`] and a [`table_body`].
pub fn table() -> TableElement {
    table::table()
}

/// A table's header group: a [`table_row`] of [`table_head`] titles.
pub fn table_header() -> TableHeaderElement {
    table::table_header()
}

/// A table's body group: its [`table_row`]s of [`table_cell`]s.
pub fn table_body() -> TableBodyElement {
    table::table_body()
}

/// A row of a table: in a body pressable and selectable (`.selected(..)`,
/// `.on_press(..)`), in a header the row of titles.
pub fn table_row() -> TableRowElement {
    table::table_row()
}

/// A column's title in a header's row: `table_head("Size").width(Some(96.0)).numeric(true)`.
pub fn table_head<M>(title: impl IntoProp<SharedString, M>) -> TableHeadElement {
    table::table_head().title(title)
}

/// A cell of a body row: `table_cell("hyprland")`.
pub fn table_cell<M>(text: impl IntoProp<SharedString, M>) -> TableCellElement {
    table::table_cell().text(text)
}

/// A virtualized table of `rows` under `columns`, the rows named by their
/// `id`: `data_table(columns, rows).selected(..).on_selected_change(..)`.
/// Give it a height.
pub fn data_table<C, R>(
    columns: impl IntoProp<Vec<TableColumn>, C>,
    rows: impl IntoProp<Vec<DataRow>, R>,
) -> DataTableElement {
    data_table::data_table().columns(columns).rows(rows)
}

/// A virtualized list of `count` rows, each made by `row` from its
/// position (read `at.index()` in closures: a cell is reused for other
/// rows), usually a [`list_item`]. Rows are estimated a control tall. It
/// hides the platform's indicator: pair it with a [`scrollbar`]. Give it a
/// height. TSX uses NSH's own `<List>`.
pub fn list<R: IntoElement>(
    count: impl IntoReactive<usize>,
    row: impl Fn(Position) -> R + 'static,
) -> VirtualList {
    crate::components::list::list(count, row)
}

/// A list's row: `list_item().selected(..).on_press(..).child(..)`.
pub fn list_item() -> ListItemElement {
    list_ui::list_item()
}

/// A scrollbar over the scroll view or list `target` names (its
/// [`ElementRef`]), placed over the target's trailing edge.
pub fn scrollbar(target: impl Into<ScrollbarTarget>) -> ScrollbarElement {
    list_ui::scrollbar().target(target.into())
}

/// A tree of `items`: `tree(items).expanded(..).selected(..)`. Give it a
/// height.
pub fn tree<M>(items: impl IntoProp<Vec<TreeItem>, M>) -> TreeElement {
    tree_ui::tree().items(items)
}

/// Panels side by side (`.vertical()`: stacked), resized by dragging the
/// lines between them; its children are [`resizable_panel`]s.
pub fn resizable_panel_group() -> ResizablePanelGroupElement {
    resizable::resizable_panel_group()
}

/// A panel of a [`resizable_panel_group`]: `resizable_panel().size(Some(200.0))`.
pub fn resizable_panel() -> ResizablePanelElement {
    resizable::resizable_panel()
}
