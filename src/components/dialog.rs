//! Omarchy dialogs: a modal card over a scrim, square, 1px border, outline
//! actions.
//!
//! Built on `nsh_base::dialog()` (the core's native modal layer), so
//! Escape, the Android back button and screen readers' dialog role come
//! from NSH. This module adds the look, the backdrop press, the first
//! focus and the focus return to the control that opened it.
//!
//! ```ignore
//! let open = signal(false);
//! let trigger = element_ref();
//! view()
//!     .child(overlay_action_button("Rename", OverlayTone::Neutral, move || open.set(true)).element_ref(&trigger))
//!     .child(
//!         dialog(open, "Rename workspace")
//!             .description("The name shows in the bar and the switcher.")
//!             .cancel("Cancel")
//!             .action("Rename", OverlayTone::Primary, move || rename())
//!             .return_focus(trigger),
//!     )
//! ```
//!
//! For a yes-or-no the platform draws, [`confirm_native`] wraps
//! `nsh_base::alert_dialog()`.

use std::rc::Rc;

use nsh_base::button;
use nsh_core::element::AdaptiveColor;
use nsh_core::prelude::*;

use crate::tokens;

// ---- Shared overlay pieces (used by the other overlay modules too) --------

/// What an overlay action means; its edge and label carry it (Omarchy's
/// outline actions: no fills for priority).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OverlayTone {
    /// A regular action: foreground label, control border.
    #[default]
    Neutral,
    /// The action the overlay is for: accent edge and label.
    Primary,
    /// Destroys or deletes something: danger edge and label.
    Danger,
}

impl OverlayTone {
    fn label_color(self) -> AdaptiveColor {
        match self {
            OverlayTone::Neutral => tokens::foreground(),
            OverlayTone::Primary => tokens::accent(),
            OverlayTone::Danger => tokens::danger(),
        }
    }

    fn edge_color(self) -> AdaptiveColor {
        match self {
            OverlayTone::Neutral => tokens::control_border(),
            OverlayTone::Primary => tokens::accent(),
            OverlayTone::Danger => tokens::danger(),
        }
    }
}

/// An outline action button for overlay footers (dialogs, sheets, toasts):
/// transparent, 1px edge in the tone's color, foreground-tinted hover and
/// pressed washes, a visible focus border. Returns the `nsh_base` button,
/// so callers keep its builder API (`.element_ref`, `.disabled`, ...).
pub fn overlay_action_button(
    label: impl Into<SharedString>,
    tone: OverlayTone,
    on_press: impl Fn() + 'static,
) -> nsh_base::ButtonElement {
    let label: SharedString = label.into();
    button()
        .on_press(on_press)
        .accessibility_label(label.clone())
        .h(px(tokens::CONTROL_MD))
        .px(px(tokens::SPACE_3))
        .flex_row()
        .items_center()
        .justify_center()
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(move || tone.edge_color())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| s.border_color(move || tokens::focus_border()))
        .disabled_style(|s| s.border_color(move || tokens::divider()))
        .child(
            text(label)
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || tone.label_color()),
        )
}

/// A square, label-only icon button (`×`) that closes an overlay.
pub fn overlay_close_button(
    label: impl Into<SharedString>,
    on_press: impl Fn() + 'static,
) -> nsh_base::ButtonElement {
    button()
        .on_press(on_press)
        .accessibility_label(label.into())
        .w(px(tokens::CONTROL_SM))
        .h(px(tokens::CONTROL_SM))
        .items_center()
        .justify_center()
        .rounded(px(tokens::RADIUS))
        .border(px(tokens::BORDER))
        .border_color(move || AdaptiveColor::TRANSPARENT)
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .active(|s| s.bg(move || tokens::pressed_fill()))
        .focus_visible(|s| s.border_color(move || tokens::focus_border()))
        .child(super::icon::indicator(super::icon::IconName::X, 16.0))
}

/// The dimming behind modal overlays. A private token: the window
/// background, mostly opaque, so it follows light and dark themes.
pub(crate) fn overlay_scrim() -> AdaptiveColor {
    tokens::scrim()
}

/// Focuses `target` each time `open` goes from true to false: the focus
/// returns to the control that opened the overlay.
pub(crate) fn overlay_focus_return(open: Signal<bool>, target: Option<ElementRef>) {
    let Some(target) = target else { return };
    let was = stored(open.get_untracked());
    effect(move || {
        let now = open.get();
        if was.replace(now) && !now {
            target.focus();
        }
    });
}

/// A 0→1 value that runs once, over `ms`, as the element it is bound to
/// mounts: opacity and a short rise. At once under reduced motion.
pub(crate) fn overlay_enter(ms: f32) -> AnimatedValue {
    let progress = animated_value(0.0);
    if reduce_motion() {
        progress.set(1.0);
    } else {
        let _ = progress.timing(1.0, ms, Easing::EaseOut);
    }
    progress
}

// ---- Dialog ---------------------------------------------------------------

struct DialogAction {
    label: SharedString,
    tone: OverlayTone,
    run: Rc<dyn Fn()>,
    /// Takes the focus as the dialog opens.
    initial: bool,
    /// Only closes: reported as a dismissal.
    cancel: bool,
}

/// A modal dialog built by [`dialog`] or [`alert_dialog`]: a title, an
/// optional description and body, and a row of outline actions. It is in
/// the view while its `open` signal is true; Escape, the back button, a
/// backdrop press (not for an alert dialog) and every action set it false.
pub struct Dialog {
    open: Signal<bool>,
    title: SharedString,
    description: Option<SharedString>,
    body: Option<Rc<dyn Fn() -> AnyElement>>,
    actions: Vec<DialogAction>,
    return_focus: Option<ElementRef>,
    alert: bool,
    width: f32,
    on_dismiss: Option<Rc<dyn Fn()>>,
    /// `None`: a plain dialog's backdrop press dismisses it, an alert
    /// dialog's does not.
    backdrop_dismisses: Option<bool>,
    escape_dismisses: bool,
}

/// A dialog titled `title`, shown while `open` is true. The dialog writes
/// `open` false when the user dismisses it or picks an action.
pub fn dialog(open: Signal<bool>, title: impl Into<SharedString>) -> Dialog {
    Dialog {
        open,
        title: title.into(),
        description: None,
        body: None,
        actions: Vec::new(),
        return_focus: None,
        alert: false,
        width: 420.0,
        on_dismiss: None,
        backdrop_dismisses: None,
        escape_dismisses: true,
    }
}

/// A dialog that asks for a decision (a destructive confirm): announced as
/// an alert dialog, and a backdrop press does not dismiss it (Escape
/// still cancels). Its [`Dialog::cancel`] action takes the first focus, so
/// Enter never confirms by accident.
pub fn alert_dialog(open: Signal<bool>, title: impl Into<SharedString>) -> Dialog {
    Dialog {
        alert: true,
        width: 380.0,
        ..dialog(open, title)
    }
}

impl Dialog {
    /// Secondary text under the title.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Content between the description and the actions, made on each
    /// opening (state that must survive closing belongs to the caller).
    pub fn body<E: IntoElement>(mut self, body: impl Fn() -> E + 'static) -> Self {
        self.body = Some(Rc::new(move || body().into_element()));
        self
    }

    /// An action: closes the dialog, then runs `on_press`. In a plain
    /// dialog the first [`OverlayTone::Primary`] action takes the first
    /// focus.
    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        tone: OverlayTone,
        on_press: impl Fn() + 'static,
    ) -> Self {
        self.actions.push(DialogAction {
            label: label.into(),
            tone,
            run: Rc::new(on_press),
            initial: false,
            cancel: false,
        });
        self
    }

    /// A neutral action that only closes the dialog (reported to
    /// [`Dialog::on_dismiss`]). In an alert dialog it takes the first
    /// focus.
    pub fn cancel(mut self, label: impl Into<SharedString>) -> Self {
        self.actions.push(DialogAction {
            label: label.into(),
            tone: OverlayTone::Neutral,
            run: Rc::new(|| {}),
            initial: self.alert,
            cancel: true,
        });
        self
    }

    /// Where the focus goes when the dialog closes: the control that
    /// opened it (give it `.element_ref(&target)`).
    pub fn return_focus(mut self, target: ElementRef) -> Self {
        self.return_focus = Some(target);
        self
    }

    /// The card's width in points (default 420; 380 for an alert dialog).
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Runs when the user dismisses it without an action: Escape, back, a
    /// backdrop press or a [`Dialog::cancel`] action.
    pub fn on_dismiss(mut self, on_dismiss: impl Fn() + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(on_dismiss));
        self
    }

    /// Whether a press on the backdrop dismisses it (default: yes for a
    /// plain dialog, no for an alert dialog).
    pub fn close_on_backdrop_press(mut self, close: bool) -> Self {
        self.backdrop_dismisses = Some(close);
        self
    }

    /// Whether Escape and the back button dismiss it (default true).
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.escape_dismisses = close;
        self
    }
}

impl IntoElement for Dialog {
    fn into_element(self) -> AnyElement {
        let Dialog {
            open,
            title,
            description,
            body,
            actions,
            return_focus,
            alert,
            width,
            on_dismiss,
            backdrop_dismisses,
            escape_dismisses,
        } = self;
        let backdrop_dismisses = backdrop_dismisses.unwrap_or(!alert);
        overlay_focus_return(open, return_focus);
        let actions = Rc::new(actions);
        let report = Rc::new(move || {
            if let Some(f) = &on_dismiss {
                f();
            }
        });
        let dismiss = {
            let report = report.clone();
            Rc::new(move || {
                if open.get_untracked() {
                    open.set(false);
                    report();
                }
            })
        };
        when(open, move || {
            let enter = overlay_enter(tokens::DURATION_BASE);
            let rise = enter.interpolate(&[0.0, 1.0], &[tokens::ENTER_DISTANCE, 0.0]);
            // The action that takes the first focus: a marked one, else
            // (not in an alert dialog) the first primary one, else the first.
            let initial = actions
                .iter()
                .position(|a| a.initial)
                .or_else(|| {
                    actions
                        .iter()
                        .position(|a| !alert && a.tone == OverlayTone::Primary)
                })
                .or_else(|| (!actions.is_empty()).then_some(0));
            let first = element_ref();
            let buttons = actions
                .iter()
                .enumerate()
                .map(|(index, action)| {
                    let run = action.run.clone();
                    let cancel = action.cancel;
                    let report = report.clone();
                    let b = overlay_action_button(action.label.clone(), action.tone, move || {
                        // Closed first: the action may open another overlay.
                        open.set(false);
                        if cancel {
                            report();
                        }
                        run();
                    });
                    if Some(index) == initial {
                        b.element_ref(&first).into_element()
                    } else {
                        b.into_element()
                    }
                })
                .collect::<Vec<_>>();
            let backdrop = view()
                .absolute()
                .inset_0()
                .bg(move || overlay_scrim())
                .focusable(false)
                .bind_opacity(&enter);
            let backdrop = if backdrop_dismisses {
                let dismiss = dismiss.clone();
                backdrop.on_press(move || dismiss())
            } else {
                backdrop
            };
            let card = view()
                .w(px(width))
                .max_w(relative(1.0))
                .gap(px(tokens::SPACE_3))
                .p(px(tokens::SPACE_4))
                .rounded(px(tokens::RADIUS))
                .border(px(tokens::BORDER))
                .border_color(move || tokens::border())
                .bg(move || tokens::surface())
                .text_color(move || tokens::foreground())
                .text_size(px(tokens::TEXT_MD))
                .bind_opacity(&enter)
                .bind_translate_y(&rise)
                .child(
                    text(title.clone())
                        .text_size(px(tokens::TEXT_LG))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(move || tokens::bright()),
                )
                .child(description.clone().map(|d| {
                    text(d)
                        .text_size(px(tokens::TEXT_MD))
                        .text_color(move || tokens::secondary())
                }))
                .child(body.as_ref().map(|b| b()))
                .child((!buttons.is_empty()).then(|| {
                    view()
                        .flex_row()
                        .justify_end()
                        .gap(px(tokens::SPACE_2))
                        .pt(px(tokens::SPACE_1))
                        .children(buttons)
                }));
            let dismiss = dismiss.clone();
            let mut modal = nsh_base::dialog()
                .animation(ModalAnimation::None)
                .label(title.clone())
                .on_cancel(move || {
                    if escape_dismisses {
                        dismiss()
                    }
                })
                .on_open(move || first.focus())
                .flex_1()
                .items_center()
                .justify_center()
                .p(px(tokens::SPACE_6));
            if alert {
                modal = modal.role(AccessibilityRole::AlertDialog);
            }
            if let Some(d) = description.clone() {
                modal = modal.accessibility_description(d);
            }
            modal.child(backdrop).child(card)
        })
        .into_element()
    }
}

/// The platform's own confirm (`nsh_base::alert_dialog()`): resolves true
/// when the user picks the destructive action. Run it in a task:
/// `spawn(async move { if confirm_native(..).await { .. } })`.
pub async fn confirm_native(
    title: impl Into<SharedString>,
    description: impl Into<SharedString>,
    confirm_label: impl Into<SharedString>,
) -> bool {
    let answer = nsh_base::alert_dialog()
        .title(title)
        .description(description)
        .cancel("cancel", "Cancel")
        .destructive("confirm", confirm_label)
        .show()
        .await;
    matches!(answer, Ok(r) if r.action.as_deref() == Some("confirm"))
}
