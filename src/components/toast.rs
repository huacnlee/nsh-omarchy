//! Omarchy toasts: brief notices stacked at the bottom right of the window.
//!
//! A [`Toaster`] is the app-level store: made once (in the root
//! component, with [`toaster`], which also provides it to the tree), it
//! keeps `nsh_base`'s `ToastManager` and advances it while toasts are
//! mounted. Its [`Toaster::view`] is the stack, an overlay at the window's
//! bottom right drawn with `nsh_base::toast_stack()`. Anything below the
//! root shows a notice with [`use_toaster`] and [`toast`]:
//!
//! ```ignore
//! if let Some(toaster) = use_toaster() {
//!     toaster.show(toast("Window closed").action("Undo", move || reopen()));
//! }
//! ```
//!
//! Behavior: a notice expires after [`TOAST_TIMEOUT`] (6 s); an
//! [`ToastKind::Error`] stays until dismissed. The pointer on the stack
//! expands it and pauses every timeout. Escape dismisses the newest notice,
//! a swipe right or its close button dismisses one, and an action runs then
//! dismisses its notice. Each notice is read by screen readers as it shows.

use std::rc::Rc;
use std::time::Duration;

use nsh_base::{
    Anchor, ToastManager, ToastMotion, ToastOptions, ToastStackState, ToastTransitionStatus,
    toast_stack,
};
use nsh_core::element::AdaptiveColor;
use nsh_core::prelude::*;
use nsh_core::time::now;

use super::dialog::{OverlayTone, overlay_action_button, overlay_close_button};
use crate::tokens;

/// How long a notice stays, unless it is an error or sets its own.
pub const TOAST_TIMEOUT: Duration = Duration::from_secs(6);
/// How many notices show at once (older ones wait under them).
const TOAST_VISIBLE: usize = 3;
/// The stack's width.
const TOAST_WIDTH: f32 = 340.0;
/// How often the lifecycle advances while notices are mounted.
const TOAST_TICK: Duration = Duration::from_millis(50);

/// What a notice reports; its marker bar takes the matching color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastKind {
    #[default]
    Info,
    Success,
    Warning,
    /// Stays until dismissed.
    Error,
}

impl ToastKind {
    fn color(self) -> AdaptiveColor {
        match self {
            ToastKind::Info => tokens::accent(),
            ToastKind::Success => tokens::success(),
            ToastKind::Warning => tokens::warning(),
            ToastKind::Error => tokens::danger(),
        }
    }
}

/// A notice's action button (one per notice).
#[derive(Clone)]
struct ToastAction {
    label: SharedString,
    run: Rc<dyn Fn()>,
}

impl PartialEq for ToastAction {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && Rc::ptr_eq(&self.run, &other.run)
    }
}

/// A notice for [`Toaster::show`], made by [`toast`].
#[derive(Clone, PartialEq)]
pub struct ToastMessage {
    title: SharedString,
    description: Option<SharedString>,
    kind: ToastKind,
    action: Option<ToastAction>,
    /// `None`: the kind's default.
    timeout: Option<Option<Duration>>,
}

/// A notice titled `title`.
pub fn toast(title: impl Into<SharedString>) -> ToastMessage {
    ToastMessage {
        title: title.into(),
        description: None,
        kind: ToastKind::Info,
        action: None,
        timeout: None,
    }
}

impl ToastMessage {
    /// Secondary text under the title.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// What it reports (default [`ToastKind::Info`]).
    pub fn kind(mut self, kind: ToastKind) -> Self {
        self.kind = kind;
        self
    }

    /// An action button (`Undo`): runs `on_press`, then dismisses the
    /// notice.
    pub fn action(mut self, label: impl Into<SharedString>, on_press: impl Fn() + 'static) -> Self {
        self.action = Some(ToastAction {
            label: label.into(),
            run: Rc::new(on_press),
        });
        self
    }

    /// How long it stays (default [`TOAST_TIMEOUT`]; errors stay).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(Some(timeout));
        self
    }

    /// It stays until dismissed.
    pub fn persistent(mut self) -> Self {
        self.timeout = Some(None);
        self
    }

    fn resolved_timeout(&self) -> Option<Duration> {
        self.timeout.unwrap_or(match self.kind {
            ToastKind::Error => None,
            _ => Some(TOAST_TIMEOUT),
        })
    }
}

type Shown = (u64, ToastMessage, ToastTransitionStatus);

/// The app's notices: storage, lifecycle and the stack that draws them. A
/// `Copy` handle; make it once with [`toaster`] (or [`Toaster::new`]).
#[derive(Clone, Copy)]
pub struct Toaster {
    manager: Stored<ToastManager<u64, ToastMessage>>,
    shown: Signal<Vec<Shown>>,
    next: Stored<u64>,
    ticking: Stored<bool>,
    state: ToastStackState,
    owner: Owner,
}

/// A toaster of the current owner, provided to everything below it
/// ([`use_toaster`]). Place its [`Toaster::view`] once.
pub fn toaster() -> Toaster {
    let toaster = Toaster::new();
    provide_context(toaster);
    toaster
}

/// The toaster an ancestor provided with [`toaster`].
pub fn use_toaster() -> Option<Toaster> {
    use_context::<Toaster>()
}

fn toast_motion() -> ToastMotion {
    ToastMotion {
        duration: Duration::from_millis(tokens::DURATION_BASE as u64),
        exit_duration: Duration::from_millis(tokens::DURATION_BASE as u64),
        collapsed_peek: px(tokens::SPACE_2),
        expanded_gap: px(tokens::SPACE_2),
        ..ToastMotion::sonner()
    }
}

impl Default for Toaster {
    fn default() -> Self {
        Self::new()
    }
}

impl Toaster {
    /// A toaster of the current owner (not provided to the tree).
    pub fn new() -> Self {
        Self {
            manager: stored(ToastManager::new(toast_motion())),
            shown: signal(Vec::new()),
            next: stored(0),
            ticking: stored(false),
            state: ToastStackState::new(),
            owner: Owner::current(),
        }
    }

    /// Shows `message`, newest at the bottom; its id.
    pub fn show(&self, message: ToastMessage) -> u64 {
        let id = self.next.with(|n| {
            *n += 1;
            *n
        });
        let options = ToastOptions::timeout(message.resolved_timeout());
        self.manager.with(|m| m.push(id, message, options, now()));
        self.sync();
        self.tick_while_mounted();
        id
    }

    /// Dismisses notice `id` (it runs its exit).
    pub fn dismiss(&self, id: u64) {
        if self.manager.with(|m| m.dismiss(&id, now())) {
            self.sync();
        }
    }

    /// Dismisses every notice.
    pub fn dismiss_all(&self) {
        if !self.manager.with(|m| m.dismiss_all(now())).is_empty() {
            self.sync();
        }
    }

    /// The notices shown, ending ones included (tracked).
    pub fn len(&self) -> usize {
        self.shown.with(|v| v.len())
    }

    /// No notice is shown (tracked).
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn dismiss_newest(&self) {
        let newest = self.manager.with(|m| {
            m.iter()
                .rev()
                .find(|(_, _, status)| *status != ToastTransitionStatus::Ending)
                .map(|(id, _, _)| *id)
        });
        if let Some(id) = newest {
            self.dismiss(id);
        }
    }

    fn sync(&self) {
        let shown = self.manager.with(|m| {
            m.visible(TOAST_VISIBLE)
                .map(|(id, message, status)| (*id, message.clone(), status))
                .collect::<Vec<_>>()
        });
        self.shown.set(shown);
    }

    /// Advances the lifecycle every tick while notices are mounted, paused
    /// while the stack is expanded (the pointer on it).
    fn tick_while_mounted(&self) {
        if self.ticking.replace(true) {
            return;
        }
        let this = *self;
        self.owner.run(move || {
            let _ = spawn(async move {
                loop {
                    timer(TOAST_TICK).await;
                    let paused = untracked(|| this.state.is_expanded());
                    let advanced = this.manager.with(|m| m.advance(now(), paused));
                    if advanced.changed {
                        this.sync();
                    }
                    if this.manager.with(|m| m.is_empty()) {
                        this.ticking.replace(false);
                        break;
                    }
                }
            });
        });
    }

    /// The stack: an overlay at the window's bottom right. Place it once.
    pub fn view(&self) -> impl IntoElement {
        let this = *self;
        overlay()
            .right(px(tokens::SPACE_4))
            .bottom(px(tokens::SPACE_4))
            .w(px(TOAST_WIDTH))
            .max_w(relative(1.0))
            .child(
                toast_stack()
                    .placement(Anchor::BottomRight)
                    .motion(toast_motion())
                    .state(self.state)
                    .on_cancel(move || this.dismiss_newest())
                    .w_full()
                    .child(each(
                        self.shown,
                        |(id, _, _): &Shown| *id,
                        move |item, _| notice_view(this, item),
                    )),
            )
    }
}

fn notice_view(toaster: Toaster, item: ReadSignal<Shown>) -> impl IntoElement {
    let (id, message, _) = item.get_untracked();
    let ToastMessage {
        title,
        description,
        kind,
        action,
        ..
    } = message;
    let spoken: SharedString = match &description {
        Some(d) => format!("{title}. {d}").into(),
        None => title.clone(),
    };
    let action_button = action.map(|a| {
        let run = a.run.clone();
        overlay_action_button(a.label.clone(), OverlayTone::Primary, move || {
            run();
            toaster.dismiss(id);
        })
        .h(px(tokens::CONTROL_SM))
        .px(px(tokens::SPACE_2))
    });
    nsh_base::toast()
        .transition_status(move || item.get().2)
        .accessibility_label(Some(spoken))
        .on_dismiss_request(move || toaster.dismiss(id))
        .flex_row()
        .w_full()
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(move || tokens::border())
        .bg(move || tokens::surface())
        .text_color(move || tokens::foreground())
        .transition_all()
        .duration(tokens::DURATION_BASE)
        .starting_class("opacity-0 translate-y-2")
        .ending_class("opacity-0 translate-x-4")
        .swiping_class("transition-none")
        // The marker: a 2px bar in the kind's color.
        .child(view().w(px(2.0)).bg(move || kind.color()))
        .child(
            view()
                .flex_1()
                .min_w(px(0.0))
                .gap(px(tokens::SPACE_1))
                .py(px(tokens::SPACE_3))
                .pl(px(tokens::SPACE_3))
                .child(
                    text(title)
                        .text_size(px(tokens::TEXT_MD))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(move || tokens::bright()),
                )
                .child(description.map(|d| {
                    text(d)
                        .text_size(px(tokens::TEXT_SM))
                        .text_color(move || tokens::secondary())
                }))
                .child(action_button.map(|b| view().flex_row().pt(px(tokens::SPACE_1)).child(b))),
        )
        .child(
            view()
                .p(px(tokens::SPACE_1))
                .child(overlay_close_button("Dismiss notification", move || {
                    toaster.dismiss(id)
                })),
        )
}
