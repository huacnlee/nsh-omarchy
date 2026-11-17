//! Omarchy radios: a square box with a square dot while checked, and its
//! label, on `nsh_base`'s radio; [`radio_group`] holds a set of them.
//!
//! The group is one stop of Tab, at the checked radio; Up/Down and
//! Left/Right move the focus around the set and check the radio it lands
//! on; Space checks the focused radio (WAI-ARIA's radio group).
//!
//! States: hover and focus-visible wash the row and outline it in the
//! focus border; a press darkens the wash; checked draws the dot; disabled
//! dims the text, box and dot and takes no interaction styles.

use nsh_base::{RadioGroupElement, radio as base_radio, radio_group as base_radio_group};
use nsh_core::prelude::*;

use crate::tokens;

/// The box's side.
const BOX: f32 = 16.0;
/// The dot's side.
const DOT: f32 = 8.0;

/// The props of [`RadioControl`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct RadioControlProps {
    /// The label after the box, also what screen readers say.
    #[prop(default)]
    pub label: Prop<SharedString>,
    /// Checked or not. The owner keeps which radio of the set is.
    #[prop(default)]
    pub checked: Prop<bool>,
    /// No presses, skipped by the arrow keys, dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
}

/// The events of a [`RadioControl`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum RadioControlEvent {
    /// An unchecked radio was pressed or reached by an arrow key: it asks to
    /// be checked (always `true`).
    #[args(checked)]
    Change(bool),
}

/// An Omarchy radio. Prefer [`radio`], which sets the label.
#[component]
pub fn RadioControl(props: RadioControlProps, cx: Scope<RadioControlEvent>) -> impl IntoElement {
    let RadioControlProps {
        label,
        checked,
        disabled,
    } = props;
    base_radio()
        .checked(checked)
        .disabled(disabled)
        .accessibility_label(move || Some(label.get()))
        .on_change(move |next: &bool| cx.emit(RadioControlEvent::Change(*next)))
        .flex_row()
        // Hug the box and its label instead of stretching across a column.
        .self_start()
        .items_center()
        .gap(px(tokens::SPACE_2))
        .min_h(px(tokens::CONTROL_MD))
        .px(px(tokens::SPACE_1))
        .border_1()
        .border_color(move || AdaptiveColor::TRANSPARENT)
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::foreground())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
        .focus_visible(|s| {
            s.bg(move || tokens::hover_fill())
                .border_color(move || tokens::focus_border())
        })
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .disabled_style(|s| s.text_color(move || tokens::disabled()))
        .child(
            view()
                .w(px(BOX))
                .h(px(BOX))
                .flex_shrink_0()
                .items_center()
                .justify_center()
                .border_1()
                .border_color(move || {
                    if disabled.get() {
                        tokens::divider()
                    } else {
                        tokens::control_border()
                    }
                })
                .bg(move || tokens::normal_fill())
                .child(
                    view()
                        .w(px(DOT))
                        .h(px(DOT))
                        .bg(move || {
                            if disabled.get() {
                                tokens::disabled()
                            } else {
                                tokens::bright()
                            }
                        })
                        .opacity(move || if checked.get() { 1.0_f32 } else { 0.0 })
                        .transition_opacity()
                        .duration(tokens::DURATION_FAST),
                ),
        )
        .child(text(label))
}

/// A radio labelled `label`, inside a [`radio_group`]: set `.checked(..)`,
/// `.disabled(..)` and check it when `.on_change(..)` asks.
///
/// ```ignore
/// radio_group()
///     .child(radio("Light").checked(move || mode.get() == 0).on_change(move |_: &bool| mode.set(0)))
///     .child(radio("Dark").checked(move || mode.get() == 1).on_change(move |_: &bool| mode.set(1)))
/// ```
pub fn radio(label: impl Into<SharedString>) -> RadioControlElement {
    radio_control().label(label.into())
}

/// The container of a set of [`radio`]s: one stop of Tab, arrow keys move
/// between them. A column; `.flex_row()` lays them in a row.
pub fn radio_group() -> RadioGroupElement {
    base_radio_group().gap(px(tokens::SPACE_1))
}
