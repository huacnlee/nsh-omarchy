//! The toggles, the groups of actions and the link: [`Toggle`],
//! [`ToggleGroup`], [`ButtonGroup`] and [`Link`], drawn with
//! [`crate::components`] and the Omarchy tokens.

use std::rc::Rc;

use nsh_base::{
    LinkOpenWith, toggle as base_toggle, toggle_group as base_toggle_group,
};
use nsh_core::{Checked, prelude::*};

use super::button::ButtonGroupContext;
use crate::components::button::ControlSize;
use crate::components::icon::{IconName, icon};
use crate::components::link::link_style;
use crate::components::toggle::toggle_look;
use crate::tokens;

// ---- Toggle -------------------------------------------------------------------

/// What a [`ToggleGroup`] gives its [`Toggle`]s, through context.
#[derive(Clone)]
struct Toggles {
    /// The values that are on.
    value: Memo<Vec<SharedString>>,
    size: Prop<ControlSize>,
    disabled: Prop<bool>,
    /// Asks to turn the toggle of a value on or off.
    request: Rc<dyn Fn(SharedString, bool)>,
}

/// The props of [`Toggle`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ToggleProps {
    /// The visible label, which screen readers say.
    pub label: Prop<SharedString>,
    /// On or off. Set, the app owns it: write back what `onChange` asks
    /// for. Left out, the toggle keeps its own (off at first).
    pub checked: Prop<Option<bool>>,
    /// A glyph before the label, by its Lucide name (`"bold"`).
    pub icon: Prop<Option<IconName>>,
    /// `small` (24 points tall), `medium` (28, the default) or `large` (32).
    /// Inside a `ToggleGroup`, the group's size.
    #[prop(default)]
    pub size: Prop<ControlSize>,
    /// No presses, no hover, focus or press feedback; dimmed text.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// Its id inside a `ToggleGroup`: on while the group's `value` holds
    /// it. The group then owns whether it is on.
    pub value: Prop<Option<SharedString>>,
}

/// The events of a [`Toggle`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ToggleEvent {
    /// It was pressed (a click, a tap, Space or Return): the value asked
    /// for, the opposite of `checked`.
    #[args(checked)]
    Change(bool),
}

/// A button that is on or off:
/// `<Toggle label="Bold" checked={bold()} onChange={({ checked }) => bold.set(checked)} />`.
/// In a `ToggleGroup` it is one of the group's, named by its `value`.
#[component]
pub fn Toggle(props: ToggleProps, cx: Scope<ToggleEvent>) -> impl IntoElement {
    let ToggleProps {
        label,
        checked,
        icon: glyph,
        size,
        disabled,
        value,
    } = props;
    let group = use_context::<Toggles>();
    let own = signal(false);
    let in_group = group.clone();
    let on = computed(move || match (&in_group, value.get()) {
        (Some(g), Some(v)) => g.value.with(|values| values.contains(&v)),
        _ => checked.get().unwrap_or_else(|| own.get()),
    });
    let (group_size, group_disabled) = match &group {
        Some(g) => (Some(g.size), Some(g.disabled)),
        None => (None, None),
    };
    let size = move || group_size.map_or_else(|| size.get(), |s| s.get());
    let off = move || disabled.get() || group_disabled.is_some_and(|d| d.get());
    let roving = group.is_some();
    let request = move |next: bool| {
        match (&group, value.get_untracked()) {
            (Some(g), Some(v)) => (g.request)(v, next),
            _ if checked.get_untracked().is_none() => own.set(next),
            _ => {}
        }
        cx.emit(ToggleEvent::Change(next));
    };
    let glyph = move || dynamic(move || glyph.get().map(|name| icon(name).into_element()));
    if roving {
        // In a group: `nsh_base`'s toggle, which joins the group's arrow
        // keys and its one stop of Tab.
        toggle_look!(base_toggle(), size)
            .pressed(move || on.get())
            .disabled(off)
            .accessibility_label(move || Some(label.get()))
            .on_change(move |next: &bool| request(*next))
            .child(glyph())
            .child(text(move || label.get()))
            .into_element()
    } else {
        // Alone: a plain view, so it works in a trigger slot.
        toggle_look!(view(), size)
            .role(AccessibilityRole::Button)
            .disabled(off)
            .label(move || label.get())
            .accessibility_state(move || AccessibilityState {
                pressed: Some(if on.get() { Checked::True } else { Checked::False }),
                ..AccessibilityState::default()
            })
            .on_press(move || request(!on.get_untracked()))
            .child(glyph())
            .child(text(move || label.get()))
            .into_element()
    }
}

// ---- ToggleGroup ----------------------------------------------------------------

/// The props of [`ToggleGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ToggleGroupProps {
    /// The `value`s of the toggles that are on. Set, the app owns it:
    /// write back what `onChange` asks for. Left out, the group keeps its
    /// own (none on at first).
    pub value: Prop<Option<Vec<SharedString>>>,
    /// At most one toggle on: turning one on turns the others off.
    #[prop(default)]
    pub single: Prop<bool>,
    /// `small`, `medium` (the default) or `large`: every toggle's size.
    #[prop(default)]
    pub size: Prop<ControlSize>,
    /// Every toggle disabled.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The `Toggle`s, each with a `value`.
    pub children: Prop<Children>,
}

/// The events of a [`ToggleGroup`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum ToggleGroupEvent {
    /// A toggle was pressed: the values that would be on.
    #[args(value)]
    Change(Vec<SharedString>),
}

/// A row of toggles, one stop of Tab: the arrows move between them, Space
/// and Return turn the focused one on or off.
#[component]
pub fn ToggleGroup(props: ToggleGroupProps, cx: Scope<ToggleGroupEvent>) -> impl IntoElement {
    let ToggleGroupProps {
        value,
        single,
        size,
        disabled,
        children,
    } = props;
    let own = signal(Vec::<SharedString>::new());
    let current = computed(move || value.get().unwrap_or_else(|| own.get()));
    provide_context(Toggles {
        value: current,
        size,
        disabled,
        request: Rc::new(move |id, on| {
            let mut next: Vec<SharedString> = if on && single.get_untracked() {
                Vec::new()
            } else {
                current.get_untracked().into_iter().filter(|v| *v != id).collect()
            };
            if on {
                next.push(id);
            }
            if value.get_untracked().is_none() {
                own.set(next.clone());
            }
            cx.emit(ToggleGroupEvent::Change(next));
        }),
    });
    view().self_start().child(
        base_toggle_group()
            .disabled(move || disabled.get())
            .flex_row()
            .flex_wrap()
            .self_start()
            .items_center()
            .gap(px(tokens::SPACE_1))
            .child(children),
    )
}

// ---- ButtonGroup ----------------------------------------------------------------

/// The props of [`ButtonGroup`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ButtonGroupProps {
    /// `small`, `medium` (the default) or `large`: every button's size.
    #[prop(default)]
    pub size: Prop<ControlSize>,
    /// Every button disabled.
    #[prop(default)]
    pub disabled: Prop<bool>,
    /// The `Button`s and `IconButton`s, joined edge to edge.
    pub children: Prop<Children>,
}

/// Buttons joined edge to edge into one control:
/// `<ButtonGroup><Button label="Copy" /><Button label="Paste" /></ButtonGroup>`.
/// Each button is its own stop of Tab and reports its own presses.
#[component]
pub fn ButtonGroup(props: ButtonGroupProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    provide_context(ButtonGroupContext {
        size: props.size,
        disabled: props.disabled,
    });
    view()
        .role(AccessibilityRole::Group)
        .flex_row()
        .self_start()
        .items_center()
        // The first button's edge, which its `-1` start margin overlaps.
        .pl(px(tokens::BORDER))
        .child(props.children)
}

// ---- Link -------------------------------------------------------------------------

/// Who opens a [`Link`]'s `href` when it is pressed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PropEnum)]
#[non_exhaustive]
pub enum LinkTarget {
    /// The system: a URL of another app or site (`https:`, `mailto:`)
    /// opens in the system browser or the app for it. The default.
    #[default]
    System,
    /// The app: nothing opens; the app's `onPress` goes there (a route of
    /// its own).
    App,
}

/// The props of [`Link`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct LinkProps {
    /// The underlined text, which screen readers say.
    pub label: Prop<SharedString>,
    /// Where it goes: a URL, or a path of the app's. Shown while the
    /// pointer rests on the link.
    pub href: Prop<SharedString>,
    /// `system` (the default) opens a URL outside the app; `app` leaves
    /// it to `onPress`.
    #[prop(default)]
    pub open_with: Prop<LinkTarget>,
}

/// The events of a [`Link`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum LinkEvent {
    /// It was pressed (a click, a tap, Return or Space), after the system
    /// was asked to open `href` (with `openWith="system"`).
    Press,
}

/// Underlined accent text that goes somewhere:
/// `<Link label="Manual" href="https://omarchy.org/manual" />`.
#[component]
pub fn Link(props: LinkProps, cx: Scope<LinkEvent>) -> impl IntoElement {
    let LinkProps {
        label,
        href,
        open_with,
    } = props;
    link_style()
        .href(move || Some(href.get()))
        .open_with(move || match open_with.get() {
            LinkTarget::App => LinkOpenWith::App,
            _ => LinkOpenWith::System,
        })
        .on_press(move || cx.emit(LinkEvent::Press))
        .child(text(move || label.get()))
}

crate::sizable!(ToggleElement, ToggleGroupElement, ButtonGroupElement);

/// Registers this area's components for TSX.
pub(crate) fn register(registry: &mut Registry) {
    registry
        .register::<super::button::Button>()
        .register::<super::button::IconButton>()
        .register::<ButtonGroup>()
        .register::<Toggle>()
        .register::<ToggleGroup>()
        .register::<Link>();
}
