//! `<Button>` and `<IconButton>` for TSX, drawn by
//! [`crate::components::button`].

use nsh_core::prelude::*;

use crate::components::button::{ButtonSize, ButtonVariant, button_look};


/// A view that presses as a button: role, disabled and selected states.
/// The TSX buttons are views of their own rather than `nsh_base`'s `Button`:
/// a menu's or popover's trigger slot reaches a component one level deep.
fn pressable(disabled: Prop<bool>, selected: Prop<bool>) -> View {
    view()
        .role(AccessibilityRole::Button)
        .disabled(move || disabled.get())
        .accessibility_state(move || AccessibilityState {
            selected: selected.get(),
            ..AccessibilityState::default()
        })
}

/// Gives `view` the app's `tooltip` slot (a `<Tooltip>`) as its tooltip.
fn with_tooltip(view: View, tooltip: Option<Slot>) -> View {
    match tooltip {
        Some(tooltip) => view.tooltip(tooltip.element().into_element()),
        None => view,
    }
}

/// The props of [`Button`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ButtonProps {
    /// The visible label.
    pub label: Prop<Option<SharedString>>,
    /// Default: a control-colored outline. Primary: the accent outline.
    /// Ghost: no outline. Danger: destructive.
    #[prop(default)]
    pub variant: Prop<ButtonVariant>,
    /// 24, 28 (default) or 32 points tall.
    #[prop(default)]
    pub size: Prop<ButtonSize>,
    /// No presses, no hover, focus or press feedback; dimmed text.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Keeps the selected fill: the current choice among buttons.
    #[prop(default)]
    pub selected: Prop<bool>,
    /// What screen readers say when the visible content does not (an icon).
    pub accessibility_label: Prop<Option<SharedString>>,
    /// A glyph before the label (`<Icon name="plus" />`).
    pub icon: Prop<Option<Slot>>,
    /// Content after the label (a keycap, a badge), or the whole content
    /// without `label`.
    pub children: Prop<Children>,
    /// A hint shown while the pointer rests on it or the keyboard focuses
    /// it: `<Tooltip label="Save" shortcut="mod-s" />`.
    pub tooltip: Prop<Option<Slot>>,
}

/// The events of a [`Button`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ButtonEvent {
    /// It was pressed (a click, a tap, Return or Space).
    Press,
}

/// A labelled action: `<Button label="Save" variant="primary" onPress={save} />`.
#[component]
pub fn Button(props: ButtonProps, cx: Scope<ButtonEvent>) -> impl IntoElement {
    let label = props.label;
    pressable(props.disabled, props.selected)
        .map(|v| button_look!(v, move || props.variant.get(), move || props.size.get()))
        .optional_label(move || props.accessibility_label.get())
        .on_press(move || cx.emit(ButtonEvent::Press))
        .map(|v| with_tooltip(v, props.tooltip.get()))
        .slot(props.icon.get(), |s| s)
        .child(dynamic(move || {
            label.get().map(|label| text(label).into_element())
        }))
        .child(props.children)
}

/// The props of [`IconButton`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct IconButtonProps {
    /// The action, for screen readers and tests (`label:Add`).
    pub label: Prop<SharedString>,
    /// The glyph (`<Icon name="plus" />`).
    pub children: Prop<Children>,
    /// Ghost (default), Default, Primary or Danger.
    #[prop(default = ButtonVariant::Ghost)]
    pub variant: Prop<ButtonVariant>,
    /// 24, 28 (default) or 32 points square.
    #[prop(default)]
    pub size: Prop<ButtonSize>,
    /// No presses, no feedback.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Keeps the selected fill.
    #[prop(default)]
    pub selected: Prop<bool>,
    /// A hint shown while the pointer rests on it or the keyboard focuses
    /// it: `<Tooltip label="Add" shortcut="mod-n" />`.
    pub tooltip: Prop<Option<Slot>>,
}

/// A square, icon-only action: `<IconButton label="Add"><Icon name="plus" /></IconButton>`.
#[component]
pub fn IconButton(props: IconButtonProps, cx: Scope<ButtonEvent>) -> impl IntoElement {
    let size = move || props.size.get();
    pressable(props.disabled, props.selected)
        .map(|v| button_look!(v, move || props.variant.get(), size))
        .w(move || px(size().height()))
        .px(px(0.0))
        .label(move || props.label.get())
        .on_press(move || cx.emit(ButtonEvent::Press))
        .map(|v| with_tooltip(v, props.tooltip.get()))
        .child(props.children)
}
