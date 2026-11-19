//! The toggles, the single-choice groups, the surfaces, the static pieces
//! and the type scale for TSX: `<Toggle>`, `<ToggleGroup>`, `<ButtonGroup>`,
//! `<Card>`, `<Panel>`, `<Separator>`, `<Badge>`, `<Keycap>`, `<Shortcut>`,
//! `<EmptyState>`, `<Avatar>`, `<Heading>`, `<Title>`, `<Body>`,
//! `<Caption>` and `<Code>`, drawn with [`crate::components`] and the
//! Omarchy tokens.
//!
//! Square, flat, one border; no shadows, no gradients. Every root is a
//! plain view or text (an app may put a toggle or an avatar in a menu's or
//! popover's trigger slot, which reaches a component one level deep).

use nsh_base::{toggle as base_toggle, toggle_group as base_toggle_group};
use nsh_core::{Checked, prelude::*};

use crate::components::button::{ButtonSize, actions_duration};
use crate::components::toggle::toggle_look;
use crate::components::avatar::{AvatarSize, avatar as avatar_view};
use crate::components::surface;
use crate::tokens;

// ---- Toggle ---------------------------------------------------------------

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
    pub size: Prop<ButtonSize>,
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
    pub size: Prop<ButtonSize>,
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
    pub size: Prop<ButtonSize>,
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
fn segment_padding(size: ButtonSize) -> f32 {
    match size {
        ButtonSize::Sm => tokens::SPACE_3,
        ButtonSize::Md => tokens::SPACE_4,
        ButtonSize::Lg => tokens::SPACE_4 + tokens::SPACE_1,
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

// ---- Surfaces -------------------------------------------------------------

/// The props of [`Card`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct CardProps {
    /// The card's title, a screen reader's header.
    pub title: Prop<Option<SharedString>>,
    /// A line under the title.
    pub description: Prop<Option<SharedString>>,
    /// The body.
    pub children: Prop<Children>,
    /// Actions under the body, right-aligned.
    pub footer: Prop<Option<Slot>>,
}

/// A raised surface with an optional title, description and footer.
#[component]
pub fn Card(props: CardProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let title = props.title;
    let description = props.description;
    let footer = props.footer;
    surface::card()
        .child(dynamic(move || {
            let (title, description) = (title.get(), description.get());
            if title.is_none() && description.is_none() {
                return None;
            }
            Some(
                view()
                    .flex_col()
                    .gap(px(tokens::SPACE_1))
                    .children(title.map(|t| {
                        text(t)
                            .role(AccessibilityRole::Header)
                            .text_size(px(tokens::TEXT_LG))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(move || tokens::bright())
                            .into_element()
                    }))
                    .children(description.map(|d| {
                        text(d)
                            .text_size(px(tokens::TEXT_MD))
                            .text_color(move || tokens::secondary())
                            .into_element()
                    }))
                    .into_element(),
            )
        }))
        .child(props.children)
        .child(dynamic(move || {
            footer.get().map(|footer| {
                view()
                    .flex_row()
                    .justify_end()
                    .gap(px(tokens::SPACE_2))
                    .border_t(px(tokens::BORDER))
                    .border_color(move || tokens::divider())
                    .pt(px(tokens::SPACE_3))
                    .slot(Some(footer), |s| s)
                    .into_element()
            })
        }))
}

/// A [`Panel`]'s surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PropEnum)]
pub enum PanelVariant {
    /// Raised, as a card.
    #[default]
    Raised,
    /// A recessed well.
    Inset,
}

/// The props of [`Panel`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PanelProps {
    /// `raised` (default) or `inset` (a recessed well).
    #[prop(default)]
    pub variant: Prop<PanelVariant>,
    /// The content.
    pub children: Prop<Children>,
}

/// A plain surface: `raised` (default) or `inset` (a recessed well).
#[component]
pub fn Panel(props: PanelProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    // The variant is read once: a panel does not change surface.
    match props.variant.get() {
        PanelVariant::Raised => surface::card().p(px(tokens::SPACE_3)),
        PanelVariant::Inset => surface::inset_well().border_color(AdaptiveColor::TRANSPARENT),
    }
    .child(props.children)
}

/// Which way a [`Separator`] runs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PropEnum)]
pub enum Orientation {
    /// Across its container.
    #[default]
    Horizontal,
    /// Down a row, as tall as the row.
    Vertical,
}

/// The props of [`Separator`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SeparatorProps {
    /// `horizontal` (default) or `vertical`.
    #[prop(default)]
    pub orientation: Prop<Orientation>,
}

/// A 1pt divider: horizontal (default) or vertical.
#[component]
pub fn Separator(props: SeparatorProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    match props.orientation.get() {
        Orientation::Horizontal => surface::separator().self_stretch(),
        Orientation::Vertical => view()
            .w(px(tokens::BORDER))
            .self_stretch()
            .flex_shrink_0()
            .bg(move || tokens::divider())
            .accessibility_elements_hidden(true),
    }
}

// ---- Badge, Keycap, Shortcut ----------------------------------------------

/// A [`Badge`]'s meaning.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PropEnum)]
pub enum BadgeTone {
    /// No meaning of its own.
    #[default]
    Neutral,
    /// The theme's accent: new, highlighted.
    Accent,
    /// Done, online.
    Success,
    /// Needs a look.
    Warning,
    /// Failed.
    Danger,
}

impl BadgeTone {
    /// The tone's color: an outlined badge's border and text, a solid
    /// badge's fill.
    fn color(self) -> AdaptiveColor {
        match self {
            BadgeTone::Neutral => tokens::control_border(),
            BadgeTone::Accent => tokens::accent(),
            BadgeTone::Success => tokens::success(),
            BadgeTone::Warning => tokens::warning(),
            BadgeTone::Danger => tokens::danger(),
        }
    }
}

/// The props of [`Badge`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct BadgeProps {
    /// The status or count, shown in capitals.
    pub label: Prop<SharedString>,
    /// Neutral (default), accent, success, warning or danger.
    #[prop(default)]
    pub tone: Prop<BadgeTone>,
    /// Filled in its tone instead of outlined.
    #[prop(default)]
    pub solid: Prop<bool>,
}

/// A short status or count: outlined in its tone, or `solid`.
#[component]
pub fn Badge(props: BadgeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let tone = props.tone;
    let solid = props.solid;
    let label = props.label;
    view()
        .flex_row()
        .flex_shrink_0()
        .self_start()
        .items_center()
        .justify_center()
        .h(px(20.0))
        .px(px(tokens::SPACE_1 * 1.5))
        .border(px(tokens::BORDER))
        .border_color(move || {
            if solid.get() {
                AdaptiveColor::TRANSPARENT
            } else {
                tone.get().color()
            }
        })
        .bg(move || match (solid.get(), tone.get()) {
            (false, _) => AdaptiveColor::TRANSPARENT,
            (true, BadgeTone::Neutral) => tokens::selected_fill(),
            (true, tone) => tone.color(),
        })
        .child(
            text(move || label.get())
                .text_size(px(tokens::TEXT_XS))
                .uppercase()
                .letter_spacing(px(0.25))
                .text_color(move || match (solid.get(), tone.get()) {
                    (false, BadgeTone::Neutral) => tokens::foreground(),
                    (false, tone) => tone.color(),
                    (true, BadgeTone::Neutral) => tokens::bright(),
                    (true, _) => tokens::on_accent(),
                }),
        )
}

/// The props of [`Keycap`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct KeycapProps {
    /// The key as printed (`Ctrl`, `esc`).
    pub label: Prop<SharedString>,
}

/// One key, in monospace on an inset square: `<Keycap label="Ctrl" />`.
#[component]
pub fn Keycap(props: KeycapProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let label = props.label;
    keycap_view(move || label.get())
}

/// [`crate::components::keycap`]'s look, its key following `label` (a prop that may change).
fn keycap_view(label: impl IntoReactive<SharedString>) -> View {
    view()
        .flex_row()
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .h(px(18.0))
        .min_w(px(18.0))
        .px(px(tokens::SPACE_1))
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::inset())
        .role(AccessibilityRole::KeyboardKey)
        .child(
            text(label)
                .font_family(move || tokens::font_mono())
                .text_size(px(tokens::TEXT_XS))
                .text_color(move || tokens::foreground()),
        )
}

/// The props of [`Shortcut`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct ShortcutProps {
    /// The keys, in order: `["Super", "Shift", "Enter"]`.
    pub keys: Prop<Vec<SharedString>>,
}

/// A key combination: `<Shortcut keys={["Super", "Shift", "Enter"]} />`.
/// Screen readers hear it as one element, the keys in order.
#[component]
pub fn Shortcut(props: ShortcutProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let keys = props.keys;
    view()
        .flex_row()
        .flex_shrink_0()
        .self_start()
        .items_center()
        .gap(px(tokens::SPACE_1))
        .accessible(true)
        .label(move || SharedString::from(keys.get().join(" ")))
        .child(each(
            move || keys.get(),
            |key: &SharedString| key.clone(),
            |key, _| keycap_view(move || key.get()),
        ))
}

// ---- EmptyState, Avatar ---------------------------------------------------

/// The props of [`EmptyState`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct EmptyStateProps {
    /// What is missing: "No windows".
    pub title: Prop<SharedString>,
    /// A hint at what to do.
    pub description: Prop<Option<SharedString>>,
    /// An action under the hint (`<Button label="Open" />`).
    pub action: Prop<Option<Slot>>,
}

/// What a view shows while it has nothing: a title, a hint, an action, in a
/// dashed frame.
#[component]
pub fn EmptyState(props: EmptyStateProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let title = props.title;
    let description = props.description;
    view()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(tokens::SPACE_2))
        .p(px(tokens::SPACE_4 * 2.0))
        .border(px(tokens::BORDER))
        .border_dashed()
        .border_color(move || tokens::control_border())
        .child(
            text(move || title.get())
                .role(AccessibilityRole::Header)
                .text_size(px(tokens::TEXT_LG))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::bright()),
        )
        .child(dynamic(move || {
            description.get().map(|d| {
                text(d)
                    .text_size(px(tokens::TEXT_MD))
                    .text_center()
                    .text_color(move || tokens::secondary())
                    .into_element()
            })
        }))
        // A view sized to the action, centered: the action's own
        // `self-start` (a button's) then sits in the middle.
        .child(view().slot(props.action.get(), |s| s))
}

/// An [`Avatar`]'s size.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PropEnum)]
pub enum AvatarSizeProp {
    /// 24 points.
    Sm,
    /// 32 points.
    #[default]
    Md,
    /// 40 points.
    Lg,
}

/// The props of [`Avatar`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AvatarProps {
    /// The person: screen readers say it, the initials come from it.
    pub name: Prop<SharedString>,
    /// The picture (a URL or a path), shown once it loads.
    pub src: Prop<Option<SharedString>>,
    /// `sm` (24), `md` (32, default) or `lg` (40 points).
    #[prop(default)]
    pub size: Prop<AvatarSizeProp>,
}

/// A square avatar: the picture, or the person's initials while it loads,
/// when it fails and without one.
#[component]
pub fn Avatar(props: AvatarProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    let name = props.name;
    let src = props.src;
    let size = props.size;
    // A view around `nsh_base`'s avatar: the root a trigger slot reaches.
    view().flex_shrink_0().child(dynamic(move || {
        let (base, side) = match size.get() {
            AvatarSizeProp::Sm => (AvatarSize::Small, tokens::CONTROL_SM),
            AvatarSizeProp::Md => (AvatarSize::Medium, tokens::CONTROL_LG),
            AvatarSizeProp::Lg => (AvatarSize::Large, 40.0),
        };
        avatar_view(name.get(), base)
            .size(px(side))
            .src(move || src.get().map(Into::into))
            .into_element()
    }))
}

// ---- Typography -----------------------------------------------------------

/// The props of the type scale: [`Heading`], [`Title`], [`Body`],
/// [`Caption`] and [`Code`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TypeProps {
    /// The text (strings, and spans of their own).
    pub children: Prop<Children>,
}

/// A text of the scale holding `children`.
fn type_text(children: Prop<Children>, size: f32) -> Text {
    text(SharedString::default())
        .text_size(px(size))
        .child(children)
}

/// A page's heading: 20pt, bright, a screen reader's header.
#[component]
pub fn Heading(props: TypeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    type_text(props.children, tokens::TEXT_2XL)
        .role(AccessibilityRole::Header)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::bright())
}

/// A section's title: 14pt, bright, a screen reader's header.
#[component]
pub fn Title(props: TypeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    type_text(props.children, tokens::TEXT_LG)
        .role(AccessibilityRole::Header)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::bright())
}

/// Body text: 12pt.
#[component]
pub fn Body(props: TypeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    type_text(props.children, tokens::TEXT_MD).text_color(move || tokens::foreground())
}

/// Captions and hints: 11pt, secondary.
#[component]
pub fn Caption(props: TypeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    type_text(props.children, tokens::TEXT_SM).text_color(move || tokens::secondary())
}

/// Inline code: monospace on the inset surface.
#[component]
pub fn Code(props: TypeProps, _cx: Scope<NoEvent>) -> impl IntoElement {
    type_text(props.children, tokens::TEXT_SM)
        .font_family(move || tokens::font_mono())
        .px(px(tokens::SPACE_1))
        .bg(move || tokens::inset())
        .text_color(move || tokens::bright())
}
