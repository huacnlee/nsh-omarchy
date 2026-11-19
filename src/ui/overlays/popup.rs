//! [`Popover`], [`HoverCard`] and [`Tooltip`]: popups anchored to a
//! trigger.

use std::time::Duration;

use nsh_core::prelude::*;

use super::{OverlayEvent, follow};
use crate::components::hover_card::hover_card as card_popup;
use crate::components::popover::{popover as popover_popup, popover_title};
use crate::components::shortcut::shortcut_label;
use crate::tokens;


/// The props of [`Popover`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PopoverProps {
    /// What toggles it: a button usually.
    pub trigger: Prop<Option<Slot>>,
    /// The content, made on each opening.
    #[prop(lazy)]
    pub children: Prop<Children>,
    /// A bright heading above the content.
    pub title: Prop<Option<SharedString>>,
    /// Open. Given, it is the app's: write it back in `on_open_change`.
    /// Left out, the popover keeps it (its trigger toggles it).
    pub open: Prop<Option<bool>>,
    /// The side of the trigger it opens on (below when left out).
    #[prop(default)]
    pub placement: Prop<Placement>,
    /// How it lines up with the trigger (centered when left out).
    #[prop(default)]
    pub align: Prop<Align>,
}

/// An anchored, non-modal surface: a press outside and Escape close it,
/// the focus goes back to the trigger.
#[component]
pub fn Popover(props: PopoverProps, cx: Scope<OverlayEvent>) -> impl IntoElement {
    // Driven here, as `DropdownMenu` is: a slot forwarded into
    // `nsh_base`'s trigger sits in a node of the base's own, whose press
    // the trigger (a `Button`) would take.
    let open = follow(props.open);
    let change = move |next: bool| {
        if props.open.get_untracked().is_none() {
            open.set(next);
        }
        cx.emit(OverlayEvent::OpenChange(next));
    };
    let mut popup = popover_popup()
        .placement(move || props.placement.get())
        .align(move || props.align.get())
        .open(move || Some(open.get()))
        .on_open_change(move |e: &nsh_base::PopoverEvent| {
            if let nsh_base::PopoverEvent::OpenChange(next, _) = e {
                change(*next);
            }
        })
        .child(dynamic(move || props.title.get().map(popover_title)))
        .child(props.children);
    if let Some(trigger) = props.trigger.get() {
        popup = popup.trigger(
            view().slot(Some(trigger), move |s| {
                s.on_press(move || change(!open.get_untracked()))
            }),
        );
    }
    popup
}

/// The props of [`Tooltip`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct TooltipProps {
    /// What it says.
    pub label: Prop<SharedString>,
    /// A shortcut after the label, in monospace, as the platform writes
    /// it: `"mod-s"` reads `cmd-s` on macOS, `ctrl-s` elsewhere.
    pub shortcut: Prop<Option<SharedString>>,
}

/// A small square label next to a control, given to its `tooltip` prop:
/// `<IconButton tooltip={<Tooltip label="Save" shortcut="mod-s" />} ..>`.
/// It shows above the control after the pointer rests, at once with the
/// keyboard focus, and hides on Escape. Keep an accessible label on the
/// control: a tooltip only adds.
#[component]
pub fn Tooltip(props: TooltipProps) -> impl IntoElement {
    // The look of `components::tooltip`; the control's `tooltip` makes the
    // hint overlay around it.
    view()
        .flex_row()
        .items_center()
        .gap(px(tokens::SPACE_2))
        .max_w(px(320.0))
        .px(px(tokens::SPACE_2))
        .py(px(tokens::SPACE_1))
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(move || tokens::control_border())
        .bg(move || tokens::surface())
        .text_size(px(tokens::TEXT_SM))
        .child(text(move || props.label.get()).text_color(move || tokens::foreground()))
        .child(dynamic(move || {
            props.shortcut.get().map(|keys| {
                text(shortcut_label(&keys))
                    .font_family("monospace")
                    .text_color(move || tokens::secondary())
            })
        }))
}

/// The props of [`HoverCard`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct HoverCardProps {
    /// What it previews; it keeps its own role and presses.
    pub trigger: Prop<Option<Slot>>,
    /// The content, made on each opening.
    #[prop(lazy)]
    pub children: Prop<Children>,
    /// The side of the trigger it opens on (below when left out).
    #[prop(default)]
    pub placement: Prop<Placement>,
}

/// How long the pointer rests on a hover card's trigger before it opens.
const HOVER_OPEN: Duration = Duration::from_millis(400);
/// How long the pointer is away from both parts before it closes.
const HOVER_CLOSE: Duration = Duration::from_millis(200);

/// A preview of what the trigger points at: it opens after the pointer
/// rests 400 ms and closes 200 ms after it left both. The pointer alone
/// opens it, so keep what a keyboard user needs reachable elsewhere.
#[component]
pub fn HoverCard(props: HoverCardProps, cx: Scope<OverlayEvent>) -> impl IntoElement {
    // Driven here: the pointer on our slot (the app's node itself) opens
    // it; the base card's own handlers follow the pointer on the card.
    let open = signal(false);
    let over_trigger = stored(false);
    let pending = stored(None::<Spawned<()>>);
    let change = move |next: bool| {
        if open.get_untracked() != next {
            open.set(next);
            cx.emit(OverlayEvent::OpenChange(next));
        }
    };
    let after = move |next: bool, delay: Duration| {
        if let Some(task) = pending.replace(None) {
            task.cancel();
        }
        pending.replace(Some(set_timeout(delay, move || {
            pending.replace(None);
            change(next);
        })));
    };
    let mut card = card_popup()
        .placement(move || props.placement.get())
        .open(move || Some(open.get()))
        .on_open_change(move |e: &nsh_base::HoverCardEvent| {
            if let nsh_base::HoverCardEvent::OpenChange(next, _) = e {
                // The pointer left the card: the trigger may hold it.
                if !*next && over_trigger.get() {
                    return;
                }
                if let Some(task) = pending.replace(None) {
                    task.cancel();
                }
                change(*next);
            }
        })
        .child(props.children);
    if let Some(trigger) = props.trigger.get() {
        card = card.trigger(view().slot(Some(trigger), move |s| {
            s.on_mouse_enter(move |_: &MouseEvent| {
                over_trigger.replace(true);
                after(true, HOVER_OPEN);
            })
            .on_mouse_leave(move |_: &MouseEvent| {
                over_trigger.replace(false);
                after(false, HOVER_CLOSE);
            })
        }));
    }
    card
}

