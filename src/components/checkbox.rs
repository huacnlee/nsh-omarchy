//! Omarchy checkbox: a square box with a drawn check (or a dash while
//! indeterminate) and its label, on `nsh_base`'s checkbox.
//!
//! States: hover and focus-visible wash the row and outline it in the
//! focus border; a press darkens the wash; checked and indeterminate fill
//! the box with the selected fill; disabled dims the text and box and takes
//! no hover, press or focus styles. Colors fade over `DURATION_FAST`.

use nsh_base::checkbox as base_checkbox;
use nsh_core::prelude::*;

use crate::tokens;

/// The box's side.
const BOX: f32 = 16.0;

/// The props of [`CheckboxControl`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CheckboxControlProps {
    /// The label after the box, also what screen readers say.
    #[prop(default)]
    pub label: Prop<SharedString>,
    /// Checked or not. The owner keeps it and writes the value `on_change`
    /// reports.
    #[prop(default)]
    pub checked: Prop<bool>,
    /// Neither checked nor unchecked (a "select all" over a partial
    /// selection); wins over `checked`. A press asks for checked.
    #[prop(default)]
    pub indeterminate: Prop<bool>,
    /// No presses, dimmed, no interaction styles.
    #[prop(default)]
    pub disabled: Prop<bool>,
}

/// The events of a [`CheckboxControl`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum CheckboxControlEvent {
    /// A press (click, tap, Space or Return) asks for this value.
    #[args(checked)]
    Change(bool),
}

/// An Omarchy checkbox. Prefer [`checkbox`], which sets the label.
#[component]
pub fn CheckboxControl(
    props: CheckboxControlProps,
    cx: Scope<CheckboxControlEvent>,
) -> impl IntoElement {
    let CheckboxControlProps {
        label,
        checked,
        indeterminate,
        disabled,
    } = props;
    let shows_check = computed(move || checked.get() && !indeterminate.get());
    let marked = computed(move || checked.get() || indeterminate.get());
    let mark_color = move || {
        if disabled.get() {
            tokens::disabled()
        } else {
            tokens::bright()
        }
    };
    base_checkbox()
        .checked(checked)
        .indeterminate(indeterminate)
        .disabled(disabled)
        .accessibility_label(move || Some(label.get()))
        .on_change(move |next: &bool| cx.emit(CheckboxControlEvent::Change(*next)))
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
        .indicator_style(move |s| {
            s.w(px(BOX))
                .h(px(BOX))
                .flex_shrink_0()
                .items_center()
                .justify_center()
                .border_1()
                .border_color(move || {
                    if disabled.get() {
                        tokens::divider()
                    } else if marked.get() {
                        AdaptiveColor::TRANSPARENT
                    } else {
                        tokens::control_border()
                    }
                })
                .bg(move || {
                    if marked.get() && !disabled.get() {
                        tokens::selected_fill()
                    } else {
                        tokens::normal_fill()
                    }
                })
                .transition_colors()
                .duration(tokens::DURATION_FAST)
        })
        .indicator(
            view()
                .items_center()
                .justify_center()
                .child(when(shows_check, move || check_mark(mark_color)))
                .child(when(indeterminate, move || dash_mark(mark_color))),
        )
        .child(text(label))
}

/// A checkbox labelled `label`: set `.checked(..)`, `.indeterminate(..)`,
/// `.disabled(..)` and write the value `.on_change(..)` reports.
///
/// ```ignore
/// checkbox("Show hidden files")
///     .checked(show)
///     .on_change(move |next: &bool| show.set(*next))
/// ```
pub fn checkbox(label: impl Into<SharedString>) -> CheckboxControlElement {
    checkbox_control().label(label.into())
}

/// A check drawn as the bottom-right corner of a box turned 45°.
fn check_mark(color: impl Fn() -> AdaptiveColor + Copy + 'static) -> View {
    view()
        .w(px(5.0))
        .h(px(9.0))
        .mt(px(-2.0))
        .border_r(px(2.0))
        .border_b(px(2.0))
        .border_color(move || color())
        .rotate(deg(45.0))
}

/// A dash: the indeterminate mark.
fn dash_mark(color: impl Fn() -> AdaptiveColor + Copy + 'static) -> View {
    view().w(px(8.0)).h(px(2.0)).bg(move || color())
}
