//! [`OtpInput`], on `nsh_base`'s one-time-code field
//! ([`crate::components::otp_input`]).

use nsh_base::OtpInputCompleteEvent;
use nsh_core::prelude::*;

use super::Value;
use crate::components::input::state_classes::{CONTROL_BORDER, DISABLED, FOCUS_BORDER};
use crate::components::otp_input::otp_field;
use crate::types::Controlled;

/// The props of [`OtpInput`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct OtpInputProps {
    /// How many digits the code has: one cell each.
    pub length: Prop<u32>,
    /// The digits typed so far. Set, the app owns them and writes back
    /// what `onChange` reports; left out, the field keeps them.
    #[prop(default)]
    pub value: Prop<Controlled<SharedString>>,
    /// How many equal groups the cells split into, apart (default 1: one
    /// run; `2` splits six cells 3 + 3).
    #[prop(default = 1)]
    pub groups: Prop<u32>,
    /// Shows bullets in place of the digits (a PIN).
    #[prop(default)]
    pub masked: Prop<bool>,
    /// No typing, no focus; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Draws the cells' borders in the danger color (a wrong code).
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// What screen readers call the code (`"Verification code, 6
    /// digits"`).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of an [`OtpInput`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum OtpInputEvent {
    /// The user changed the code (typed, deleted, pasted, autofilled): the
    /// digits asked for.
    #[args(value)]
    Change(SharedString),
    /// The user's change filled the last cell, after its `onChange`: the
    /// code, to verify.
    #[args(value)]
    Complete(SharedString),
}

/// A numeric one-time code entered into square cells: `<OtpInput
/// length={6} onComplete={({ value }) => verify(value)} />`. Typing,
/// paste and autofill fill the cells; Backspace removes the last digit.
#[component]
pub fn OtpInput(props: OtpInputProps, cx: Scope<OtpInputEvent>) -> impl IntoElement {
    let label = super::spoken(props.accessibility_label);
    let value = Value::new(props.value, SharedString::default());
    otp_field(props.length.get_untracked())
        .length(move || props.length.get())
        .groups(move || props.groups.get())
        .masked(move || props.masked.get())
        .disabled(move || props.disabled.get())
        .accessibility_label(move || label())
        .cell_class(move || {
            let border = if props.invalid.get() {
                "border-danger"
            } else {
                CONTROL_BORDER
            };
            SharedString::from(format!("{border} {FOCUS_BORDER} {DISABLED}"))
        })
        .value(move || Some(value.get()))
        .on_change(move |code: &SharedString| {
            value.request(code.clone());
            cx.emit(OtpInputEvent::Change(code.clone()));
        })
        .on_complete(move |done: &OtpInputCompleteEvent| {
            cx.emit(OtpInputEvent::Complete(done.value.clone()))
        })
}
