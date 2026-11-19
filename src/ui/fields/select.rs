//! [`Select`] and [`Combobox`], on `nsh_base`'s select and combobox
//! ([`crate::components::select`]).

use nsh_base::SelectValue;
use nsh_core::prelude::*;

use super::{Value, rest_border};
use crate::components::button::ControlSize;
use crate::components::select::{combobox_field, select_popup, select_trigger};
use crate::types::Controlled;
use crate::fields::{SelectItem};

/// `items` as `nsh_base`'s.
fn base_items(items: &[SelectItem]) -> Vec<nsh_base::SelectItem> {
    items.iter().map(SelectItem::to_base).collect()
}

/// The props of [`Select`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SelectProps {
    /// The items, in order (`group` sections them).
    pub items: Prop<Vec<SelectItem>>,
    /// The chosen item's `value`, `null` for none. Set, the app owns it
    /// and writes back what `onChange` reports; left out, the select keeps
    /// the choice.
    #[prop(default)]
    pub value: Prop<Controlled<Option<SharedString>>>,
    /// What the trigger says while nothing is chosen.
    pub placeholder: Prop<Option<SharedString>>,
    /// A search field above the items filters them.
    #[prop(default)]
    pub searchable: Prop<bool>,
    /// `small` (24 points tall), `medium` (28, the default) or `large` (32).
    #[prop(default)]
    pub size: Prop<ControlSize>,
    /// It does not open; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Draws the danger border.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// What the choice is about, for screen readers (`"Theme"`).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`Select`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SelectEvent {
    /// The user chose an item: its `value`.
    #[args(value)]
    Change(SharedString),
}

/// A trigger that opens a list: Up / Down / Home / End and typeahead move,
/// Return or Space choose, Escape and a press outside close, the keyboard
/// goes back to the trigger.
#[component]
pub fn Select(props: SelectProps, cx: Scope<SelectEvent>) -> impl IntoElement {
    let label = super::spoken(props.accessibility_label);
    let value = Value::new(props.value, None);
    let shown = computed(move || {
        let chosen = value.get()?;
        props.items.with(|items| {
            items
                .iter()
                .find(|item| item.value == chosen)
                .map(|item| item.label.clone())
        })
    });
    let size = move || props.size.get();
    let trigger = select_trigger(
        move || shown.get(),
        move || props.placeholder.get().unwrap_or_default(),
        move || props.disabled.get(),
    )
    .h(move || px(size().height()))
    .text_size(move || px(size().text_size()))
    .border_color(move || rest_border(props.invalid.get()));
    select_popup(trigger)
        .self_start()
        .min_w(px(160.0))
        .cover(false)
        .items(move || props.items.with(|items| base_items(items)))
        .value(move || SelectValue::Controlled(value.get()))
        .placeholder(move || props.placeholder.get())
        .searchable(move || props.searchable.get())
        .disabled(move || props.disabled.get())
        .accessibility_label(move || label())
        .on_change(move |next: &Option<SharedString>| {
            if let Some(next) = next {
                value.request(Some(next.clone()));
                cx.emit(SelectEvent::Change(next.clone()));
            }
        })
}

/// The props of [`Combobox`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ComboboxProps {
    /// The items, in order (`group` sections them).
    pub items: Prop<Vec<SelectItem>>,
    /// The chosen items' values. Set, the app owns them and writes back
    /// what `onChange` reports; left out, the combobox keeps them.
    #[prop(default)]
    pub value: Prop<Controlled<Vec<SharedString>>>,
    /// Several values: a choice toggles one and the list stays open.
    #[prop(default)]
    pub multiple: Prop<bool>,
    /// Shown in the empty field.
    pub placeholder: Prop<Option<SharedString>>,
    /// No typing, no list; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What the choice is about, for screen readers; tests type into the
    /// field by it (`label:Theme`).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`Combobox`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ComboboxEvent {
    /// The user chose or removed an item: the values asked for.
    #[args(value)]
    Change(Vec<SharedString>),
}

/// A field whose typing filters the list; Down / Up or the chevron open
/// it, Return chooses the highlighted row.
#[component]
pub fn Combobox(props: ComboboxProps, cx: Scope<ComboboxEvent>) -> impl IntoElement {
    let label = super::spoken(props.accessibility_label);
    let value = Value::new(props.value, Vec::new());
    combobox_field(Vec::new())
        .self_start()
        .min_w(px(224.0))
        .items(move || props.items.with(|items| base_items(items)))
        .value(move || Some(value.get()))
        .multiple(move || props.multiple.get())
        .placeholder(move || props.placeholder.get())
        .disabled(move || props.disabled.get())
        .accessibility_label(move || label())
        .on_change(move |next: &Vec<SharedString>| {
            value.request(next.clone());
            cx.emit(ComboboxEvent::Change(next.clone()));
        })
}
