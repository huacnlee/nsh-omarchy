//! [`DataTable`]: a virtualized table of the app's rows, named by their
//! `id`, over `nsh_base`'s `DataTable` as [`crate::components::table`]
//! styles it; and its data types [`TableColumn`], [`DataRow`],
//! [`TableSort`] and [`SortDirection`].

use nsh_base::{
    DataTableAlign, DataTableColumn, DataTablePress, DataTableRows, DataTableSelection,
    DataTableSort, DataTableSortDirection, SelectValue,
};
use nsh_core::element::{JsHost, JsIn, JsKind, JsOwned, JsPrimitive, JsValue, PropError, TsTypes};
use nsh_core::prelude::*;

use crate::components::table as look;

/// A column of a [`DataTable`]: the `key` of the rows' field it shows, its
/// title, and its width, alignment and sorting.
/// `TableColumn::new("size", "Size").width(96.0).numeric().sortable()`;
/// TSX `{ key, title, width?, numeric?, sortable? }`.
#[derive(Debug, Clone, Default, PartialEq, JsObject)]
#[non_exhaustive]
pub struct TableColumn {
    /// The field of the rows its cells show; also what [`TableSort`] names.
    pub key: SharedString,
    /// Its title.
    pub title: SharedString,
    /// Its width in points (default 120).
    pub width: Option<f32>,
    /// A column of numbers: at the end, in monospaced tabular figures.
    #[prop(default)]
    pub numeric: bool,
    /// Its title has a sort button, which reports the next sort
    /// (`on_sort_change`); the owner sorts its rows.
    #[prop(default)]
    pub sortable: bool,
}

impl TableColumn {
    /// The column of the rows' field `key`, titled `title`.
    pub fn new(key: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            ..Self::default()
        }
    }

    /// Its width in points.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// A column of numbers: at the end, in monospaced tabular figures.
    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self
    }

    /// Its title has a sort button.
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    /// The `nsh_base` column, in the Omarchy look.
    fn to_base(&self) -> DataTableColumn {
        let mut column = DataTableColumn::default();
        column.key = self.key.clone();
        column.title = Some(self.title.clone());
        column.width = self.width.map(px);
        column.sortable = self.sortable;
        if self.numeric {
            column.align = DataTableAlign::End;
            column.cell_class = Some("font-mono tabular-nums".into());
        }
        column
    }
}

/// A row of a [`DataTable`]: its `id` (what selection and events name it
/// by) and its cells by column `key`. `DataRow::new("a").cell("name",
/// "Alpha").cell("size", 12)`; in TSX a record with an `id`:
/// `{ id: "a", name: "Alpha", size: 12 }`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DataRow {
    id: SharedString,
    cells: Vec<(SharedString, SharedString)>,
}

impl DataRow {
    /// A row named `id`, with no cells yet.
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            cells: Vec::new(),
        }
    }

    /// Its cell of column `key`: `value` as text (a number as JS writes
    /// it). Given again, it replaces the cell.
    pub fn cell(mut self, key: impl Into<SharedString>, value: impl std::fmt::Display) -> Self {
        let key = key.into();
        let value = SharedString::from(value.to_string());
        match self.cells.iter_mut().find(|(k, _)| *k == key) {
            Some(cell) => cell.1 = value,
            None => self.cells.push((key, value)),
        }
        self
    }

    /// Its id.
    pub fn id(&self) -> &SharedString {
        &self.id
    }

    /// Its cell of column `key`, if it has one.
    pub fn get(&self, key: &str) -> Option<&SharedString> {
        self.cells.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

/// A number written as JS writes it: no trailing `.0`.
fn js_number(n: f64) -> String {
    if n.is_nan() {
        String::new()
    } else if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{n}")
    }
}

impl JsValue for DataRow {
    fn ts_type(_types: &mut TsTypes) -> String {
        "{ id: string; [key: string]: string | number | boolean | null | undefined }".into()
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        if value.kind() != JsKind::Object {
            return Err(value.expected("a row: an object with an `id`"));
        }
        let mut row = DataRow::default();
        let mut id = None;
        for (key, cell) in value.primitive_entries()? {
            let text = match cell {
                JsPrimitive::String(s) => s,
                JsPrimitive::Number(n) | JsPrimitive::Date(n) => js_number(n),
                JsPrimitive::Bool(b) => b.to_string(),
                JsPrimitive::Other => continue,
            };
            if key == "id" {
                id = Some(SharedString::from(text.clone()));
            }
            row.cells.push((key.into(), text.into()));
        }
        row.id = id.ok_or_else(|| value.expected("a row with an `id`"))?;
        Ok(row)
    }

    fn to_js<'a>(&self, host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        let object = JsOwned::new(host, host.new_object()?);
        host.set_field(object.get().raw(), "id", host.new_string(&self.id)?)?;
        for (key, value) in &self.cells {
            if key != "id" {
                host.set_field(object.get().raw(), key, host.new_string(value)?)?;
            }
        }
        Ok(object)
    }
}

/// Which way a column is sorted.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PropEnum)]
#[non_exhaustive]
pub enum SortDirection {
    /// Smallest first.
    #[default]
    Ascending,
    /// Largest first.
    Descending,
}

/// What a [`DataTable`] is sorted by: a column's `key` and a direction.
/// TSX `{ key, direction: "ascending" | "descending" }`.
#[derive(Debug, Clone, Default, PartialEq, JsObject)]
pub struct TableSort {
    /// The `key` of the sorted column.
    pub key: SharedString,
    /// Which way.
    #[prop(default)]
    pub direction: SortDirection,
}

impl TableSort {
    /// Sorted by column `key`, `direction`.
    pub fn new(key: impl Into<SharedString>, direction: SortDirection) -> Self {
        Self {
            key: key.into(),
            direction,
        }
    }

    fn to_base(&self) -> DataTableSort {
        let mut sort = DataTableSort::default();
        sort.key = self.key.clone();
        sort.direction = match self.direction {
            SortDirection::Descending => DataTableSortDirection::Descending,
            _ => DataTableSortDirection::Ascending,
        };
        sort
    }

    fn from_base(sort: &DataTableSort) -> Self {
        Self::new(
            sort.key.clone(),
            match sort.direction {
                DataTableSortDirection::Descending => SortDirection::Descending,
                _ => SortDirection::Ascending,
            },
        )
    }
}

/// The rows as `nsh_base` reads them: a text column per column key.
fn base_rows(rows: &[DataRow], columns: &[TableColumn]) -> DataTableRows {
    columns.iter().fold(DataTableRows::new(rows.len()), |data, column| {
        let key = column.key.as_ref();
        data.texts(key, rows.iter().map(|r| r.get(key).map_or("", |v| v.as_ref())))
    })
}

/// The props of [`DataTable`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DataTableProps {
    /// The columns, left to right.
    pub columns: Prop<Vec<TableColumn>>,
    /// The rows, in the order shown (the owner sorts them): each with an
    /// `id` and its cells by column `key`.
    pub rows: Prop<Vec<DataRow>>,
    /// The selected row's `id`, `null` for none: controlled when given
    /// (set it in `onSelectedChange`); left out, the table keeps its own.
    #[prop(default)]
    pub selected: Prop<SelectValue>,
    /// What the rows are sorted by, shown on its column's title: given, the
    /// owner sorts and sets it in `onSortChange`; left out, the title shows
    /// the last sort asked for.
    pub sort: Prop<Option<TableSort>>,
    /// What screen readers call the table.
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`DataTable`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum DataTableEvent {
    /// The user selected a row (a press, the arrow keys): its `id`.
    #[args(selected)]
    SelectedChange(SharedString),
    /// The user pressed a sortable column's title: the sort asked for
    /// (ascending, then descending, then ascending again). Sort the rows
    /// and set `sort`.
    #[args(sort)]
    SortChange(TableSort),
    /// A row was pressed (each press, after the selection it makes): its
    /// `id`.
    #[args(id)]
    RowPress(SharedString),
}

/// A virtualized table of any number of rows, named by their `id`:
/// secondary titles on a raised strip, divider rows and column lines, a
/// hover wash, the selected row's fill. Rows select by a press and by the
/// arrow keys, Home, End, Page Up and Down. Give it a height.
#[component]
pub fn DataTable(props: DataTableProps, cx: Scope<DataTableEvent>) -> impl IntoElement {
    let own_selected = signal(None::<SharedString>);
    let own_sort = signal(None::<TableSort>);
    let selected = move || match props.selected.get() {
        SelectValue::Controlled(id) => id,
        _ => own_selected.get(),
    };
    let sort = move || props.sort.get().or_else(|| own_sort.get());
    let ids = computed(move || {
        props
            .rows
            .with(|rows| rows.iter().map(|r| r.id.clone()).collect::<Vec<_>>())
    });
    let id_at = move |row: Option<u32>| -> Option<SharedString> {
        ids.with_untracked(|ids| ids.get(row? as usize).cloned())
    };
    let data = computed(move || props.rows.with(|rows| props.columns.with(|c| base_rows(rows, c))));
    look::data_table()
        .accessibility_label(move || props.accessibility_label.get())
        .columns(move || props.columns.with(|c| c.iter().map(TableColumn::to_base).collect::<Vec<_>>()))
        .data(move || data.get())
        .sort(move || sort().map(|s| s.to_base()))
        .selection(move || {
            let id = selected();
            let mut selection = DataTableSelection::default();
            selection.row = id.and_then(|id| ids.with(|ids| ids.iter().position(|i| *i == id)))
                .map(|row| row as u32);
            Some(selection)
        })
        .on_selection_change(move |next: &Option<DataTableSelection>| {
            let Some(id) = id_at(next.and_then(|s| s.row)) else {
                return;
            };
            if untracked(selected).as_ref() != Some(&id) {
                own_selected.set(Some(id.clone()));
                cx.emit(DataTableEvent::SelectedChange(id));
            }
        })
        .on_sort_change(move |next: &Option<DataTableSort>| {
            // The base cycles back to unsorted after descending: ascending
            // again here, so a sort always names a column.
            let next = match next {
                Some(next) => TableSort::from_base(next),
                None => match untracked(sort) {
                    Some(now) => TableSort::new(now.key, SortDirection::Ascending),
                    None => return,
                },
            };
            own_sort.set(Some(next.clone()));
            cx.emit(DataTableEvent::SortChange(next));
        })
        .on_row_press(move |press: &DataTablePress| {
            if let Some(id) = id_at(Some(press.row)) {
                cx.emit(DataTableEvent::RowPress(id));
            }
        })
}
