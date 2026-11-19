//! Omarchy switch: a square track whose square thumb slides from the start
//! (off) to the end (on), and its label, on `nsh_base`'s switch.
//!
//! States: hover and focus-visible wash the row and outline it in the
//! focus border; a press darkens the wash. Turning on fills the track with
//! the selected fill, drops its border and brightens the thumb; the thumb
//! travels over `DURATION_BASE` (ease-out) and the colors fade over
//! `DURATION_FAST`. With reduced motion NSH applies both at once. Disabled
//! dims the text, track and thumb (the thumb keeps its side).

use nsh_base::switch as base_switch;
use nsh_core::prelude::*;

use crate::tokens;

/// The track's outer size.
const TRACK_W: f32 = 42.0;
const TRACK_H: f32 = 22.0;
/// The track's padding around the thumb.
const TRACK_PAD: f32 = 2.0;
/// The thumb's side.
const THUMB: f32 = 16.0;
/// How far the thumb travels: the track less its borders, padding and thumb.
const TRAVEL: f32 = TRACK_W - 2.0 * tokens::BORDER - 2.0 * TRACK_PAD - THUMB;

/// The props of [`SwitchControl`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SwitchControlProps {
    /// The label after the track, also what screen readers say.
    #[prop(default)]
    pub label: Prop<SharedString>,
    /// On or off. The owner keeps it and writes the value `on_change`
    /// reports.
    #[prop(default)]
    pub checked: Prop<bool>,
    /// No presses, dimmed, no interaction styles.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers say when there is no label.
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`SwitchControl`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SwitchControlEvent {
    /// A press (click, tap, Space or Return) asks for this value.
    #[args(checked)]
    Change(bool),
}

/// An Omarchy switch. Prefer [`switch`], which sets the label.
#[component]
pub fn SwitchControl(props: SwitchControlProps, cx: Scope<SwitchControlEvent>) -> impl IntoElement {
    let SwitchControlProps {
        label,
        checked,
        disabled,
        accessibility_label,
    } = props;
    base_switch()
        .checked(checked)
        .disabled(disabled)
        .accessibility_label(move || {
            accessibility_label
                .get()
                .or_else(|| Some(label.get()).filter(|l| !l.is_empty()))
        })
        .on_change(move |next: &bool| cx.emit(SwitchControlEvent::Change(*next)))
        .flex_row()
        // Hug the track and its label instead of stretching across a column.
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
        .track_style(move |s| {
            s.w(px(TRACK_W))
                .h(px(TRACK_H))
                .flex_shrink_0()
                .justify_center()
                .p(px(TRACK_PAD))
                .border_1()
                .border_color(move || {
                    if disabled.get() {
                        tokens::divider()
                    } else if checked.get() {
                        AdaptiveColor::TRANSPARENT
                    } else {
                        tokens::control_border()
                    }
                })
                .bg(move || {
                    if checked.get() && !disabled.get() {
                        tokens::selected_fill()
                    } else {
                        tokens::normal_fill()
                    }
                })
                .transition_colors()
                .duration(tokens::DURATION_FAST)
        })
        .thumb_style(move |s| {
            s.w(px(THUMB))
                .h(px(THUMB))
                .bg(move || {
                    if disabled.get() {
                        tokens::disabled()
                    } else if checked.get() {
                        tokens::bright()
                    } else {
                        tokens::secondary()
                    }
                })
                .translate_x(move || px(if checked.get() { TRAVEL } else { 0.0 }))
                // Colors and the travel animate together, at the thumb's pace.
                .transition()
                .duration(tokens::DURATION_BASE)
                .ease_out()
        })
        .child(when(move || !label.get().is_empty(), move || text(label)))
}

/// A switch labelled `label`: set `.checked(..)`, `.disabled(..)` and write
/// the value `.on_change(..)` reports.
///
/// ```ignore
/// switch("Wi-Fi")
///     .checked(wifi)
///     .on_change(move |next: &bool| wifi.set(*next))
/// ```
pub fn switch(label: impl Into<SharedString>) -> SwitchControlElement {
    switch_control().label(label.into())
}
