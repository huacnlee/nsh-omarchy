//! The static table: [`Table`], [`TableHeader`], [`TableBody`],
//! [`TableRow`], [`TableHead`] and [`TableCell`], for a few rows the app
//! writes. Drawn by [`crate::components::table`].
//!
//! A row knows whether it is a header's (a plain row of titles) or a
//! body's (a pressable, selectable row) from the group it is in, through
//! context.

use nsh_core::prelude::*;

use crate::components::table as look;

/// Whether the rows made under it are a header's: given by [`TableHeader`]
/// (true) and [`TableBody`] (false) to their rows.
#[derive(Clone, Copy)]
struct InHeader(bool);

/// The props of [`Table`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableProps {
    /// What screen readers call the table.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Its parts: a `TableHeader` and a `TableBody`.
    pub children: Prop<Children>,
}

/// A table of the app's own rows inside a 1px border:
/// `<Table><TableHeader>..</TableHeader><TableBody>..</TableBody></Table>`.
#[component]
pub fn Table(props: TableProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    view()
        .role(AccessibilityRole::Table)
        .optional_label(props.accessibility_label)
        .map(look::table_look)
        .child(props.children)
}

/// The props of [`TableHeader`] and [`TableBody`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableGroupProps {
    /// Its rows: `TableRow`s.
    pub children: Prop<Children>,
}

/// The header group of a [`Table`]: a row of [`TableHead`] titles on a
/// raised strip, the table's border under it.
#[component]
pub fn TableHeader(props: TableGroupProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_context(InHeader(true));
    view()
        .role(AccessibilityRole::RowGroup)
        .map(look::header_look)
        .child(props.children)
}

/// The body group of a [`Table`]: its rows of [`TableCell`]s.
#[component]
pub fn TableBody(props: TableGroupProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_context(InHeader(false));
    view()
        .role(AccessibilityRole::RowGroup)
        .child(props.children)
}

/// The props of [`TableRow`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableRowProps {
    /// Keeps the selected fill and bright text: the row the app selected.
    #[prop(default)]
    pub selected: Prop<bool>,
    /// What screen readers call the row (tests find it by it).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Its cells: `TableHead`s in a header, `TableCell`s in a body.
    pub children: Prop<Children>,
}

/// The events of a [`TableRow`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum TableRowEvent {
    /// A body row was pressed: a click, a tap, Return or Space.
    Press,
}

/// A row of a table. In a [`TableBody`] it is pressable and focusable (Tab
/// reaches it, Space and Return press it) with a divider above it, a hover
/// wash and, while `selected`, the selected fill; in a [`TableHeader`] it
/// is the row of titles.
#[component]
pub fn TableRow(props: TableRowProps, cx: Scope<TableRowEvent>) -> impl IntoElement {
    match use_context::<InHeader>() {
        Some(InHeader(true)) => view()
            .role(AccessibilityRole::Row)
            .optional_label(props.accessibility_label)
            .map(look::header_row_look)
            .child(props.children)
            .into_element(),
        _ => look::body_row(move || props.selected.get())
            .accessibility_label(move || props.accessibility_label.get())
            .on_press(move || cx.emit(TableRowEvent::Press))
            .child(props.children)
            .into_element(),
    }
}

/// The props of [`TableHead`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableHeadProps {
    /// The column's title.
    pub title: Prop<SharedString>,
    /// The column's width in points; left out, the columns without one
    /// share the row. Give a column's cells the same width.
    pub width: Prop<Option<f32>>,
    /// A column of numbers: the title at the end.
    #[prop(default)]
    pub numeric: Prop<bool>,
}

/// A column's title in a header's row: small, semibold, secondary.
#[component]
pub fn TableHead(props: TableHeadProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let title = props.title;
    let head = view().role(AccessibilityRole::ColumnHeader);
    look::head_look(
        head,
        move || title.get(),
        move || props.width.get(),
        move || props.numeric.get(),
    )
}

/// The props of [`TableCell`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TableCellProps {
    /// What the cell shows, on one line.
    pub text: Prop<SharedString>,
    /// The column's width in points, as its [`TableHead`]'s.
    pub width: Prop<Option<f32>>,
    /// A number: monospaced tabular figures at the end.
    #[prop(default)]
    pub numeric: Prop<bool>,
}

/// A cell of a body row, with a column line at its trailing edge.
#[component]
pub fn TableCell(props: TableCellProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let cell = view().role(AccessibilityRole::Cell);
    look::cell_look(
        cell,
        props.text,
        move || props.width.get(),
        move || props.numeric.get(),
    )
}
