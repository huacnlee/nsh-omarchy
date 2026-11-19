//! Overlays and feedback: `Dialog`, `AlertDialog`, `Sheet`, `Popover`,
//! `HoverCard`, `Tooltip`, `Toaster`, and the functions `toast`,
//! `dismissToast` and `confirm` (a flat module: TSX imports them by name
//! from `nsh-omarchy`). Drawn by [`crate::components::dialog`],
//! [`crate::components::sheet`], [`crate::components::popover`],
//! [`crate::components::hover_card`] and [`crate::components::toast`].

pub(crate) mod dialog;
pub(crate) mod popup;
pub(crate) mod toast;

use nsh_core::prelude::*;

pub use dialog::{AlertDialog, AlertDialogElement, Dialog, DialogElement, Sheet, SheetElement};
pub use popup::{HoverCard, HoverCardElement, Popover, PopoverElement, Tooltip, TooltipElement};
pub use toast::{
    ConfirmOptions, Toast, ToastKind, Toaster, ToasterElement, Toasts, confirm, dismiss_toast,
    show_toast,
};

/// The events of the dialogs, the sheet and the popups.
#[derive(Debug, Clone, PartialEq, Event)]
pub enum OverlayEvent {
    /// It asks to open or to close (Escape, back, the backdrop, a press
    /// outside, its trigger): write `open` back when you set it.
    #[args(open)]
    OpenChange(bool),
}

/// The shown state of an overlay whose `open` prop is controlled when
/// given: it follows the prop, and keeps its own value while the prop is
/// left out (closed at first).
pub(crate) fn follow(open: Prop<Option<bool>>) -> Signal<bool> {
    let state = signal(open.get_untracked().unwrap_or(false));
    effect(move || {
        if let Some(value) = open.get() {
            state.set(value);
        }
    });
    state
}

/// Registers this area's components and the toast functions for TSX.
pub(crate) fn register(registry: &mut Registry) {
    registry
        .module::<Toasts>()
        .register::<Dialog>()
        .register::<AlertDialog>()
        .register::<Sheet>()
        .register::<Popover>()
        .register::<HoverCard>()
        .register::<Tooltip>()
        .register::<Toaster>();
}
