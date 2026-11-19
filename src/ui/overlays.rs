//! The overlays for TSX: `<Dialog>`, `<AlertDialog>`, `<Sheet>`,
//! `<Popover>`, `<Tooltip>`, `<HoverCard>`, `<Toaster>` and the functions
//! `toast`, `dismissToast` and `confirm`, drawn by
//! [`crate::components::dialog`], [`crate::components::sheet`],
//! [`crate::components::popover`], [`crate::components::hover_card`] and
//! [`crate::components::toast`].

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::time::Duration;

use nsh_core::element::{Arg, JsHost, JsIn, JsOwned, JsValue, PropError, TsTypes};
use nsh_core::prelude::*;

use crate::components::dialog::{self as dialogs, OverlayTone};
use crate::components::hover_card::hover_card as card_popup;
use crate::components::popover::{popover as popover_popup, popover_title};
use crate::components::sheet::sheet as sheet_panel;
use crate::components::shortcut::shortcut_label;
use crate::components::toast::{
    ToastKind as Kind, ToastMessage, Toaster as Store, toast as message,
};
use crate::tokens;

/// The events of the dialogs, the sheet and the popups.
#[derive(Debug, Clone, PartialEq, Event)]
pub enum OverlayEvent {
    /// It opened, or the user closed it (Escape, back, the backdrop, a
    /// press outside): write `open` back.
    #[args(open)]
    OpenChange(bool),
}

/// A signal that follows the controlled `open` prop (closed while it is
/// left out), for the builders that write their own state.
fn follow(open: Prop<Option<bool>>) -> Signal<bool> {
    let state = signal(open.get_untracked().unwrap_or(false));
    effect(move || {
        if let Some(value) = open.get() {
            state.set(value);
        }
    });
    state
}

// ---- Dialog ---------------------------------------------------------------

/// What screen readers announce a dialog as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum DialogRole {
    /// A dialog.
    #[default]
    Dialog,
    /// A dialog that asks the user to confirm (`alertdialog`).
    #[prop(rename = "alertdialog")]
    AlertDialog,
}

/// The props of [`Dialog`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct DialogProps {
    /// Shown (controlled): write it back in `onOpenChange`.
    pub open: Prop<bool>,
    /// The heading, also the dialog's name for screen readers.
    pub title: Prop<SharedString>,
    /// Secondary text under the title.
    pub description: Prop<Option<SharedString>>,
    /// The body, made on each opening.
    #[prop(lazy)]
    pub children: Prop<Children>,
    /// The actions, right-aligned under the body (`Button`s).
    pub footer: Prop<Option<Slot>>,
    /// A press on the scrim closes it (default true).
    #[prop(default = true)]
    pub close_on_backdrop_press: Prop<bool>,
    /// Escape and the back button close it (default true).
    #[prop(default = true)]
    pub close_on_escape: Prop<bool>,
    /// `alertdialog` for one that asks the user to confirm.
    #[prop(default)]
    pub role: Prop<DialogRole>,
}

/// A modal card over a scrim, square, 1pt border, outline actions.
#[component]
pub fn Dialog(props: DialogProps, cx: Scope<OverlayEvent>) -> impl IntoElement {
    let open = follow_bool(props.open);
    let alert = props.role.get_untracked() == DialogRole::AlertDialog;
    let base = if alert { dialogs::alert_dialog } else { dialogs::dialog };
    let mut modal = base(open, props.title.get_untracked())
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

/// A signal that follows the required `open` prop.
fn follow_bool(open: Prop<bool>) -> Signal<bool> {
    let state = signal(open.get_untracked());
    effect(move || state.set(open.get()));
    state
}

/// The tone of an [`AlertDialog`]'s confirming action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum AlertTone {
    /// Destroys or deletes something: danger edge and label.
    #[default]
    Danger,
    /// Any other decision: the accent.
    Primary,
}

/// The props of [`AlertDialog`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct AlertDialogProps {
    /// Shown (controlled): write it back in `onOpenChange`.
    pub open: Prop<bool>,
    /// The question.
    pub title: Prop<SharedString>,
    /// What follows from it.
    pub description: Prop<Option<SharedString>>,
    /// The confirming action's label ("Delete").
    pub action_label: Prop<SharedString>,
    /// The cancelling action's label (default "Cancel").
    pub cancel_label: Prop<Option<SharedString>>,
    /// `danger` for a destructive action (default), `primary` otherwise.
    #[prop(default)]
    pub tone: Prop<AlertTone>,
}

/// The events of an [`AlertDialog`].
#[derive(Debug, Clone, PartialEq, Event)]
pub enum AlertDialogEvent {
    /// It closed (an action, Escape or back): write `open` back.
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
    let open = follow_bool(props.open);
    let tone = match props.tone.get_untracked() {
        AlertTone::Danger => OverlayTone::Danger,
        AlertTone::Primary => OverlayTone::Primary,
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

// ---- Sheet ----------------------------------------------------------------

/// The props of [`Sheet`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct SheetProps {
    /// Shown (controlled): write it back in `onOpenChange`.
    pub open: Prop<bool>,
    /// The heading, also the panel's name for screen readers.
    pub title: Prop<SharedString>,
    /// Secondary text under the title.
    pub description: Prop<Option<SharedString>>,
    /// The edge it is on (default `right`).
    #[prop(default = Placement::Right)]
    pub side: Prop<Placement>,
    /// The content, made on each opening; it scrolls.
    #[prop(lazy)]
    pub children: Prop<Children>,
    /// Actions at the bottom of the panel, over a divider.
    pub footer: Prop<Option<Slot>>,
    /// The close button's label, for screen readers (default "Close").
    pub close_label: Prop<Option<SharedString>>,
}

/// A panel at an edge of the window over a scrim, with a header and a
/// close button.
#[component]
pub fn Sheet(props: SheetProps, cx: Scope<OverlayEvent>) -> impl IntoElement {
    let open = follow_bool(props.open);
    let mut panel = sheet_panel(open, props.title.get_untracked())
        .side(props.side.get_untracked())
        .close_label(
            props
                .close_label
                .get_untracked()
                .unwrap_or_else(|| "Close".into()),
        )
        .on_dismiss(move || cx.emit(OverlayEvent::OpenChange(false)))
        .body(move || view().gap(px(tokens::SPACE_4)).child(props.children));
    if let Some(footer) = props.footer.get_untracked() {
        panel = panel.footer(move || {
            view()
                .flex_row()
                .gap(px(tokens::SPACE_2))
                .slot(Some(footer), |s| s.flex_row().gap(px(tokens::SPACE_2)))
        });
    }
    if let Some(description) = props.description.get_untracked() {
        panel = panel.description(description);
    }
    panel
}

// ---- Popover, Tooltip, HoverCard ------------------------------------------

/// The props of [`Popover`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct PopoverProps {
    /// What toggles it: a `Button` usually.
    pub trigger: Prop<Option<Slot>>,
    /// The content, made on each opening.
    #[prop(lazy)]
    pub children: Prop<Children>,
    /// A bright heading above the content.
    pub title: Prop<Option<SharedString>>,
    /// Open (controlled), with `onOpenChange`; without it the popover
    /// keeps its own state.
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

// ---- Toasts ---------------------------------------------------------------

/// What a toast is about: its icon and color; an error stays until
/// dismissed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
pub enum ToastKind {
    /// News (default).
    #[default]
    Info,
    /// Something worked.
    Success,
    /// Something needs attention.
    Warning,
    /// Something failed; it stays until dismissed.
    Error,
}

impl From<ToastKind> for Kind {
    fn from(kind: ToastKind) -> Self {
        match kind {
            ToastKind::Info => Kind::Info,
            ToastKind::Success => Kind::Success,
            ToastKind::Warning => Kind::Warning,
            ToastKind::Error => Kind::Error,
        }
    }
}

/// A toast's button: runs `onAction`, then dismisses the toast.
#[derive(Debug, Clone)]
pub struct ToastAction {
    /// The button's label ("Undo").
    pub label: String,
    /// Runs when it is pressed.
    pub on_action: Callback<()>,
}

impl JsValue for ToastAction {
    fn ts_type(types: &mut TsTypes) -> String {
        format!(
            "{{ /** The button's label (\"Undo\"). */ label: string; /** Runs when it is pressed. */ onAction: {} }}",
            <Callback<()> as Arg>::ts_type(types)
        )
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        if value.is_nullish() {
            return Err(value.expected("{ label, onAction }"));
        }
        Ok(Self {
            label: <String as JsValue>::from_js(value.field("label")?.get())?,
            on_action: <Callback<()> as Arg>::from_js(value.field("onAction")?.get())?,
        })
    }

    fn to_js<'a>(&self, _host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        Err(PropError("a toast's action is not sent to JS".into()))
    }
}

/// What `toast` shows.
#[derive(Debug, Clone, JsObject)]
pub struct ToastContent {
    /// The notice.
    pub title: String,
    /// Secondary text under the title.
    pub description: Option<String>,
    /// What it is about (default `info`).
    pub kind: Option<ToastKind>,
    /// A button that runs `onAction`, then dismisses the toast.
    pub action: Option<ToastAction>,
    /// Milliseconds it stays (default `TOAST_TIMEOUT`; an error stays
    /// until dismissed).
    pub duration: Option<f64>,
    /// Names it: a toast with the same id replaces it.
    pub id: Option<String>,
}

impl ToastContent {
    fn message(self) -> ToastMessage {
        let kind = self.kind.unwrap_or_default();
        let mut m = message(self.title).kind(kind.into());
        if let Some(d) = self.description {
            m = m.description(d);
        }
        if let Some(action) = self.action {
            let run = action.on_action;
            m = m.action(action.label, move || run.post(()));
        }
        if kind != ToastKind::Error
            && let Some(ms) = self.duration
        {
            m = m.timeout(Duration::from_secs_f64(ms.max(0.0) / 1000.0));
        }
        m
    }
}

thread_local! {
    /// The mounted `Toaster`'s store.
    static STORE: Cell<Option<Store>> = const { Cell::new(None) };
    /// The store's id of each toast, by the id JS knows.
    static IDS: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    /// Toasts shown before a `Toaster` was mounted.
    static PENDING: RefCell<Vec<(String, ToastMessage)>> = const { RefCell::new(Vec::new()) };
    static COUNT: Cell<u64> = const { Cell::new(0) };
}

fn show(store: Store, id: String, message: ToastMessage) {
    if let Some(old) = IDS.with_borrow(|ids| ids.get(&id).copied()) {
        store.dismiss(old);
    }
    let shown = store.show(message);
    IDS.with_borrow_mut(|ids| ids.insert(id, shown));
}

/// Where toasts show: the window's bottom right. Place it once, at the
/// root, after the content. The pointer on the stack expands it and pauses
/// every timeout; Escape dismisses the newest, a swipe right or its close
/// button one; screen readers read each as it shows.
#[component]
pub fn Toaster() -> impl IntoElement {
    let store = Store::new();
    STORE.set(Some(store));
    for (id, message) in PENDING.take() {
        show(store, id, message);
    }
    cleanup(move || {
        // The thread may be ending (the engine stopping).
        let _ = STORE.try_with(|s| s.set(None));
        let _ = IDS.try_with(|ids| ids.borrow_mut().clear());
    });
    store.view().into_element()
}

/// The toast functions: `toast({ title: "Saved" })` from anywhere shows a
/// notice in the app's `Toaster`.
#[module]
pub struct Toasts;

#[module(flat)]
impl Toasts {
    /// Milliseconds a notice stays on screen, unless it is an error or
    /// sets its own `duration`.
    pub const TOAST_TIMEOUT: u32 = 6000;

    /// Shows a notice in the app's `Toaster`: `toast({ title: "Saved" })`.
    /// Resolves with its id, for `dismissToast` (or name it with `id`).
    pub fn toast(content: ToastContent, _cx: &mut ModuleCx) -> String {
        let id = content.id.clone().unwrap_or_else(|| {
            let n = COUNT.get() + 1;
            COUNT.set(n);
            format!("toast-{n}")
        });
        let message = content.message();
        match STORE.get() {
            Some(store) => show(store, id.clone(), message),
            None => PENDING.with_borrow_mut(|p| p.push((id.clone(), message))),
        }
        id
    }

    /// Dismisses the toast `id`, or every toast without one.
    pub fn dismiss_toast(id: Option<String>, _cx: &mut ModuleCx) {
        let Some(store) = STORE.get() else {
            match id {
                Some(id) => PENDING.with_borrow_mut(|p| p.retain(|(i, _)| *i != id)),
                None => PENDING.with_borrow_mut(Vec::clear),
            }
            return;
        };
        match id {
            Some(id) => {
                if let Some(shown) = IDS.with_borrow_mut(|ids| ids.remove(&id)) {
                    store.dismiss(shown);
                }
            }
            None => {
                IDS.with_borrow_mut(HashMap::clear);
                store.dismiss_all();
            }
        }
    }

    /// Asks with the platform's own alert (drawn by the system, not
    /// styled): resolves `true` when the user chose `actionLabel`.
    pub async fn confirm(options: ConfirmOptions) -> bool {
        let cancel = options.cancel_label.unwrap_or_else(|| "Cancel".into());
        let mut alert = nsh_base::alert_dialog()
            .title(options.title)
            .cancel("cancel", cancel);
        if let Some(description) = options.description {
            alert = alert.description(description);
        }
        alert = if options.destructive {
            alert.destructive("confirm", options.action_label)
        } else {
            alert.action("confirm", options.action_label)
        };
        matches!(alert.show().await, Ok(r) if r.action.as_deref() == Some("confirm"))
    }
}

/// What `confirm` asks.
#[derive(Debug, Clone, JsObject)]
pub struct ConfirmOptions {
    /// The question.
    pub title: String,
    /// What follows from it.
    pub description: Option<String>,
    /// The confirming choice's label ("Restart").
    pub action_label: String,
    /// The cancelling choice's label (default "Cancel").
    pub cancel_label: Option<String>,
    /// The confirming choice destroys something: the platform marks it.
    #[prop(default)]
    pub destructive: bool,
}

/// Registers this area's components and the `Toasts` module for TSX.
pub(crate) fn register(registry: &mut Registry) {
    registry
        .module::<Toasts>()
        .register::<Dialog>()
        .register::<AlertDialog>()
        .register::<Sheet>()
        .register::<Popover>()
        .register::<Tooltip>()
        .register::<HoverCard>()
        .register::<Toaster>();
}
