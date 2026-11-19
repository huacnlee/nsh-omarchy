//! [`Dialog`], [`AlertDialog`] and [`Sheet`]: modal overlays.

use nsh_core::prelude::*;

use super::{OverlayEvent, follow};
use crate::Tone;
use crate::components::dialog::{self as dialogs, OverlayTone};
use crate::components::sheet::sheet as sheet_panel;
use crate::tokens;

// ---- Dialog ---------------------------------------------------------------

/// The props of [`Dialog`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DialogProps {
    /// The heading, also the dialog's name for screen readers.
    pub title: Prop<SharedString>,
    /// Shown. Given, it is the app's: write it back in `on_open_change`.
    /// Left out, the dialog keeps it (closed).
    pub open: Prop<Option<bool>>,
    /// Secondary text under the title.
    pub description: Prop<Option<SharedString>>,
    /// The actions, right-aligned under the body (buttons).
    pub footer: Prop<Option<Slot>>,
    /// A press on the scrim closes it (default true).
    #[prop(default = true)]
    pub close_on_backdrop_press: Prop<bool>,
    /// Escape and the back button close it (default true).
    #[prop(default = true)]
    pub close_on_escape: Prop<bool>,
    /// The body, made on each opening.
    #[prop(lazy)]
    pub children: Prop<Children>,
}

/// A modal card over a scrim, square, 1pt border, outline actions:
/// `<Dialog title="Rename" open={open()} onOpenChange={..}>..</Dialog>`.
#[component]
pub fn Dialog(props: DialogProps, cx: Scope<OverlayEvent>) -> impl IntoElement {
    let open = follow(props.open);
    let mut modal = dialogs::dialog(open, props.title.get_untracked())
        .width(420.0)
        .close_on_backdrop_press(props.close_on_backdrop_press.get_untracked())
        .close_on_escape(props.close_on_escape.get_untracked())
        .on_dismiss(move || cx.emit(OverlayEvent::OpenChange(false)))
        .body(move || {
            view()
                .gap(px(tokens::SPACE_3))
                .child(props.children)
                .slot(props.footer.get(), |s| {
                    s.flex_row()
                        .justify_end()
                        .gap(px(tokens::SPACE_2))
                        .pt(px(tokens::SPACE_1))
                })
        });
    if let Some(description) = props.description.get_untracked() {
        modal = modal.description(description);
    }
    modal
}

// ---- AlertDialog ----------------------------------------------------------

/// The props of [`AlertDialog`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AlertDialogProps {
    /// The question ("Delete workspace?").
    pub title: Prop<SharedString>,
    /// The confirming action's label ("Delete").
    pub action_label: Prop<SharedString>,
    /// Shown. Given, it is the app's: write it back in `on_open_change`.
    /// Left out, the dialog keeps it (closed).
    pub open: Prop<Option<bool>>,
    /// What follows from it.
    pub description: Prop<Option<SharedString>>,
    /// The cancelling action's label (default "Cancel").
    pub cancel_label: Prop<Option<SharedString>>,
    /// The confirming action's color: `accent` (the default), `success`,
    /// `warning` or `danger` (destroys or deletes something).
    #[prop(default)]
    pub tone: Prop<Tone>,
}

/// The events of an [`AlertDialog`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum AlertDialogEvent {
    /// It asks to close (an action, Escape or back): write `open` back.
    #[args(open)]
    OpenChange(bool),
    /// The user confirmed; the dialog closes.
    Action,
}

/// A dialog that asks for a decision: only its buttons and Escape close it
/// (not the scrim), Cancel takes the first focus, and screen readers
/// announce it as an alert dialog.
#[component]
pub fn AlertDialog(props: AlertDialogProps, cx: Scope<AlertDialogEvent>) -> impl IntoElement {
    let open = follow(props.open);
    let tone = match props.tone.get_untracked() {
        Tone::Danger => OverlayTone::Danger,
        Tone::Success => OverlayTone::Success,
        Tone::Warning => OverlayTone::Warning,
        _ => OverlayTone::Primary,
    };
    let cancel = props
        .cancel_label
        .get_untracked()
        .unwrap_or_else(|| "Cancel".into());
    let mut modal = dialogs::alert_dialog(open, props.title.get_untracked())
        .width(420.0)
        .on_dismiss(move || cx.emit(AlertDialogEvent::OpenChange(false)))
        .cancel(cancel)
        .action(props.action_label.get_untracked(), tone, move || {
            cx.emit(AlertDialogEvent::OpenChange(false));
            cx.emit(AlertDialogEvent::Action);
        });
    if let Some(description) = props.description.get_untracked() {
        modal = modal.description(description);
    }
    modal
}

impl AlertDialogElement {
    /// A destructive action, in the danger color: `.tone(Tone::Danger)`.
    pub fn danger(self) -> Self {
        self.tone(Tone::Danger)
    }
}

// ---- Sheet ----------------------------------------------------------------

/// The props of [`Sheet`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SheetProps {
    /// The heading, also the panel's name for screen readers.
    pub title: Prop<SharedString>,
    /// Shown. Given, it is the app's: write it back in `on_open_change`.
    /// Left out, the sheet keeps it (closed).
    pub open: Prop<Option<bool>>,
    /// The edge it is on (default `right`).
    #[prop(default = Placement::Right)]
    pub placement: Prop<Placement>,
    /// Secondary text under the title.
    pub description: Prop<Option<SharedString>>,
    /// Actions at the bottom of the panel, over a divider (buttons).
    pub footer: Prop<Option<Slot>>,
    /// The close button's label, for screen readers (default "Close").
    pub close_label: Prop<Option<SharedString>>,
    /// The content, made on each opening; it scrolls.
    #[prop(lazy)]
    pub children: Prop<Children>,
}

/// A panel at an edge of the window over a scrim, with a header and a
/// close button: `<Sheet title="Settings" open={open()} onOpenChange={..}>`.
#[component]
pub fn Sheet(props: SheetProps, cx: Scope<OverlayEvent>) -> impl IntoElement {
    let open = follow(props.open);
    let mut panel = sheet_panel(open, props.title.get_untracked())
        .side(props.placement.get_untracked())
        .close_label(
            props
                .close_label
                .get_untracked()
                .unwrap_or_else(|| "Close".into()),
        )
        .on_dismiss(move || cx.emit(OverlayEvent::OpenChange(false)))
        .body(move || view().gap(px(tokens::SPACE_4)).child(props.children));
    if props.footer.get_untracked().is_some() {
        panel = panel.footer(move || {
            view()
                .flex_row()
                .gap(px(tokens::SPACE_2))
                .slot(props.footer.get(), |s| s.flex_row().gap(px(tokens::SPACE_2)))
        });
    }
    if let Some(description) = props.description.get_untracked() {
        panel = panel.description(description);
    }
    panel
}
