//! [`Toaster`], the [`Toast`] notices it shows, and [`ConfirmOptions`] for
//! the platform's confirm. TSX calls them through the flat [`Toasts`]
//! module: `toast({..})`, `dismissToast(id)`, `await confirm({..})`.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use nsh_core::element::{Arg, JsHost, JsIn, JsKind, JsOwned, JsValue, PropError, TsTypes};
use nsh_core::prelude::*;

use crate::Tone;
use crate::components::toast::{
    ToastKind as Kind, ToastMessage, Toaster as Store, toast as message,
};

// ---- Toast ----------------------------------------------------------------

/// What a toast is about: its icon and color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PropEnum)]
#[non_exhaustive]
pub enum ToastKind {
    /// News (the default).
    #[default]
    Info,
    /// Something worked.
    Success,
    /// Something needs attention.
    Warning,
    /// Something failed; it stays until dismissed unless it sets a
    /// timeout.
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

/// A notice for [`toast`](crate::toast()):
/// `Toast::new("Settings saved").description("Restart to apply").success()`.
/// In TSX, the object `toast({ title, description?, kind?, action?, timeout?, id? })`
/// takes.
#[derive(Clone)]
pub struct Toast {
    title: SharedString,
    description: Option<SharedString>,
    kind: ToastKind,
    action: Option<(SharedString, Rc<dyn Fn()>)>,
    /// `None`: the default ([`TOAST_TIMEOUT`](crate::TOAST_TIMEOUT); an
    /// error stays).
    timeout: Option<Option<Duration>>,
    id: Option<SharedString>,
}

impl fmt::Debug for Toast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Toast")
            .field("title", &self.title)
            .field("description", &self.description)
            .field("kind", &self.kind)
            .field("action", &self.action.as_ref().map(|(label, _)| label))
            .field("timeout", &self.timeout)
            .field("id", &self.id)
            .finish()
    }
}

impl Toast {
    /// A notice titled `title`: an info that leaves after
    /// [`TOAST_TIMEOUT`](crate::TOAST_TIMEOUT).
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            kind: ToastKind::Info,
            action: None,
            timeout: None,
            id: None,
        }
    }

    /// Secondary text under the title.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// What it is about: its icon and color (default
    /// [`ToastKind::Info`]).
    pub fn kind(mut self, kind: ToastKind) -> Self {
        self.kind = kind;
        self
    }

    /// News: `.kind(ToastKind::Info)`.
    pub fn info(self) -> Self {
        self.kind(ToastKind::Info)
    }

    /// Something worked: `.kind(ToastKind::Success)`.
    pub fn success(self) -> Self {
        self.kind(ToastKind::Success)
    }

    /// Something needs attention: `.kind(ToastKind::Warning)`.
    pub fn warning(self) -> Self {
        self.kind(ToastKind::Warning)
    }

    /// Something failed: `.kind(ToastKind::Error)`.
    pub fn error(self) -> Self {
        self.kind(ToastKind::Error)
    }

    /// A button (`"Undo"`) that runs `on_action`, then dismisses the toast.
    pub fn action(mut self, label: impl Into<SharedString>, on_action: impl Fn() + 'static) -> Self {
        self.action = Some((label.into(), Rc::new(on_action)));
        self
    }

    /// How long it stays: `Some(duration)`, or `None` until dismissed
    /// (default [`TOAST_TIMEOUT`](crate::TOAST_TIMEOUT); an error stays).
    pub fn timeout(mut self, timeout: impl Into<Option<Duration>>) -> Self {
        self.timeout = Some(timeout.into());
        self
    }

    /// Names it: toasting an id that is shown replaces that toast (default:
    /// a new id).
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }

    fn message(self) -> ToastMessage {
        let mut m = message(self.title).kind(self.kind.into());
        if let Some(d) = self.description {
            m = m.description(d);
        }
        if let Some((label, run)) = self.action {
            m = m.action(label, move || run());
        }
        match self.timeout {
            Some(Some(timeout)) => m.timeout(timeout),
            Some(None) => m.persistent(),
            None => m,
        }
    }
}

/// Field `key` of `value`: `None` when it is `undefined` or `null`.
fn field<T: JsValue>(value: JsIn<'_>, key: &str) -> Result<Option<T>, PropError> {
    let owned = value.field(key)?;
    let field = owned.get();
    if field.is_nullish() {
        return Ok(None);
    }
    T::from_js(field)
        .map(Some)
        .map_err(|e| PropError(format!("`{key}`: {e}")))
}

impl JsValue for Toast {
    fn ts_type(types: &mut TsTypes) -> String {
        types.declare("Toast", |types| {
            let kind = <ToastKind as JsValue>::ts_type(types);
            let on_action = <Callback<()> as Arg>::ts_type(types);
            format!(
                "/** A notice for `toast`. */\nexport interface Toast {{\n  \
                 /** The notice (\"Settings saved\"). */\n  title: string;\n  \
                 /** Secondary text under the title. */\n  description?: string;\n  \
                 /** What it is about: its icon and color (default `info`). */\n  kind?: {kind};\n  \
                 /** A button that runs `onAction`, then dismisses the toast. */\n  \
                 action?: {{ /** The button's label (\"Undo\"). */ label: string; /** Runs when it is pressed. */ onAction: {on_action} }};\n  \
                 /** Milliseconds it stays (default `TOAST_TIMEOUT`; an error stays); `null` stays until dismissed. */\n  \
                 timeout?: number | null;\n  \
                 /** Names it: toasting an id that is shown replaces that toast. */\n  id?: string;\n}}"
            )
        })
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        if value.kind() != JsKind::Object {
            return Err(value.expected("a toast: { title, .. }"));
        }
        let title: String = field(value, "title")?.ok_or_else(|| PropError("`title` is missing".into()))?;
        let mut toast = Toast::new(title);
        if let Some(description) = field::<String>(value, "description")? {
            toast = toast.description(description);
        }
        if let Some(kind) = field::<ToastKind>(value, "kind")? {
            toast = toast.kind(kind);
        }
        let action = value.field("action")?;
        let action = action.get();
        if !action.is_nullish() {
            let label: String =
                field(action, "label")?.ok_or_else(|| PropError("`action.label` is missing".into()))?;
            let run: Callback<()> = <Callback<()> as Arg>::from_js(action.field("onAction")?.get())?;
            toast = toast.action(label, move || run.post(()));
        }
        let timeout = value.field("timeout")?;
        let timeout = timeout.get();
        match timeout.kind() {
            JsKind::Undefined => {}
            JsKind::Null => toast = toast.timeout(None),
            _ => {
                let ms = timeout
                    .as_number()
                    .ok_or_else(|| timeout.expected("milliseconds or null"))?;
                toast = toast.timeout(Duration::from_secs_f64(ms.max(0.0) / 1000.0));
            }
        }
        if let Some(id) = field::<SharedString>(value, "id")? {
            toast = toast.id(id);
        }
        Ok(toast)
    }

    fn to_js<'a>(&self, _host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        Err(PropError("a toast is not sent to JS".into()))
    }
}

// ---- The store ------------------------------------------------------------

thread_local! {
    /// The mounted `Toaster`'s store.
    static STORE: Cell<Option<Store>> = const { Cell::new(None) };
    /// The store's id of each toast, by the app's id.
    static IDS: RefCell<HashMap<SharedString, u64>> = RefCell::new(HashMap::new());
    /// Toasts shown before a `Toaster` was mounted.
    static PENDING: RefCell<Vec<(SharedString, ToastMessage)>> = const { RefCell::new(Vec::new()) };
    /// The last generated id's number.
    static COUNT: Cell<u64> = const { Cell::new(0) };
}

fn place(store: Store, id: SharedString, message: ToastMessage) {
    if let Some(old) = IDS.with_borrow(|ids| ids.get(&id).copied()) {
        store.dismiss(old);
    }
    let shown = store.show(message);
    IDS.with_borrow_mut(|ids| ids.insert(id, shown));
}

/// A new toast id (no signal: a JS call may take one outside the
/// reactive runtime).
fn next_id() -> SharedString {
    let n = COUNT.get() + 1;
    COUNT.set(n);
    format!("toast-{n}").into()
}

/// Shows `toast` in the app's [`Toaster`] (or once one mounts): its id.
pub fn show_toast(toast: Toast) -> SharedString {
    let id = toast.id.clone().unwrap_or_else(next_id);
    let message = toast.message();
    match STORE.get() {
        Some(store) => place(store, id.clone(), message),
        None => PENDING.with_borrow_mut(|p| {
            p.retain(|(i, _)| *i != id);
            p.push((id.clone(), message));
        }),
    }
    id
}

/// Dismisses the toast `id` (nothing when it is not shown).
pub fn dismiss_toast(id: &str) {
    match STORE.get() {
        Some(store) => {
            if let Some(shown) = IDS.with_borrow_mut(|ids| ids.remove(id)) {
                store.dismiss(shown);
            }
        }
        None => PENDING.with_borrow_mut(|p| p.retain(|(i, _)| &**i != id)),
    }
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
        place(store, id, message);
    }
    cleanup(move || {
        // The thread may be ending (the engine stopping).
        let _ = STORE.try_with(|s| s.set(None));
        let _ = IDS.try_with(|ids| ids.borrow_mut().clear());
    });
    store.view().into_element()
}

// ---- Confirm --------------------------------------------------------------

/// What [`confirm`](crate::confirm()) asks:
/// `ConfirmOptions::new("Delete 3 files?").action_label("Delete").danger()`.
/// In TSX, the object `confirm({ title, description?, actionLabel?, cancelLabel?, tone? })`
/// takes.
#[derive(Debug, Clone, PartialEq, JsObject)]
pub struct ConfirmOptions {
    /// The question ("Delete 3 files?").
    title: SharedString,
    /// What follows from it.
    description: Option<SharedString>,
    /// The confirming choice's label (default "OK").
    action_label: Option<SharedString>,
    /// The cancelling choice's label (default "Cancel").
    cancel_label: Option<SharedString>,
    /// `danger` when the confirming choice destroys something: the
    /// platform marks it (default `accent`).
    #[prop(default)]
    tone: Tone,
}

impl ConfirmOptions {
    /// Asks `title`, with OK and Cancel.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            action_label: None,
            cancel_label: None,
            tone: Tone::Accent,
        }
    }

    /// What follows from it.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// The confirming choice's label (default "OK").
    pub fn action_label(mut self, label: impl Into<SharedString>) -> Self {
        self.action_label = Some(label.into());
        self
    }

    /// The cancelling choice's label (default "Cancel").
    pub fn cancel_label(mut self, label: impl Into<SharedString>) -> Self {
        self.cancel_label = Some(label.into());
        self
    }

    /// The confirming choice's tone; `Tone::Danger` marks it destructive.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    /// The confirming choice destroys something: `.tone(Tone::Danger)`.
    pub fn danger(self) -> Self {
        self.tone(Tone::Danger)
    }
}

/// Asks with the platform's own alert: true when the user chose the
/// action.
pub async fn confirm(options: ConfirmOptions) -> bool {
    let ConfirmOptions {
        title,
        description,
        action_label,
        cancel_label,
        tone,
    } = options;
    let mut alert = nsh_base::alert_dialog()
        .title(title)
        .cancel("cancel", cancel_label.unwrap_or_else(|| "Cancel".into()));
    if let Some(description) = description {
        alert = alert.description(description);
    }
    let action = action_label.unwrap_or_else(|| "OK".into());
    alert = if tone == Tone::Danger {
        alert.destructive("confirm", action)
    } else {
        alert.action("confirm", action)
    };
    matches!(alert.show().await, Ok(r) if r.action.as_deref() == Some("confirm"))
}

// ---- The TSX functions ----------------------------------------------------

/// The toast and confirm functions, imported by name:
/// `import { toast, dismissToast, confirm } from "nsh-omarchy"`.
#[module]
pub struct Toasts;

#[module(flat)]
impl Toasts {
    /// Milliseconds a toast stays unless it sets its own `timeout` (an
    /// error stays until dismissed).
    pub const TOAST_TIMEOUT: u32 = 6000;

    /// Shows a notice in the app's `Toaster`:
    /// `const id = toast({ title: "Saved", kind: "success" })`. Returns its
    /// id, for `dismissToast`.
    // A JS call runs outside the reactive runtime: the id is taken at
    // once, the store is written in a task of the core, in order.
    pub fn toast(toast: Toast, cx: CallCx) -> SharedString {
        let id = toast.id.clone().unwrap_or_else(next_id);
        let toast = toast.id(id.clone());
        cx.start(async move {
            show_toast(toast);
        });
        id
    }

    /// Dismisses the toast `id` (nothing when it is not shown).
    pub fn dismiss_toast(id: SharedString, cx: CallCx) {
        cx.start(async move { dismiss_toast(&id) });
    }

    /// Asks with the platform's own alert:
    /// `await confirm({ title: "Delete 3 files?", actionLabel: "Delete", tone: "danger" })`
    /// is true when the user chose the action.
    pub async fn confirm(options: ConfirmOptions) -> bool {
        confirm(options).await
    }
}
