//! [`Button`] and [`IconButton`], drawn by [`crate::components::button`]'s
//! `button_look!`.
//!
//! Both draw on a plain view rather than `nsh_base`'s `Button`: an app's
//! button placed in a menu's or popover's trigger slot must be one component
//! deep for the trigger's press to reach it (nsh-core 0.6).

use nsh_core::prelude::*;

use crate::components::button::{ButtonVariant, ControlSize, button_look};
use crate::components::icon::{IconName, icon};

/// A view that presses as a button: role, disabled, selected and busy
/// states.
pub(crate) fn pressable(disabled: impl Fn() -> bool + 'static, selected: Prop<bool>) -> View {
    view()
        .role(AccessibilityRole::Button)
        .disabled(disabled)
        .accessibility_state(move || AccessibilityState {
            selected: selected.get(),
            ..AccessibilityState::default()
        })
}

/// Gives `view` the app's `tooltip` slot (a [`Tooltip`](crate::Tooltip))
/// as its tooltip.
pub(crate) fn with_tooltip(view: View, tooltip: Option<Slot>) -> View {
    match tooltip {
        Some(tooltip) => view.tooltip(tooltip.element().into_element()),
        None => view,
    }
}

/// The glyph before a button's label: the loader, turning, while `loading`.
fn leading_icon(icon_name: Prop<Option<IconName>>, loading: Prop<bool>) -> impl IntoElement {
    dynamic(move || {
        if loading.get() {
            Some(icon(IconName::Loader).app_class(SharedString::from("animate-spin")).into_element())
        } else {
            icon_name.get().map(|name| icon(name).into_element())
        }
    })
}

/// The props of [`Button`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ButtonProps {
    /// The visible label, which screen readers say.
    pub label: Prop<SharedString>,
    /// `default` (a control-colored outline), `primary` (the accent
    /// outline), `ghost` (no outline) or `danger` (destructive).
    #[prop(default)]
    pub variant: Prop<ButtonVariant>,
    /// `small` (24 points tall), `medium` (28, the default) or `large` (32).
    #[prop(default)]
    pub size: Prop<ControlSize>,
    /// A glyph before the label, by its Lucide name (`"plus"`).
    pub icon: Prop<Option<IconName>>,
    /// No presses and no hover, focus or press feedback; dimmed text.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Keeps the selected fill: the current choice among buttons.
    #[prop(default)]
    pub selected: Prop<bool>,
    /// Working: the loader turns in place of the icon and presses are
    /// ignored.
    #[prop(default)]
    pub loading: Prop<bool>,
    /// What screen readers say in place of the label.
    pub accessibility_label: Prop<Option<SharedString>>,
    /// A hint shown while the pointer rests on it or the keyboard focuses
    /// it: a [`Tooltip`](crate::Tooltip) (`<Tooltip label="Save" shortcut="mod-s" />`).
    pub tooltip: Prop<Option<Slot>>,
    /// Content after the label: a badge, a shortcut.
    pub children: Prop<Children>,
}

/// The events of a [`Button`] and an [`IconButton`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ButtonEvent {
    /// It was pressed: a click, a tap, Return or Space.
    Press,
}

/// A labelled action: `<Button label="Save" variant="primary" onPress={save} />`.
#[component]
pub fn Button(props: ButtonProps, cx: Scope<ButtonEvent>) -> impl IntoElement {
    let label = props.label;
    let disabled = move || props.disabled.get() || props.loading.get();
    pressable(disabled, props.selected)
        .map(|v| button_look!(v, move || props.variant.get(), move || props.size.get()))
        .label(move || props.accessibility_label.get().unwrap_or_else(|| label.get()))
        .on_press(move || cx.emit(ButtonEvent::Press))
        .map(|v| with_tooltip(v, props.tooltip.get()))
        .child(leading_icon(props.icon, props.loading))
        .child(text(move || label.get()))
        .child(props.children)
}

/// The props of [`IconButton`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct IconButtonProps {
    /// The glyph, by its Lucide name (`"plus"`).
    pub icon: Prop<IconName>,
    /// The action, which screen readers say (`"Add"`); tests find it by it.
    pub label: Prop<SharedString>,
    /// `ghost` (the default), `default`, `primary` or `danger`.
    #[prop(default = ButtonVariant::Ghost)]
    pub variant: Prop<ButtonVariant>,
    /// `small` (24 points square), `medium` (28, the default) or `large` (32).
    #[prop(default)]
    pub size: Prop<ControlSize>,
    /// No presses, no feedback.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Keeps the selected fill.
    #[prop(default)]
    pub selected: Prop<bool>,
    /// Working: the loader turns in place of the glyph.
    #[prop(default)]
    pub loading: Prop<bool>,
    /// A hint shown while the pointer rests on it or the keyboard focuses
    /// it: a [`Tooltip`](crate::Tooltip).
    pub tooltip: Prop<Option<Slot>>,
}

/// A square, icon-only action: `<IconButton icon="plus" label="Add" />`.
#[component]
pub fn IconButton(props: IconButtonProps, cx: Scope<ButtonEvent>) -> impl IntoElement {
    let size = move || props.size.get();
    let disabled = move || props.disabled.get() || props.loading.get();
    let glyph = props.icon;
    pressable(disabled, props.selected)
        .map(|v| button_look!(v, move || props.variant.get(), size))
        .w(move || px(size().height()))
        .px(px(0.0))
        .label(move || props.label.get())
        .on_press(move || cx.emit(ButtonEvent::Press))
        .map(|v| with_tooltip(v, props.tooltip.get()))
        .child(leading_icon(
            Prop::bind(move || Some(glyph.get())),
            props.loading,
        ))
}

crate::button_variants!(ButtonElement, IconButtonElement);
crate::sizable!(ButtonElement, IconButtonElement);
