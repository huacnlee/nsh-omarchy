//! Overlays and feedback: dialogs, sheets, popovers, hover cards,
//! tooltips, toasts and the platform's confirm.

use std::future::Future;

use nsh_core::element::IntoProp;
use nsh_core::prelude::*;

use crate::ui::overlays as ui;
pub use crate::components::toast::TOAST_TIMEOUT;
pub use crate::ui::overlays::{
    AlertDialog, AlertDialogElement, ConfirmOptions, Dialog, DialogElement, HoverCard,
    HoverCardElement, Popover, PopoverElement, Sheet, SheetElement, Toast, ToastKind, Toaster,
    ToasterElement, Tooltip, TooltipElement,
};

/// A modal card over a scrim; its children are the body:
/// `dialog("Rename workspace").open(move || open.get()).on_open_change(move |o: &bool| open.set(*o))`.
pub fn dialog<M>(title: impl IntoProp<SharedString, M>) -> DialogElement {
    ui::dialog::dialog().title(title)
}

/// A dialog that asks for a decision, confirmed by `action_label`:
/// `alert_dialog("Delete workspace?", "Delete").danger().on_action(delete)`.
pub fn alert_dialog<M1, M2>(
    title: impl IntoProp<SharedString, M1>,
    action_label: impl IntoProp<SharedString, M2>,
) -> AlertDialogElement {
    ui::dialog::alert_dialog().title(title).action_label(action_label)
}

/// A panel at an edge of the window (the right by default); its children
/// are the body: `sheet("Settings").open(move || open.get())`.
pub fn sheet<M>(title: impl IntoProp<SharedString, M>) -> SheetElement {
    ui::dialog::sheet().title(title)
}

/// An anchored, non-modal surface its trigger toggles; its children are
/// the content: `popover().trigger(button("Details")).child(..)`.
pub fn popover() -> PopoverElement {
    ui::popup::popover()
}

/// A preview shown while the pointer rests on its trigger:
/// `hover_card().trigger(button("@dhh").ghost()).child(..)`.
pub fn hover_card() -> HoverCardElement {
    ui::popup::hover_card()
}

/// A hint for a control's `tooltip(..)`:
/// `button("Save").tooltip(tooltip("Save").shortcut("mod-s"))`.
pub fn tooltip<M>(label: impl IntoProp<SharedString, M>) -> TooltipElement {
    ui::popup::tooltip().label(label)
}

/// Where toasts show; place it once, at the root, after the content.
pub fn toaster() -> ToasterElement {
    ui::toast::toaster()
}

/// Shows `toast` in the app's [`Toaster`] (or as soon as one mounts) and
/// returns its id: `toast(Toast::new("Saved").success())`. Toasting an id
/// that is shown replaces it.
pub fn toast(toast: Toast) -> SharedString {
    ui::show_toast(toast)
}

/// Dismisses the toast `id` (nothing when it is not shown).
pub fn dismiss_toast(id: &str) {
    ui::dismiss_toast(id);
}

/// Asks with the platform's own alert; true when the user chose the
/// action:
/// `confirm(ConfirmOptions::new("Delete 3 files?").action_label("Delete").danger()).await`.
pub fn confirm(options: ConfirmOptions) -> impl Future<Output = bool> {
    ui::confirm(options)
}
