//! [`ListItem`], a list's pressable row, and [`Scrollbar`], a square thumb
//! over a scroll view or list. Drawn by [`crate::components::list`]. The
//! list itself is NSH's (`crate::list` in Rust, `<List>` in TSX).

use nsh_base::ScrollbarTarget;
use nsh_core::prelude::*;

use crate::components::list as look;

/// The props of [`ListItem`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ListItemProps {
    /// Keeps the selected fill and bright text: the row the app selected.
    #[prop(default)]
    pub selected: Prop<bool>,
    /// No presses, no feedback, dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers call the row (tests find it by it).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Its content, laid out in a row: a glyph, a label, a detail.
    pub children: Prop<Children>,
}

/// The events of a [`ListItem`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ListItemEvent {
    /// It was pressed: a click, a tap, Return or Space.
    Press,
}

/// A list's row, a control tall: focusable (Tab reaches it, Space and
/// Return press it), a hover wash, the focus border and, while `selected`,
/// the selected fill and bright text. In a virtualized list, read the
/// row's data in closures: a cell shows another row when it is reused.
#[component]
pub fn ListItem(props: ListItemProps, cx: Scope<ListItemEvent>) -> impl IntoElement {
    look::list_row(move || props.selected.get())
        .disabled(move || props.disabled.get())
        .accessibility_label(move || props.accessibility_label.get())
        .style_with(move |s| match props.disabled.get() {
            true => s.opacity(0.45),
            false => s,
        })
        .on_press(move || cx.emit(ListItemEvent::Press))
        .child(props.children)
}

/// The props of [`Scrollbar`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ScrollbarProps {
    /// The scroll view or list it follows and moves: its `ref` (a Rust
    /// view's `ElementRef`).
    pub target: Prop<ScrollbarTarget>,
}

/// A scrollbar over the scroll view or list `target` names: a square
/// thumb in a slim, faintly filled track, shown while it scrolls. Place it
/// in a relative parent over the target's trailing edge
/// (`absolute top-0 bottom-0 right-0`).
#[component]
pub fn Scrollbar(props: ScrollbarProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    look::scrollbar(move || props.target.get())
}
