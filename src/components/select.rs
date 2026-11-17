//! Choosing from a list: [`select_field`] (a trigger that opens the list)
//! and [`combobox_field`] (a field whose typing filters the list).
//!
//! Behavior is `nsh_base`'s `select` / `combobox`: the popup placed at the
//! trigger, Up / Down / Home / End / Page keys, typeahead, Return or Space
//! to choose, Escape and a press outside to close, the keyboard back on the
//! trigger, screen readers' combobox / listbox / option roles. This module
//! draws it the Omarchy way: a square trigger in the control look, a
//! popup on the popover background with a 1px border, rows that wash with
//! `hover_fill` while highlighted and carry `selected_fill` with bright
//! text while chosen.
//!
//! Items are `nsh_base::SelectItem`s (re-exported here). The names avoid
//! `nsh_base::select` / `nsh_base::combobox`.

use nsh_base::{ComboboxElement, SelectElement, combobox, select};
use nsh_core::element::StyleRefinement;
use nsh_core::prelude::*;

pub use nsh_base::{ComboboxEvent, SelectEvent, SelectItem};

use super::input::state_classes::{DISABLED, HIGHLIGHT, PLACEHOLDER};
use crate::tokens;

/// The tallest the list grows before it scrolls.
const LIST_MAX_HEIGHT: f32 = 224.0;
/// What the chevron of a closed list shows.
const CHEVRON: &str = "\u{25be}";

/// The popup the list is in.
fn popup_style(s: StyleRefinement) -> StyleRefinement {
    s.p(px(tokens::SPACE_1))
        .rounded(px(tokens::RADIUS))
        .border_1()
        .border_color(move || tokens::control_border())
        .bg(move || tokens::popover())
        .text_color(move || tokens::foreground())
}

/// The scroll view of the rows.
fn list_style(s: StyleRefinement) -> StyleRefinement {
    s.max_h(px(LIST_MAX_HEIGHT))
}

/// Each row: its highlight is the class's (`focus:`), its chosen state
/// this style's (`selected:`, over the class).
fn item_style(s: StyleRefinement) -> StyleRefinement {
    s.flex_row()
        .items_center()
        .h(px(tokens::CONTROL_MD))
        .px(px(tokens::SPACE_2))
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .selected(|s| {
            s.bg(move || tokens::selected_fill())
                .text_color(move || tokens::bright())
        })
}

fn item_class() -> SharedString {
    format!("{HIGHLIGHT} {DISABLED}").into()
}

/// What the list shows while nothing matches.
fn empty_note(words: &'static str) -> impl IntoElement {
    text(words)
        .px(px(tokens::SPACE_2))
        .py(px(tokens::SPACE_2))
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::secondary())
}

fn chevron() -> Text {
    text(CHEVRON)
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::secondary())
}

/// One value chosen from `items`, shown on a trigger that reads `value`
/// (the chosen item's `value`, `None` for none, shown as `placeholder`).
///
/// Controlled: the owner writes the value it hears.
///
/// ```ignore
/// let layout = signal(None::<SharedString>);
/// select_field(items, move || layout.get(), "Choose a layout")
///     .on_change(move |v: &Option<SharedString>| layout.set(v.clone()))
///     .accessibility_label("Layout")
/// ```
///
/// `.searchable(true)` adds a filtering field above the rows,
/// `.disabled(..)` dims it; the rest of `nsh_base::select`'s props apply.
/// The trigger fills the width it is given.
pub fn select_field(
    items: Vec<SelectItem>,
    value: impl Fn() -> Option<SharedString> + Clone + 'static,
    placeholder: impl Into<SharedString>,
) -> SelectElement {
    let placeholder: SharedString = placeholder.into();
    let titles = items.clone();
    let shown = {
        let value = value.clone();
        computed(move || {
            let chosen = value()?;
            titles
                .iter()
                .find(|item| item.value == chosen)
                .map(|item| item.title.clone())
        })
    };
    let trigger = view()
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_2))
        .h(px(tokens::CONTROL_MD))
        .px(px(tokens::SPACE_2))
        .rounded(px(tokens::RADIUS))
        .border_1()
        .border_color(move || tokens::control_border())
        .bg(move || tokens::normal_fill())
        .cursor_pointer()
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| s.border_color(move || tokens::focus_border()))
        .expanded(|s| s.border_color(move || tokens::focus_border()))
        .disabled_style(|s| {
            s.text_color(move || tokens::disabled())
                .border_color(move || tokens::divider())
        })
        .child(
            text(move || shown.get().unwrap_or_else(|| placeholder.clone()))
                .flex_1()
                .min_w(px(0.0))
                .number_of_lines(1)
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || {
                    if shown.with(Option::is_some) {
                        tokens::foreground()
                    } else {
                        tokens::secondary()
                    }
                }),
        )
        .child(chevron());
    select()
        .items(items)
        .value(move || value())
        .offset(px(tokens::SPACE_1))
        .popup_style(popup_style)
        .list_style(list_style)
        .item_class(item_class())
        .item_style(item_style)
        .search_placeholder("Search options\u{2026}")
        .search_class(SharedString::from(PLACEHOLDER))
        .search_style(|s: StyleRefinement| {
            s.h(px(tokens::CONTROL_MD))
                .mb(px(tokens::SPACE_1))
                .px(px(tokens::SPACE_2))
                .border_1()
                .border_color(move || tokens::focus_border())
                .bg(move || tokens::normal_fill())
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || tokens::foreground())
        })
        .empty(empty_note("No matching options"))
        .trigger(trigger)
}

/// One value (or, with `.multiple(true)`, several) chosen from `items` by
/// typing to filter them. Controlled: `.value(move || Some(chosen.get()))`
/// (a `Vec<SharedString>`) plus
/// `.on_change(move |v: &Vec<SharedString>| ..)`; `.placeholder(..)` for the
/// empty field. Typing, Down / Up or the chevron opens the list; Return
/// chooses the highlighted row. While an input method composes, the field
/// is the platform's and nothing is filtered or rewritten.
pub fn combobox_field(items: Vec<SelectItem>) -> ComboboxElement {
    combobox()
        .items(items)
        .flex_row()
        .items_stretch()
        .h(px(tokens::CONTROL_MD))
        .min_w(px(0.0))
        .rounded(px(tokens::RADIUS))
        .border_1()
        .border_color(move || tokens::control_border())
        .bg(move || tokens::normal_fill())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .focus_within(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
        .disabled_style(|s| {
            s.text_color(move || tokens::disabled())
                .border_color(move || tokens::divider())
        })
        .input_class(SharedString::from(PLACEHOLDER))
        .input_style(|s: StyleRefinement| {
            s.flex_1()
                .min_w(px(0.0))
                .px(px(tokens::SPACE_2))
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || tokens::foreground())
        })
        .trigger_button(
            view()
                .w(px(tokens::CONTROL_MD))
                .h_full()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .hover(|s| s.bg(move || tokens::hover_fill()))
                .active(|s| s.bg(move || tokens::pressed_fill()))
                .child(chevron()),
        )
        .offset(px(tokens::SPACE_1))
        .popup_style(popup_style)
        .list_style(list_style)
        .item_class(item_class())
        .item_style(item_style)
        .empty(empty_note("No matching options"))
}
