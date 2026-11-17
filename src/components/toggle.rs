//! Omarchy toggles: a button that is on or off, alone or in a toggle group,
//! after gpui-omarchy's `toggle` and `toggle_group`.
//!
//! A toggle rests with a `border()` outline; hover and keyboard focus wash it
//! with `hover_fill()` and draw the `focus_border()`; a press in progress
//! brightens the outline; while on it keeps `selected_fill()` and `bright()`
//! text. A disabled toggle draws `disabled()` text and none of those states.
//!
//! Toggles are controlled: give `.pressed(..)` the owner's value and write
//! it back in `.on_change(..)`, which reports the value asked for. The
//! constructors return `nsh_base`'s elements, so their builder API stays.

use nsh_base::{ToggleElement, toggle as base_toggle, toggle_group as base_toggle_group};
use nsh_core::prelude::*;

use super::button::{ButtonSize, actions_duration};
use super::button_group::labelled_group;
use crate::tokens;

/// A styled toggle with no content: add a label, an icon or both with
/// `.child(..)`. Give an icon-only toggle an `.accessibility_label(..)`.
pub fn toggle_style(size: ButtonSize) -> ToggleElement {
    base_toggle()
        .flex_row()
        .flex_shrink_0()
        .self_start()
        .items_center()
        .justify_center()
        .h(px(size.height()))
        .px(px(size.padding()))
        .gap(px(size.gap()))
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(move || tokens::border())
        .bg(AdaptiveColor::TRANSPARENT)
        .text_color(move || tokens::foreground())
        .text_size(px(size.text_size()))
        .cursor_pointer()
        .transition_colors()
        .duration(move || actions_duration(tokens::DURATION_FAST))
        .hover(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
        .focus_visible(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
        .active(|s| {
            s.bg(move || tokens::pressed_fill())
                .border_color(move || tokens::bright())
        })
        // On: the selection fill and bright text, the outline left to hover
        // and focus.
        .aria_pressed(|s| {
            s.bg(move || tokens::selected_fill())
                .text_color(move || tokens::bright())
        })
        // `disabled:` ranks above hover, focus and press; `aria-pressed:`
        // ranks above it, so a disabled toggle that is on keeps its fill.
        .disabled_style(|s| {
            s.bg(AdaptiveColor::TRANSPARENT)
                .border_color(move || tokens::divider())
                .text_color(move || tokens::disabled())
                .cursor(Cursor::Default)
        })
}

/// A labelled toggle, medium size:
/// `toggle("Bold").pressed(bold).on_change(move |on: &bool| bold.set(*on))`.
pub fn toggle(label: impl IntoReactive<SharedString>) -> ToggleElement {
    toggle_style(ButtonSize::Md).child(text(label))
}

/// A labelled toggle of `size`.
pub fn toggle_sized(label: impl IntoReactive<SharedString>, size: ButtonSize) -> ToggleElement {
    toggle_style(size).child(text(label))
}

/// A toggle with an icon before its label.
pub fn toggle_with_icon(
    icon: impl IntoElement,
    label: impl IntoReactive<SharedString>,
    size: ButtonSize,
) -> ToggleElement {
    toggle_style(size).child(icon).child(text(label))
}

/// A square, icon-only toggle. `label` says what it turns on, for screen
/// readers and test selectors.
pub fn icon_toggle(
    icon: impl IntoElement,
    label: impl IntoReactive<SharedString>,
    size: ButtonSize,
) -> ToggleElement {
    let label: Prop<SharedString> = label.into_reactive().into();
    toggle_style(size)
        .accessibility_label(move || Some(label.get()))
        .w(px(size.height()))
        .px(px(0.0))
        .child(icon)
}

/// A row of `toggles`, any number of them on, named `label` for screen
/// readers: one stop of Tab, whose arrow keys move between its toggles (Home
/// and End to the ends) without pressing them; Space and Return press the
/// focused one. Each toggle keeps its own controlled `pressed`.
pub fn toggle_group(
    label: impl IntoReactive<SharedString>,
    toggles: impl IntoIterator<Item = impl IntoElement>,
) -> View {
    labelled_group(
        label,
        base_toggle_group()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(tokens::SPACE_1))
            .children(toggles.into_iter().map(IntoElement::into_element)),
    )
}
