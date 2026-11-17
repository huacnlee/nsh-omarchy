//! Omarchy single-choice groups, after gpui-omarchy's `button_group` and
//! `tab_list`: equal segments in one square frame, one stop of Tab each.
//!
//! - [`button_group`]: a setting with one value (a toolbar's position).
//!   Left / Right (and Up / Down, Home, End) move the keyboard focus between
//!   segments without choosing; Space and Return choose the focused one.
//!   The chosen segment keeps `selected_fill()` and `bright()` text.
//! - [`tab_list`]: page tabs. The arrows move the focus and select as they
//!   go (WAI-ARIA's automatic activation); the selection is the selected
//!   fill, which slides to the selected tab.
//!
//! Both skip disabled items and are controlled: they show `selected` and
//! report the index asked for with `on_change`, never the current one.

use std::rc::Rc;

use nsh_base::{
    tab as base_tab, tabs as base_tabs, toggle as base_toggle, toggle_group as base_toggle_group,
};
use nsh_core::prelude::*;

use super::button::actions_duration;
use crate::tokens;

/// One choice of a [`button_group`] or a [`tab_list`].
#[derive(Clone, Debug, PartialEq)]
pub struct ButtonGroupItem {
    /// What the segment shows and screen readers say.
    pub label: SharedString,
    /// Not choosable, and skipped by the arrow keys.
    pub disabled: bool,
}

impl ButtonGroupItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl From<&'static str> for ButtonGroupItem {
    fn from(label: &'static str) -> Self {
        Self::new(label)
    }
}

impl From<SharedString> for ButtonGroupItem {
    fn from(label: SharedString) -> Self {
        Self::new(label)
    }
}

type Change = Rc<dyn Fn(usize)>;

/// Reports `index` unless it is already the selected one.
fn change_to(selected: Prop<Option<usize>>, on_change: &Change, index: usize) {
    if selected.get_untracked() != Some(index) {
        on_change(index);
    }
}

/// The group screen readers hear `label` name, around `group`: `nsh_base`'s
/// groups take no label of their own.
pub(crate) fn labelled_group(
    label: impl IntoReactive<SharedString>,
    group: impl IntoElement,
) -> View {
    view()
        .role(AccessibilityRole::Group)
        .label(label)
        .child(group)
}

/// A single-choice setting drawn as equal segments in one frame, named
/// `label` for screen readers. The owner keeps `selected` and writes it in
/// `on_change`. The items are read when the group is made.
pub fn button_group(
    label: impl IntoReactive<SharedString>,
    items: impl IntoIterator<Item = impl Into<ButtonGroupItem>>,
    selected: impl IntoReactive<Option<usize>>,
    on_change: impl Fn(usize) + 'static,
) -> View {
    let selected: Prop<Option<usize>> = selected.into_reactive().into();
    let on_change: Change = Rc::new(on_change);
    let segments: Vec<AnyElement> = items
        .into_iter()
        .map(Into::into)
        .enumerate()
        .map(|(index, item)| segment(index, item, selected, on_change.clone()).into_element())
        .collect();
    labelled_group(
        label,
        base_toggle_group()
            .flex_row()
            .gap(px(tokens::SPACE_1))
            .p(px(tokens::SPACE_1))
            .rounded(px(tokens::RADIUS))
            .border(px(tokens::BORDER))
            .border_color(move || tokens::divider())
            .children(segments),
    )
}

fn segment(
    index: usize,
    item: ButtonGroupItem,
    selected: Prop<Option<usize>>,
    on_change: Change,
) -> impl IntoElement {
    let chosen = move || selected.get() == Some(index);
    base_toggle()
        .pressed(chosen)
        .disabled(item.disabled)
        .on_change(move |on: &bool| {
            // A press on the chosen segment asks to turn it off: a
            // single choice keeps it.
            if *on {
                change_to(selected, &on_change, index);
            }
        })
        .flex_1()
        .flex_row()
        .items_center()
        .justify_center()
        .h(px(tokens::CONTROL_SM))
        .px(px(tokens::SPACE_3))
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(AdaptiveColor::TRANSPARENT)
        .bg(AdaptiveColor::TRANSPARENT)
        .text_color(move || tokens::foreground())
        .text_size(px(tokens::TEXT_MD))
        .cursor_pointer()
        .transition_colors()
        .duration(move || actions_duration(tokens::DURATION_FAST))
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .focus_visible(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .aria_pressed(|s| {
            s.bg(move || tokens::selected_fill())
                .text_color(move || tokens::bright())
        })
        .disabled_style(|s| {
            s.bg(AdaptiveColor::TRANSPARENT)
                .border_color(AdaptiveColor::TRANSPARENT)
                .text_color(move || tokens::disabled())
                .cursor(Cursor::Default)
        })
        .child(text(item.label))
}

/// Page tabs in one square frame, named `label` for screen readers. The
/// owner keeps `selected`, writes it in `on_change` and shows the selected
/// tab's panel. The items are read when the list is made.
pub fn tab_list(
    label: impl IntoReactive<SharedString>,
    items: impl IntoIterator<Item = impl Into<ButtonGroupItem>>,
    selected: impl IntoReactive<Option<usize>>,
    on_change: impl Fn(usize) + 'static,
) -> View {
    let selected: Prop<Option<usize>> = selected.into_reactive().into();
    let on_change: Change = Rc::new(on_change);
    let tabs: Vec<AnyElement> = items
        .into_iter()
        .map(Into::into)
        .enumerate()
        .map(|(index, item)| tab_item(index, item, selected, on_change.clone()).into_element())
        .collect();
    let list = base_tabs()
        .flex_row()
        .self_start()
        .gap(px(tokens::SPACE_1))
        .p(px(tokens::SPACE_1))
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(move || tokens::divider())
        .bg(move || tokens::normal_fill())
        // Behind the selected tab; it slides to the next one natively (at
        // once with reduced motion).
        .indicator_style(|s| {
            s.top(px(tokens::SPACE_1))
                .bottom(px(tokens::SPACE_1))
                .rounded(px(tokens::RADIUS))
                .bg(move || tokens::selected_fill())
        })
        .children(tabs);
    labelled_group(label, list).items_start()
}

fn tab_item(
    index: usize,
    item: ButtonGroupItem,
    selected: Prop<Option<usize>>,
    on_change: Change,
) -> impl IntoElement {
    let tab = base_tab()
        .selected(move || selected.get() == Some(index))
        .disabled(item.disabled)
        .on_press(move || change_to(selected, &on_change, index))
        .flex_row()
        .flex_shrink_0()
        .items_center()
        .h(px(tokens::CONTROL_SM))
        .px(px(tokens::SPACE_3))
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(AdaptiveColor::TRANSPARENT)
        .text_color(move || tokens::secondary())
        .text_size(px(tokens::TEXT_MD))
        .cursor_pointer()
        .transition_colors()
        .duration(move || actions_duration(tokens::DURATION_FAST))
        .hover(|s| {
            s.bg(move || tokens::hover_fill())
                .text_color(move || tokens::foreground())
        })
        .focus_visible(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .disabled_style(|s| {
            s.bg(AdaptiveColor::TRANSPARENT)
                .border_color(AdaptiveColor::TRANSPARENT)
                .text_color(move || tokens::disabled())
                .cursor(Cursor::Default)
        })
        .child(text(item.label));
    Styled::selected(tab, |s| s.text_color(move || tokens::bright()))
}
