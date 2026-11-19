//! [`Input`], [`Textarea`] and [`NumberInput`], on `nsh_base`'s native
//! text field ([`crate::components::input`]).

use nsh_core::prelude::*;

use super::{Value, rest_border};
use crate::components::button::ControlSize;
use crate::components::input::{number_field, text_input, textarea as textarea_field};
use crate::fields::Controlled;
use crate::tokens;

/// The methods of an [`Input`]'s or a [`Textarea`]'s ref.
pub struct InputApi {
    field: ElementRef,
}

#[methods]
impl InputApi {
    /// Takes the keyboard focus.
    pub fn focus(&self) {
        self.field.focus();
    }

    /// Gives up the keyboard focus.
    pub fn blur(&self) {
        self.field.blur();
    }

    /// Selects `start..end` (UTF-16 units; a caret at `start` without
    /// `end`).
    pub fn set_selection(&self, start: u32, end: Option<u32>) {
        self.field.set_selection(start, end.unwrap_or(start));
    }

    /// Inserts `text` in place of the selection as the user's edit:
    /// `onChange` reports it.
    pub fn insert_text(&self, text: String) {
        self.field.insert_text(text);
    }
}

/// The props of [`Input`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct InputProps {
    /// The text. Set, the app owns it and writes back what `onChange`
    /// reports; left out, the field keeps what is typed.
    #[prop(default)]
    pub value: Prop<Controlled<String>>,
    /// Shown while empty.
    pub placeholder: Prop<Option<SharedString>>,
    /// `small` (24 points tall), `medium` (28, the default) or `large` (32).
    #[prop(default)]
    pub size: Prop<ControlSize>,
    /// No typing, no focus; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Selectable, not editable.
    #[prop(default)]
    pub readonly: Prop<bool>,
    /// Draws the danger border.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// Hides what is typed (a password).
    #[prop(default)]
    pub masked: Prop<bool>,
    /// Takes the keyboard focus when it appears.
    #[prop(default)]
    pub auto_focus: Prop<bool>,
    /// What screen readers call the field (default: the label of the
    /// `Field` around it).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of an [`Input`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum InputEvent {
    /// The user changed the text: the text asked for.
    #[args(value)]
    Change(String),
    /// Return: the text.
    #[args(value)]
    Submit(String),
}

/// A one-line text field: `<Input value={name()} onChange={({ value }) =>
/// name.set(value)} />`. Its ref has `focus()`, `blur()`,
/// `setSelection(start, end?)` and `insertText(text)`.
#[component(methods = InputApi)]
pub fn Input(props: InputProps, cx: Scope<InputEvent>) -> impl IntoElement {
    let field = element_ref();
    cx.methods(InputApi {
        field: field.clone(),
    });
    let label = super::spoken(props.accessibility_label);
    let value = Value::new(props.value, String::new());
    let size = move || props.size.get();
    text_input()
        .element_ref(&field)
        .h(move || px(size().height()))
        .text_size(move || px(size().text_size()))
        .border_color(move || rest_border(props.invalid.get()))
        .value(move || value.get())
        .placeholder(move || {
            props
                .placeholder
                .get()
                .map(|p| p.to_string())
                .unwrap_or_default()
        })
        .masked(move || props.masked.get())
        .disabled(move || props.disabled.get())
        .readonly(move || props.readonly.get())
        .auto_focus(move || props.auto_focus.get())
        .optional_label(move || label())
        .on_change(move |text: &str| {
            value.request(text.to_owned());
            cx.emit(InputEvent::Change(text.to_owned()));
        })
        .on_submit(move |text: &str| cx.emit(InputEvent::Submit(text.to_owned())))
}

/// The props of [`Textarea`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TextareaProps {
    /// The text. Set, the app owns it and writes back what `onChange`
    /// reports; left out, the field keeps what is typed.
    #[prop(default)]
    pub value: Prop<Controlled<String>>,
    /// Shown while empty.
    pub placeholder: Prop<Option<SharedString>>,
    /// Visible lines before it grows (default 4).
    #[prop(default = 4)]
    pub rows: Prop<u32>,
    /// No typing, no focus; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Selectable, not editable.
    #[prop(default)]
    pub readonly: Prop<bool>,
    /// Draws the danger border.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// What screen readers call the field (default: the label of the
    /// `Field` around it).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`Textarea`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum TextareaEvent {
    /// The user changed the text: the text asked for.
    #[args(value)]
    Change(String),
}

/// A text field of several lines; Return types a new line. Its ref is an
/// `Input`'s.
#[component(methods = InputApi)]
pub fn Textarea(props: TextareaProps, cx: Scope<TextareaEvent>) -> impl IntoElement {
    let field = element_ref();
    cx.methods(InputApi {
        field: field.clone(),
    });
    let label = super::spoken(props.accessibility_label);
    let value = Value::new(props.value, String::new());
    textarea_field()
        .element_ref(&field)
        .min_h(move || px(8.0 + 16.0 * props.rows.get() as f32))
        .border_color(move || rest_border(props.invalid.get()))
        .value(move || value.get())
        .placeholder(move || {
            props
                .placeholder
                .get()
                .map(|p| p.to_string())
                .unwrap_or_default()
        })
        .disabled(move || props.disabled.get())
        .readonly(move || props.readonly.get())
        .optional_label(move || label())
        .on_change(move |text: &str| {
            value.request(text.to_owned());
            cx.emit(TextareaEvent::Change(text.to_owned()));
        })
}

/// The props of [`NumberInput`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct NumberInputProps {
    /// The number, `null` for an empty field. Set, the app owns it; left
    /// out, the field keeps its own.
    #[prop(default)]
    pub value: Prop<Controlled<Option<f64>>>,
    /// The least it takes.
    pub min: Prop<Option<f64>>,
    /// The most it takes.
    pub max: Prop<Option<f64>>,
    /// Buttons, Up / Down (default 1); Page Up / Page Down go ten.
    #[prop(default = 1.0)]
    pub step: Prop<f64>,
    /// A unit after the number (`"px"`, `"%"`).
    pub suffix: Prop<Option<SharedString>>,
    /// Shown while empty.
    pub placeholder: Prop<Option<SharedString>>,
    /// No typing, no stepping; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Selectable, not editable.
    #[prop(default)]
    pub readonly: Prop<bool>,
    /// Draws the danger border.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// What screen readers call the field (default: the label of the
    /// `Field` around it).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`NumberInput`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum NumberInputEvent {
    /// The user changed the number (`null`: emptied the field): the number
    /// asked for.
    #[args(value)]
    Change(Option<f64>),
}

/// A number with − / + steppers on either side, 144 points wide. Typing is
/// held to `min` / `max` when the field is left or Return is pressed.
#[component]
pub fn NumberInput(props: NumberInputProps, cx: Scope<NumberInputEvent>) -> impl IntoElement {
    let label = super::spoken(props.accessibility_label);
    let value = Value::new(props.value, None);
    let suffix = props.suffix;
    let field = number_field()
        .self_start()
        .w(px(144.0))
        .border_color(move || rest_border(props.invalid.get()))
        .value(move || value.get())
        .min(move || props.min.get())
        .max(move || props.max.get())
        .step(move || props.step.get())
        .disabled(move || props.disabled.get())
        .readonly(move || props.readonly.get())
        .placeholder(move || props.placeholder.get())
        .accessibility_label(move || label())
        .on_change(move |next: &Option<f64>| {
            value.request(*next);
            cx.emit(NumberInputEvent::Change(*next));
        });
    // The unit's slot is made once: a field with no unit has none.
    if suffix.get_untracked().is_none() {
        return field;
    }
    field.suffix(
        text(move || suffix.get().unwrap_or_default())
            .self_center()
            .pr(px(tokens::SPACE_1))
            .text_size(px(tokens::TEXT_SM))
            .text_color(move || tokens::secondary()),
    )
}
