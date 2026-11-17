//! Omarchy sheets: a panel at an edge of the window, over a scrim.
//!
//! [`sheet`] builds on `nsh_base::sheet()`, which owns the slide in and out
//! (at once under reduced motion), the backdrop press and Escape. This
//! module adds the panel's look (surface, a 1px border on its inner edge,
//! square), a header with the title and a close button, outline footer
//! actions, the first focus and the focus return.
//!
//! ```ignore
//! let open = signal(false);
//! sheet(open, "Display")
//!     .description("Applies to every monitor.")
//!     .body(|| text("Scale 1.25×"))
//!     .action("Apply", OverlayTone::Primary, || apply())
//! ```

use std::rc::Rc;

use nsh_base::SheetEvent;
use nsh_core::prelude::*;

use super::dialog::{
    OverlayTone, overlay_action_button, overlay_close_button, overlay_focus_return, overlay_scrim,
};
use crate::tokens;

struct SheetAction {
    label: SharedString,
    tone: OverlayTone,
    run: Rc<dyn Fn()>,
}

/// A sheet built by [`sheet`]: shown while its `open` signal is true; the
/// close button, Escape, a backdrop press and every action set it false.
pub struct Sheet {
    open: Signal<bool>,
    title: SharedString,
    description: Option<SharedString>,
    body: Option<Rc<dyn Fn() -> AnyElement>>,
    actions: Vec<SheetAction>,
    side: Placement,
    size: f32,
    return_focus: Option<ElementRef>,
    on_dismiss: Option<Rc<dyn Fn()>>,
}

/// A sheet titled `title` on the right edge, shown while `open` is true.
pub fn sheet(open: Signal<bool>, title: impl Into<SharedString>) -> Sheet {
    Sheet {
        open,
        title: title.into(),
        description: None,
        body: None,
        actions: Vec::new(),
        side: Placement::Right,
        size: 360.0,
        return_focus: None,
        on_dismiss: None,
    }
}

impl Sheet {
    /// The edge it is on (default right).
    pub fn side(mut self, side: Placement) -> Self {
        self.side = side;
        self
    }

    /// Its width at the left and right edges, its height at the top and
    /// bottom, in points (default 360).
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Secondary text under the title.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// The content, made on each opening.
    pub fn body<E: IntoElement>(mut self, body: impl Fn() -> E + 'static) -> Self {
        self.body = Some(Rc::new(move || body().into_element()));
        self
    }

    /// A footer action: closes the sheet, then runs `on_press`.
    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        tone: OverlayTone,
        on_press: impl Fn() + 'static,
    ) -> Self {
        self.actions.push(SheetAction {
            label: label.into(),
            tone,
            run: Rc::new(on_press),
        });
        self
    }

    /// Where the focus goes when the sheet closes.
    pub fn return_focus(mut self, target: ElementRef) -> Self {
        self.return_focus = Some(target);
        self
    }

    /// Runs when the user dismisses it without an action (the close
    /// button, Escape, a backdrop press).
    pub fn on_dismiss(mut self, on_dismiss: impl Fn() + 'static) -> Self {
        self.on_dismiss = Some(Rc::new(on_dismiss));
        self
    }
}

impl IntoElement for Sheet {
    fn into_element(self) -> AnyElement {
        let Sheet {
            open,
            title,
            description,
            body,
            actions,
            side,
            size,
            return_focus,
            on_dismiss,
        } = self;
        overlay_focus_return(open, return_focus);
        let actions = Rc::new(actions);
        let close_ref = element_ref();
        let dismiss = Rc::new(move || {
            if open.get_untracked() {
                open.set(false);
                if let Some(f) = &on_dismiss {
                    f();
                }
            }
        });
        let dismiss_event = dismiss.clone();
        let label = title.clone();
        let content = move || {
            let dismiss = dismiss.clone();
            let header = view()
                .flex_row()
                .items_start()
                .gap(px(tokens::SPACE_2))
                .child(
                    view()
                        .flex_1()
                        .min_w(px(0.0))
                        .gap(px(tokens::SPACE_1))
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
                        })),
                )
                .child(overlay_close_button("Close", move || dismiss()).element_ref(&close_ref));
            let buttons = actions
                .iter()
                .map(|action| {
                    let run = action.run.clone();
                    overlay_action_button(action.label.clone(), action.tone, move || {
                        open.set(false);
                        run();
                    })
                    .into_element()
                })
                .collect::<Vec<_>>();
            view()
                .flex_1()
                .gap(px(tokens::SPACE_4))
                .role(AccessibilityRole::Dialog)
                .label(label.clone())
                .child(header)
                .child(
                    scroll()
                        .flex_1()
                        .min_h(px(0.0))
                        .child(body.as_ref().map(|b| b())),
                )
                .child((!buttons.is_empty()).then(|| {
                    view()
                        .flex_row()
                        .justify_end()
                        .gap(px(tokens::SPACE_2))
                        .pt(px(tokens::SPACE_3))
                        .border_t(px(tokens::BORDER))
                        .border_color(move || tokens::divider())
                        .children(buttons)
                }))
                .into_element()
        };
        nsh_base::sheet()
            .open(move || Some(open.get()))
            .side(side)
            .on_open_change(move |e: &SheetEvent| {
                if let SheetEvent::OpenChange(false, _) = e {
                    dismiss_event();
                }
            })
            .on_open(move || close_ref.focus())
            .backdrop_style(|s| s.bg(move || overlay_scrim()))
            .popup_style(move |s| {
                let s = s
                    .p(px(tokens::SPACE_4))
                    .bg(move || tokens::surface())
                    .border_color(move || tokens::border())
                    .text_color(move || tokens::foreground())
                    .text_size(px(tokens::TEXT_MD))
                    .rounded(px(tokens::RADIUS));
                match side {
                    Placement::Left => s
                        .w(px(size))
                        .max_w(relative(1.0))
                        .border_r(px(tokens::BORDER)),
                    Placement::Top => s
                        .h(px(size))
                        .max_h(relative(1.0))
                        .border_b(px(tokens::BORDER)),
                    Placement::Bottom => s
                        .h(px(size))
                        .max_h(relative(1.0))
                        .border_t(px(tokens::BORDER)),
                    Placement::Right => s
                        .w(px(size))
                        .max_w(relative(1.0))
                        .border_l(px(tokens::BORDER)),
                }
            })
            .child(dynamic(move || untracked(&content)))
            .into_element()
    }
}
