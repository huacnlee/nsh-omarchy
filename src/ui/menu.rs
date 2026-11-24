//! [`Menu`], [`ContextMenu`], [`MenuItem`], [`MenuSeparator`] and
//! [`MenuLabel`], drawn by [`crate::components::menu`].

use nsh_core::prelude::*;

use crate::components::icon::{IconName, icon};
use crate::components::menu::{
    check_mark, checkable, menu_context, menu_divider, menu_dropdown, menu_heading, menu_shortcut, row_look,
};
use crate::tokens;

/// The props of [`Menu`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct MenuProps {
    /// What opens it: a `Button` usually. Press, Return, Space or Down
    /// open it.
    pub trigger: Prop<Option<Slot>>,
    /// `MenuItem`, `MenuSeparator`, `MenuLabel`: made on each opening.
    #[prop(lazy)]
    pub children: Prop<Children>,
    /// Open or closed. Given, the app owns it and writes back what
    /// `onOpenChange` asks for; left out, the menu keeps it itself.
    pub open: Prop<Option<bool>>,
    /// The side of the trigger it opens on (below when left out).
    #[prop(default)]
    pub placement: Prop<Placement>,
    /// How it lines up with the trigger (its start when left out).
    #[prop(default)]
    pub align: Prop<Align>,
}

/// The events of [`Menu`] and [`ContextMenu`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum MenuEvent {
    /// It asks to open (`true`) or close: the trigger, a chosen row,
    /// Escape or a press outside.
    #[args(open)]
    OpenChange(bool),
}

/// A menu under its trigger: `<Menu trigger={<Button label="File" />}>
/// <MenuItem label="New" /></Menu>`. Press, Return, Space or Down on the
/// trigger open it; arrows, Home / End and typeahead move between rows,
/// Return chooses, Escape and a press outside close it, and the focus goes
/// back to the trigger.
#[component]
pub fn Menu(props: MenuProps, cx: Scope<MenuEvent>) -> impl IntoElement {
    // The menu is driven here: a slot forwarded into `nsh_base`'s trigger
    // sits in a node of the base menu's own, whose press the trigger (a
    // `Button`) would take. Our slot is the app's node itself, so its press
    // handler joins the trigger's.
    let open = signal(props.open.get_untracked().unwrap_or(false));
    effect(move || {
        if let Some(value) = props.open.get() {
            open.set(value);
        }
    });
    let change = move |next: bool| {
        if props.open.get_untracked().is_none() {
            open.set(next);
        }
        cx.emit(MenuEvent::OpenChange(next));
    };
    let mut menu = menu_dropdown()
        .placement(move || props.placement.get())
        .align(move || props.align.get())
        .open(move || Some(open.get()))
        .on_open_change(move |e: &nsh_base::MenuEvent| {
            if let nsh_base::MenuEvent::OpenChange(next, _) = e {
                change(*next);
            }
        })
        .child(props.children);
    if let Some(trigger) = props.trigger.get() {
        menu = menu.trigger(
            view().slot(Some(trigger), move |s| s.on_press(move || change(!open.get_untracked()))),
        );
    }
    menu
}

/// The props of [`ContextMenu`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ContextMenuProps {
    /// The menu's rows: `MenuItem`, `MenuSeparator`, `MenuLabel` (in Rust,
    /// a `view()` of them).
    pub menu: Prop<Option<Slot>>,
    /// A secondary click opens nothing.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The area a secondary click (a long press on touch screens, Shift+F10
    /// or the Menu key) opens it over.
    pub children: Prop<Children>,
}

/// A menu at the pointer, opened by a secondary click on its area:
/// `<ContextMenu menu={<><MenuItem label="Copy" /></>}>…</ContextMenu>`.
#[component]
pub fn ContextMenu(props: ContextMenuProps, cx: Scope<MenuEvent>) -> impl IntoElement {
    let mut menu = menu_context()
        .disabled(props.disabled)
        .on_open_change(move |e: &nsh_base::ContextMenuEvents| {
            if let nsh_base::ContextMenuEvents::OpenChange(open, _) = e {
                cx.emit(MenuEvent::OpenChange(*open));
            }
        })
        .trigger(view().child(props.children));
    if let Some(rows) = props.menu.get() {
        // In a view of ours: a slot placed straight into the base menu's
        // children is looked up there.
        menu = menu.child(view().gap(px(1.0)).slot(Some(rows), |s| s.gap(px(1.0))));
    }
    menu
}

/// The props of [`MenuItem`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct MenuItemProps {
    /// The row's text.
    pub label: Prop<SharedString>,
    /// A glyph before the label, by its Lucide name (`"copy"`).
    pub icon: Prop<Option<IconName>>,
    /// A shortcut at the row's end, as the platform writes it: `"mod-s"`
    /// reads `cmd-s` on macOS, `ctrl-s` elsewhere.
    pub shortcut: Prop<Option<SharedString>>,
    /// A checkable row: an accent check at its right edge while `true`.
    /// Left out, the row is not checkable.
    pub checked: Prop<Option<bool>>,
    /// Destructive: danger text.
    #[prop(default)]
    pub danger: Prop<bool>,
    /// Not choosable; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Choosing it closes the menu (`true` when left out; a checkable row
    /// stays open).
    pub close_on_select: Prop<Option<bool>>,
}

/// The events of a [`MenuItem`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum MenuItemEvent {
    /// It was chosen (a press, Return or Space).
    Select,
}

/// A row of a menu: `<MenuItem label="New" shortcut="mod-n" onSelect={newFile} />`.
#[component]
pub fn MenuItem(props: MenuItemProps, cx: Scope<MenuItemEvent>) -> impl IntoElement {
    let checked = props.checked;
    let is_checkable = checked.with_untracked(Option::is_some);
    let label = props.label;
    let glyph = props.icon;
    let danger = props.danger;
    let disabled = props.disabled;
    let mut row = row_look(nsh_base::menu_item());
    if is_checkable {
        row = checkable(row, move || checked.get().unwrap_or(false));
    }
    let mut row = row
        .disabled(disabled)
        .on_select(move || cx.emit(MenuItemEvent::Select))
        .style_when(
            move || danger.get() && !disabled.get(),
            |s| s.text_color(move || tokens::danger()),
        )
        .child(dynamic(move || glyph.get().map(|name| icon(name).size(px(14.0)).into_element())))
        .child(text(move || label.get()).flex_1().number_of_lines(1));
    if let Some(close) = props.close_on_select.get_untracked() {
        row = row.close_on_select(close);
    }
    if let Some(keys) = props.shortcut.get_untracked() {
        row = row.child(menu_shortcut(keys));
    }
    if is_checkable {
        row = row.child(check_mark(move || checked.get().unwrap_or(false)));
    }
    row
}

/// A 1pt divider between groups of rows.
#[component]
pub fn MenuSeparator() -> impl IntoElement {
    menu_divider()
}

/// The props of [`MenuLabel`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct MenuLabelProps {
    /// The group's heading.
    pub label: Prop<SharedString>,
}

/// A group's heading inside a menu: not focusable, not choosable.
#[component]
pub fn MenuLabel(props: MenuLabelProps) -> impl IntoElement {
    menu_heading(props.label)
}

/// Registers the menus for TSX.
pub(crate) fn register(registry: &mut Registry) {
    registry
        .register::<Menu>()
        .register::<ContextMenu>()
        .register::<MenuItem>()
        .register::<MenuSeparator>()
        .register::<MenuLabel>();
}
