//! The toggles and the groups of actions: `Toggle`, `ToggleGroup`,
//! `ButtonGroup`, drawn with [`crate::components`] and the Omarchy tokens.

use nsh_base::{toggle as base_toggle, toggle_group as base_toggle_group};
use nsh_core::{Checked, prelude::*};

use crate::components::button::{ControlSize, actions_duration};
use crate::components::toggle::toggle_look;
use crate::tokens;

/// The props of [`Toggle`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ToggleProps {
    /// On or off (controlled): write `onChange`'s value back to it.
    #[prop(default)]
    pub pressed: Prop<bool>,
    /// The visible label.
    pub label: Prop<Option<SharedString>>,
    /// A glyph before the label (`<Icon name="bold" />`).
    pub icon: Prop<Option<Slot>>,
    /// Content after the label, or the whole content without `label`.
    pub children: Prop<Children>,
    /// 24, 28 (default) or 32 points tall.
    #[prop(default)]
    pub size: Prop<ControlSize>,
    /// No presses, no hover, focus or press feedback; dimmed text.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// What screen readers say when the visible content does not (an icon).
    pub accessibility_label: Prop<Option<SharedString>>,
}

/// The events of a [`Toggle`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ToggleEvent {
    /// It was pressed: the value asked for, the opposite of `pressed`.
    #[args(pressed)]
    Change(bool),
}

/// A button that is on or off:
/// `<Toggle label="Bold" pressed={bold()} onChange={({ pressed }) => bold.set(pressed)} />`.
#[component]
pub fn Toggle(props: ToggleProps, cx: Scope<ToggleEvent>) -> impl IntoElement {
    let pressed = props.pressed;
    let label = props.label;
    let size = props.size;
    toggle_look!(view(), move || size.get())
        .role(AccessibilityRole::Button)
        .disabled(move || props.disabled.get())
        .optional_label(move || props.accessibility_label.get())
        .accessibility_state(move || AccessibilityState {
            pressed: Some(if pressed.get() { Checked::True } else { Checked::False }),
            ..AccessibilityState::default()
        })
        .on_press(move || cx.emit(ToggleEvent::Change(!pressed.get_untracked())))
        .slot(props.icon.get(), |s| s)
        .child(dynamic(move || label.get().map(|label| text(label).into_element())))
        .child(props.children)
}

// ---- ToggleGroup and ButtonGroup ------------------------------------------

/// One option of a [`ToggleGroup`] or a [`ButtonGroup`].
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct GroupItem {
    /// The app's id of the option, never its label.
    pub value: SharedString,
    /// What the option shows and screen readers say.
    pub label: SharedString,
    /// Not choosable, and skipped by the arrow keys.
    #[prop(default)]
    pub disabled: bool,
}

/// The props of [`ToggleGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ToggleGroupProps {
    /// The options, one toggle each.
    pub items: Prop<Vec<GroupItem>>,
    /// The values that are on (controlled).
    #[prop(default)]
    pub value: Prop<Vec<SharedString>>,
    /// Several on at once (default); `false`: at most one.
    #[prop(default = true)]
    pub multiple: Prop<bool>,
    /// Every toggle disabled.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// 24, 28 (default) or 32 points tall.
    #[prop(default)]
    pub size: Prop<ControlSize>,
}

/// The events of a [`ToggleGroup`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ToggleGroupEvent {
    /// A toggle was pressed: the values as they would be. Write them back
    /// to `value`.
    #[args(value)]
    Change(Vec<SharedString>),
}

/// A row of toggles, one stop of Tab: the arrows move between them, Space
/// and Return turn the focused one on or off.
#[component]
pub fn ToggleGroup(props: ToggleGroupProps, cx: Scope<ToggleGroupEvent>) -> impl IntoElement {
    let value = props.value;
    let multiple = props.multiple;
    let size = props.size;
    let rows = each(
        move || props.items.get(),
        |item: &GroupItem| item.value.clone(),
        move |item, _index| {
            let id = move || item.get().value;
            base_toggle()
                .pressed(move || value.get().contains(&id()))
                .disabled(move || item.get().disabled)
                .on_change(move |on: &bool| {
                    let id = id();
                    let mut next: Vec<SharedString> = value
                        .get_untracked()
                        .into_iter()
                        .filter(|v| *v != id)
                        .collect();
                    if *on {
                        if !multiple.get_untracked() {
                            next.clear();
                        }
                        next.push(id);
                    }
                    cx.emit(ToggleGroupEvent::Change(next));
                })
                .map(|t| toggle_look!(t, move || size.get()))
                .child(text(move || item.get().label))
        },
    );
    view().self_start().child(
        base_toggle_group()
            .disabled(move || props.disabled.get())
            .flex_row()
            .self_start()
            .gap(px(tokens::SPACE_1))
            .child(rows),
    )
}

/// The props of [`ButtonGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ButtonGroupProps {
    /// The options, one segment each.
    pub items: Prop<Vec<GroupItem>>,
    /// The chosen option's value (controlled).
    pub value: Prop<Option<SharedString>>,
    /// Every segment disabled.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// 24, 28 (default) or 32 points tall.
    #[prop(default)]
    pub size: Prop<ControlSize>,
}

/// The events of a [`ButtonGroup`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ButtonGroupEvent {
    /// A segment other than the chosen one was chosen: its value. Write it
    /// back to `value`.
    #[args(value)]
    Change(SharedString),
}

/// The horizontal padding of a segment of `size`.
fn segment_padding(size: ControlSize) -> f32 {
    match size {
        ControlSize::Small => tokens::SPACE_3,
        ControlSize::Medium => tokens::SPACE_4,
        ControlSize::Large => tokens::SPACE_4 + tokens::SPACE_1,
    }
}

/// A setting with one value: equal segments in one square frame, one stop
/// of Tab. The arrows move the focus between segments without choosing;
/// Space and Return choose. The chosen segment keeps the selected fill and
/// bright text.
#[component]
pub fn ButtonGroup(props: ButtonGroupProps, cx: Scope<ButtonGroupEvent>) -> impl IntoElement {
    let value = props.value;
    let size = props.size;
    let rows = each(
        move || props.items.get(),
        |item: &GroupItem| item.value.clone(),
        move |item, index| {
            let id = move || item.get().value;
            base_toggle()
                .pressed(move || value.get().as_ref() == Some(&id()))
                .disabled(move || item.get().disabled)
                .on_change(move |on: &bool| {
                    // A press on the chosen segment asks to turn it off: a
                    // single choice keeps it.
                    if *on {
                        cx.emit(ButtonGroupEvent::Change(id()));
                    }
                })
                .min_w(px(64.0))
                .flex_shrink_0()
                .flex_row()
                .items_center()
                .justify_center()
                .h(move || px(size.get().height()))
                .px(move || px(segment_padding(size.get())))
                .text_size(move || px(size.get().text_size()))
                .rounded(px(tokens::RADIUS))
                .border(px(tokens::BORDER))
                .border_color(AdaptiveColor::TRANSPARENT)
                .border_l_color(move || {
                    if index.get() > 0 {
                        tokens::divider()
                    } else {
                        AdaptiveColor::TRANSPARENT
                    }
                })
                .bg(AdaptiveColor::TRANSPARENT)
                .text_color(move || tokens::foreground())
                .cursor_pointer()
                .transition_colors()
                .duration(move || actions_duration(tokens::DURATION_FAST))
                .hover(|s| s.bg(move || tokens::hover_fill()))
                .focus_visible(|s| {
                    s.bg(move || tokens::hover_fill())
                        .border_color(move || tokens::focus_border())
                })
                .active(|s| s.bg(move || tokens::pressed_fill()))
                .aria_pressed(|s| {
                    s.bg(move || tokens::selected_fill())
                        .text_color(move || tokens::bright())
                })
                .disabled_style(|s| {
                    s.bg(AdaptiveColor::TRANSPARENT)
                        .text_color(move || tokens::disabled())
                        .cursor(Cursor::Default)
                })
                .child(text(move || item.get().label))
        },
    );
    view().self_start().child(
        base_toggle_group()
            .disabled(move || props.disabled.get())
            .flex_row()
            .self_start()
            .border(px(tokens::BORDER))
            .border_color(move || tokens::border())
            .child(rows),
    )
}

/// Registers this area's components for TSX.
pub(crate) fn register(registry: &mut Registry) {
    registry
        .register::<super::button::Button>()
        .register::<super::button::IconButton>()
        .register::<Toggle>()
        .register::<ToggleGroup>()
        .register::<ButtonGroup>();
}
