//! [`Tree`]: items that hold items, expanded and collapsed, one selected,
//! over `nsh_base`'s virtualized `Tree` as [`crate::components::tree`]
//! styles it; and its item type [`TreeItem`].

use std::collections::HashMap;
use std::rc::Rc;

use nsh_base::{SelectValue, TreeEntry};
use nsh_core::prelude::*;

use crate::components::icon::IconName;
use crate::components::tree as look;

/// An item of a [`Tree`], with the items under it (with any, it is a
/// folder). `TreeItem::new("src", "src").icon(IconName::Folder)
/// .child(TreeItem::new("main.rs", "main.rs"))`; TSX
/// `{ id, label, icon?, disabled?, children? }`.
#[derive(Debug, Clone, Default, PartialEq, JsObject)]
#[non_exhaustive]
pub struct TreeItem {
    /// What names it in `expanded`, `selected` and the events, unique in
    /// the tree (a path, a key), never its label.
    pub id: SharedString,
    /// What its row shows and screen readers say.
    pub label: SharedString,
    /// A glyph before the label.
    pub icon: Option<IconName>,
    /// It cannot be selected, expanded or collapsed: dimmed, passed over.
    #[prop(default)]
    pub disabled: bool,
    /// The items under it, in order.
    #[prop(default)]
    pub children: Vec<TreeItem>,
}

impl TreeItem {
    /// An item named `id`, labelled `label`.
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            ..Self::default()
        }
    }

    /// A glyph before its label.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Whether the user cannot act on it.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Adds `child` under it.
    pub fn child(mut self, child: TreeItem) -> Self {
        self.children.push(child);
        self
    }

    /// Adds `children` under it.
    pub fn children(mut self, children: impl IntoIterator<Item = TreeItem>) -> Self {
        self.children.extend(children);
        self
    }

    /// The `nsh_base` item, with its items; its glyph noted in `icons`.
    fn to_base(&self, icons: &mut HashMap<SharedString, IconName>) -> nsh_base::TreeItem {
        if let Some(icon) = self.icon {
            icons.insert(self.id.clone(), icon);
        }
        nsh_base::TreeItem::new(self.id.clone(), self.label.clone())
            .disabled(self.disabled)
            .children(self.children.iter().map(|c| c.to_base(icons)))
    }
}

/// The items as `nsh_base` takes them, and their glyphs by id.
#[derive(Clone, PartialEq)]
struct Items {
    base: Vec<nsh_base::TreeItem>,
    icons: Rc<HashMap<SharedString, IconName>>,
}

/// The props of [`Tree`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TreeProps {
    /// The items at the top, each with the items under it.
    pub items: Prop<Vec<TreeItem>>,
    /// The `id`s of the expanded items: controlled when given (set them in
    /// `onExpandedChange`); left out, the tree keeps its own.
    pub expanded: Prop<Option<Vec<SharedString>>>,
    /// The selected item's `id`, `null` for none: controlled when given
    /// (set it in `onSelectedChange`); left out, the tree keeps its own.
    #[prop(default)]
    pub selected: Prop<SelectValue>,
    /// What screen readers call the tree.
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`Tree`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum TreeEvent {
    /// The user expanded or collapsed an item (a press, Return or Space,
    /// Right or Left): the expanded items' ids as they would be.
    #[args(expanded)]
    ExpandedChange(Vec<SharedString>),
    /// The user selected another item (a press, the arrows, Home or End,
    /// typing its label's start): its id.
    #[args(selected)]
    SelectedChange(SharedString),
}

/// A tree of rows a control tall: a folder's chevron that turns as it
/// expands, a glyph and a label indented by depth, a hover wash, the
/// selected row's fill. One Tab stop: Up and Down move the selection,
/// Right expands or enters a folder, Left collapses it or goes to its
/// parent. Give it a height.
#[component]
pub fn Tree(props: TreeProps, cx: Scope<TreeEvent>) -> impl IntoElement {
    let items = computed(move || {
        let mut icons = HashMap::new();
        let base = props
            .items
            .with(|items| items.iter().map(|i| i.to_base(&mut icons)).collect());
        Items {
            base,
            icons: Rc::new(icons),
        }
    });
    let own_expanded = signal(Vec::<SharedString>::new());
    let own_selected = signal(None::<SharedString>);
    look::tree()
        .accessibility_label(move || props.accessibility_label.get())
        .items(move || items.with(|i| i.base.clone()))
        .expanded(move || Some(props.expanded.get().unwrap_or_else(|| own_expanded.get())))
        .selected(move || match props.selected.get() {
            SelectValue::Controlled(id) => SelectValue::Controlled(id),
            _ => SelectValue::Controlled(own_selected.get()),
        })
        .render_item(move |entry: &TreeEntry| {
            let glyph = items.with_untracked(|i| i.icons.get(&entry.id).copied());
            look::tree_row(entry, glyph)
        })
        .on_expanded_change(move |next: &Vec<SharedString>| {
            own_expanded.set(next.clone());
            cx.emit(TreeEvent::ExpandedChange(next.clone()));
        })
        .on_selected_change(move |id: &SharedString| {
            own_selected.set(Some(id.clone()));
            cx.emit(TreeEvent::SelectedChange(id.clone()));
        })
}
