//! The fields: [`Input`], [`Textarea`], [`NumberInput`], [`OtpInput`],
//! [`Select`], [`Combobox`], [`Calendar`], [`DatePicker`], [`ColorPicker`]
//! and [`Field`], drawn by [`crate::components`]' `input`, `otp_input`,
//! `select`, `calendar`, `date_picker` and `color_picker`.
//!
//! A field is a resting-fill well with the control border; while it edits
//! (or its list is open) the border turns to the accent and the well takes
//! the hover wash. Each value is controlled when the app sets it and kept
//! by the field when left out ([`Controlled`]); none rewrites what is
//! typed, so an input method's composition is left alone.

use nsh_core::prelude::*;

use crate::types::Controlled;
use crate::tokens;

mod color;
mod date;
mod field;
mod otp;
mod select;
mod text;

pub use color::{ColorPicker, ColorPickerElement};
pub use date::{Calendar, CalendarElement, DatePicker, DatePickerElement};
pub use field::{Field, FieldElement};
pub use otp::{OtpInput, OtpInputElement};
pub use select::{Combobox, ComboboxElement, Select, SelectElement};
pub use text::{Input, InputElement, NumberInput, NumberInputElement, Textarea, TextareaElement};

pub(crate) use color::color_picker;
pub(crate) use date::{calendar, date_picker};
pub(crate) use field::field;
pub(crate) use otp::otp_input;
pub(crate) use select::{combobox, select};
pub(crate) use text::{input, number_input, textarea};

crate::sizable!(InputElement, SelectElement, DatePickerElement);

/// The border of a field: the danger color while `invalid`, else the
/// control border.
fn rest_border(invalid: bool) -> AdaptiveColor {
    if invalid {
        tokens::danger()
    } else {
        tokens::control_border()
    }
}

/// The label of the [`Field`] around a control, provided to what it holds.
#[derive(Clone, Copy)]
struct FieldLabel(Prop<SharedString>);

/// What screen readers call a control: its `accessibility_label`, else the
/// label of the [`Field`] around it. Call it in the control's body.
fn spoken(own: Prop<Option<SharedString>>) -> impl Fn() -> Option<SharedString> + Copy + 'static {
    let field = use_context::<FieldLabel>();
    move || own.get().or_else(|| field.map(|FieldLabel(label)| label.get()))
}

/// A field's value: the app's while it sets it, else the field's own.
struct Value<T: 'static> {
    prop: Prop<Controlled<T>>,
    own: Signal<T>,
}

impl<T: 'static> Clone for Value<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Copy for Value<T> {}

impl<T: Clone + PartialEq + 'static> Value<T> {
    /// The value of `prop`, the field's own starting at `initial`.
    fn new(prop: Prop<Controlled<T>>, initial: T) -> Self {
        Self {
            prop,
            own: signal(initial),
        }
    }

    /// The value to draw (tracked).
    fn get(self) -> T {
        self.prop.with(|p| p.or_else(|| self.own.get()))
    }

    /// The user asked for `next`: the field keeps it while the app does
    /// not own the value (the app hears it from the event either way).
    fn request(self, next: T) {
        if self.prop.with_untracked(Controlled::is_uncontrolled) {
            self.own.set(next);
        }
    }
}

/// Registers this area's components for TSX.
pub(crate) fn register(registry: &mut Registry) {
    registry
        .register::<Input>()
        .register::<Textarea>()
        .register::<NumberInput>()
        .register::<OtpInput>()
        .register::<Select>()
        .register::<Combobox>()
        .register::<Calendar>()
        .register::<DatePicker>()
        .register::<ColorPicker>()
        .register::<Field>();
}
