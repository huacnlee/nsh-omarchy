//! `<Input>`, `<Textarea>`, `<NumberInput>`, `<Select>`, `<Combobox>` and
//! `<Field>` for TSX, drawn by [`crate::components::input`] and
//! [`crate::components::select`].
//!
//! A field is a resting-fill well with the control border; while it edits
//! (or its list is open) the border turns to the accent and the well takes
//! the hover wash. All are controlled (`value` plus `onChange`), and none
//! rewrites what is typed, so an input method's composition is left alone.

use nsh_core::prelude::*;

use crate::components::button::ButtonSize;
use crate::components::input::{number_field, text_input, textarea as textarea_field};
use crate::components::select::{combobox_field, select_popup, select_trigger};
use crate::tokens;

pub use nsh_base::SelectItem;

/// The border of a field: the danger color while `invalid`, else the
/// control border.
fn rest_border(invalid: bool) -> AdaptiveColor {
    if invalid {
        tokens::danger()
    } else {
        tokens::control_border()
    }
}

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
    /// The text (controlled): write `onChange`'s value back to it.
    pub value: Prop<SharedString>,
    /// Shown while empty.
    pub placeholder: Prop<Option<SharedString>>,
    /// What screen readers call the field when no visible label names it.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Selectable, not editable.
    #[prop(default)]
    pub readonly: Prop<bool>,
    /// Draws the danger border.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// Hides what is typed (a password).
    #[prop(default)]
    pub masked: Prop<bool>,
    /// 24, 28 (default) or 32 points tall.
    #[prop(default)]
    pub size: Prop<ButtonSize>,
    /// Takes the keyboard focus when it appears.
    #[prop(default)]
    pub auto_focus: Prop<bool>,
}

/// The events of an [`Input`] and a [`Textarea`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum InputEvent {
    /// The user changed the text: write it back to `value`.
    #[args(value)]
    Change(SharedString),
    /// Return in a one-line field: its text.
    #[args(text)]
    Submit(SharedString),
}

/// A one-line text field. Its ref (`ref={(field) => ..}`) has `focus()`,
/// `blur()`, `setSelection()` and `insertText()`.
#[component(methods = InputApi)]
pub fn Input(props: InputProps, cx: Scope<InputEvent>) -> impl IntoElement {
    let field = element_ref();
    cx.methods(InputApi {
        field: field.clone(),
    });
    let size = move || props.size.get();
    text_input()
        .element_ref(&field)
        .h(move || px(size().height()))
        .text_size(move || px(size().text_size()))
        .border_color(move || rest_border(props.invalid.get()))
        .value(move || props.value.get().to_string())
        .placeholder(move || props.placeholder.get().map(|p| p.to_string()).unwrap_or_default())
        .masked(move || props.masked.get())
        .readonly(move || props.readonly.get())
        .auto_focus(move || props.auto_focus.get())
        .optional_label(move || props.accessibility_label.get())
        .on_change(move |text: &str| cx.emit(InputEvent::Change(text.to_owned().into())))
        .on_submit(move |text: &str| cx.emit(InputEvent::Submit(text.to_owned().into())))
}

/// The props of [`Textarea`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TextareaProps {
    /// The text (controlled): write `onChange`'s value back to it.
    pub value: Prop<SharedString>,
    /// Shown while empty.
    pub placeholder: Prop<Option<SharedString>>,
    /// What screen readers call the field when no visible label names it.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// Selectable, not editable.
    #[prop(default)]
    pub readonly: Prop<bool>,
    /// Draws the danger border.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// Visible lines before it grows (default 4).
    #[prop(default = 4)]
    pub rows: Prop<u32>,
}

/// A text field of several lines; Return types a new line. Its ref is an
/// `Input`'s.
#[component(methods = InputApi)]
pub fn Textarea(props: TextareaProps, cx: Scope<InputEvent>) -> impl IntoElement {
    let field = element_ref();
    cx.methods(InputApi {
        field: field.clone(),
    });
    textarea_field()
        .element_ref(&field)
        .min_h(move || px(8.0 + 16.0 * props.rows.get() as f32))
        .border_color(move || rest_border(props.invalid.get()))
        .value(move || props.value.get().to_string())
        .placeholder(move || props.placeholder.get().map(|p| p.to_string()).unwrap_or_default())
        .readonly(move || props.readonly.get())
        .optional_label(move || props.accessibility_label.get())
        .on_change(move |text: &str| cx.emit(InputEvent::Change(text.to_owned().into())))
}

/// The props of [`NumberInput`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct NumberInputProps {
    /// The number, or `null` for an empty field (controlled).
    pub value: Prop<Option<f64>>,
    /// The least it takes.
    pub min: Prop<Option<f64>>,
    /// The most it takes.
    pub max: Prop<Option<f64>>,
    /// Buttons, Up / Down (default 1); Page Up / Page Down go ten.
    #[prop(default = 1.0)]
    pub step: Prop<f64>,
    /// No typing, no stepping; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Selectable, not editable.
    #[prop(default)]
    pub readonly: Prop<bool>,
    /// Draws the danger border.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// Shown while empty.
    pub placeholder: Prop<Option<SharedString>>,
    /// What screen readers call the field when no visible label names it.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// A unit after the number (`"px"`, `"%"`).
    pub suffix: Prop<Option<SharedString>>,
}

/// The events of a [`NumberInput`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum NumberInputEvent {
    /// The user changed the number (`null`: emptied the field): write it
    /// back to `value`.
    #[args(value)]
    Change(Option<f64>),
}

/// A number with − / + steppers on either side, 144 points wide.
#[component]
pub fn NumberInput(props: NumberInputProps, cx: Scope<NumberInputEvent>) -> impl IntoElement {
    let mut field = number_field()
        .self_start()
        .w(px(144.0))
        .border_color(move || rest_border(props.invalid.get()))
        .value(move || props.value.get())
        .min(move || props.min.get())
        .max(move || props.max.get())
        .step(move || props.step.get())
        .disabled(move || props.disabled.get())
        .readonly(move || props.readonly.get())
        .placeholder(move || props.placeholder.get())
        .accessibility_label(move || props.accessibility_label.get())
        .on_change(move |value: &Option<f64>| cx.emit(NumberInputEvent::Change(*value)));
    if let Some(unit) = props.suffix.get_untracked() {
        field = field.suffix(
            text(unit)
                .self_center()
                .pr(px(tokens::SPACE_1))
                .text_size(px(tokens::TEXT_SM))
                .text_color(move || tokens::secondary()),
        );
    }
    field
}

/// The props of [`Select`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SelectProps {
    /// The items, in order (`group` sections them).
    pub items: Prop<Vec<SelectItem>>,
    /// The chosen item's value, `null` for none (controlled).
    pub value: Prop<Option<SharedString>>,
    /// What the trigger says while nothing is chosen.
    pub placeholder: Prop<Option<SharedString>>,
    /// What the choice is about, for screen readers.
    pub accessibility_label: Prop<SharedString>,
    /// A search field above the items.
    #[prop(default)]
    pub searchable: Prop<bool>,
    /// It does not open; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Draws the danger border.
    #[prop(default)]
    pub invalid: Prop<bool>,
    /// 24, 28 (default) or 32 points tall.
    #[prop(default)]
    pub size: Prop<ButtonSize>,
}

/// The events of a [`Select`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum SelectEvent {
    /// The value asked for: write it back to `value`.
    #[args(value)]
    Change(Option<SharedString>),
}

/// A trigger that opens a list: Up / Down / Home / End and typeahead move,
/// Return or Space choose, Escape and a press outside close, the keyboard
/// goes back to the trigger.
#[component]
pub fn Select(props: SelectProps, cx: Scope<SelectEvent>) -> impl IntoElement {
    let shown = computed(move || {
        let chosen = props.value.get()?;
        props.items.with(|items| {
            items
                .iter()
                .find(|item| item.value == chosen)
                .map(|item| item.title.clone())
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
        .items(move || props.items.get())
        .value(move || props.value.get())
        .placeholder(move || props.placeholder.get())
        .searchable(move || props.searchable.get())
        .disabled(move || props.disabled.get())
        .accessibility_label(move || Some(props.accessibility_label.get()))
        .on_change(move |value: &Option<SharedString>| {
            cx.emit(SelectEvent::Change(value.clone()))
        })
}

/// The props of [`Combobox`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ComboboxProps {
    /// The items, in order (`group` sections them).
    pub items: Prop<Vec<SelectItem>>,
    /// The selected values (controlled).
    pub value: Prop<Vec<SharedString>>,
    /// Several values: a choice toggles one and the list stays open.
    #[prop(default)]
    pub multiple: Prop<bool>,
    /// Shown in the empty field.
    pub placeholder: Prop<Option<SharedString>>,
    /// What the choice is about, for screen readers.
    pub accessibility_label: Prop<SharedString>,
    /// No typing, no list; dimmed.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Finds the text field in tests.
    #[prop(rename = "inputTestID")]
    pub input_test_id: Prop<Option<SharedString>>,
}

/// The events of a [`Combobox`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ComboboxEvent {
    /// The selected values asked for: write them back to `value`.
    #[args(value)]
    Change(Vec<SharedString>),
}

/// A field whose typing filters the list.
#[component]
pub fn Combobox(props: ComboboxProps, cx: Scope<ComboboxEvent>) -> impl IntoElement {
    combobox_field(Vec::new())
        .self_start()
        .min_w(px(224.0))
        .items(move || props.items.get())
        .value(move || Some(props.value.get()))
        .multiple(move || props.multiple.get())
        .placeholder(move || props.placeholder.get())
        .disabled(move || props.disabled.get())
        .accessibility_label(move || Some(props.accessibility_label.get()))
        .input_test_id(move || props.input_test_id.get())
        .on_change(move |value: &Vec<SharedString>| {
            cx.emit(ComboboxEvent::Change(value.clone()))
        })
}

/// The props of [`Field`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldProps {
    /// The label above the field.
    pub label: Prop<SharedString>,
    /// A hint below the field (while there is no error).
    pub hint: Prop<Option<SharedString>>,
    /// An error below the field, in the danger color.
    pub error: Prop<Option<SharedString>>,
    /// The field.
    pub children: Prop<Children>,
}

/// A field's label above it and an optional hint or error below.
#[component]
pub fn Field(props: FieldProps) -> impl IntoElement {
    let note = move || props.error.get().or_else(|| props.hint.get());
    view()
        .gap(px(tokens::SPACE_1))
        .child(
            text(move || props.label.get())
                .text_size(px(tokens::TEXT_SM))
                .text_color(move || tokens::secondary()),
        )
        .child(props.children)
        .child(dynamic(move || {
            note().map(|words| {
                text(words)
                    .text_size(px(tokens::TEXT_SM))
                    .text_color(move || {
                        if props.error.with(Option::is_some) {
                            tokens::danger()
                        } else {
                            tokens::secondary()
                        }
                    })
                    .into_element()
            })
        }))
}
